// Source: ink-c-sharp/compiler/ParsedHierarchy/IncludedFile.cs

use crate::ParsedHierarchy::Story::Story;

#[derive(Clone, Debug, Default)]
pub struct IncludedFile {
    pub includedStory: Story,
}

impl IncludedFile {
    // C# signature: public IncludedFile (Parsed.Story includedStory)
    pub fn new(includedStory: Story) -> Self {
        Self { includedStory }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        crate::stub::PortStub::default()
    }

    // C# signature: Parsed.Story includedStory { get; }
    pub fn get_includedStory(&self) -> &Story {
        &self.includedStory
    }
}
