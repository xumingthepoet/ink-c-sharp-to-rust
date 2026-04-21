// Source: ink-c-sharp/ink-engine-runtime/Divert.cs

use crate::Container::ContentItem;
use crate::Path::Path;
use crate::SearchResult::SearchResult;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Divert {
    parent: RefCell<Option<Rc<crate::Container::Container>>>,
    path: Option<Path>,
    targetPath: Option<Path>,
    variableDivertName: Option<String>,
    pushesToStack: bool,
    stackPushType: crate::PushPop::PushPopType,
    isExternal: bool,
    externalArgs: i32,
    isConditional: bool,
}

impl Default for Divert {
    fn default() -> Self {
        Self {
            parent: RefCell::new(None),
            path: None,
            targetPath: None,
            variableDivertName: None,
            pushesToStack: false,
            stackPushType: crate::PushPop::PushPopType::Tunnel,
            isExternal: false,
            externalArgs: 0,
            isConditional: false,
        }
    }
}

impl Divert {
    // C# signature: public Divert ()
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public Divert(PushPopType stackPushType)
    pub fn new_overload_2(stackPushType: crate::PushPop::PushPopType) -> Self {
        Self {
            pushesToStack: true,
            stackPushType,
            ..Default::default()
        }
    }

    // C# signature: public override bool Equals (object obj)
    pub fn Equals(&self, obj: &Divert) -> bool {
        if self.hasVariableTarget() == obj.hasVariableTarget() {
            if self.hasVariableTarget() {
                self.variableDivertName == obj.variableDivertName
            } else {
                self.targetPath == obj.targetPath
            }
        } else {
            false
        }
    }

    // C# signature: public override int GetHashCode ()
    pub fn GetHashCode(&self) -> i32 {
        if self.hasVariableTarget() {
            self.variableDivertName.as_deref().unwrap_or("").len() as i32 + 12345
        } else {
            self.targetPath
                .as_ref()
                .map(|path| path.GetHashCode())
                .unwrap_or(0)
                + 54321
        }
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        if let Some(name) = &self.variableDivertName {
            format!("Divert(variable: {})", name)
        } else if self.targetPath.is_none() {
            "Divert(null)".to_string()
        } else {
            let mut result = String::from("Divert");
            if self.isConditional {
                result.push('?');
            }
            if self.pushesToStack {
                match self.stackPushType {
                    crate::PushPop::PushPopType::Function => result.push_str(" function"),
                    crate::PushPop::PushPopType::Tunnel => result.push_str(" tunnel"),
                    _ => {}
                }
            }

            result.push_str(" -> ");
            result.push_str(&self.get_targetPathString());
            result.push_str(" (");
            result.push_str(
                &self
                    .get_targetPath()
                    .map(|path| path.ToString())
                    .unwrap_or_default(),
            );
            result.push(')');
            result
        }
    }

    fn root_content_container(&self) -> Option<crate::Container::Container> {
        let mut ancestor = self.parent.borrow().clone()?;
        while let Some(parent) = ancestor.get_parent() {
            ancestor = parent;
        }
        Some((*ancestor).clone())
    }

    fn resolve_path_from_context(&self, path: &Path) -> SearchResult {
        if path.get_isRelative() {
            if let Some(parent) = self.parent.borrow().clone() {
                let mut parent = (*parent).clone();
                let relative_path = if path.get_length() > 0
                    && path
                        .GetComponent(0)
                        .map(|component| component.get_isParent())
                        .unwrap_or(false)
                {
                    path.get_tail()
                } else {
                    path.clone()
                };
                return parent.ContentAtPath(relative_path.clone(), 0, -1);
            }

            return SearchResult {
                obj: None,
                approximate: true,
            };
        }

        if let Some(mut root) = self.root_content_container() {
            root.ContentAtPath(path.clone(), 0, -1)
        } else {
            SearchResult {
                obj: None,
                approximate: true,
            }
        }
    }

    fn path_without_last_component(path: &Path) -> Path {
        let length = path.get_length();
        if length <= 0 {
            return Path::new();
        }

        let mut components = Vec::new();
        for index in 0..(length - 1) {
            if let Some(component) = path.GetComponent(index) {
                components.push(component.clone());
            }
        }
        Path::new_overload_3(components, path.get_isRelative())
    }

    fn get_own_path(&self) -> Path {
        self.path.clone().unwrap_or_else(Path::new)
    }

