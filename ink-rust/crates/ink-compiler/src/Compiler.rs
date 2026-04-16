// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/Compiler.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Compiler {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct Options {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct CommandLineInputResult {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct DebugSourceRange {
    pub _port_marker: (),
}

impl Compiler {
    // C# signature: public Compiler (string inkSource, Options options = null)
    pub fn new(_inkSource: String, _options: crate::stub::Options) -> Self {
        Default::default()
    }

    // C# signature: public Parsed.Story Parse()
    pub fn Parse(&mut self) -> crate::stub::Story {
        Default::default()
    }

    // C# signature: public Runtime.Story Compile ()
    pub fn Compile(&mut self) -> crate::stub::Story {
        Default::default()
    }

    // C# signature: public CommandLineInputResult HandleInput (CommandLineInput inputResult)
    pub fn HandleInput(
        &mut self,
        _inputResult: crate::stub::CommandLineInput,
    ) -> crate::stub::CommandLineInputResult {
        Default::default()
    }

    // C# signature: public void RetrieveDebugSourceForLatestContent ()
    pub fn RetrieveDebugSourceForLatestContent(&mut self) {}

    // C# signature: Parsed.Story parsedStory { get; }
    pub fn get_parsedStory(&mut self) -> crate::stub::Story {
        Default::default()
    }
}
