// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/CommandLineInput.cs

use std::any::Any;

#[derive(Default)]
pub struct CommandLineInput {
    pub isHelp: bool,
    pub isExit: bool,
    pub choiceInput: Option<i32>,
    pub debugSource: Option<i32>,
    pub debugPathLookup: Option<String>,
    pub userImmediateModeStatement: Option<Box<dyn Any>>,
}

impl CommandLineInput {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::CommandLineInput;

    #[test]
    fn defaults_match_csharp_data_shape() {
        let input = CommandLineInput::new();
        assert!(!input.isHelp);
        assert!(!input.isExit);
        assert!(input.choiceInput.is_none());
        assert!(input.debugSource.is_none());
        assert!(input.debugPathLookup.is_none());
        assert!(input.userImmediateModeStatement.is_none());
    }
}
