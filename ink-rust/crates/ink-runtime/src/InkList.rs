// Source: ink-c-sharp/ink-engine-runtime/InkList.cs

use crate::ListDefinition::ListDefinition;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InkListItem {
    pub originName: Option<String>,
    pub itemName: Option<String>,
}

impl InkListItem {
    // C# signature: public InkListItem (string originName, string itemName)
    pub fn new(originName: Option<String>, itemName: Option<String>) -> Self {
        Self {
            originName,
            itemName,
        }
    }

    // C# signature: public InkListItem (string fullName)
    pub fn new_overload_2(fullName: String) -> Self {
        let mut parts = fullName.split('.');
        Self {
            originName: parts.next().map(|s| s.to_string()),
            itemName: parts.next().map(|s| s.to_string()),
        }
    }

    // C# signature: public static InkListItem Null { get; }
    pub fn Null() -> Self {
        Self::new(None, None)
    }

    // C# signature: bool isNull { get; }
    pub fn get_isNull(&self) -> bool {
        self.originName.is_none() && self.itemName.is_none()
    }

    // C# signature: string fullName { get; }
    pub fn get_fullName(&self) -> String {
        format!(
            "{}.{}",
            self.originName.as_deref().unwrap_or("?"),
            self.itemName.as_deref().unwrap_or("")
        )
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        self.get_fullName()
    }
}

impl Hash for InkListItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.originName.hash(state);
        self.itemName.hash(state);
    }
}

impl std::fmt::Display for InkListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_fullName())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListBound {
    Int(i32),
    List(InkList),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InkList {
    entries: HashMap<InkListItem, i32>,
    pub origins: Option<Vec<ListDefinition>>,
    origin_names: Option<Vec<String>>,
}

impl InkList {
    // C# signature: public InkList ()
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public InkList(InkList otherList)
    pub fn new_overload_2(otherList: InkList) -> Self {
        otherList
    }

    // C# signature: public InkList (string singleOriginListName, Story originStory)
    pub fn new_overload_3(singleOriginListName: String, originStory: crate::Story::Story) -> Self {
        let mut list = Self::new();
        list.SetInitialOriginName(singleOriginListName.clone());

        let mut originStory = originStory;
        if let Some(def) = originStory
            .get_listDefinitions()
            .TryListGetDefinition(singleOriginListName.clone())
            .cloned()
        {
            list.origins = Some(vec![def]);
            return list;
        }

        panic!(
            "InkList origin could not be found in story when constructing new list: {}",
            singleOriginListName
        );
    }

    // C# signature: public InkList (KeyValuePair<InkListItem, int> singleElement)
    pub fn new_overload_4(singleElement: (InkListItem, i32)) -> Self {
        let mut list = Self::new();
        list.entries.insert(singleElement.0, singleElement.1);
        list
    }

    fn ordered_items(&self) -> Vec<(InkListItem, i32)> {
        let mut ordered = self
            .entries
            .iter()
            .map(|(item, value)| (item.clone(), *value))
            .collect::<Vec<_>>();
        ordered.sort_by(|left, right| match left.1.cmp(&right.1) {
            Ordering::Equal => left
                .0
                .originName
                .as_deref()
                .unwrap_or("")
                .cmp(right.0.originName.as_deref().unwrap_or("")),
            other => other,
        });
        ordered
    }

    fn max_item(&self) -> (InkListItem, i32) {
        self.entries
            .iter()
            .max_by_key(|(_, value)| **value)
            .map(|(item, value)| (item.clone(), *value))
            .unwrap_or_else(|| (InkListItem::Null(), 0))
    }

    fn min_item(&self) -> (InkListItem, i32) {
        self.entries
            .iter()
            .min_by_key(|(_, value)| **value)
            .map(|(item, value)| (item.clone(), *value))
            .unwrap_or_else(|| (InkListItem::Null(), 0))
    }

    fn clone_with_same_metadata(&self) -> Self {
        Self {
            entries: self.entries.clone(),
            origins: self.origins.clone(),
            origin_names: self.origin_names.clone(),
        }
    }

    pub fn get_entries(&self) -> &HashMap<InkListItem, i32> {
        &self.entries
    }

    pub fn insert_entry(&mut self, item: InkListItem, value: i32) {
        self.entries.insert(item, value);
    }

