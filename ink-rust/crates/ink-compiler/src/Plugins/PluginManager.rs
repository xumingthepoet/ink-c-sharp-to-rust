// Source: ink-c-sharp/compiler/Plugins/PluginManager.cs

use crate::ParsedHierarchy::Story::Story as ParsedStory;
use ink_runtime::Story::Story as RuntimeStory;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PluginManager {
    pluginDirectories: Vec<String>,
}

impl PluginManager {
    // C# signature: public PluginManager (List<string> pluginDirectories)
    pub fn new(pluginDirectories: Vec<String>) -> Self {
        Self { pluginDirectories }
    }

    // C# signature: public string PreParse(string storyContent)
    pub fn PreParse(&mut self, storyContent: String) -> String {
        storyContent
    }

    // C# signature: public Parsed.Story PostParse(Parsed.Story parsedStory)
    pub fn PostParse(&mut self, parsedStory: ParsedStory) -> ParsedStory {
        parsedStory
    }

    // C# signature: public Runtime.Story PostExport(Parsed.Story parsedStory, Runtime.Story runtimeStory)
    pub fn PostExport(
        &mut self,
        _parsedStory: ParsedStory,
        runtimeStory: RuntimeStory,
    ) -> RuntimeStory {
        runtimeStory
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParsedHierarchy::Object::Object;
    use crate::ParsedHierarchy::Story::Story as ParsedStory;
    use ink_runtime::Container::Container;
    use ink_runtime::Story::Story as RuntimeStory;

    #[test]
    fn plugin_manager_is_an_inert_compatibility_shim() {
        let mut manager = PluginManager::new(vec!["plugins".to_string()]);
        assert_eq!(manager.PreParse("story".to_string()), "story");
        assert!(!manager
            .PostParse(ParsedStory::new(Vec::<Object>::new(), false))
            .get_isInclude());

        let runtime_story = RuntimeStory::new(Container::new(), Vec::new());
        let _ = manager.PostExport(ParsedStory::new(Vec::<Object>::new(), false), runtime_story);
    }
}
