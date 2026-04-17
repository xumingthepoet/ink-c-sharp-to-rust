#[cfg(test)]
mod multi_flow_tests {
    use crate::blade::common::{compile_string, continue_all};

    // From blade-ink-rs/conformance-tests/tests/multi_flow_test.rs

    #[test]
    fn basics_test() {
        let story = compile_string(
            "=== knot1\nknot 1 line 1\nknot 1 line 2\n-> END\n\n=== knot2\nknot 2 line 1\nknot 2 line 2\n-> END\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("knot 1 line 1"));
    }

    #[test]
    fn multiflow_save_load_threads() {
        let story = compile_string(
            "Default line 1\n\n=== blue ===\nHello I'm blue\n* Thread 1 blue choice\n    -> after_blue\n* Thread 2 blue choice\n    -> after_blue\n\n=== red ===\nHello I'm red\n* Thread 1 red choice\n    -> after_red\n* Thread 2 red choice\n    -> after_red\n\n=== after_blue ===\nAfter thread 1 choice (blue)\n-> END\n\n=== after_red ===\nAfter thread 2 choice (red)\n-> END\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Default line 1"));
    }
}
