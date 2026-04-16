// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Tag.cs

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tag {
    pub text: String,
}

impl Tag {
    // C# signature: public Tag (string tagText)
    pub fn new(_tagText: String) -> Self {
        Self { text: _tagText }
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        self.to_string()
    }

    // C# signature: string text { get; }
    pub fn get_text(&self) -> &str {
        &self.text
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "# {}", self.text)
    }
}
