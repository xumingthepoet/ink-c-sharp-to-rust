// Source: ink-c-sharp/compiler/ParsedHierarchy/List.cs

use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Story::Story;
use ink_runtime::Container::ContentItem;
use ink_runtime::InkList::{InkList, InkListItem};
use ink_runtime::Value::{ListValue, Value};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct List {
    pub itemIdentifierList: Vec<Identifier>,
    resolvedRuntimeList: Option<ListValue>,
}

impl List {
    // C# signature: public List (List<Identifier> itemIdentifierList)
    pub fn new(itemIdentifierList: Vec<Identifier>) -> Self {
        Self {
            itemIdentifierList,
            resolvedRuntimeList: None,
        }
    }

    pub fn ResolveReferences(&mut self, context: &mut Story) {
        let mut runtimeRawList = InkList::new();

        for itemIdentifier in &self.itemIdentifierList {
            let name = itemIdentifier.name.clone().unwrap_or_default();
            let nameParts = name.split('.').collect::<Vec<_>>();

            let (listName, listItemName) = if nameParts.len() > 1 {
                (Some(nameParts[0].to_string()), nameParts[1].to_string())
            } else {
                (None, nameParts[0].to_string())
            };

            let list_item = context.ResolveListItem(
                listName.clone().unwrap_or_default(),
                listItemName.clone(),
                Default::default(),
            );

            let Some(list_item) = list_item else {
                if listName.is_none() {
                    context.Error(
                        format!(
                            "Could not find list definition that contains item '{}'",
                            itemIdentifier
                        ),
                        Default::default(),
                        false,
                    );
                } else {
                    context.Error(
                        format!("Could not find list item {}", itemIdentifier),
                        Default::default(),
                        false,
                    );
                }
                continue;
            };

            let listName = listName.unwrap_or_else(|| {
                list_item
                    .get_fullName()
                    .and_then(|full_name| full_name.split('.').next().map(|name| name.to_string()))
                    .unwrap_or_default()
            });

            let item = InkListItem::new(
                Some(listName),
                Some(list_item.get_name().unwrap_or_default().to_string()),
            );
            if runtimeRawList.get_entries().contains_key(&item) {
                context.Error(
                    format!("Duplicate of item '{}' in list.", itemIdentifier),
                    Default::default(),
                    true,
                );
            } else {
                runtimeRawList.insert_entry(item, list_item.seriesValue);
            }
        }

        self.resolvedRuntimeList = Some(ListValue::new_overload_2(
            runtimeRawList.get_entries().clone(),
        ));
    }

    // C# signature: public override void GenerateIntoContainer (Runtime.Container container)
    pub fn GenerateIntoContainer(&self, container: &mut ink_runtime::Container::Container) {
        let runtime_list = self.resolvedRuntimeList.clone().unwrap_or_else(|| {
            panic!("List::GenerateIntoContainer called before ResolveReferences")
        });
        container.AddContent(ContentItem::Value(Value::new_list(runtime_list)));
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        let names = self
            .itemIdentifierList
            .iter()
            .map(|identifier| identifier.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!("({})", names)
    }

    pub fn get_itemIdentifierList(&self) -> &[Identifier] {
        &self.itemIdentifierList
    }
}

#[cfg(test)]
mod tests {
    use super::List;
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::ListDefinition::{ListDefinition, ListElementDefinition};
    use crate::ParsedHierarchy::Story::Story;

    #[test]
    fn builds_and_generates_list_values_from_resolved_items() {
        let mut story = Story::new(vec![], false);
        let mut def = ListDefinition::new(vec![ListElementDefinition::new(
            Identifier {
                name: Some("apple".to_string()),
                debugMetadata: None,
            },
            true,
            Some(2),
        )]);
        def.identifier = Some(Identifier {
            name: Some("food".to_string()),
            debugMetadata: None,
        });
        for item in &mut def.itemDefinitions {
            item.set_parent_list_name(Some("food".to_string()));
        }
        story.register_list_definition(def);

        let mut list = List::new(vec![Identifier {
            name: Some("food.apple".to_string()),
            debugMetadata: None,
        }]);
        list.ResolveReferences(&mut story);
        let mut container = ink_runtime::Container::Container::new();
        list.GenerateIntoContainer(&mut container);

        assert_eq!(list.ToString(), "(food.apple)");
        assert_eq!(container.get_content().len(), 1);
    }
}
