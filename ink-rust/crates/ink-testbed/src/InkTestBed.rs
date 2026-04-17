//! InkTestBed is a lightweight interactive harness for runtime/debugging.
//!
//! The original C# testbed is compiler-centric. The Rust port currently
//! provides the runtime-facing play loop, JSON roundtrip, and file utilities
//! that are already usable. Source compilation remains blocked on the
//! compiler front-end port, so the compiler entry points return explicit
//! errors instead of faking success.

use ink_runtime::Error::{ErrorHandler, ErrorType};
use ink_runtime::Story::Story;
use ink_runtime::StoryException::StoryException;
use std::cell::RefCell;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

#[derive(Default)]
pub struct InkTestBed {
    pub story: Option<Story>,
}

impl InkTestBed {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_story(mut story: Story) -> Self {
        let mut testbed = Self::new();
        testbed.attach_error_handler(&mut story);
        testbed.story = Some(story);
        testbed
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.play()
    }

    pub fn play(&mut self) -> Result<(), String> {
        loop {
            let can_continue = self
                .story
                .as_mut()
                .ok_or_else(|| "InkTestBed requires a loaded story".to_string())?
                .get_canContinue();
            let has_choices = self
                .story
                .as_mut()
                .ok_or_else(|| "InkTestBed requires a loaded story".to_string())?
                .get_currentChoices()
                .len()
                > 0;

            if !can_continue && !has_choices {
                break;
            }

            if can_continue {
                self.continue_maximally()?;
            }

            if has_choices {
                self.player_choice()?;
            }
        }

        Ok(())
    }

    pub fn continue_once(&mut self) -> Result<(), String> {
        let output = {
            let story = self
                .story
                .as_mut()
                .ok_or_else(|| "InkTestBed requires a loaded story".to_string())?;
            story.Continue()
        };

        print!("{}", output);
        io::stdout().flush().map_err(|err| err.to_string())?;
        self.print_choices_if_necessary()?;
        Ok(())
    }

    pub fn continue_maximally(&mut self) -> Result<(), String> {
        loop {
            let can_continue = {
                let story = self
                    .story
                    .as_mut()
                    .ok_or_else(|| "InkTestBed requires a loaded story".to_string())?;
                story.get_canContinue()
            };

            if !can_continue {
                break;
            }

            let (output, tags) = {
                let story = self
                    .story
                    .as_mut()
                    .ok_or_else(|| "InkTestBed requires a loaded story".to_string())?;
                (story.Continue(), story.get_currentTags())
            };

            print!("{}", output);
            for tag in tags {
                println!("# {}", tag);
            }
        }

        io::stdout().flush().map_err(|err| err.to_string())?;
        self.print_choices_if_necessary()?;
        Ok(())
    }

    pub fn choose(&mut self, choiceIdx: i32) -> Result<(), String> {
        let story = self
            .story
            .as_mut()
            .ok_or_else(|| "InkTestBed requires a loaded story".to_string())?;

        story.ChooseChoiceIndex(choiceIdx);
        Ok(())
    }

    pub fn player_choice(&mut self) -> Result<(), String> {
        loop {
            print!(">>> ");
            io::stdout().flush().map_err(|err| err.to_string())?;

            let mut user_input = String::new();
            let read = io::stdin()
                .read_line(&mut user_input)
                .map_err(|err| err.to_string())?;
            if read == 0 {
                return Err("<User input stream closed.>".to_string());
            }

            let choice_num: i32 = match user_input.trim().parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Not a number");
                    continue;
                }
            };

            let choice_index = choice_num - 1;
            let Some(story) = self.story.as_mut() else {
                return Err("InkTestBed requires a loaded story".to_string());
            };

