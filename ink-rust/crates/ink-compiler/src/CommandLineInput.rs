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
