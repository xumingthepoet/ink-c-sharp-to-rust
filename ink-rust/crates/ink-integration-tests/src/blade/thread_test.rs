#[cfg(test)]
mod thread_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn thread_test() {
        let story = compile_string("<- thread_content\nMain content\n-> END\n\n== thread_content ==\nThread content\n-> DONE\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Main content"));
        assert!(text.join("").contains("Thread content"));
    }

    // From blade-ink-rs/conformance-tests/tests/thread_test.rs

    #[test]
    fn thread_test_bug() {
        let story = compile_string(
            "-> start\n\n=== start ===\nHere is some gold. Do you want it?\n- (top)\n    <- choices(-> top)\n    + Yes\n        You win!\n        -> END\n\n=== choices(-> goback) ===\n+ No\n    Try again!\n    -> goback\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Here is some gold"));
        assert_eq!(2, s.get_currentChoices().len());
        assert_eq!("No", s.get_currentChoices()[0].text);
        assert_eq!("Yes", s.get_currentChoices()[1].text);
    }
}
