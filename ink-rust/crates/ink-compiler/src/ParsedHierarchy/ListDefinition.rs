// Source: ink-c-sharp/compiler/ParsedHierarchy/ListDefinition.cs

use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Story::Story;
use crate::ParsedHierarchy::VariableAssignment::VariableAssignment;
use ink_runtime::InkList::{InkList, InkListItem};
use ink_runtime::ListDefinition::ListDefinition as RuntimeListDefinition;
use ink_runtime::Value::ListValue;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListDefinition {
    pub identifier: Option<Identifier>,
    pub itemDefinitions: Vec<ListElementDefinition>,
    pub variableAssignment: Option<VariableAssignment>,
    elementsByName: Option<HashMap<String, usize>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListElementDefinition {
    pub identifier: Option<Identifier>,
    pub explicitValue: Option<i32>,
    pub seriesValue: i32,
    pub inInitialList: bool,
    parentListName: Option<String>,
}

impl ListDefinition {
    // C# signature: public ListDefinition (List<ListElementDefinition> elements)
    pub fn new(elements: Vec<ListElementDefinition>) -> Self {
        let mut itemDefinitions = elements;
        let mut currentValue = 1;

        for element in &mut itemDefinitions {
            if let Some(explicitValue) = element.explicitValue {
                currentValue = explicitValue;
            }

            element.seriesValue = currentValue;
            currentValue += 1;
        }

        Self {
            identifier: None,
            itemDefinitions,
            variableAssignment: None,
            elementsByName: None,
        }
    }

    fn identifier_name(&self) -> Option<&str> {
        self.identifier
            .as_ref()
            .and_then(|identifier| identifier.name.as_deref())
    }

    fn ensure_elements_by_name(&mut self) {
        if self.elementsByName.is_some() {
            return;
        }

        let mut elements_by_name = HashMap::new();
        for (index, element) in self.itemDefinitions.iter().enumerate() {
            if let Some(name) = element.get_name() {
                elements_by_name.insert(name.to_string(), index);
            }
        }

        self.elementsByName = Some(elements_by_name);
    }

    // C# signature: public ListElementDefinition ItemNamed (string itemName)
    pub fn ItemNamed(&mut self, itemName: String) -> Option<&ListElementDefinition> {
        self.ensure_elements_by_name();

        self.elementsByName
            .as_ref()
            .and_then(|elements_by_name| elements_by_name.get(&itemName).copied())
            .and_then(|index| self.itemDefinitions.get(index))
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> ListValue {
        let mut initialValues = InkList::new();

        for itemDef in &self.itemDefinitions {
            if itemDef.inInitialList {
                let item = InkListItem::new(
                    self.identifier_name().map(|name| name.to_string()),
                    itemDef.get_name().map(|name| name.to_string()),
                );
                initialValues =
                    initialValues.Union(InkList::new_overload_4((item, itemDef.seriesValue)));
            }
        }

        initialValues.SetInitialOriginName(self.identifier_name().unwrap_or_default().to_string());

        ListValue::new_overload_2(initialValues.get_entries().clone())
    }

    // C# signature: Runtime.ListDefinition runtimeListDefinition { get; }
    pub fn get_runtimeListDefinition(&self) -> RuntimeListDefinition {
        let mut allItems = HashMap::new();

        for element in &self.itemDefinitions {
            let name = element
                .get_name()
                .map(|name| name.to_string())
                .unwrap_or_default();
            if allItems.contains_key(&name) {
                panic!(
                    "List '{}' contains dupicate items called '{}'",
                    self.identifier_name().unwrap_or(""),
                    name
                );
            }
            allItems.insert(name, element.seriesValue);
        }

        RuntimeListDefinition::new(
            self.identifier_name().unwrap_or_default().to_string(),
            allItems,
        )
    }

    // C# signature: string name { get; }
    pub fn get_name(&self) -> Option<&str> {
        self.identifier_name()
    }

    // C# signature: string typeName { get; }
    pub fn get_typeName(&self) -> String {
        "List definition".to_string()
    }

    pub fn ResolveReferences(&mut self, context: &mut Story) {
        if let Some(identifier) = self.identifier.clone() {
            let parent_name = identifier.name.clone();
            for element in &mut self.itemDefinitions {
                element.set_parent_list_name(parent_name.clone());
                element.ResolveReferences(context);
            }
            context.CheckForNamingCollisions(
                Default::default(),
                identifier,
                crate::ParsedHierarchy::Story::SymbolType::List,
                String::new(),
            );
        }
        context.register_list_definition(self.clone());
    }
}

impl ListElementDefinition {
    // C# signature: public ListElementDefinition (Identifier identifier, bool inInitialList, int? explicitValue = null)
    pub fn new(identifier: Identifier, inInitialList: bool, explicitValue: Option<i32>) -> Self {
        Self {
            identifier: Some(identifier),
            explicitValue,
            seriesValue: 0,
            inInitialList,
            parentListName: None,
        }
    }

    pub fn set_parent_list_name(&mut self, parentListName: Option<String>) {
        self.parentListName = parentListName;
    }

    // C# signature: string name { get; }
    pub fn get_name(&self) -> Option<&str> {
        self.identifier
            .as_ref()
            .and_then(|identifier| identifier.name.as_deref())
    }

    // C# signature: string fullName { get; }
    pub fn get_fullName(&self) -> Option<String> {
        let parent = self
            .parentListName
            .as_deref()
            .expect("Can't get full name without a parent list");
        Some(format!("{}.{}", parent, self.get_name().unwrap_or("")))
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        panic!("ListElementDefinition.GenerateRuntimeObject is not implemented in C#");
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        if let Some(identifier) = self.identifier.clone() {
            context.CheckForNamingCollisions(
                Default::default(),
                identifier,
                crate::ParsedHierarchy::Story::SymbolType::ListItem,
                String::new(),
            );
        }
    }

    // C# signature: string typeName { get; }
    pub fn get_typeName(&self) -> String {
        "List element".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{ListDefinition, ListElementDefinition};
    use crate::ParsedHierarchy::Identifier::Identifier;

    #[test]
    fn builds_runtime_list_definition_and_initial_values() {
        let mut list_def = ListDefinition::new(vec![
            ListElementDefinition::new(
                Identifier {
                    name: Some("apple".to_string()),
                    debugMetadata: None,
                },
                true,
                Some(2),
            ),
            ListElementDefinition::new(
                Identifier {
                    name: Some("pear".to_string()),
                    debugMetadata: None,
                },
                false,
                None,
            ),
        ]);

        list_def.identifier = Some(Identifier {
            name: Some("food".to_string()),
            debugMetadata: None,
        });
        for item in &mut list_def.itemDefinitions {
            item.set_parent_list_name(Some("food".to_string()));
        }

        assert_eq!(list_def.get_runtimeListDefinition().get_name(), "food");
        assert_eq!(
            list_def.ItemNamed("apple".to_string()).unwrap().seriesValue,
            2
        );
        assert_eq!(list_def.get_typeName(), "List definition");
    }
}
