// Source: ink-c-sharp/compiler/ParsedHierarchy/IncludedFile.cs

use crate::ParsedHierarchy::Story::Story;

#[derive(Clone, Debug, Default)]
pub struct IncludedFile {
    pub includedStory: Option<Story>,
}

impl IncludedFile {
    // C# signature: public IncludedFile (Parsed.Story includedStory)
    pub fn new(includedStory: Option<Story>) -> Self {
        Self { includedStory }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&self) -> Option<()> {
        None
    }

    // C# signature: Parsed.Story includedStory { get; }
    pub fn get_includedStory(&self) -> Option<&Story> {
        self.includedStory.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::IncludedFile;
    use crate::ParsedHierarchy::Story::Story;

    #[test]
    fn stores_included_story_and_generates_nothing() {
        let story = Story::default();
        let included = IncludedFile::new(Some(story.clone()));

        assert!(included.GenerateRuntimeObject().is_none());
        assert_eq!(included.get_includedStory().unwrap().constants.len(), 0);
    }
}
