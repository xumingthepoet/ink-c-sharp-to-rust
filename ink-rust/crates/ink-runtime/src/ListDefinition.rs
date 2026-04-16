// Source: ink-c-sharp/ink-engine-runtime/ListDefinition.cs

use crate::InkList::InkListItem;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct ListDefinition {
    name: String,
    itemNameToValues: HashMap<String, i32>,
    items: Option<HashMap<InkListItem, i32>>,
}

impl ListDefinition {
    // C# signature: public ListDefinition (string name, Dictionary<string, int> items)
    pub fn new(name: String, items: HashMap<String, i32>) -> Self {
        Self {
            name,
            itemNameToValues: items,
            items: None,
        }
    }

    pub fn get_items(&mut self) -> &HashMap<InkListItem, i32> {
        if self.items.is_none() {
            let mut built = HashMap::new();
            for (itemName, val) in &self.itemNameToValues {
                built.insert(
                    InkListItem::new(Some(self.name.clone()), Some(itemName.clone())),
                    *val,
                );
            }
            self.items = Some(built);
        }

        self.items.as_ref().unwrap()
    }

    // C# signature: public int ValueForItem (InkListItem item)
    pub fn ValueForItem(&self, item: &InkListItem) -> i32 {
        item.itemName
            .as_ref()
            .and_then(|name| self.itemNameToValues.get(name).copied())
            .unwrap_or(0)
    }

    // C# signature: public bool ContainsItem (InkListItem item)
    pub fn ContainsItem(&self, item: &InkListItem) -> bool {
        item.originName.as_deref() == Some(self.name.as_str())
            && item
                .itemName
                .as_deref()
                .is_some_and(|itemName| self.itemNameToValues.contains_key(itemName))
    }

    // C# signature: public bool ContainsItemWithName (string itemName)
    pub fn ContainsItemWithName(&self, itemName: String) -> bool {
        self.itemNameToValues.contains_key(&itemName)
    }

    // C# signature: public bool TryGetItemWithValue (int val, out InkListItem item)
    pub fn TryGetItemWithValue(&self, val: i32) -> Option<InkListItem> {
        self.itemNameToValues.iter().find_map(|(name, value)| {
            if *value == val {
                Some(InkListItem::new(
                    Some(self.name.clone()),
                    Some(name.clone()),
                ))
            } else {
                None
            }
        })
    }

    // C# signature: public bool TryGetValueForItem (InkListItem item, out int intVal)
    pub fn TryGetValueForItem(&self, item: &InkListItem) -> Option<i32> {
        item.itemName
            .as_ref()
            .and_then(|name| self.itemNameToValues.get(name).copied())
    }

    // C# signature: string name { get; }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::{InkListItem, ListDefinition};
    use std::collections::HashMap;

    #[test]
    fn exposes_item_lookup_and_origin_checks() {
        let mut items = HashMap::new();
        items.insert("apples".to_string(), 2);
        let def = ListDefinition::new("food".to_string(), items);
        let item = InkListItem::new(Some("food".to_string()), Some("apples".to_string()));

        assert_eq!(def.ValueForItem(&item), 2);
        assert!(def.ContainsItem(&item));
        assert!(def.ContainsItemWithName("apples".to_string()));
        assert_eq!(
            def.TryGetItemWithValue(2).unwrap().get_fullName(),
            "food.apples"
        );
        assert_eq!(def.TryGetValueForItem(&item), Some(2));
        assert_eq!(def.get_name(), "food");
    }
}
