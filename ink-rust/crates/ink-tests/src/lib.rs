#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[cfg(test)]
mod tests {
    use ink_compiler::Compiler::Compiler;
    use ink_compiler::Compiler::Options;

    #[test]
    fn skeleton_workspace_loads() {
        assert!(true);
    }

    #[test]
    fn compiles_and_runs_simple_source_story() {
        let mut compiler = Compiler::new("Hello world\n".to_string(), Options::default());
        let mut story = compiler
            .Compile()
            .expect("source compilation should succeed");

        let output = story.Continue();
        assert_eq!(output, "Hello world\n");
        assert!(!story.get_canContinue());
    }

    #[test]
    fn compiles_top_level_knot_as_named_content_without_auto_entering() {
        let mut compiler = Compiler::new(
            "== intro ==\nHello from knot\n".to_string(),
            Options::default(),
        );
        let mut story = compiler
            .Compile()
            .expect("source compilation should succeed");

        let output = story.Continue();
        let root = story.get_mainContentContainer();

        assert_eq!(output, "");
        assert!(root.get_namedContent().contains_key("intro"));
    }

    #[test]
    fn compiles_and_runs_divert_to_top_level_knot() {
        let mut compiler = Compiler::new(
            "-> intro\n== intro ==\nHello from knot\n-> DONE\n".to_string(),
            Options::default(),
        );
        let mut story = compiler
            .Compile()
            .expect("source compilation should succeed");

        let output = story.Continue();

        assert_eq!(output, "Hello from knot\n");
        assert!(!story.get_canContinue());
    }

    #[test]
    fn simple_stories_start_without_choices() {
        let mut compiler = Compiler::new("Hello world\n".to_string(), Options::default());
        let mut story = compiler
            .Compile()
            .expect("source compilation should succeed");

        assert!(story.get_currentChoices().is_empty());
    }
}
