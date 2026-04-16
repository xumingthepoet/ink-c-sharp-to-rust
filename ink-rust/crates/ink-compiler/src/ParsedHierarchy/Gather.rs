// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Gather.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Gather {
    pub _port_marker: (),
}

impl Gather {
    // C# signature: public Gather (Identifier identifier, int indentationDepth)
    pub fn new(_identifier: crate::stub::Identifier, _indentationDepth: i32) -> Self {
        Default::default()
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: string name { get; }
    pub fn get_name(&mut self) -> String {
        Default::default()
    }

    // C# signature: Identifier identifier { get; }
    pub fn get_identifier(&mut self) -> crate::stub::Identifier {
        Default::default()
    }

    // C# signature: int indentationDepth { get; }
    pub fn get_indentationDepth(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: Runtime.Container runtimeContainer { get; }
    pub fn get_runtimeContainer(&mut self) -> crate::stub::Container {
        Default::default()
    }
}
