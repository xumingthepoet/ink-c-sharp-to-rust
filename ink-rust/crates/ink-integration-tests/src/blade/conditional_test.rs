#[cfg(test)]
mod conditional_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn iftrue_test() {
        let story = compile_string("{true: yes}\n");
        let mut s = story.clone();
        continue_all(&mut s);
        assert!(s.get_currentText().contains("yes"));
    }

    #[test]
    fn iffalse_test() {
        let story = compile_string("{false: yes}\n");
        let mut s = story.clone();
        continue_all(&mut s);
        assert!(!s.get_currentText().contains("yes"));
    }

    #[test]
    fn ifelse_test() {
        let story = compile_string("{true: yes|no}\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("yes"));
    }

    // From blade-ink-rs/conformance-tests/tests/conditional_test.rs

    #[test]
    fn ifelse_ext_test() {
        let story = compile_string(
            "
VAR x = 0
VAR y = 3
{ x > 0:
    ~ y = x - 1
- else:
    ~ y = x + 1
}
The value is {y}. -> END
",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("The value is -1."));
    }

    #[test]
    fn ifelse_ext_text1_test() {
        let story = compile_string(
            "
VAR x = 0
{
    - x == 0:
      This is text 1.
    - x > 0:
      This is text 2.
    - else:
      This is text 3.
}
+ [The Choice.] -> to_end
=== to_end
This is the end. -> END
",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("This is text 1."));
        assert_eq!(1, s.get_currentChoices().len());
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert_eq!(2, text.len());
        assert!(text.join("").contains("This is the end."));
    }

    #[test]
    fn ifelse_ext_text2_test() {
        let story = compile_string(
            "
VAR x = -2
{
    - x == 0:
      This is text 1.
    - x > 0:
      This is text 2.
    - else:
      This is text 3.
}
+ [The Choice.] -> to_end
=== to_end
This is the end. -> END
",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("This is text 3."));
        assert_eq!(1, s.get_currentChoices().len());
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert_eq!(2, text.len());
        assert!(text.join("").contains("This is the end."));
    }

    #[test]
    fn ifelse_ext_text3_test() {
        let story = compile_string(
            "
VAR x = -5
{ x > 0:
    ~ y = x - 1
- else:
    ~ y = x + 1
}
The value is {y}. -> END
",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("The value is -4."));
    }

    #[test]
    fn cond_text1_test() {
        let story = compile_string(
            "\"We are going on a trip,\" said Monsieur Fogg.\n* [The wager.] -> know_about_wager\n* [I was surprised.] -> i_stared\n\n=== know_about_wager\nI had heard about the wager.\n-> i_stared\n\n=== i_stared\nI stared at Monsieur Fogg.\n{ know_about_wager:\n    <> \"But surely you are not serious?\" I demanded.\n- else:\n    <> \"But there must be a reason for this trip,\" I observed.\n}\nHe said nothing in reply, merely considering his newspaper with as much thoroughness as entomologist considering his latest pinned addition.\n-> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("But surely you are not serious?"));
    }

    #[test]
    fn cond_text2_test() {
        let story = compile_string(
            "\"We are going on a trip,\" said Monsieur Fogg.\n* [The wager.] -> know_about_wager\n* [I was surprised.] -> i_stared\n\n=== know_about_wager\nI had heard about the wager.\n-> i_stared\n\n=== i_stared\nI stared at Monsieur Fogg.\n{ know_about_wager:\n    <> \"But surely you are not serious?\" I demanded.\n- else:\n    <> \"But there must be a reason for this trip,\" I observed.\n}\nHe said nothing in reply, merely considering his newspaper with as much thoroughness as entomologist considering his latest pinned addition.\n-> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(1);
        let text = continue_all(&mut s);
        assert!(text
            .join("")
            .contains("But there must be a reason for this trip"));
    }

    #[test]
    fn cond_opt1_test() {
        let story = compile_string(
            "I looked...\n* [at the door]\n  -> door_open\n* [outside]\n  -> leave\n\n=== door_open\nat the door. It was open.\n-> leave\n\n=== leave\nI stood up and...\n{ door_open:\n    *   I strode out of the compartment[] and I fancied I heard my master quietly tutting to himself. -> END\n- else:\n    *   I asked permission to leave[] and Monsieur Fogg looked surprised.    -> END\n    *   I stood and went to open the door[]. Monsieur Fogg seemed untroubled by this small rebellion. -> END\n}\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert_eq!(1, s.get_currentChoices().len());
    }

    #[test]
    fn cond_opt2_test() {
        let story = compile_string(
            "I looked...\n* [at the door]\n  -> door_open\n* [outside]\n  -> leave\n\n=== door_open\nat the door. It was open.\n-> leave\n\n=== leave\nI stood up and...\n{ door_open:\n    *   I strode out of the compartment[] and I fancied I heard my master quietly tutting to himself. -> END\n- else:\n    *   I asked permission to leave[] and Monsieur Fogg looked surprised.    -> END\n    *   I stood and went to open the door[]. Monsieur Fogg seemed untroubled by this small rebellion. -> END\n}\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(1);
        let text = continue_all(&mut s);
        assert_eq!(2, s.get_currentChoices().len());
    }

    #[test]
    fn stopping_test() {
        let story = compile_string(
            "-> test\n\n=== test\n{ stopping:\n    - I entered the casino.\n    - I entered the casino again.\n    - Once more, I went inside.\n}\n+ [Try again] -> test\n",
        );
        let mut s = story.clone();
        let text1 = continue_all(&mut s);
        assert!(text1.join("").contains("I entered the casino."));
        s.ChooseChoiceIndex(0);
        let text2 = continue_all(&mut s);
        assert!(text2.join("").contains("I entered the casino again."));
        s.ChooseChoiceIndex(0);
        let text3 = continue_all(&mut s);
        assert!(text3.join("").contains("Once more, I went inside."));
        s.ChooseChoiceIndex(0);
        let text4 = continue_all(&mut s);
        assert!(text4.join("").contains("Once more, I went inside."));
    }

    #[test]
    fn cycle_test() {
        let story = compile_string(
            "-> test\n\n=== test\n{ cycle:\n    - I held my breath.\n    - I waited impatiently.\n    - I paused.\n}\n+ [Try again] -> test\n",
        );
        let mut s = story.clone();
        let text1 = continue_all(&mut s);
        assert!(text1.join("").contains("I held my breath."));
        s.ChooseChoiceIndex(0);
        let text2 = continue_all(&mut s);
        assert!(text2.join("").contains("I waited impatiently."));
        s.ChooseChoiceIndex(0);
        let text3 = continue_all(&mut s);
        assert!(text3.join("").contains("I paused."));
        s.ChooseChoiceIndex(0);
        let text4 = continue_all(&mut s);
        assert!(text4.join("").contains("I held my breath."));
    }

    #[test]
    fn once_test() {
        let story = compile_string(
            "-> test\n\n=== test\n{ once:\n    - Would my luck hold?\n    - Could I win the hand?\n}\n+ [Try again] -> test\n",
        );
        let mut s = story.clone();
        let text1 = continue_all(&mut s);
        assert!(text1.join("").contains("Would my luck hold?"));
        s.ChooseChoiceIndex(0);
        let text2 = continue_all(&mut s);
        assert!(text2.join("").contains("Could I win the hand?"));
        s.ChooseChoiceIndex(0);
        let text3 = continue_all(&mut s);
        assert_eq!(0, text3.len());
        s.ChooseChoiceIndex(0);
        let text4 = continue_all(&mut s);
        assert_eq!(0, text4.len());
    }

    #[test]
    fn shuffle_test() {
        let story = compile_string(
            "-> test\n\n=== test\n{ shuffle:\n    - Ace of Hearts.\n    - King of Spades.\n    - 2 of Diamonds.\n}\n+ [Try again] -> test\n",
        );
        let mut s = story.clone();
        let text1 = continue_all(&mut s);
        assert_eq!(1, text1.len());
        s.ChooseChoiceIndex(0);
        let text2 = continue_all(&mut s);
        assert_eq!(1, text2.len());
        s.ChooseChoiceIndex(0);
        let text3 = continue_all(&mut s);
        assert_eq!(1, text3.len());
    }

    #[test]
    fn shuffle_stopping() {
        let story = compile_string(
            "-> test\n\n=== test\n{ stopping shuffle:\n    - one\n    - two\n    - final\n}\n+ [Try again] -> test\n",
        );
        let mut s = story.clone();
        let text1 = continue_all(&mut s);
        assert_eq!(1, text1.len());
        s.ChooseChoiceIndex(0);
        let text2 = continue_all(&mut s);
        assert_eq!(1, text2.len());
        s.ChooseChoiceIndex(0);
        let text3 = continue_all(&mut s);
        assert_eq!(1, text3.len());
        assert!(text3.join("").contains("final"));
    }

    #[test]
    fn shuffle_once() {
        let story = compile_string(
            "-> test\n\n=== test\n{ shuffle once:\n    - one\n    - two\n}\n+ [Try again] -> test\n",
        );
        let mut s = story.clone();
        let text1 = continue_all(&mut s);
        assert_eq!(1, text1.len());
        s.ChooseChoiceIndex(0);
        let text2 = continue_all(&mut s);
        assert_eq!(1, text2.len());
        s.ChooseChoiceIndex(0);
        let text3 = continue_all(&mut s);
        assert_eq!(0, text3.len());
    }

    #[test]
    fn multiline_test() {
        let story = compile_string(
            "-> test\n\n=== test\n{ stopping:\n    -   At the table, I drew a card. Ace of Hearts.\n    -   <> 2 of Diamonds.\n        \"Should I hit you again,\" the croupier asks.\n    -   <> King of Spades.\n    \"You lose,\" he crowed.\n}\n+ [Draw a card] I drew a card. -> test\n",
        );
        let mut s = story.clone();
        let text1 = continue_all(&mut s);
        assert!(text1.join("").contains("Ace of Hearts."));
        s.ChooseChoiceIndex(0);
        let text2 = continue_all(&mut s);
        assert!(text2.join("").contains("2 of Diamonds."));
        assert!(text2.join("").contains("Should I hit you again"));
        s.ChooseChoiceIndex(0);
        let text3 = continue_all(&mut s);
        assert!(text3.join("").contains("King of Spades."));
        assert!(text3.join("").contains("You lose"));
    }

    #[test]
    fn multiline_divert_test() {
        let story = compile_string(
            "->test\n\n=== test\n{ stopping:\n    -   At the table, I drew a card. Ace of Hearts.\n    -   <> 2 of Diamonds.\n        \"Should I hit you again,\" the croupier asks.\n    -   <> King of Spades.\n        -> he_crowed\n}\n+ [Draw a card] I drew a card. -> test\n\n== he_crowed\n\"You lose,\" he crowed.\n\n-> END\n",
        );
        let mut s = story.clone();
        let text1 = continue_all(&mut s);
        assert!(text1.join("").contains("Ace of Hearts."));
        s.ChooseChoiceIndex(0);
        let text2 = continue_all(&mut s);
        assert!(text2.join("").contains("2 of Diamonds."));
        s.ChooseChoiceIndex(0);
        let text3 = continue_all(&mut s);
        assert!(text3.join("").contains("King of Spades."));
        assert!(text3.join("").contains("You lose"));
    }

    #[test]
    fn multiline_choice_test() {
        let story = compile_string(
            "-> test\n=== test\n{ stopping:\n    -   At the table, I drew a card. Ace of Hearts.\n    -   2 of Diamonds.\n        \"Should I hit you again,\" the croupier asks.\n        * [No.] I left the table. -> END\n    -   King of Spades.\n        \"You lose,\" he crowed.\n        -> END\n}\n+ [Draw a card] I drew a card. -> test\n",
        );
        let mut s = story.clone();
        let text1 = continue_all(&mut s);
        assert!(text1.join("").contains("Ace of Hearts."));
        s.ChooseChoiceIndex(0);
        let text2 = continue_all(&mut s);
        assert_eq!(2, s.get_currentChoices().len());
        s.ChooseChoiceIndex(0);
        let text3 = continue_all(&mut s);
        assert!(text3.join("").contains("I left the table."));
    }
}