    fn ConvertPathToRelative(&self, globalPath: Path) -> Path {
        let ownPath = self.get_own_path();
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
            newPathComps.push(crate::Path::Component::ToParent());
        }

        for down in (lastSharedPathCompIndex + 1)..globalPath.get_length() {
            if let Some(component) = globalPath.GetComponent(down) {
                newPathComps.push(component.clone());
            }
        }

        Path::new_overload_3(newPathComps, true)
    }

    fn CompactPathString(&self, otherPath: Path) -> String {
        let (relativePathStr, globalPathStr) = if otherPath.get_isRelative() {
            (
                otherPath.get_componentsString(),
                self.get_own_path()
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

    // C# signature: Path targetPath { get; }
    pub fn get_targetPath(&self) -> Option<Path> {
        let targetPath = self.targetPath.as_ref()?;
        if targetPath.get_isRelative() {
            let targetObj = self.get_targetPointer().Resolve();
            if let Some(crate::Container::ContentItem::Container(container)) = targetObj {
                return Some(container.get_path().clone());
            }
        }
        Some(targetPath.clone())
    }

    // C# signature: Pointer targetPointer { get; }
    pub fn get_targetPointer(&self) -> crate::Pointer::Pointer {
        let Some(target_path) = self.targetPath.as_ref() else {
            return crate::Pointer::Pointer::Null();
        };

        let target_path = target_path.clone();
        let resolution = self.resolve_path_from_context(&target_path);
        let Some(target_obj) = resolution.obj else {
            return crate::Pointer::Pointer::Null();
        };

        if let Some(last_component) = target_path.get_lastComponent() {
            if last_component.get_isIndex() {
                let target_container_path = Self::path_without_last_component(&target_path);
                let target_container = self.resolve_path_from_context(&target_container_path);
                if let Some(crate::Container::ContentItem::Container(container)) =
                    target_container.obj
                {
                    return crate::Pointer::Pointer::new(
                        container.clone(),
                        last_component.get_index(),
                    );
                }
                return crate::Pointer::Pointer::Null();
            }
        }

        match target_obj {
            crate::Container::ContentItem::Container(container) => {
                crate::Pointer::Pointer::StartOf(container.clone())
            }
            _ => crate::Pointer::Pointer::Null(),
        }
    }

    // C# signature: string targetPathString { get; }
    pub fn get_targetPathString(&self) -> String {
        self.get_targetPath()
            .map(|path| self.CompactPathString(path))
            .unwrap_or_default()
    }

    pub fn set_targetPathString(&mut self, value: Option<String>) {
        self.targetPath = value.map(Path::new_overload_4);
    }

    pub fn set_targetPath(&mut self, value: Option<Path>) {
        self.targetPath = value;
    }

    // C# signature: string variableDivertName { get; }
    pub fn get_variableDivertName(&self) -> Option<&str> {
        self.variableDivertName.as_deref()
    }

    pub fn set_variableDivertName(&mut self, value: Option<String>) {
        self.variableDivertName = value;
    }

    // C# signature: bool hasVariableTarget { get; }
    pub fn hasVariableTarget(&self) -> bool {
        self.variableDivertName.is_some()
    }

    // C# signature: bool pushesToStack { get; }
    pub fn get_pushesToStack(&self) -> bool {
        self.pushesToStack
    }

    pub fn set_pushesToStack(&mut self, value: bool) {
        self.pushesToStack = value;
    }

    pub fn get_stackPushType(&self) -> crate::PushPop::PushPopType {
        self.stackPushType
    }

    // C# signature: bool isExternal { get; }
    pub fn get_isExternal(&self) -> bool {
        self.isExternal
    }

    pub fn set_isExternal(&mut self, value: bool) {
        self.isExternal = value;
    }

    // C# signature: int externalArgs { get; }
    pub fn get_externalArgs(&self) -> i32 {
        self.externalArgs
    }

    pub fn set_externalArgs(&mut self, value: i32) {
        self.externalArgs = value;
    }

    // C# signature: bool isConditional { get; }
    pub fn get_isConditional(&self) -> bool {
        self.isConditional
    }

    pub fn set_isConditional(&mut self, value: bool) {
        self.isConditional = value;
    }

    pub fn set_parent(&self, parent: Option<Rc<crate::Container::Container>>) {
        self.parent.replace(parent);
    }

    pub fn get_parent(&self) -> Option<Rc<crate::Container::Container>> {
        self.parent.borrow().clone()
    }

    pub fn set_path(&mut self, path: Path) {
        self.path = Some(path);
    }

    pub fn get_path(&self) -> Option<&Path> {
        self.path.as_ref()
    }
}

impl PartialEq for Divert {
    fn eq(&self, other: &Self) -> bool {
        self.Equals(other)
    }
}

#[cfg(test)]
mod tests {
    use super::Divert;
    use crate::Container::{Container, ContentItem};
    use crate::ControlCommand::ControlCommand;
    use crate::Path::{Component, Path};
    use crate::PushPop::PushPopType;
    use crate::Value::Value;
    use std::collections::HashMap;
    use std::rc::Rc;

    #[test]
    fn stringifies_variable_and_stack_diverts() {
        let mut divert = Divert::new();
        divert.set_variableDivertName(Some("score".to_string()));
        assert_eq!(divert.ToString(), "Divert(variable: score)");

        let mut call = Divert::new_overload_2(PushPopType::Function);
        call.set_targetPathString(Some("knot.stitch".to_string()));
        assert!(call.ToString().contains("Divert function -> knot.stitch"));
    }

    #[test]
    fn resolves_relative_target_pointers_from_parent_container() {
        let mut child = Container::new();
        child.AddContent(ControlCommand::BeginString());

        let mut root = Container::new();
        root.AddContent(child);

        let mut divert = Divert::new();
        divert.set_targetPathString(Some(".0".to_string()));
        root.AddContent(divert);

        let stored_divert = match root.get_content().get(1) {
            Some(ContentItem::Divert(divert)) => divert,
            _ => panic!("divert missing"),
        };

        let pointer = stored_divert.get_targetPointer();
        assert!(matches!(pointer.Resolve(), Some(ContentItem::Container(_))));
        assert_eq!(pointer.get_path().unwrap().ToString(), "0");
    }

    #[test]
    fn resolves_named_only_target_pointers_from_parent_chain() {
        let mut named_only_child = Container::new();
        named_only_child.set_name(Some("s".to_string()));
        named_only_child.AddContent(ControlCommand::BeginString());

        let mut inner = Container::new();
        let mut named_only = HashMap::new();
        named_only.insert(
            "s".to_string(),
            ContentItem::Container(Rc::new(named_only_child.clone())),
        );
        inner.set_namedOnlyContent(Some(named_only));

        let mut branch = Container::new();
        let mut divert = Divert::new();
        divert.set_targetPathString(Some("0.2.s".to_string()));
        branch.AddContent(divert);

        let mut body = Container::new();
        body.AddContent(Value::new_string("hello".to_string()));
        body.AddContent(Value::new_string("\n".to_string()));
        body.AddContent(inner.clone());
        body.AddContent(branch.clone());

        let mut root = Container::new();
        root.AddContent(body.clone());

        let direct = root.ContentAtPath(
            Path::new_overload_3(
                vec![
                    Component::new(0),
                    Component::new(2),
                    Component::new_overload_2("s".to_string()),
                ],
                false,
            ),
            0,
            -1,
        );
        assert!(!direct.approximate);
        assert_eq!(direct.get_container().map(|c| c.get_name()), Some("s"));
        let stored_body = match root.get_content().first() {
            Some(ContentItem::Container(container)) => container.as_ref().clone(),
            _ => panic!("body container missing"),
        };

        let stored_branch = match stored_body.get_content().get(3) {
            Some(ContentItem::Container(container)) => container.as_ref().clone(),
            _ => panic!("branch container missing"),
        };

        let stored_divert = match stored_branch.get_content().first() {
            Some(ContentItem::Divert(divert)) => divert,
            _ => panic!("divert missing"),
        };

        let mut root_from_divert = stored_divert
            .get_parent()
            .and_then(|parent| parent.get_rootContentContainer())
            .expect("root from divert");
        let via_divert_root = root_from_divert.ContentAtPath(
            Path::new_overload_3(
                vec![
                    Component::new(0),
                    Component::new(2),
                    Component::new_overload_2("s".to_string()),
                ],
                false,
            ),
            0,
            -1,
        );
        let pointer = stored_divert.get_targetPointer();
        assert!(!pointer.get_isNull());
        assert_eq!(pointer.get_path().unwrap().ToString(), "0.2.s.0");
    }
}