            let current_choices = story.get_currentChoices();
            if choice_index >= 0 && (choice_index as usize) < current_choices.len() {
                story.ChooseChoiceIndex(choice_index);
                return Ok(());
            } else {
                println!("Choice out of range");
            }
        }
    }

    pub fn load_story_json_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let json = fs::read_to_string(path.as_ref()).map_err(|err| err.to_string())?;
        self.load_story_json(json)
    }

    pub fn load_story_json(&mut self, json: String) -> Result<(), String> {
        let mut story = Story::new_overload_2(json);
        self.attach_error_handler(&mut story);
        self.story = Some(story);
        Ok(())
    }

    pub fn json_roundtrip(&mut self) -> Result<(), String> {
        let story = self
            .story
            .as_mut()
            .ok_or_else(|| "InkTestBed requires a loaded story".to_string())?;

        let json_str = story.ToJson();
        println!("{}", json_str);
        println!("---------------------------------------------------");

        let mut reloaded_story = Story::new_overload_2(json_str);
        self.attach_error_handler(&mut reloaded_story);
        let new_json_str = reloaded_story.ToJson();
        println!("{}", new_json_str);
        self.story = Some(reloaded_story);
        Ok(())
    }

    pub fn split_file<P: AsRef<Path>>(path: P) -> Result<(String, String), String> {
        const SPLIT_STR: &str = "------ SECOND INK VERSION ------";
        let full_source = fs::read_to_string(path.as_ref()).map_err(|err| err.to_string())?;
        let Some(idx) = full_source.find(SPLIT_STR) else {
            return Err(format!(
                "Split point not found in {}",
                path.as_ref().display()
            ));
        };

        let ink1 = full_source[..idx].to_string();
        let ink2 = full_source[idx + SPLIT_STR.len()..].to_string();
        Ok((ink1, ink2))
    }

    pub fn ink_changing_test(
        &mut self,
        _test1: fn(&mut Self),
        _test2: fn(&mut Self),
    ) -> Result<(), String> {
        Err("InkChangingTest still depends on the unported compiler front-end".to_string())
    }

    pub fn simple_diff(s1: &str, s2: &str) {
        if s1 == s2 {
            println!("Identical!");
            return;
        }

        let mut found_diff = false;
        for (i, (a, b)) in s1.chars().zip(s2.chars()).enumerate() {
            if a != b {
                found_diff = true;
                let diff_i = i.saturating_sub(10);
                let s1_slice: String = s1.chars().skip(diff_i).take(40).collect();
                let s2_slice: String = s2.chars().skip(diff_i).take(40).collect();
                println!(
                    "Difference at idx {}: \n\t{}\nv.s.\n\t{}",
                    i, s1_slice, s2_slice
                );
                break;
            }
        }

        if !found_diff {
            let start_of_extension = s1.chars().count().min(s2.chars().count());
            let longer_text = if s1.chars().count() > s2.chars().count() {
                s1
            } else {
                s2
            };
            let extension: String = longer_text.chars().skip(start_of_extension).collect();
            println!(
                "Difference in length: {} v.s. {}. Extended: {}",
                s1.chars().count(),
                s2.chars().count(),
                extension
            );
        }
    }

    pub fn millisecs<F>(mut action: F, times: i32, ignoreWarmupTimes: i32) -> f32
    where
        F: FnMut(),
    {
        let real_times = times - ignoreWarmupTimes;

        if times == 1 && ignoreWarmupTimes == 0 {
            let start = Instant::now();
            action();
            return start.elapsed().as_secs_f32() * 1000.0;
        }

        for _ in 0..ignoreWarmupTimes.max(0) {
            action();
        }

        let start = Instant::now();
        for _ in 0..real_times.max(0) {
            action();
        }

        let millisecs = start.elapsed().as_secs_f32() * 1000.0;
        if real_times <= 0 {
            0.0
        } else {
            millisecs / real_times as f32
        }
    }

    pub fn create_compiler(&self, _filename: Option<&Path>) -> Result<(), String> {
        Err(
            "InkTestBed::CreateCompiler still depends on the unported compiler front-end"
                .to_string(),
        )
    }

    pub fn compile(&mut self, _inkSource: String) -> Result<(), String> {
        Err("InkTestBed::Compile still depends on the unported compiler front-end".to_string())
    }

    pub fn compile_file(&mut self, _filename: Option<&Path>) -> Result<(), String> {
        Err("InkTestBed::CompileFile still depends on the unported compiler front-end".to_string())
    }

    fn print_choices_if_necessary(&mut self) -> Result<(), String> {
        let Some(story) = self.story.as_mut() else {
            return Ok(());
        };

        if !story.get_canContinue() {
            let current_choices = story.get_currentChoices();
            if !current_choices.is_empty() {
                for (index, choice) in current_choices.iter().enumerate() {
                    let mut choice_tags = String::new();
                    if !choice.tags.is_empty() {
                        choice_tags.push_str(" (CHOICE TAGS: ");
                        for tag in &choice.tags {
                            choice_tags.push_str("# ");
                            choice_tags.push_str(tag);
                        }
                        choice_tags.push(')');
                    }
                    println!(" {}) {}{}", index + 1, choice.text, choice_tags);
                }
            }
        }

        Ok(())
    }

    fn attach_error_handler(&self, story: &mut Story) {
        let handler: Rc<RefCell<ErrorHandler>> =
            Rc::new(RefCell::new(Box::new(|message, error_type| {
                let label = match error_type {
                    ErrorType::Warning => "Warning",
                    ErrorType::Author => "Author",
                    ErrorType::Error => "Error",
                };

                eprintln!("{}: {}", label, message);
                panic!(
                    "{}",
                    StoryException::new_overload_2(format!("{}: {}", label, message))
                );
            })));

        story.on_error = Some(handler);
    }
}

#[cfg(test)]
mod tests {
    use super::InkTestBed;
    use ink_runtime::Container::Container;
    use ink_runtime::Story::Story;
    use std::fs;

    #[test]
    fn split_file_separates_versions() {
        let dir = std::env::temp_dir();
        let path = dir.join("ink_testbed_split_file.ink");
        fs::write(
            &path,
            "first half\n------ SECOND INK VERSION ------\nsecond half\n",
        )
        .unwrap();

        let (first, second) = InkTestBed::split_file(&path).unwrap();
        assert_eq!(first, "first half\n");
        assert_eq!(second, "\nsecond half\n");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn json_roundtrip_empty_story_preserves_story_shape() {
        let story = Story::new(Container::new(), Vec::new());
        let mut bed = InkTestBed::with_story(story);

        let before = bed.story.as_mut().unwrap().ToJson();
        bed.json_roundtrip().unwrap();
        let after = bed.story.as_mut().unwrap().ToJson();

        assert_eq!(before, after);
    }
}
