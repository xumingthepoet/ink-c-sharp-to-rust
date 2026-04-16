// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Identifier.cs

use ink_runtime::DebugMetadata::DebugMetadata;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Identifier {
    pub name: Option<String>,
    pub debugMetadata: Option<DebugMetadata>,
}

impl Identifier {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public override string ToString()
    pub fn ToString(&self) -> String {
        self.to_string()
    }

    pub fn Done() -> Self {
        Self {
            name: Some("DONE".to_string()),
            debugMetadata: None,
        }
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_deref().unwrap_or(""))
    }
}