    // C# signature: public static InkList FromString(string myListItem, Story originStory)
    pub fn FromString(myListItem: String, originStory: crate::Story::Story) -> Self {
        if myListItem.is_empty() {
            return Self::new();
        }

        let mut originStory = originStory;
        if let Some(list_value) = originStory
            .get_listDefinitions()
            .FindSingleItemListWithName(myListItem.clone())
            .cloned()
        {
            let mut list = Self {
                entries: list_value.value.clone(),
                origins: list_value.origins.clone(),
                origin_names: list_value.originNames.clone(),
            };
            if let Some(origin_name) = list_value
                .value
                .keys()
                .next()
                .and_then(|item| item.originName.clone())
            {
                list.SetInitialOriginName(origin_name.clone());
                if let Some(def) = originStory
                    .get_listDefinitions()
                    .TryListGetDefinition(origin_name)
                    .cloned()
                {
                    list.origins = Some(vec![def]);
                }
            }
            return list;
        }

        panic!(
            "Could not find the InkListItem from the string '{}' to create an InkList because it doesn't exist in the original list definition in ink.",
            myListItem
        );
    }

    // C# signature: public void AddItem (InkListItem item)
    pub fn AddItem(&mut self, item: InkListItem) {
        if item.originName.is_none() {
            self.AddItem_overload_2(item.itemName.unwrap_or_default(), Default::default());
            return;
        }

        let origin_name = item.originName.clone().unwrap();
        if let Some(origins) = self.origins.as_ref() {
            for origin in origins {
                if origin.get_name() == origin_name {
                    let value = origin.ValueForItem(&item);
                    self.entries.insert(item, value);
                    return;
                }
            }
        }

        panic!(
            "Failed to add item to list because the item was from a new list definition that wasn't previously known to this list."
        );
    }

    // C# signature: public void AddItem(string itemName, Story storyObject = null)
    pub fn AddItem_overload_2(&mut self, itemName: String, storyObject: crate::Story::Story) {
        let mut storyObject = storyObject;
        let mut foundListDef: Option<ListDefinition> = None;

        if let Some(origins) = self.origins.as_ref() {
            for origin in origins {
                if origin.ContainsItemWithName(itemName.clone()) {
                    if foundListDef.is_some() {
                        panic!(
                            "Could not add the item {} to this list because it could come from either {} or {}",
                            itemName,
                            origin.get_name(),
                            foundListDef.as_ref().unwrap().get_name()
                        );
                    } else {
                        foundListDef = Some(origin.clone());
                    }
                }
            }
        }

        if let Some(foundListDef) = foundListDef {
            let item = InkListItem::new(Some(foundListDef.get_name().to_string()), Some(itemName));
            let itemVal = foundListDef.ValueForItem(&item);
            self.entries.insert(item, itemVal);
        } else {
            if let Some(def) = storyObject
                .get_listDefinitions()
                .FindSingleItemListWithName(itemName.clone())
                .cloned()
            {
                self.entries.extend(def.value.clone());
            } else {
                panic!(
                    "Could not find the InkListItem from the string '{}' to create an InkList because it doesn't exist in the original list definition in ink.",
                    itemName
                );
            }
        }
    }

    // C# signature: public bool ContainsItemNamed (string itemName)
    pub fn ContainsItemNamed(&self, itemName: String) -> bool {
        self.entries
            .keys()
            .any(|item| item.itemName.as_deref() == Some(itemName.as_str()))
    }

    // Story has to set this so that the value knows its origin.
    pub fn SetInitialOriginName(&mut self, initialOriginName: String) {
        self.origin_names = Some(vec![initialOriginName]);
    }

    pub fn SetInitialOriginNames(&mut self, initialOriginNames: Vec<String>) {
        self.origin_names = Some(initialOriginNames);
    }

    // C# signature: List<ListDefinition> origins { get; }
    pub fn get_origins(&self) -> Option<&[ListDefinition]> {
        self.origins.as_deref()
    }

    // C# signature: ListDefinition originOfMaxItem { get; }
    pub fn get_originOfMaxItem(&self) -> Option<ListDefinition> {
        let max_origin_name = self.max_item().0.originName?;
        self.origins.as_ref().and_then(|origins| {
            origins
                .iter()
                .find(|origin| origin.get_name() == max_origin_name)
                .cloned()
        })
    }

    // C# signature: List<string> originNames { get; }
    pub fn get_originNames(&mut self) -> Option<Vec<String>> {
        if self.entries.is_empty() {
            return self.origin_names.clone();
        }

        let mut names = Vec::with_capacity(self.entries.len());
        for item in self.entries.keys() {
            if let Some(origin_name) = &item.originName {
                names.push(origin_name.clone());
            }
        }
        self.origin_names = Some(names.clone());
        Some(names)
    }

