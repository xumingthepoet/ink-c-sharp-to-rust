#[cfg(test)]
mod divert_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn simple_divert_test() {
        let story = compile_string("-> target\n== target ==\nhello\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("hello"));
    }

    #[test]
    fn invisible_divert_test() {
        let story = compile_string("hello -> target\n== target ==\nworld\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("hello"));
        assert!(text.join("").contains("world"));
    }

    // From blade-ink-rs/conformance-tests/tests/divert_test.rs

    #[test]
    fn divert_on_choice_test() {
        let story = compile_string(
            "->paragraph_1\n== paragraph_1 ===\nYou stand by the wall of Analand, sword in hand.\n* [Open the gate] -> paragraph_2\n\n=== paragraph_2 ===\nYou open the gate, and step out onto the path. -> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("You open the gate"));
    }

    #[test]
    fn complex_branching1_test() {
        let story = compile_string(
            "-> back_in_london\n=== back_in_london ===\n\nWe arrived into London at 9.45pm exactly.\n\n*   \"There is not a moment to lose!\"[] I declared.\n    -> hurry_outside\n\n*   \"Monsieur, let us savour this moment!\"[] I declared.\n    My master clouted me firmly around the head and dragged me out of the door.\n    -> dragged_outside\n\n*   [We hurried home] -> hurry_outside\n\n\n=== hurry_outside ===\nWe hurried home to Savile Row -> as_fast_as_we_could\n\n\n=== dragged_outside ===\nHe insisted that we hurried home to Savile Row -> as_fast_as_we_could\n\n\n=== as_fast_as_we_could ===\n<>as fast as we could.  -> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("There is not a moment to lose"));
        assert!(text.join("").contains("We hurried home to Savile Row"));
    }

    #[test]
    fn complex_branching2_test() {
        let story = compile_string(
            "-> back_in_london\n=== back_in_london ===\n\nWe arrived into London at 9.45pm exactly.\n\n*   \"There is not a moment to lose!\"[] I declared.\n    -> hurry_outside\n\n*   \"Monsieur, let us savour this moment!\"[] I declared.\n    My master clouted me firmly around the head and dragged me out of the door.\n    -> dragged_outside\n\n*   [We hurried home] -> hurry_outside\n\n\n=== hurry_outside ===\nWe hurried home to Savile Row -> as_fast_as_we_could\n\n\n=== dragged_outside ===\nHe insisted that we hurried home to Savile Row -> as_fast_as_we_could\n\n\n=== as_fast_as_we_could ===\n<>as fast as we could.  -> END\n",
        );
        let mut s = story.clone();
        continue_all(&mut s);
        s.ChooseChoiceIndex(1);
        let text = continue_all(&mut s);
        assert!(text
            .join("")
            .contains("Monsieur, let us savour this moment"));
        assert!(text.join("").contains("My master clouted me firmly"));
        assert!(text.join("").contains("He insisted that we hurried home"));
    }
}
