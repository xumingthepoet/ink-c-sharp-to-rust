// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/ExternalDeclaration.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct ExternalDeclaration {
    pub _port_marker: (),
}

impl ExternalDeclaration {
    // C# signature: public ExternalDeclaration (Identifier identifier, List<string> argumentNames)
    pub fn new(_identifier: crate::stub::Identifier, _argumentNames: Vec<String>) -> Self {
        Default::default()
    }

    // C# signature: public override Ink.Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: string name { get; }
    pub fn get_name(&mut self) -> String {
        Default::default()
    }

    // C# signature: Identifier identifier { get; }
    pub fn get_identifier(&mut self) -> crate::stub::Identifier {
        Default::default()
    }

    // C# signature: List<string> argumentNames { get; }
    pub fn get_argumentNames(&mut self) -> Vec<String> {
        Default::default()
    }
}
