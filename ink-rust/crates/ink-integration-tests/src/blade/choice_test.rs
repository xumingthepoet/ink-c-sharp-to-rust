#[cfg(test)]
mod choice_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn no_choice_test() {
        let story = compile_string("Hello world!\nHello back!\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert_eq!("Hello world!", text[0]);
        assert!(s.get_currentChoices().is_empty());
    }

    #[test]
    fn conditional_choice_test() {
        let story = compile_string("* {true} visible choice\n  -> END\n");
        let mut s = story.clone();
        continue_all(&mut s);
        assert_eq!(1, s.get_currentChoices().len());
    }

    // From blade-ink-rs/conformance-tests/tests/choice_test.rs

    #[test]
    fn one_test() {
        let story = compile_string("Hello world!\n*   Hello back!\n\n->DONE\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert_eq!(3, text.len());
        assert!(text.join("").contains("Hello world!"));
        assert!(text.join("").contains("Hello back!"));
    }

    #[test]
    fn multi_choice_test() {
        // Choice 0: Hello back!
        let story = compile_string(
            "Hello, world!\n* Hello back!\n  Nice to hear from you\n* Goodbye\n  See you later\n- \n-> END\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert_eq!(2, s.get_currentChoices().len());
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Hello back!"));
        assert!(text.join("").contains("Nice to hear from you"));
    }

    #[test]
    fn single_choice1_test() {
        let story = compile_string("Hello world!\n*   Hello back!\n\n->DONE\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert_eq!(1, text.len());
        assert_eq!("Hello world!", text[0]);
    }

    #[test]
    fn single_choic2_test() {
        let story = compile_string("Hello world!\n*   Hello back!\n\n->DONE\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert_eq!(1, text.len());
        assert_eq!("Hello, world!", text[0]);
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert_eq!(2, text.len());
        assert_eq!("Hello back!", text[0]);
        assert_eq!("Nice to hear from you", text[1]);
    }

    #[test]
    fn suppress_choice_test() {
        let story = compile_string("Line.\n");
        let mut s = story.clone();
        continue_all(&mut s);
        assert!(s.get_currentChoices().is_empty());
    }

    #[test]
    fn mixed_choice_test() {
        let story = compile_string(
            "Hello world!\n*   Hello [back!] right back to you!\n            Nice to hear from you.\n\n->DONE\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        assert_eq!("Hello back!", s.get_currentChoices()[0].text);
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert_eq!(2, text.len());
        assert_eq!("Hello right back to you!", text[0]);
        assert_eq!("Nice to hear from you.", text[1]);
    }

    #[test]
    fn varying_choice_test() {
        let story = compile_string(
            "-> find_help\n\n=== find_help ===\n\nYou search desperately for a friendly face in the crowd.\n*   The woman in the hat[?] pushes you roughly aside. -> find_help\n*   The man with the briefcase[?] looks disgusted as you stumble past him. -> find_help\n\n->DONE\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        assert_eq!(2, s.get_currentChoices().len());
        s.ChooseChoiceIndex(0);
        continue_all(&mut s);
        assert_eq!(1, s.get_currentChoices().len());
        assert_eq!(
            "The man with the briefcase?",
            s.get_currentChoices()[0].text
        );
    }

    #[test]
    fn sticky_choice_test() {
        let story = compile_string(
            "VAR x = 2\n\n{ x:\n    - 1: one\n    - 2: two\n}\n* sticky\n  -> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        assert_eq!(2, s.get_currentChoices().len());
        s.ChooseChoiceIndex(0);
        continue_all(&mut s);
        assert_eq!(2, s.get_currentChoices().len());
    }

    #[test]
    fn fallback_choice_test() {
        let story = compile_string(
            "VAR x = 2\n\n{ x:\n    - 1: one\n    - 2: two\n}\n* { true } option1\n* { false } option2\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        assert_eq!(2, s.get_currentChoices().len());
    }

    #[test]
    fn fallback_choice2_test() {
        let story = compile_string(
            "VAR x = 2\n\n{ x:\n    - 1: one\n    - 2: two\n}\n* { true } option1\n* { false } option2\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        assert_eq!(2, s.get_currentChoices().len());
        s.ChooseChoiceIndex(0);
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        continue_all(&mut s);
        assert!(!s.get_canContinue());
    }

    #[test]
    fn label_flow_test() {
        let story = compile_string(
            "-> meet_guard\n\n=== meet_guard ===\nThe guard frowns at you.\n*   (greet) [Greet him]\n    'Greetings.'\n*   (get_out) 'Get out of my way[.'],' you tell the guard.\n-   'Hmm,' replies the guard.\n*   {greet}     'Having a nice day?'\n*   'Hmm?'[] you reply.\n*   {get_out} [Shove him aside]\n    You shove him sharply. He stares in reply, and draws his sword!\n    -> END\n-   'Mff,' the guard replies, and then offers you a paper bag. 'Toffee?'\n    -> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        assert_eq!(2, s.get_currentChoices().len());
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert_eq!(2, s.get_currentChoices().len());
        assert_eq!("'Having a nice day?'", s.get_currentChoices()[0].text);
    }

    #[test]
    fn label_flow2_test() {
        let story = compile_string(
            "-> meet_guard\n\n=== meet_guard ===\nThe guard frowns at you.\n*   (greet) [Greet him]\n    'Greetings.'\n*   (get_out) 'Get out of my way[.'],' you tell the guard.\n-   'Hmm,' replies the guard.\n*   {greet}     'Having a nice day?'\n*   'Hmm?'[] you reply.\n*   {get_out} [Shove him aside]\n    You shove him sharply. He stares in reply, and draws his sword!\n    -> END\n-   'Mff,' the guard replies, and then offers you a paper bag. 'Toffee?'\n    -> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        assert_eq!(2, s.get_currentChoices().len());
        s.ChooseChoiceIndex(1);
        let text = continue_all(&mut s);
        assert_eq!(2, s.get_currentChoices().len());
        assert_eq!("Shove him aside", s.get_currentChoices()[1].text);
    }

    #[test]
    fn label_scope_test() {
        let story = compile_string(
            "-> knot\n\n=== knot ===\n= stitch_one\n* an option\n- (gatherpoint) Some content.\n  -> knot.stitch_two\n= stitch_two\n* {knot.stitch_one.gatherpoint} Found gatherpoint\n-> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert_eq!(1, s.get_currentChoices().len());
        assert_eq!("Found gatherpoint", s.get_currentChoices()[0].text);
    }

    #[test]
    fn divert_choice_test() {
        let story = compile_string(
            "-> find_help\n=== find_help ===\n\nYou search desperately for a friendly face in the crowd.\n*   The woman in the hat[?] pushes you roughly aside. -> find_help\n*   The man with the briefcase[?] looks disgusted as you stumble past him. -> find_help\n*   ->\n    - But it is too late: you collapse onto the station platform. This is the end.\n    -> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        assert_eq!(2, s.get_currentChoices().len());
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert_eq!(2, text.len());
    }
}
