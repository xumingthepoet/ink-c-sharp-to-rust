// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/InkList.cs

use crate::stub::*;
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
        let mut name_parts = fullName.split('.');
        Self {
            originName: Some(name_parts.next().unwrap().to_string()),
            itemName: Some(name_parts.next().unwrap().to_string()),
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

#[derive(Clone, Debug, Default)]
pub struct InkList {
    pub _port_marker: (),
}

impl InkList {
    // C# signature: public InkList ()
    pub fn new() -> Self {
        Default::default()
    }

    // C# signature: public InkList(InkList otherList)
    pub fn new_overload_2(_otherList: crate::stub::InkList) -> Self {
        Default::default()
    }

    // C# signature: public InkList (string singleOriginListName, Story originStory)
    pub fn new_overload_3(_singleOriginListName: String, _originStory: crate::stub::Story) -> Self {
        Default::default()
    }

    // C# signature: public InkList (KeyValuePair<InkListItem, int> singleElement)
    pub fn new_overload_4(_singleElement: (crate::stub::InkListItem, i32)) -> Self {
        Default::default()
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: public override bool Equals (object obj)
    pub fn Equals(&mut self, _obj: crate::stub::PortStub) -> bool {
        Default::default()
    }

    // C# signature: public bool Equals (InkListItem otherItem)
    pub fn Equals_overload_2(&mut self, _otherItem: crate::stub::InkListItem) -> bool {
        Default::default()
    }

    // C# signature: public static bool operator == (InkListItem left, InkListItem right)
    pub fn operator_stub(
        _left: crate::stub::InkListItem,
        _right: crate::stub::InkListItem,
    ) -> bool {
        Default::default()
    }

    // C# signature: public override int GetHashCode ()
    pub fn GetHashCode(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: public static InkList FromString(string myListItem, Story originStory)
    pub fn FromString(
        _myListItem: String,
        _originStory: crate::stub::Story,
    ) -> crate::stub::InkList {
        Default::default()
    }

    // C# signature: public void AddItem (InkListItem item)
    pub fn AddItem(&mut self, _item: crate::stub::InkListItem) {}

    // C# signature: public void AddItem(string itemName, Story storyObject = null)
    pub fn AddItem_overload_2(&mut self, _itemName: String, _storyObject: crate::stub::Story) {}

    // C# signature: public bool ContainsItemNamed (string itemName)
    pub fn ContainsItemNamed(&mut self, _itemName: String) -> bool {
        Default::default()
    }

    // C# signature: public void SetInitialOriginName (string initialOriginName)
    pub fn SetInitialOriginName(&mut self, _initialOriginName: String) {}

    // C# signature: public void SetInitialOriginNames (List<string> initialOriginNames)
    pub fn SetInitialOriginNames(&mut self, _initialOriginNames: Vec<String>) {}

    // C# signature: public InkList Union (InkList otherList)
    pub fn Union(&mut self, _otherList: crate::stub::InkList) -> crate::stub::InkList {
        Default::default()
    }

    // C# signature: public InkList Intersect (InkList otherList)
    pub fn Intersect(&mut self, _otherList: crate::stub::InkList) -> crate::stub::InkList {
        Default::default()
    }

    // C# signature: public bool HasIntersection(InkList otherList)
    pub fn HasIntersection(&mut self, _otherList: crate::stub::InkList) -> bool {
        Default::default()
    }

    // C# signature: public InkList Without (InkList listToRemove)
    pub fn Without(&mut self, _listToRemove: crate::stub::InkList) -> crate::stub::InkList {
        Default::default()
    }

    // C# signature: public bool Contains (InkList otherList)
    pub fn Contains(&mut self, _otherList: crate::stub::InkList) -> bool {
        Default::default()
    }

    // C# signature: public bool Contains(string listItemName)
    pub fn Contains_overload_2(&mut self, _listItemName: String) -> bool {
        Default::default()
    }

    // C# signature: public bool GreaterThan (InkList otherList)
    pub fn GreaterThan(&mut self, _otherList: crate::stub::InkList) -> bool {
        Default::default()
    }

    // C# signature: public bool GreaterThanOrEquals (InkList otherList)
    pub fn GreaterThanOrEquals(&mut self, _otherList: crate::stub::InkList) -> bool {
        Default::default()
    }

    // C# signature: public bool LessThan (InkList otherList)
    pub fn LessThan(&mut self, _otherList: crate::stub::InkList) -> bool {
        Default::default()
    }

    // C# signature: public bool LessThanOrEquals (InkList otherList)
    pub fn LessThanOrEquals(&mut self, _otherList: crate::stub::InkList) -> bool {
        Default::default()
    }

    // C# signature: public InkList MaxAsList ()
    pub fn MaxAsList(&mut self) -> crate::stub::InkList {
        Default::default()
    }

    // C# signature: public InkList MinAsList ()
    pub fn MinAsList(&mut self) -> crate::stub::InkList {
        Default::default()
    }

    // C# signature: public InkList ListWithSubRange(object minBound, object maxBound)
    pub fn ListWithSubRange(
        &mut self,
        _minBound: crate::stub::PortStub,
        _maxBound: crate::stub::PortStub,
    ) -> crate::stub::InkList {
        Default::default()
    }

    // C# signature: public override bool Equals (object other)
    pub fn Equals_overload_3(&mut self, _other: crate::stub::PortStub) -> bool {
        Default::default()
    }

    // C# signature: InkListItem Null { get; }
    pub fn get_Null() -> crate::stub::InkListItem {
        Default::default()
    }

    // C# signature: bool isNull { get; }
    pub fn get_isNull(&mut self) -> bool {
        Default::default()
    }

    // C# signature: string fullName { get; }
    pub fn get_fullName(&mut self) -> String {
        Default::default()
    }

    // C# signature: ListDefinition originOfMaxItem { get; }
    pub fn get_originOfMaxItem(&mut self) -> crate::stub::ListDefinition {
        Default::default()
    }

    // C# signature: List<string> originNames { get; }
    pub fn get_originNames(&mut self) -> Vec<String> {
        Default::default()
    }

    // C# signature: InkList inverse { get; }
    pub fn get_inverse(&mut self) -> crate::stub::InkList {
        Default::default()
    }

    // C# signature: InkList all { get; }
    pub fn get_all(&mut self) -> crate::stub::InkList {
        Default::default()
    }

    // C# signature: InkListItem singleItem { get; }
    pub fn get_singleItem(&mut self) -> crate::stub::InkListItem {
        Default::default()
    }
}
