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
    pub fn GenerateRuntimeObject(&self) -> Option<()> {
        None
    }

    // C# signature: Parsed.Story includedStory { get; }
    pub fn get_includedStory(&self) -> &Story {
        &self.includedStory
    }
}

#[cfg(test)]
mod tests {
    use super::IncludedFile;
    use crate::ParsedHierarchy::Story::Story;

    #[test]
    fn stores_included_story_and_generates_nothing() {
        let story = Story::default();
        let included = IncludedFile::new(story.clone());

        assert!(included.GenerateRuntimeObject().is_none());
        assert_eq!(included.get_includedStory().constants.len(), 0);
    }
}
