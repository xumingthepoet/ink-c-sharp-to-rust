#[cfg(test)]
mod tunnel_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn tunnel_onwards_divert_override_test() {
        let story = compile_string("-> tunnel ->\nreturn here\n\n== tunnel ==\nin tunnel\n->-> destination\n\n== destination ==\nat destination\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("in tunnel"));
        assert!(text.join("").contains("at destination"));
    }
}
