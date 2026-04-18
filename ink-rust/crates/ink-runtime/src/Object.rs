// Source: ink-c-sharp/ink-engine-runtime/Object.cs

use crate::Container::Container;
use crate::DebugMetadata::DebugMetadata;
use crate::Path::Path;
use crate::SearchResult::SearchResult;
use std::rc::Rc;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Object {
    parent: Option<Rc<Container>>,
    debug_metadata: Option<DebugMetadata>,
    path: Option<Path>,
}

impl Object {
    // C# signature: public Object ()
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public int? DebugLineNumberOfPath(Path path)
    pub fn DebugLineNumberOfPath(&self, path: Path) -> Option<i32> {
        if path.get_length() == 0 {
            return None;
        }

        let root = self.get_rootContentContainer();
        let Some(mut root) = root else {
            return None;
        };

        let target_content = root.ContentAtPath(path, 0, -1).obj?;
        match target_content {
            crate::Container::ContentItem::Container(container) => container
                .get_debugMetadata()
                .map(|debug_metadata| debug_metadata.startLineNumber),
            _ => None,
        }
    }

    // C# signature: public SearchResult ResolvePath(Path path)
    pub fn ResolvePath(&self, path: Path) -> SearchResult {
        if path.get_isRelative() {
            let Some(parent) = self.parent.clone() else {
                return SearchResult {
                    obj: None,
                    approximate: true,
                };
            };

            let mut nearest_container = (*parent).clone();
            let resolved_path = if path.get_length() > 0
                && path
                    .GetComponent(0)
                    .map(|component| component.get_isParent())
                    .unwrap_or(false)
            {
                path.get_tail()
            } else {
                path
            };

            return nearest_container.ContentAtPath(resolved_path, 0, -1);
        }

        if let Some(mut root) = self.get_rootContentContainer() {
            root.ContentAtPath(path, 0, -1)
        } else {
            SearchResult {
                obj: None,
                approximate: true,
            }
        }
    }

    // C# signature: public Path ConvertPathToRelative(Path globalPath)
    pub fn ConvertPathToRelative(&self, globalPath: Path) -> Path {
        let ownPath = self.get_path();

        let minPathLength = ownPath.get_length().min(globalPath.get_length());
        let mut lastSharedPathCompIndex = -1;

        for i in 0..minPathLength {
            let Some(ownComp) = ownPath.GetComponent(i) else {
                break;
            };
            let Some(otherComp) = globalPath.GetComponent(i) else {
                break;
            };

            if ownComp.Equals(otherComp) {
                lastSharedPathCompIndex = i;
            } else {
                break;
            }
        }

        if lastSharedPathCompIndex == -1 {
            return globalPath;
        }

        let numUpwardsMoves = (ownPath.get_length() - 1) - lastSharedPathCompIndex;
        let mut newPathComps = Vec::new();

        for _ in 0..numUpwardsMoves {
            newPathComps.push(Path::ToParent());
        }

        for down in (lastSharedPathCompIndex + 1)..globalPath.get_length() {
            if let Some(component) = globalPath.GetComponent(down) {
                newPathComps.push(component.clone());
            }
        }

        Path::new_overload_3(newPathComps, true)
    }

    // C# signature: public string CompactPathString(Path otherPath)
    pub fn CompactPathString(&self, otherPath: Path) -> String {
        let (relativePathStr, globalPathStr) = if otherPath.get_isRelative() {
            (
                otherPath.get_componentsString(),
                self.get_path()
                    .PathByAppendingPath(&otherPath)
                    .get_componentsString(),
            )
        } else {
            let relativePath = self.ConvertPathToRelative(otherPath.clone());
            (
                relativePath.get_componentsString(),
                otherPath.get_componentsString(),
            )
        };

        if relativePathStr.len() < globalPathStr.len() {
            relativePathStr
        } else {
            globalPathStr
        }
    }

    // C# signature: public virtual Object Copy()
    pub fn Copy(&self) -> Self {
        panic!("{} doesn't support copying", std::any::type_name::<Self>())
    }

    // C# signature: public void SetChild<T>(ref T obj, T value)
    pub fn SetChild<T>(&mut self, obj: &mut T, value: T) {
        *obj = value;
    }

    // C# signature: Runtime.Object parent { get; }
    pub fn get_parent(&self) -> Option<Rc<Container>> {
        self.parent.clone()
    }

    pub fn set_parent(&mut self, parent: Option<Rc<Container>>) {
        self.parent = parent;
    }

    // C# signature: Runtime.DebugMetadata debugMetadata { get; }
    pub fn get_debugMetadata(&self) -> Option<&DebugMetadata> {
        self.debug_metadata.as_ref().or_else(|| {
            self.parent
                .as_deref()
                .and_then(|parent| parent.get_debugMetadata())
        })
    }

    pub fn set_debugMetadata(&mut self, debug_metadata: Option<DebugMetadata>) {
        self.debug_metadata = debug_metadata;
    }

    // C# signature: Runtime.DebugMetadata ownDebugMetadata { get; }
    pub fn get_ownDebugMetadata(&self) -> Option<&DebugMetadata> {
        self.debug_metadata.as_ref()
    }

    // C# signature: Path path { get; }
    pub fn get_path(&self) -> Path {
        self.path.clone().unwrap_or_else(Path::new)
    }

    // C# signature: Container rootContentContainer { get; }
    pub fn get_rootContentContainer(&self) -> Option<Container> {
        let mut ancestor = self.parent.clone()?;
        while let Some(parent) = ancestor.get_parent() {
            ancestor = parent;
        }
        Some((*ancestor).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::Object;
    use crate::Container::Container;
    use crate::DebugMetadata::DebugMetadata;
    use crate::Path::{Component, Path};
    use std::rc::Rc;

    #[test]
    fn compact_path_prefers_shorter_representation() {
        let mut obj = Object::new();
        obj.path = Some(Path::new_overload_3(
            vec![Component::new_overload_2("chapter".to_string())],
            false,
        ));

        let target = Path::new_overload_3(
            vec![
                Component::new_overload_2("chapter".to_string()),
                Component::new_overload_2("scene".to_string()),
            ],
            false,
        );

        assert_eq!(obj.CompactPathString(target), ".scene");
    }

    #[test]
    fn returns_root_content_container_from_parent_chain() {
        let mut root = Container::new();
        root.set_name(Some("root".to_string()));

        let mut child = Container::new();
        child.set_name(Some("child".to_string()));
        root.AddContent(child.clone());

        let inserted_child = match root.get_content().first() {
            Some(crate::Container::ContentItem::Container(container)) => container.as_ref().clone(),
            _ => panic!("child container missing"),
        };

        let mut obj = Object::new();
        obj.parent = Some(Rc::new(inserted_child));

        let root_container = obj.get_rootContentContainer().expect("root container");
        assert_eq!(root_container.get_name(), "root");
    }

    #[test]
    fn reads_debug_line_number_from_container_metadata() {
        let mut root = Container::new();

        let mut child = Container::new();
        let mut debug = DebugMetadata::new();
        debug.startLineNumber = 42;
        child.set_debugMetadata(Some(debug));

        root.AddContent(child);

        let mut obj = Object::new();
        obj.parent = Some(Rc::new(root));

        assert_eq!(
            obj.DebugLineNumberOfPath(Path::new_overload_3(vec![Component::new(0)], false)),
            Some(42)
        );
    }
}
