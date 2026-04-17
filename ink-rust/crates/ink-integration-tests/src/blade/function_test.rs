#[cfg(test)]
mod function_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn fun_basic_test() {
        let story = compile_string("{ f() }\n-> END\n\n== function f() ==\nhello\n~ return true\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("hello"));
    }

    #[test]
    fn fun_none_test() {
        let story = compile_string("{ f() }\n-> END\n\n== function f() ==\n~ return\n");
        let mut s = story.clone();
        continue_all(&mut s);
    }

    #[test]
    fn setvar_test() {
        let story = compile_string("~ x = 5\n{x}\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("5"));
    }

    // From blade-ink-rs/conformance-tests/tests/function_test.rs

    #[test]
    fn fun_inline_test() {
        let story = compile_string(
            "The value of x is {lerp(2, 8, 0.4)}.\n  -> END\n\n=== function lerp(a, b, k) ===\n    ~ return ((b - a) * k) + a\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("4.4"));
    }

    #[test]
    fn complex_func1_test() {
        let story = compile_string(
            "~ derp(2, 3, 4)\n   The values are {x} and {y}.\n   -> END\n\n=== function derp(a, b, c) ===\n   VAR x = 0\n   ~ x = a + b\n   VAR y = 3\n   { x == 5:\n      ~ x = 6\n   }\n   ~ y = x + c\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("6"));
        assert!(text.join("").contains("10"));
    }

    #[test]
    fn complex_func2_test() {
        let story = compile_string(
            "~ herp(2, 3)\n  The value is {x}.\n  -> END\n\n=== function herp(a, b) ===\n  VAR x = 0.0\n  ~x = a * b\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("6"));
    }

    #[test]
    fn complex_func3_test() {
        let story = compile_string(
            "\n   ~ merchant_init()\n    \"I will pay you {fee} reales if you get the goods to their destination. The goods will take up {weight} cargo spaces.\"\n   -> END\n\n=== function merchant_init()\n   VAR weight = 20\n   VAR roll = 0\n   VAR mult = 1\n\n   { roll == 0:\n      ~ mult = 2\n   }\n\n   { mult == 2:\n      ~ roll = 1\n   }\n\n   { roll == 0:\n      ~ mult = 3\n   }\n\n   VAR dst = 5\n   VAR deadline = 0\n   ~ deadline = (dst * (100)) / 100\n   VAR fee = 0\n   ~ fee = (1 + dst) * 10 * mult\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("120"));
        assert!(text.join("").contains("20"));
    }

    // rnd test - the original uses JSON files with SEED_RANDOM
    #[test]
    fn rnd() {
        let story = compile_string(
            "~ SEED_RANDOM(1)\n{RANDOM(1,4)}\n{RANDOM(1,4)}\n{RANDOM(1,4)}\n{RANDOM(1,4)}\n-> END\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert_eq!(4, text.len());
    }

    #[test]
    fn evaluating_function_variable_state_bug_test() {
        let story = compile_string(
            "Start\n-> tunnel\n\n=== tunnel ===\nIn tunnel.\n{ evaluate_function() }\nEnd\n-> END\n\n=== function evaluate_function() ===\n~ temp result = \"RIGHT\"\n~ return result\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Start"));
        assert!(text.join("").contains("In tunnel."));
        assert!(text.join("").contains("End"));
    }
}
