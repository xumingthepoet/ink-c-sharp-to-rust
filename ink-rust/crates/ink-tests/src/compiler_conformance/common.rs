#![allow(dead_code)]

use crate::compiler_conformance::api::Story;
use std::{fs, path::Path};

use rand::Rng;

pub fn next_all(story: &mut Story, text: &mut Vec<String>) {
    while story.can_continue() {
        let line = story.cont();
        print!("{line}");

        if !line.trim().is_empty() {
            text.push(line.trim().to_string());
        }
    }

    if !story.get_current_errors().is_empty() {
        panic!("{}", join_text(&story.get_current_errors()));
    }
}

pub fn join_text(text: &[String]) -> String {
    let mut sb = String::new();
    for s in text {
        sb.push_str(s);
    }
    sb
}

pub fn run_story(
    filename: &str,
    choice_list: Option<Vec<usize>>,
    errors: &mut Vec<String>,
) -> Vec<String> {
    let mut story = compile_story(filename);
    let mut text = Vec::new();
    let mut choice_list_index = 0;
    let mut rng = rand::rng();

    while story.can_continue() || !story.get_current_choices().is_empty() {
        println!("{}", story.build_string_of_hierarchy());

        while story.can_continue() {
            let line = story.cont();
            print!("{}", line);
            text.push(line);
        }

        if !story.get_current_errors().is_empty() {
            for error_msg in story.get_current_errors() {
                println!("{}", error_msg);
                errors.push(error_msg.to_string());
            }
        }

        let current_choices = story.get_current_choices();
        if !current_choices.is_empty() {
            let len = current_choices.len();

            for choice in current_choices {
                println!("{}", choice.text);
                text.push(format!("{}\n", choice.text));
            }

            if let Some(choice_list) = &choice_list {
                if choice_list_index < choice_list.len() {
                    story.choose_choice_index(choice_list[choice_list_index]);
                    choice_list_index += 1;
                } else {
                    let random_choice_index = rng.random_range(0..len);
                    story.choose_choice_index(random_choice_index);
                }
            } else {
                let random_choice_index = rng.random_range(0..len);
                story.choose_choice_index(random_choice_index);
            }
        }
    }

    text
}

pub fn compile_story(filename: &str) -> Story {
    let source = get_ink_string(filename);
    let mut story = Story::new(&source);
    assert_compiled_json_matches_fixture(filename, &mut story);
    story
}

pub fn get_ink_string(filename: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/conformance")
        .join(filename);
    fs::read_to_string(path).expect("fixture ink must exist")
}

fn get_fixture_json_string(filename: &str) -> String {
    let filename = if let Some(prefix) = filename.strip_suffix(".ink") {
        format!("{prefix}.ink.json")
    } else {
        format!("{filename}.ink.json")
    };
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/conformance")
        .join(filename);
    fs::read_to_string(path).expect("fixture json must exist")
}

pub fn assert_compiled_json_matches_fixture(filename: &str, story: &mut Story) {
    let generated_json = story.to_json();
    let fixture_json = get_fixture_json_string(filename);
    if generated_json != fixture_json {
        panic!(
            "{}",
            format_compiled_json_mismatch(filename, &generated_json, &fixture_json)
        );
    }
}

fn format_compiled_json_mismatch(
    filename: &str,
    generated_json: &str,
    fixture_json: &str,
) -> String {
    let fixture_path = if let Some(prefix) = filename.strip_suffix(".ink") {
        format!("fixtures/conformance/{prefix}.ink.json")
    } else {
        format!("fixtures/conformance/{filename}.ink.json")
    };

    format!(
        "compiled json differs for {filename}\n--- generated ---\n{generated_json}\n--- fixture ({fixture_path}) ---\n{fixture_json}"
    )
}

pub fn is_ended(story: &Story) -> bool {
    let mut story = story.clone();
    !story.can_continue() && story.get_current_choices().is_empty()
}
