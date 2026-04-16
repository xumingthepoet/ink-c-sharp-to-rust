// Source: ink-c-sharp/compiler/ParsedHierarchy/ExternalDeclaration.cs

use crate::ParsedHierarchy::Identifier::Identifier;

#[derive(Clone, Debug, Default)]
pub struct ExternalDeclaration {
    pub identifier: Option<Identifier>,
    pub argumentNames: Vec<String>,
}

impl ExternalDeclaration {
    // C# signature: public ExternalDeclaration (Identifier identifier, List<string> argumentNames)
    pub fn new(identifier: Identifier, argumentNames: Vec<String>) -> Self {
        Self {
            identifier: Some(identifier),
            argumentNames,
        }
    }

    // C# signature: public override Ink.Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        crate::stub::PortStub::default()
    }

    // C# signature: string name { get; }
    pub fn get_name(&self) -> Option<&str> {
        self.identifier
            .as_ref()
            .and_then(|identifier| identifier.name.as_deref())
    }

    // C# signature: Identifier identifier { get; }
    pub fn get_identifier(&self) -> Option<&Identifier> {
        self.identifier.as_ref()
    }

    // C# signature: List<string> argumentNames { get; }
    pub fn get_argumentNames(&self) -> &[String] {
        &self.argumentNames
    }
}
