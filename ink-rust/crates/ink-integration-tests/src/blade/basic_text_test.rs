#[cfg(test)]
mod basic_text_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn oneline_test() {
        let story = compile_string("Hello world.\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert_eq!(1, text.len());
        assert_eq!("Hello world.", text[0]);
    }

    #[test]
    fn twolines_test() {
        let story = compile_string("Hello world\nSecond line\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert_eq!(2, text.len());
        assert_eq!("Hello world", text[0]);
        assert_eq!("Second line", text[1]);
    }
}
