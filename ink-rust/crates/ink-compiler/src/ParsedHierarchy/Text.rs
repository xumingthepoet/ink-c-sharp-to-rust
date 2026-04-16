// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Text.cs

use ink_runtime::Value::StringValue;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Text {
    pub text: String,
}

impl Text {
    // C# signature: public Text (string str)
    pub fn new(_str: String) -> Self {
        Self { text: _str }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&self) -> StringValue {
        StringValue::default()
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

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}
