// Source: ink-c-sharp/ink-engine-runtime/SearchResult.cs

use crate::Container::Container;
use crate::Container::ContentItem;
use crate::Object::Object;

#[derive(Clone, Debug)]
pub enum SearchResultObject {
    Object(Object),
    Container(Container),
    Content(ContentItem),
}

#[derive(Clone, Debug, Default)]
pub struct SearchResult {
    pub obj: Option<SearchResultObject>,
    pub approximate: bool,
}

impl SearchResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_object(obj: SearchResultObject, approximate: bool) -> Self {
        Self {
            obj: Some(obj),
            approximate,
        }
    }

    // C# signature: Runtime.Object correctObj { get; }
    pub fn get_correctObj(&self) -> Option<&SearchResultObject> {
        if self.approximate {
            None
        } else {
            self.obj.as_ref()
        }
    }

    // C# signature: Container container { get; }
    pub fn get_container(&self) -> Option<&Container> {
        match self.obj.as_ref() {
            Some(SearchResultObject::Container(container)) => Some(container),
            Some(SearchResultObject::Content(ContentItem::Container(container))) => {
                Some(container.as_ref())
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SearchResult, SearchResultObject};
    use crate::Container::Container;

    #[test]
    fn approximate_results_do_not_expose_correct_object() {
        let result =
            SearchResult::new_with_object(SearchResultObject::Container(Container::new()), true);

        assert!(result.get_correctObj().is_none());
        assert!(result.get_container().is_some());
    }
}
