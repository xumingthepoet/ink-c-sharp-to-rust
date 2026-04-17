// Source: ink-c-sharp/compiler/ParsedHierarchy/Object.cs

use crate::ParsedHierarchy::FlowLevel::FlowLevel;
use crate::ParsedHierarchy::Identifier::Identifier;
use ink_runtime::Container::Container;
use ink_runtime::DebugMetadata::DebugMetadata;
use ink_runtime::Path::{Component, Path};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObjectKind {
    Plain,
    Story,
    FlowBase,
    WeavePoint,
    Weave,
    Knot,
    Stitch,
    Sequence,
    Conditional,
    ConditionalSingleBranch,
}

impl Default for ObjectKind {
    fn default() -> Self {
        Self::Plain
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Object {
    pub kind: ObjectKind,
    pub identifier: Option<Identifier>,
    pub indentationDepth: i32,
    pub parent: Option<Box<Object>>,
    pub content: Vec<Object>,
    debugMetadata: Option<DebugMetadata>,
    runtimeObject: Option<Container>,
    alreadyHadError: bool,
    alreadyHadWarning: bool,
}

impl Object {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_kind(kind: ObjectKind) -> Self {
        Self {
            kind,
            ..Default::default()
        }
    }

    pub fn set_kind(&mut self, kind: ObjectKind) {
        self.kind = kind;
    }

    pub fn set_identifier(&mut self, identifier: Option<Identifier>) {
        self.identifier = identifier;
    }

    pub fn set_indentationDepth(&mut self, indentationDepth: i32) {
        self.indentationDepth = indentationDepth;
    }

    pub fn PathRelativeTo(&self, otherObj: &Object) -> Option<Path> {
        let ownAncestry = self.get_ancestry();
        let otherAncestry = otherObj.get_ancestry();

        let mut highestCommonAncestorIndex: Option<usize> = None;
        let minLength = ownAncestry.len().min(otherAncestry.len());
        for i in 0..minLength {
            if ownAncestry[i] == otherAncestry[i] {
                highestCommonAncestorIndex = Some(i);
            } else {
                break;
            }
        }

        let Some(commonIndex) = highestCommonAncestorIndex else {
            return None;
        };

        let commonFlowAncestor = ownAncestry[commonIndex]
            .ClosestFlowBase()
            .or_else(|| ownAncestry[commonIndex].as_flow_like_ancestor());

        let mut pathComponents = Vec::new();
        let mut hasWeavePoint = false;
        let mut baseFlow = FlowLevel::WeavePoint;

        let mut ancestor: Option<&Object> = Some(self);
        while let Some(current) = ancestor {
            if let Some(common) = &commonFlowAncestor {
                if current == common {
                    break;
                }
            }

            if !hasWeavePoint {
                if current.kind == ObjectKind::WeavePoint {
                    if let Some(identifier) = &current.identifier {
                        pathComponents.push(identifier.clone());
                        hasWeavePoint = true;
                        ancestor = current.parent.as_deref();
                        continue;
                    }
                }
            }

            if current.kind == ObjectKind::FlowBase
                || current.kind == ObjectKind::Story
                || current.kind == ObjectKind::Knot
                || current.kind == ObjectKind::Stitch
            {
                if let Some(identifier) = &current.identifier {
                    pathComponents.push(identifier.clone());
                }
                baseFlow = current.flow_level();
            }

            ancestor = current.parent.as_deref();
        }

        pathComponents.reverse();

        if pathComponents.is_empty() {
            None
        } else {
            Some(Path::new_overload_3(
                pathComponents
                    .into_iter()
                    .filter_map(|identifier| identifier.name.map(Component::new_overload_2))
                    .collect(),
                baseFlow == FlowLevel::WeavePoint,
            ))
        }
    }

    pub fn AddContent<T>(&mut self, subContent: T) -> T
    where
        T: Into<Object> + Clone,
    {
        let mut inserted = subContent.clone().into();
        inserted.parent = Some(Box::new(self.clone()));
        self.content.push(inserted);
        subContent
    }

    pub fn AddContent_overload_2<T>(&mut self, listContent: Vec<T>)
    where
        T: Into<Object> + Clone,
    {
        for obj in listContent {
            self.AddContent(obj);
        }
    }

    pub fn InsertContent<T>(&mut self, index: i32, subContent: T) -> T
    where
        T: Into<Object> + Clone,
    {
        let mut inserted = subContent.clone().into();
        inserted.parent = Some(Box::new(self.clone()));
        let index = index.max(0) as usize;
        if index >= self.content.len() {
            self.content.push(inserted);
        } else {
            self.content.insert(index, inserted);
        }
        subContent
    }

    pub fn Find(&self, queryFunc: Option<fn(&Object) -> bool>) -> Option<Object> {
        let matches = queryFunc.map(|f| f(self)).unwrap_or(true);
        if matches {
            return Some(self.clone());
        }

        for obj in &self.content {
            if let Some(found) = obj.Find(queryFunc) {
                return Some(found);
            }
        }

        None
    }

    pub fn FindAll(&self, queryFunc: Option<fn(&Object) -> bool>) -> Vec<Object> {
        let mut found = Vec::new();
        self.find_all_inner(queryFunc, &mut found);
        found
    }

    fn find_all_inner(&self, queryFunc: Option<fn(&Object) -> bool>, foundSoFar: &mut Vec<Object>) {
        let matches = queryFunc.map(|f| f(self)).unwrap_or(true);
        if matches {
            foundSoFar.push(self.clone());
        }

        for obj in &self.content {
            obj.find_all_inner(queryFunc, foundSoFar);
        }
    }

    pub fn GenerateRuntimeObject(&mut self) -> Container {
        panic!("Object::GenerateRuntimeObject is abstract in C# and needs a concrete port");
    }

    pub fn ResolveReferences(&mut self, context: &mut crate::ParsedHierarchy::Story::Story) {
        for obj in &mut self.content {
            obj.ResolveReferences(context);
        }
    }

    pub fn ClosestFlowBase(&self) -> Option<Object> {
        let mut ancestor = self.parent.as_deref();
        while let Some(current) = ancestor {
            if current.kind == ObjectKind::FlowBase
                || current.kind == ObjectKind::Story
                || current.kind == ObjectKind::Knot
                || current.kind == ObjectKind::Stitch
            {
                return Some(current.clone());
            }
            ancestor = current.parent.as_deref();
        }

        None
    }

    pub fn Error(&mut self, message: String, source: Option<Object>, isWarning: bool) {
        let source = source.unwrap_or_else(|| self.clone());

        if source.alreadyHadError && !isWarning {
            return;
        }
        if source.alreadyHadWarning && isWarning {
            return;
        }

        if let Some(parent) = &mut self.parent {
            parent.Error(message, Some(source.clone()), isWarning);
        } else {
            panic!("No parent object to send error to: {}", message);
        }

        if isWarning {
            self.alreadyHadWarning = true;
        } else {
            self.alreadyHadError = true;
        }
    }

    pub fn Warning(&mut self, message: String, source: Option<Object>) {
        self.Error(message, source, true);
    }

    pub fn get_story(&self) -> Option<Object> {
        let mut ancestor = Some(self);
        while let Some(current) = ancestor {
            if current.kind == ObjectKind::Story {
                return Some(current.clone());
            }
            ancestor = current.parent.as_deref();
        }
        None
    }

    pub fn get_debugMetadata(&self) -> Option<&DebugMetadata> {
        self.debugMetadata.as_ref().or_else(|| {
            self.parent
                .as_deref()
                .and_then(|parent| parent.get_debugMetadata())
        })
    }

    pub fn set_debugMetadata(&mut self, debugMetadata: Option<DebugMetadata>) {
        self.debugMetadata = debugMetadata;
    }

    pub fn get_hasOwnDebugMetadata(&self) -> bool {
        self.debugMetadata.is_some()
    }

    pub fn get_typeName(&self) -> String {
        match self.kind {
            ObjectKind::Plain => "Object",
            ObjectKind::Story => "Story",
            ObjectKind::FlowBase => "FlowBase",
            ObjectKind::WeavePoint => "Weave point",
            ObjectKind::Weave => "Weave",
            ObjectKind::Knot => "Knot",
            ObjectKind::Stitch => "Stitch",
            ObjectKind::Sequence => "Sequence",
            ObjectKind::Conditional => "Conditional",
            ObjectKind::ConditionalSingleBranch => "Conditional branch",
        }
        .to_string()
    }

    pub fn get_parent(&self) -> Option<&Object> {
        self.parent.as_deref()
    }

    pub fn set_parent(&mut self, parent: Option<Box<Object>>) {
        self.parent = parent;
    }

    pub fn get_content(&self) -> &[Object] {
        &self.content
    }

    pub fn get_runtimeObject(&self) -> Option<&Container> {
        self.runtimeObject.as_ref()
    }

    pub fn set_runtimeObject(&mut self, runtimeObject: Option<Container>) {
        self.runtimeObject = runtimeObject;
    }

    pub fn get_runtimePath(&self) -> Path {
        self.runtimeObject
            .as_ref()
            .map(|runtime_object| runtime_object.get_path().clone())
            .unwrap_or_else(Path::new)
    }

    pub fn get_containerForCounting(&self) -> Option<Container> {
        self.runtimeObject.clone()
    }

    pub fn get_ancestry(&self) -> Vec<Object> {
        let mut result = Vec::new();
        let mut ancestor = self.parent.as_deref();
        while let Some(current) = ancestor {
            result.push(current.clone());
            ancestor = current.parent.as_deref();
        }
        result.reverse();
        result
    }

    pub fn get_descriptionOfScope(&self) -> String {
        let mut locationNames = Vec::new();
        let mut ancestor = Some(self);
        while let Some(current) = ancestor {
            if matches!(
                current.kind,
                ObjectKind::FlowBase | ObjectKind::Story | ObjectKind::Knot | ObjectKind::Stitch
            ) {
                if let Some(identifier) = &current.identifier {
                    if let Some(name) = &identifier.name {
                        locationNames.push(format!("'{}'", name));
                    }
                }
            }

            ancestor = current.parent.as_deref();
        }

        if locationNames.is_empty() {
            "at top scope".to_string()
        } else {
            format!("{} and at top scope", locationNames.join(", "))
        }
    }

    fn flow_level(&self) -> FlowLevel {
        match self.kind {
            ObjectKind::Story => FlowLevel::Story,
            ObjectKind::Knot => FlowLevel::Knot,
            ObjectKind::Stitch => FlowLevel::Stitch,
            _ => FlowLevel::WeavePoint,
        }
    }

    fn as_flow_like_ancestor(&self) -> Option<Object> {
        if matches!(
            self.kind,
            ObjectKind::Story | ObjectKind::FlowBase | ObjectKind::Knot | ObjectKind::Stitch
        ) {
            Some(self.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Object, ObjectKind};
    use crate::ParsedHierarchy::Identifier::Identifier;

    #[test]
    fn ancestry_and_scope_follow_parent_chain() {
        let mut root = Object::with_kind(ObjectKind::Story);
        root.set_identifier(Some(Identifier {
            name: Some("root".to_string()),
            debugMetadata: None,
        }));

        let mut child = Object::new();
        child.set_parent(Some(Box::new(root.clone())));
        let ancestry = child.get_ancestry();

        assert_eq!(ancestry.len(), 1);
        assert_eq!(ancestry[0].get_typeName(), "Story");
        assert_eq!(root.get_descriptionOfScope(), "'root' and at top scope");
    }
}
