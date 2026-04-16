// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/ListDefinitionsOrigin.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct ListDefinitionsOrigin {
    pub _port_marker: (),
}

impl ListDefinitionsOrigin {
    // C# signature: public ListDefinitionsOrigin (List<Runtime.ListDefinition> lists)
    pub fn new(_lists: Vec<crate::stub::ListDefinition>) -> Self {
        Default::default()
    }

    // C# signature: public bool TryListGetDefinition (string name, out ListDefinition def)
    pub fn TryListGetDefinition(
        &mut self,
        _name: String,
        _def: &mut crate::stub::ListDefinition,
    ) -> bool {
        Default::default()
    }

    // C# signature: public ListValue FindSingleItemListWithName (string name)
    pub fn FindSingleItemListWithName(&mut self, _name: String) -> crate::stub::ListValue {
        Default::default()
    }

    // C# signature: List<Runtime.ListDefinition> lists { get; }
    pub fn get_lists(&mut self) -> Vec<crate::stub::ListDefinition> {
        Default::default()
    }
}
