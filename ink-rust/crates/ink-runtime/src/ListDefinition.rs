// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/ListDefinition.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct ListDefinition {
    pub _port_marker: (),
}

impl ListDefinition {
    // C# signature: public ListDefinition (string name, Dictionary<string, int> items)
    pub fn new(_name: String, _items: std::collections::HashMap<String, i32>) -> Self {
        Default::default()
    }

    // C# signature: public int ValueForItem (InkListItem item)
    pub fn ValueForItem(&mut self, _item: crate::stub::InkListItem) -> i32 {
        Default::default()
    }

    // C# signature: public bool ContainsItem (InkListItem item)
    pub fn ContainsItem(&mut self, _item: crate::stub::InkListItem) -> bool {
        Default::default()
    }

    // C# signature: public bool ContainsItemWithName (string itemName)
    pub fn ContainsItemWithName(&mut self, _itemName: String) -> bool {
        Default::default()
    }

    // C# signature: public bool TryGetItemWithValue (int val, out InkListItem item)
    pub fn TryGetItemWithValue(&mut self, _val: i32, _item: &mut crate::stub::InkListItem) -> bool {
        Default::default()
    }

    // C# signature: public bool TryGetValueForItem (InkListItem item, out int intVal)
    pub fn TryGetValueForItem(
        &mut self,
        _item: crate::stub::InkListItem,
        _intVal: &mut i32,
    ) -> bool {
        Default::default()
    }

    // C# signature: string name { get; }
    pub fn get_name(&mut self) -> String {
        Default::default()
    }
}
