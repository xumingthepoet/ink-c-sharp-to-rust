// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/VariableAssignment.cs

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VariableAssignment {
    pub variableName: Option<String>,
    pub isNewDeclaration: bool,
    pub isGlobal: bool,
}

impl VariableAssignment {
    // C# signature: public VariableAssignment (string variableName, bool isNewDeclaration)
    pub fn new(_variableName: String, _isNewDeclaration: bool) -> Self {
        Self {
            variableName: Some(_variableName),
            isNewDeclaration: _isNewDeclaration,
            isGlobal: false,
        }
    }

    // C# signature: public VariableAssignment()
    pub fn new_overload_2() -> Self {
        Self::default()
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        self.to_string()
    }

    // C# signature: string variableName { get; }
    pub fn get_variableName(&self) -> Option<&str> {
        self.variableName.as_deref()
    }

    // C# signature: bool isNewDeclaration { get; }
    pub fn get_isNewDeclaration(&self) -> bool {
        self.isNewDeclaration
    }

    // C# signature: bool isGlobal { get; }
    pub fn get_isGlobal(&self) -> bool {
        self.isGlobal
    }

    pub fn set_isGlobal(&mut self, is_global: bool) {
        self.isGlobal = is_global;
    }
}

impl std::fmt::Display for VariableAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VarAssign to {}",
            self.variableName.as_deref().unwrap_or("")
        )
    }
}
