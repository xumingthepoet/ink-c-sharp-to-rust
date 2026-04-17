#[cfg(test)]
mod misc_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn operations_test() {
        let story = compile_string("{ 1 + 2 }\n{ 5 - 3 }\n{ 4 * 2 }\n{ 10 / 2 }\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("3"));
        assert!(text.join("").contains("2"));
        assert!(text.join("").contains("8"));
        assert!(text.join("").contains("5"));
    }

    #[test]
    fn read_counts_test() {
        let story =
            compile_string("-> target\n{ target }\n-> END\n\n== target ==\nvisited\n->->\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("1"));
    }

    #[test]
    fn turns_since_test() {
        let story = compile_string("{ TURNS_SINCE(-> marker) }\n-> marker\n{ TURNS_SINCE(-> marker) }\n-> END\n\n== marker ==\n~ return\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("-1"));
        assert!(text.join("").contains("0"));
    }

    // From blade-ink-rs/conformance-tests/tests/misc_test.rs

    #[test]
    fn issue15_test() {
        // Issue 15: set variable during story continue
        let story =
            compile_string("This is a test\n~ x = 1\nSET_X:\n{ x:\n    - 1: X is set\n}\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("This is a test"));
        assert!(text.join("").contains("X is set"));
    }

    #[test]
    fn newlines_with_string_eval_test() {
        let story = compile_string("A\n{true:\n    B\n}\n{false:\n    C\n}\nA\n{1+2}\nB\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("A"));
        assert!(text.join("").contains("B"));
        assert!(text.join("").contains("3"));
    }

    #[test]
    fn i18n() {
        let story = compile_string("#áé\nHello\n#你好\n世界\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        // Tags with non-ASCII characters
        let tags = s.get_currentTags();
        assert!(tags.len() >= 1);
    }
}