    pub fn get_maxItem(&self) -> (InkListItem, i32) {
        self.max_item()
    }

    pub fn get_minItem(&self) -> (InkListItem, i32) {
        self.min_item()
    }

    // C# signature: public InkList inverse { get; }
    pub fn get_inverse(&self) -> InkList {
        let mut list = InkList::new();
        if let Some(origins) = self.origins.as_ref() {
            for origin in origins {
                let mut origin = origin.clone();
                for (item, value) in origin.get_items().clone() {
                    if !self.entries.contains_key(&item) {
                        list.entries.insert(item, value);
                    }
                }
            }
        }
        list
    }

    // C# signature: public InkList all { get; }
    pub fn get_all(&self) -> InkList {
        let mut list = InkList::new();
        if let Some(origins) = self.origins.as_ref() {
            for origin in origins {
                let mut origin = origin.clone();
                for (item, value) in origin.get_items().clone() {
                    list.entries.insert(item, value);
                }
            }
        }
        list
    }

    // C# signature: public InkList Union (InkList otherList)
    pub fn Union(&self, otherList: InkList) -> InkList {
        let mut union = self.clone_with_same_metadata();
        for (item, value) in otherList.entries {
            union.entries.insert(item, value);
        }
        union
    }

    // C# signature: public InkList Intersect (InkList otherList)
    pub fn Intersect(&self, otherList: InkList) -> InkList {
        let mut intersection = InkList::new();
        for (item, value) in &self.entries {
            if otherList.entries.contains_key(item) {
                intersection.entries.insert(item.clone(), *value);
            }
        }
        intersection
    }

    // C# signature: public bool HasIntersection(InkList otherList)
    pub fn HasIntersection(&self, otherList: InkList) -> bool {
        self.entries
            .keys()
            .any(|item| otherList.entries.contains_key(item))
    }

    // C# signature: public InkList Without (InkList listToRemove)
    pub fn Without(&self, listToRemove: InkList) -> InkList {
        let mut result = self.clone_with_same_metadata();
        for item in listToRemove.entries.keys() {
            result.entries.remove(item);
        }
        result
    }

    // C# signature: public bool Contains (InkList otherList)
    pub fn Contains(&self, otherList: InkList) -> bool {
        if otherList.entries.is_empty() || self.entries.is_empty() {
            return false;
        }

        otherList
            .entries
            .keys()
            .all(|item| self.entries.contains_key(item))
    }

    // C# signature: public bool Contains(string listItemName)
    pub fn Contains_overload_2(&self, listItemName: String) -> bool {
        self.entries
            .keys()
            .any(|item| item.itemName.as_deref() == Some(listItemName.as_str()))
    }

    // C# signature: public bool GreaterThan (InkList otherList)
    pub fn GreaterThan(&self, otherList: InkList) -> bool {
        if self.entries.is_empty() {
            return false;
        }
        if otherList.entries.is_empty() {
            return true;
        }

        self.min_item().1 > otherList.max_item().1
    }

    // C# signature: public bool GreaterThanOrEquals (InkList otherList)
    pub fn GreaterThanOrEquals(&self, otherList: InkList) -> bool {
        if self.entries.is_empty() {
            return false;
        }
        if otherList.entries.is_empty() {
            return true;
        }

        self.min_item().1 >= otherList.min_item().1 && self.max_item().1 >= otherList.max_item().1
    }

    // C# signature: public bool LessThan (InkList otherList)
    pub fn LessThan(&self, otherList: InkList) -> bool {
        if otherList.entries.is_empty() {
            return false;
        }
        if self.entries.is_empty() {
            return true;
        }

        self.max_item().1 < otherList.min_item().1
    }

    // C# signature: public bool LessThanOrEquals (InkList otherList)
    pub fn LessThanOrEquals(&self, otherList: InkList) -> bool {
        if otherList.entries.is_empty() {
            return false;
        }
        if self.entries.is_empty() {
            return true;
        }

        self.max_item().1 <= otherList.max_item().1 && self.min_item().1 <= otherList.min_item().1
    }

    // C# signature: public InkList MaxAsList ()
    pub fn MaxAsList(&self) -> InkList {
        if self.entries.is_empty() {
            InkList::new()
        } else {
            InkList::new_overload_4(self.max_item())
        }
    }

