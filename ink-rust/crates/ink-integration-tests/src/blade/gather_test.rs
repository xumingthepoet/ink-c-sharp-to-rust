#[cfg(test)]
mod gather_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn gather_basic_test() {
        let story = compile_string("- (point)\ncontent\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("content"));
    }

    #[test]
    fn gather_chain_test() {
        let story = compile_string("- first\n- second\n- third\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("first"));
        assert!(text.join("").contains("second"));
        assert!(text.join("").contains("third"));
    }

    // From blade-ink-rs/conformance-tests/tests/gather_test.rs

    #[test]
    fn nested_flow_test() {
        let story = compile_string(
            "Well, Poirot? Murder or suicide?\"\n*   \"Murder!\"\n    \"And who did it?\"\n    * *     \"Detective-Inspector Japp!\"\n    * *     \"Captain Hastings!\"\n    * *     \"Myself!\"\n*   \"Suicide!\"\n-   Mrs. Christie lowered her manuscript a moment. The rest of the writing group sat, open-mouthed.\n-> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        continue_all(&mut s);
        s.ChooseChoiceIndex(2);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Myself!"));
        assert!(text
            .join("")
            .contains("Mrs. Christie lowered her manuscript"));
    }

    #[test]
    fn deep_nesting_test() {
        let story = compile_string(
            "Tell us a tale, Captain!\"\n*   \"Very well, you sea-dogs. Here's a tale...\"\n    * *     \"It was a dark and stormy night...\"\n    * * *   \"...and the crew were restless...\"\n    * * * *  \"... and they said to their Captain...\"\n    * * * * *       \"...Tell us a tale Captain!\"\n*   \"No, it's past your bed-time.\"\n-   To a man, the crew began to yawn.\n-> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Tell us a tale Captain"));
        assert!(text.join("").contains("To a man, the crew began to yawn"));
    }

    #[test]
    fn complex_flow1_test() {
        let story = compile_string(
            "I looked at Monsieur Fogg\n*   ... and I could contain myself no longer.\n   'What is the purpose of our journey, Monsieur?'\n   'A wager,' he replied.\n   * *     'A wager!'[] I returned.\n       He nodded.\n       * * *   'But surely that is foolishness!'\n       * * *  'A most serious matter then!'\n       - - -   He nodded again.\n       * * *   'But can we win?'\n           'That is what we will endeavour to find out,' he answered.\n       * * *   'A modest wager, I trust?'\n           'Twenty thousand pounds,' he replied, quite flatly.\n       * * *   I asked nothing further of him then[.], and after a final, polite cough, he offered nothing more to me. <>\n   * *     'Ah[.'],' I replied, uncertain what I thought.\n   - -     After that, <>\n*   ... but I said nothing[] and <>\n- we passed the day in silence.\n- -> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(1);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("we passed the day in silence"));
    }

    #[test]
    fn complex_flow2_test() {
        let story = compile_string(
            "I looked at Monsieur Fogg\n*   ... and I could contain myself no longer.\n   'What is the purpose of our journey, Monsieur?'\n   'A wager,' he replied.\n   * *     'A wager!'[] I returned.\n       He nodded.\n       * * *   'But surely that is foolishness!'\n       * * *  'A most serious matter then!'\n       - - -   He nodded again.\n       * * *   'But can we win?'\n           'That is what we will endeavour to find out,' he answered.\n       * * *   'A modest wager, I trust?'\n           'Twenty thousand pounds,' he replied, quite flatly.\n       * * *   I asked nothing further of him then[.], and after a final, polite cough, he offered nothing more to me. <>\n   * *     'Ah[.'],' I replied, uncertain what I thought.\n   - -     After that, <>\n*   ... but I said nothing[] and <>\n- we passed the day in silence.\n- -> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("What is the purpose of our journey"));
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("A wager!"));
        s.ChooseChoiceIndex(1);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("After that"));
        s.ChooseChoiceIndex(1);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("we passed the day in silence"));
    }
}
