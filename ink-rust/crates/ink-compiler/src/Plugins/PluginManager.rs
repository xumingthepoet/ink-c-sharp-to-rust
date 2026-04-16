// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/Plugins/PluginManager.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct PluginManager {
    pub _port_marker: (),
}

impl PluginManager {
    // C# signature: public PluginManager (List<string> pluginDirectories)
    pub fn new(_pluginDirectories: Vec<String>) -> Self {
        Default::default()
    }

    // C# signature: public string PreParse(string storyContent)
    pub fn PreParse(&mut self, _storyContent: String) -> String {
        Default::default()
    }

    // C# signature: public Parsed.Story PostParse(Parsed.Story parsedStory)
    pub fn PostParse(&mut self, _parsedStory: crate::stub::Story) -> crate::stub::Story {
        Default::default()
    }

    // C# signature: public Runtime.Story PostExport(Parsed.Story parsedStory, Runtime.Story runtimeStory)
    pub fn PostExport(
        &mut self,
        _parsedStory: crate::stub::Story,
        _runtimeStory: crate::stub::Story,
    ) -> crate::stub::Story {
        Default::default()
    }
}
