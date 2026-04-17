#[cfg(test)]
mod glue_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn simple_glue_test() {
        let story = compile_string("Hello <> world\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Hello world"));
    }

    // From blade-ink-rs/conformance-tests/tests/glue_test.rs

    #[test]
    fn glue_with_divert_test() {
        let story = compile_string(
            "
We hurried home <>
-> to_savile_row

=== to_savile_row ===
to Savile Row
-> as_fast_as_we_could

=== as_fast_as_we_could ===
<> as fast as we could.
-> END
",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("We hurried home to Savile Row"));
    }

    #[test]
    fn has_left_right_glue_matching_test() {
        let story =
            compile_string("A {f():B}\nX\n\n=== function f() ===\n{true:\n    ~ return false\n}\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert_eq!(2, text.len());
        assert_eq!("A B", text[0]);
        assert_eq!("X", text[1]);
    }

    #[test]
    fn bugfix1_test() {
        let story = compile_string(
            "A\n{f():\n    Another line.\n}\n\n== function f ==\n{false:nothing}\n~ return true\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert_eq!(2, text.len());
        assert!(text.join("").contains("A"));
        assert!(text.join("").contains("Another line."));
    }

    #[test]
    fn bugfix2_test() {
        let story = compile_string(
            "A\n{f():\n    Another line.\n}\n\n== function f ==\n{false:nothing}\n~ return true\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert_eq!(2, text.len());
        assert!(text.join("").contains("A"));
        assert!(text.join("").contains("Another line."));
    }
}
