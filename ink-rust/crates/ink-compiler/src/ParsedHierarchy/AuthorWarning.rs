// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/AuthorWarning.cs

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AuthorWarning {
    pub warningMessage: String,
}

impl AuthorWarning {
    // C# signature: public AuthorWarning(string message)
    pub fn new(_message: String) -> Self {
        Self {
            warningMessage: _message,
        }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&self) -> Option<()> {
        None
    }
}
