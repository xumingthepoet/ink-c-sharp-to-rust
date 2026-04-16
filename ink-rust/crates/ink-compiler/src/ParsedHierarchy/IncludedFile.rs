// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/IncludedFile.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct IncludedFile {
    pub _port_marker: (),
}

impl IncludedFile {
    // C# signature: public IncludedFile (Parsed.Story includedStory)
    pub fn new(_includedStory: crate::stub::Story) -> Self {
        Default::default()
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: Parsed.Story includedStory { get; }
    pub fn get_includedStory(&mut self) -> crate::stub::Story {
        Default::default()
    }
}
