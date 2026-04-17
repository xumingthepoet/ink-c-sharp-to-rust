// Source: ink-c-sharp/ink-engine-runtime/DebugMetadata.cs

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DebugMetadata {
    pub startLineNumber: i32,
    pub endLineNumber: i32,
    pub startCharacterNumber: i32,
    pub endCharacterNumber: i32,
    pub fileName: Option<String>,
    pub sourceName: Option<String>,
}

impl DebugMetadata {
    // C# signature: public DebugMetadata ()
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public DebugMetadata Merge(DebugMetadata dm)
    pub fn Merge(&self, dm: &DebugMetadata) -> DebugMetadata {
        let mut new_debug_metadata = DebugMetadata {
            fileName: self.fileName.clone(),
            sourceName: self.sourceName.clone(),
            ..Default::default()
        };

        if self.startLineNumber < dm.startLineNumber {
            new_debug_metadata.startLineNumber = self.startLineNumber;
            new_debug_metadata.startCharacterNumber = self.startCharacterNumber;
        } else if self.startLineNumber > dm.startLineNumber {
            new_debug_metadata.startLineNumber = dm.startLineNumber;
            new_debug_metadata.startCharacterNumber = dm.startCharacterNumber;
        } else {
            new_debug_metadata.startLineNumber = self.startLineNumber;
            new_debug_metadata.startCharacterNumber =
                self.startCharacterNumber.min(dm.startCharacterNumber);
        }

        if self.endLineNumber > dm.endLineNumber {
            new_debug_metadata.endLineNumber = self.endLineNumber;
            new_debug_metadata.endCharacterNumber = self.endCharacterNumber;
        } else if self.endLineNumber < dm.endLineNumber {
            new_debug_metadata.endLineNumber = dm.endLineNumber;
            new_debug_metadata.endCharacterNumber = dm.endCharacterNumber;
        } else {
            new_debug_metadata.endLineNumber = self.endLineNumber;
            new_debug_metadata.endCharacterNumber =
                self.endCharacterNumber.max(dm.endCharacterNumber);
        }

        new_debug_metadata
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        self.to_string()
    }
}

impl std::fmt::Display for DebugMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(file_name) = &self.fileName {
            write!(f, "line {} of {}", self.startLineNumber, file_name)
        } else {
            write!(f, "line {}", self.startLineNumber)
        }
    }
}
