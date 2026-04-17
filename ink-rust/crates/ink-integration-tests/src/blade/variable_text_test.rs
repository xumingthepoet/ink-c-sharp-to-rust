#[cfg(test)]
mod variable_text_tests {
    use crate::blade::common::{compile_string, continue_all};

    // From blade-ink-rs/conformance-tests/tests/variable_text_test.rs

    #[test]
    fn sequence_test() {
        let story = compile_string(
            "-> test\n\n=== test\nThe radio hissed into life. {{\"Three!\"|\"Two!\"|\"One!\"|There was the white noise racket of an explosion.}}\n+ [Again] -> test\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Three!"));
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Two!"));
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("One!"));
        s.ChooseChoiceIndex(0);
        continue_all(&mut s);
        // Stops at last element
        s.ChooseChoiceIndex(0);
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        continue_all(&mut s);
    }

    #[test]
    fn cycle_test() {
        let story = compile_string(
            "-> test\n\n=== test\nThe radio hissed into life. {{&\"Three!\"|\"Two!\"|\"One!\"}}\n+ [Again] -> test\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Three!"));
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Two!"));
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("One!"));
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Three!"));
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Two!"));
    }

    #[test]
    fn once_test() {
        // {!"Three!"|"Two!"|"One!"} = once (show each element once, then stop)
        let story = compile_string(
            "-> test\n\n=== test\nThe radio hissed into life. {{!\"Three!\"|\"Two!\"|\"One!\"}}\n+ [Again] -> test\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Three!"));
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Two!"));
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("One!"));
        s.ChooseChoiceIndex(0);
        continue_all(&mut s);
        // Once done: shows empty
        s.ChooseChoiceIndex(0);
        continue_all(&mut s);
    }

    #[test]
    fn empty_elements_test() {
        let story = compile_string(
            "-> test\n\n=== test\nThe radio hissed into life. {{||\"One!\"}}\n+ [Again] -> test\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("The radio hissed into life."));
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("The radio hissed into life."));
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("One!"));
    }

    #[test]
    fn list_in_choice_test() {
        let story = compile_string(
            "-> start\n\n=== start ===\nHe looked at me oddly.\n+ [\"Hello, {{&Master|Monsieur|you}}!\"] -> start\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        let choices = s.get_currentChoices();
        assert_eq!(1, choices.len());
        // First time: Master
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        let choices = s.get_currentChoices();
        assert_eq!(1, choices.len());
        // Second time: Monsieur
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        let choices = s.get_currentChoices();
        assert_eq!(1, choices.len());
        // Third time: you
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        let choices = s.get_currentChoices();
        assert_eq!(1, choices.len());
    }
}
