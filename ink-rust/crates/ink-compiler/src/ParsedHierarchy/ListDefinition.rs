// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/ListDefinition.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct ListDefinition {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct ListElementDefinition {
    pub _port_marker: (),
}

impl ListDefinition {
    // C# signature: public ListDefinition (List<ListElementDefinition> elements)
    pub fn new(_elements: Vec<crate::stub::ListElementDefinition>) -> Self {
        Default::default()
    }

    // C# signature: public ListElementDefinition ItemNamed (string itemName)
    pub fn ItemNamed(&mut self, _itemName: String) -> crate::stub::ListElementDefinition {
        Default::default()
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: Runtime.ListDefinition runtimeListDefinition { get; }
    pub fn get_runtimeListDefinition(&mut self) -> crate::stub::ListDefinition {
        Default::default()
    }

    // C# signature: string typeName { get; }
    pub fn get_typeName(&mut self) -> String {
        Default::default()
    }

    // C# signature: string name { get; }
    pub fn get_name(&mut self) -> String {
        Default::default()
    }

    // C# signature: string fullName { get; }
    pub fn get_fullName(&mut self) -> String {
        Default::default()
    }

    // C# signature: string typeName { get; }
    pub fn get_typeName_overload_2(&mut self) -> String {
        Default::default()
    }
}
