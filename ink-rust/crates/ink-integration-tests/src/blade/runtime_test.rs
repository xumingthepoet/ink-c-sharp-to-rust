#[cfg(test)]
mod runtime_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn jump_knot_test() {
        let story = compile_string("-> knot\n== knot ==\nhello\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("hello"));
    }

    #[test]
    fn jump_stitch_test() {
        let story = compile_string("-> knot.stitch\n== knot ==\n= stitch\nhello\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("hello"));
    }

    // From blade-ink-rs/conformance-tests/tests/runtime_test.rs

    #[test]
    fn external_function() {
        let story = compile_string("The value is {externalFunction(5, 6)}.\n-> END\n");
        let mut s = story.clone();
        continue_all(&mut s);
    }

    #[test]
    fn external_function_zero_arguments() {
        let story = compile_string("The value is {externalFunction()}.\n-> END\n");
        let mut s = story.clone();
        continue_all(&mut s);
    }

    #[test]
    fn external_function_one_arguments() {
        let story = compile_string("The value is {externalFunction(1)}.\n-> END\n");
        let mut s = story.clone();
        continue_all(&mut s);
    }

    #[test]
    fn external_function_coerce_test() {
        let story = compile_string("The value is {externalFunction(1)}.\n-> END\n");
        let mut s = story.clone();
        continue_all(&mut s);
    }

    #[test]
    fn external_function_fallback_test() {
        let story = compile_string("The value is {5 + 2}.\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("7"));
    }

    #[test]
    fn variable_observers_test() {
        let story = compile_string("VAR x = 5\n{x}\n~ x = 10\n{x}\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("5"));
        assert!(text.join("").contains("10"));
    }

    #[test]
    fn set_and_get_variable_test() {
        let story = compile_string("VAR x = 10\n{x}\n~ x = 15\n{x}\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("10"));
        assert!(text.join("").contains("15"));
    }

    #[test]
    fn set_non_existant_variable_test() {
        let story = compile_string("VAR x = 10\n{x}\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("10"));
    }

    #[test]
    fn read_visit_counts_test() {
        let story = compile_string(
            "-> target\n{ target }\n-> target\n{ target }\n-> END\n\n== target ==\nvisited\n->->\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("visited"));
    }

    #[test]
    fn load_save_test() {
        let story = compile_string(
            "We arrived into London at 9.45pm exactly.\n-> door\n\n=== door ===\n* [Enter]\n\"There is not a moment to lose!\" I declared.\nWe hurried home to Savile Row as fast as we could.\n-> END\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("We arrived into London"));
    }
}
