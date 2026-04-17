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
        if self.pluginDirectories.is_empty() {
            return storyContent;
        }

        todo!(
            "plugin discovery/loading is not ported yet; reflection-driven DLL loading needs a separate integration layer"
        )
    }

    // C# signature: public Parsed.Story PostParse(Parsed.Story parsedStory)
    pub fn PostParse(&mut self, parsedStory: ParsedStory) -> ParsedStory {
        if self.pluginDirectories.is_empty() {
            return parsedStory;
        }

        todo!(
            "plugin discovery/loading is not ported yet; reflection-driven DLL loading needs a separate integration layer"
        )
    }

    // C# signature: public Runtime.Story PostExport(Parsed.Story parsedStory, Runtime.Story runtimeStory)
    pub fn PostExport(
        &mut self,
        _parsedStory: ParsedStory,
        runtimeStory: RuntimeStory,
    ) -> RuntimeStory {
        if self.pluginDirectories.is_empty() {
            return runtimeStory;
        }

        todo!(
            "plugin discovery/loading is not ported yet; reflection-driven DLL loading needs a separate integration layer"
        )
    }
}
