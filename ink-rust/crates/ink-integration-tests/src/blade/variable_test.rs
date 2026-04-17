#[cfg(test)]
mod variable_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn variable_declaration_test() {
        let story = compile_string("VAR x = 5\n{x}\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("5"));
    }

    #[test]
    fn var_calc_test() {
        let story = compile_string("VAR x = 5\n~ x = x + 3\n{x}\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("8"));
    }

    #[test]
    fn var_divert_test() {
        let story = compile_string(
            "VAR target = -> destination\n-> target\n== destination ==\narrived\n-> END\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("arrived"));
    }

    // From blade-ink-rs/conformance-tests/tests/variable_test.rs

    #[test]
    fn var_string_ink_bug_test() {
        let story =
            compile_string("VAR str = \"hello\"\n{str}\n~ str = \"world\"\n{str}\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("hello"));
        assert!(text.join("").contains("world"));
    }
}
