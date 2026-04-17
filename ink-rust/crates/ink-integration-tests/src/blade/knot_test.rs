#[cfg(test)]
mod knot_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn single_line_test() {
        let story = compile_string("== knot ==\ncontent\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("content"));
    }

    #[test]
    fn param_strings_test() {
        let story =
            compile_string("-> hello(\"world\")\n== hello(name) ==\nHello {name}\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Hello world"));
    }

    #[test]
    fn param_ints_test() {
        let story = compile_string("-> add(5, 3)\n== add(a, b) ==\n{a + b}\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("8"));
    }

    #[test]
    fn param_recurse_test() {
        let story = compile_string("-> factorial(5)\n== factorial(n) ==\n{ n <= 1:\n    ~ return 1\n- else:\n    ~ return n * factorial(n - 1)\n}\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("120"));
    }

    // From blade-ink-rs/conformance-tests/tests/knot_test.rs

    #[test]
    fn multi_line_test() {
        let story = compile_string("Hello, world!\nHello?\nHello, are you there?\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert_eq!(3, text.len());
        assert!(text.join("").contains("Hello, world!"));
        assert!(text.join("").contains("Hello?"));
        assert!(text.join("").contains("Hello, are you there?"));
    }

    #[test]
    fn strip_empty_lines_test() {
        let story = compile_string("Hello, world!\n\nHello?\n  \nHello, are you there?\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert_eq!(3, text.len());
    }

    #[test]
    fn param_floats_test() {
        let story = compile_string(
            "How much do you give?\n* [$1] -> give(1.2)\n* [$2] -> give(2.5)\n* [Nothing] -> give(0)\n\n=== give(amount) ===\nYou give {amount} dollars.\n-> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(1);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("2.5"));
    }

    #[test]
    fn param_vars_test() {
        let story = compile_string(
            "VAR x = 1\nVAR y = 2\nVAR z = 0\nHow much do you give?\n* [$1] -> give(x)\n* [$2] -> give(y)\n* [Nothing] -> give(z)\n\n=== give(amount) ===\nYou give {amount} dollars.\n-> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(1);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("2"));
    }

    #[test]
    fn param_multi_test() {
        let story = compile_string(
            "VAR x = 1\nVAR y = \"Hmm.\"\nHow much do you give?\n* [I don't know] -> give(x, 2, y)\n\n=== give(a, b, c) ===\nYou give {a} or {b} dollars. {y}\n-> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("1 or 2"));
        assert!(text.join("").contains("Hmm"));
    }
}