    // C# signature: public InkList MinAsList ()
    pub fn MinAsList(&self) -> InkList {
        if self.entries.is_empty() {
            InkList::new()
        } else {
            InkList::new_overload_4(self.min_item())
        }
    }

    // C# signature: public InkList ListWithSubRange(object minBound, object maxBound)
    pub fn ListWithSubRange(&self, minBound: ListBound, maxBound: ListBound) -> InkList {
        if self.entries.is_empty() {
            return InkList::new();
        }

        let ordered = self.ordered_items();

        let minValue = match minBound {
            ListBound::Int(value) => value,
            ListBound::List(list) if !list.entries.is_empty() => list.min_item().1,
            ListBound::List(_) => 0,
        };

        let maxValue = match maxBound {
            ListBound::Int(value) => value,
            ListBound::List(list) if !list.entries.is_empty() => list.max_item().1,
            ListBound::List(_) => i32::MAX,
        };

        let mut subList = InkList::new();
        subList.origin_names = self.origin_names.clone();
        subList.origins = self.origins.clone();

        for (item, value) in ordered {
            if value >= minValue && value <= maxValue {
                subList.entries.insert(item, value);
            }
        }

        subList
    }

    // C# signature: public override bool Equals (object other)
    pub fn Equals(&self, other: &InkList) -> bool {
        self.entries.len() == other.entries.len()
            && self
                .entries
                .keys()
                .all(|item| other.entries.contains_key(item))
    }

    // C# signature: public override int GetHashCode ()
    pub fn GetHashCode(&self) -> i32 {
        self.entries
            .keys()
            .map(|item| {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                item.hash(&mut hasher);
                hasher.finish() as i32
            })
            .sum()
    }

    // C# signature: InkListItem singleItem { get; }
    pub fn get_singleItem(&self) -> InkListItem {
        self.entries
            .keys()
            .next()
            .cloned()
            .unwrap_or_else(InkListItem::Null)
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        self.to_string()
    }
}

impl std::fmt::Display for InkList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ordered = self.ordered_items();
        for (index, (item, _)) in ordered.iter().enumerate() {
            if index > 0 {
                f.write_str(", ")?;
            }
            f.write_str(item.itemName.as_deref().unwrap_or(""))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{InkList, InkListItem, ListBound};
    use crate::ListDefinition::ListDefinition;
    use crate::Story::Story;
    use std::collections::HashMap;

    #[test]
    fn list_operations_preserve_values_and_names() {
        let a = InkList::new_overload_4((
            InkListItem::new(Some("food".into()), Some("apple".into())),
            2,
        ));
        let b = InkList::new_overload_4((
            InkListItem::new(Some("food".into()), Some("pear".into())),
            5,
        ));

        let union = a.Union(b.clone());
        assert_eq!(union.get_maxItem().1, 5);
        assert!(union.Contains_overload_2("pear".into()));
        assert!(union.HasIntersection(b.clone()));
        assert!(union.GreaterThan(a.clone()) || !a.GreaterThan(b.clone()));

        let sub = union.ListWithSubRange(ListBound::Int(3), ListBound::Int(5));
        assert!(sub.Contains_overload_2("pear".into()));
        assert!(!sub.Contains_overload_2("apple".into()));
    }

    #[test]
    fn list_definition_based_queries_work() {
        let mut items = HashMap::new();
        items.insert("apple".to_string(), 2);
        let def = ListDefinition::new("food".to_string(), items);
        let mut list = InkList::new();
        list.origins = Some(vec![def.clone()]);
        list.AddItem(InkListItem::new(Some("food".into()), Some("apple".into())));

        assert_eq!(list.get_originOfMaxItem().unwrap().get_name(), "food");
        assert_eq!(list.get_singleItem().itemName.as_deref(), Some("apple"));
        assert!(list.ContainsItemNamed("apple".into()));
        assert_eq!(list.GetHashCode(), list.GetHashCode());
    }

    #[test]
    fn story_backed_construction_and_string_lookup_work() {
        let mut items = HashMap::new();
        items.insert("apples".to_string(), 2);
        let def = ListDefinition::new("food".to_string(), items);
        let story = Story::new(crate::Container::Container::new(), vec![def]);

        let mut list = InkList::new_overload_3("food".to_string(), story.clone());
        assert_eq!(list.get_originNames(), Some(vec!["food".to_string()]));

        let from_string = InkList::FromString("apples".to_string(), story);
        assert!(from_string.ContainsItemNamed("apples".to_string()));
    }
}
