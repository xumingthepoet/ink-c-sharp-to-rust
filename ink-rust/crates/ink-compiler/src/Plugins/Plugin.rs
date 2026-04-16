// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/Plugins/Plugin.cs

use crate::ParsedHierarchy::Story::Story as ParsedStory;
use ink_runtime::Story::Story as RuntimeStory;

pub trait IPlugin {
    fn PreParse(&mut self, storyContent: &mut String);
    fn PostParse(&mut self, parsedStory: &mut ParsedStory);
    fn PostExport(&mut self, parsedStory: &ParsedStory, runtimeStory: &mut RuntimeStory);
}
