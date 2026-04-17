#[cfg(test)]
mod tag_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn tags_test() {
        let mut story = compile_string("# global_tag\n# another_tag\nHello\n-> END\n");
        let tags = story.get_globalTags();
        assert!(tags.iter().any(|t: &String| t.contains("global_tag")));
        assert!(tags.iter().any(|t: &String| t.contains("another_tag")));
    }

    #[test]
    fn tags_in_seq_test() {
        let story = compile_string("-> knot\n== knot\n{red #red|blue #blue} text\n-> END\n");
        let mut s = story.clone();
        continue_all(&mut s);
        let tags = s.get_currentTags();
        assert!(tags.iter().any(|t| t == "red" || t == "blue"));
    }

    #[test]
    fn tags_dynamic_content_test() {
        let story = compile_string("tag # pic{1+1}.jpg\n");
        let mut s = story.clone();
        continue_all(&mut s);
        let tags = s.get_currentTags();
        assert!(tags.iter().any(|t: &String| t.contains("pic2.jpg")));
    }

    // From blade-ink-rs/conformance-tests/tests/tag_test.rs

    #[test]
    fn tags_in_choice_test() {
        let story = compile_string("+ one #one [two #two] three #three -> END\n");
        let mut s = story.clone();
        continue_all(&mut s);
        let choices = s.get_currentChoices();
        assert_eq!(1, choices.len());
    }

    #[test]
    fn tags_in_choice_dynamic_content_test() {
        let story = compile_string("# {x}tag\nContent\n-> END\n");
        let mut s = story.clone();
        continue_all(&mut s);
    }
}
