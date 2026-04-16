// Source: ink-c-sharp/ink-engine-runtime/ListDefinitionsOrigin.cs

use std::collections::HashMap;

use crate::InkList::InkListItem;
use crate::ListDefinition::ListDefinition;
use crate::Value::ListValue;

#[derive(Clone, Debug, Default)]
pub struct ListDefinitionsOrigin {
    lists: HashMap<String, ListDefinition>,
    list_order: Vec<String>,
    allUnambiguousListValueCache: HashMap<String, ListValue>,
}

impl ListDefinitionsOrigin {
    // C# signature: public ListDefinitionsOrigin (List<Runtime.ListDefinition> lists)
    pub fn new(lists: Vec<ListDefinition>) -> Self {
        let mut origin = Self::default();

        for mut list in lists {
            let list_name = list.get_name().to_string();
            if !origin.lists.contains_key(&list_name) {
                origin.list_order.push(list_name.clone());
            }
            origin.lists.insert(list_name.clone(), list.clone());

            for (item, val) in list.get_items().clone() {
                let mut list_value = ListValue::new_overload_3(item.clone(), val);
                list_value.origins = Some(vec![list.clone()]);
                if let Some(item_name) = item.itemName.clone() {
                    origin
                        .allUnambiguousListValueCache
                        .insert(item_name, list_value.clone());
                }
                origin
                    .allUnambiguousListValueCache
                    .insert(item.get_fullName(), list_value);
            }
        }

        origin
    }

    // C# signature: public bool TryListGetDefinition (string name, out ListDefinition def)
    pub fn TryListGetDefinition(&self, name: String) -> Option<&ListDefinition> {
        self.lists.get(&name)
    }

    // C# signature: public ListValue FindSingleItemListWithName (string name)
    pub fn FindSingleItemListWithName(&self, name: String) -> Option<&ListValue> {
        if name.trim().is_empty() {
            None
        } else {
            self.allUnambiguousListValueCache.get(&name)
        }
    }

    // C# signature: List<Runtime.ListDefinition> lists { get; }
    pub fn get_lists(&self) -> Vec<ListDefinition> {
        self.list_order
            .iter()
            .filter_map(|name| self.lists.get(name).cloned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{ListDefinition, ListDefinitionsOrigin};
    use std::collections::HashMap;

    #[test]
    fn indexes_definitions_and_item_caches() {
        let mut items = HashMap::new();
        items.insert("apples".to_string(), 2);
        let mut other_items = HashMap::new();
        other_items.insert("beets".to_string(), 1);
        let origin = ListDefinitionsOrigin::new(vec![
            ListDefinition::new("food".to_string(), items),
            ListDefinition::new("plants".to_string(), other_items),
        ]);

        assert!(origin.TryListGetDefinition("food".to_string()).is_some());
        assert!(origin
            .FindSingleItemListWithName("apples".to_string())
            .is_some());
        assert!(origin
            .FindSingleItemListWithName("food.apples".to_string())
            .is_some());
        assert_eq!(
            origin
                .get_lists()
                .into_iter()
                .map(|list| list.get_name().to_string())
                .collect::<Vec<_>>(),
            vec!["food".to_string(), "plants".to_string()]
        );
    }
}
