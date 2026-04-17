#[cfg(test)]
mod stitch_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn auto_stitch_test() {
        let story = compile_string("== knot =\ncontent\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("content"));
    }

    #[test]
    fn manual_stitch_test() {
        let story = compile_string("-> knot.stitch\n== knot ==\n= stitch\ncontent\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("content"));
    }

    // From blade-ink-rs/conformance-tests/tests/stitch_test.rs

    #[test]
    fn auto_stitch2_test() {
        let story = compile_string(
            "-> the_orient_express\n\n=== the_orient_express ===\n\n= in_first_class\n    I settled my master.\n    *  [Move to third class]\n        -> in_third_class\n    *  [Are you sure] -> the_orient_express\n\n= in_third_class\n    I put myself in third.\n    -> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(1);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("I settled my master."));
    }

    #[test]
    fn manual_stitch2_test() {
        let story = compile_string(
            "-> the_orient_express\n\n=== the_orient_express ===\nHow shall we travel?\n* [In first class] -> in_first_class\n* [I'll go cheap] -> the_orient_express.in_third_class\n\n= in_first_class\n    I settled my master.\n    *   [Move to third class]\n        -> in_third_class\n\n= in_third_class\n    I put myself in third.\n    -> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        assert_eq!("How shall we travel?", s.get_currentChoices()[0].text);
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("I settled my master."));
    }
}
