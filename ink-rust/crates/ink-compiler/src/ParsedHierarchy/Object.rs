// Source: ink-c-sharp/compiler/ParsedHierarchy/Object.cs

use crate::ParsedHierarchy::AuthorWarning::AuthorWarning;
use crate::ParsedHierarchy::Choice::Choice;
use crate::ParsedHierarchy::ConstantDeclaration::ConstantDeclaration;
use crate::ParsedHierarchy::ContentList::ContentList;
use crate::ParsedHierarchy::Expression::{Expression, ExpressionParentContext};
use crate::ParsedHierarchy::ExternalDeclaration::ExternalDeclaration;
use crate::ParsedHierarchy::FlowLevel::FlowLevel;
use crate::ParsedHierarchy::Gather::Gather;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::IncludedFile::IncludedFile;
use crate::ParsedHierarchy::Knot::Knot;
use crate::ParsedHierarchy::ListDefinition::ListDefinition;
use crate::ParsedHierarchy::Return::Return;
use crate::ParsedHierarchy::Stitch::Stitch;
use crate::ParsedHierarchy::VariableAssignment::VariableAssignment;
use crate::ParsedHierarchy::Weave::Weave;
use ink_runtime::Container::Container;
use ink_runtime::Container::ContentItem;
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

#[derive(Clone, Debug)]
pub enum ObjectPayload {
    AuthorWarning(AuthorWarning),
    Choice(Box<Choice>),
    ConstantDeclaration(Box<ConstantDeclaration>),
    ContentList(Box<ContentList>),
    Expression(Box<Expression>),
    ExternalDeclaration(Box<ExternalDeclaration>),
    Gather(Box<Gather>),
    IncludedFile(Box<IncludedFile>),
    Knot(Box<Knot>),
    ListDefinition(Box<ListDefinition>),
    Return(Box<Return>),
    Stitch(Box<Stitch>),
    VariableAssignment(Box<VariableAssignment>),
    Weave(Box<Weave>),
}

#[derive(Clone, Debug, Default)]
pub struct Object {
    pub kind: ObjectKind,
    pub identifier: Option<Identifier>,
    pub indentationDepth: i32,
    pub isFunction: bool,
    pub parent: Option<Box<Object>>,
    pub content: Vec<Object>,
    pub payload: Option<ObjectPayload>,
    debugMetadata: Option<DebugMetadata>,
    runtimeObject: Option<Container>,
    alreadyHadError: bool,
    alreadyHadWarning: bool,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.identifier == other.identifier
            && self.indentationDepth == other.indentationDepth
            && self.isFunction == other.isFunction
            && self.content == other.content
            && self.debugMetadata == other.debugMetadata
            && self.runtimeObject == other.runtimeObject
    }
}

impl Object {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_kind(kind: ObjectKind) -> Self {
        Self {
            kind,
            isFunction: false,
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

    pub fn from_author_warning(author_warning: AuthorWarning) -> Self {
        let mut object = Object::with_kind(ObjectKind::Plain);
        object.payload = Some(ObjectPayload::AuthorWarning(author_warning));
        object
    }

    pub fn from_choice(mut choice: Choice) -> Self {
        let identifier = choice.get_identifier().cloned();
        let indentationDepth = choice.get_indentationDepth();
        let runtimeObject = Self::container_from_content_item(choice.GenerateRuntimeObject());

        let mut object = Object::with_kind(ObjectKind::WeavePoint);
        object.set_identifier(identifier);
        object.set_indentationDepth(indentationDepth);
        object.content = choice.get_base().content.clone();
        object.set_runtimeObject(runtimeObject);
        object.payload = Some(ObjectPayload::Choice(Box::new(choice)));
        object
    }

    pub fn from_constant_declaration(declaration: ConstantDeclaration) -> Self {
        let mut object = Object::with_kind(ObjectKind::Plain);
        object.set_identifier(declaration.get_constantIdentifier().cloned());
        object.payload = Some(ObjectPayload::ConstantDeclaration(Box::new(declaration)));
        object
    }

    pub fn from_content_list(content_list: ContentList) -> Self {
        let mut object = Object::with_kind(ObjectKind::Plain);
        object.payload = Some(ObjectPayload::ContentList(Box::new(content_list)));
        object
    }

    pub fn from_expression(expression: Expression) -> Self {
        let runtimeObject = Some(expression.GenerateRuntimeObject());
        let mut object = Object::with_kind(ObjectKind::Plain);
        object.set_runtimeObject(runtimeObject);
        object.payload = Some(ObjectPayload::Expression(Box::new(expression)));
        object
    }

    pub fn from_external_declaration(declaration: ExternalDeclaration) -> Self {
        let mut object = Object::with_kind(ObjectKind::Plain);
        object.set_identifier(declaration.get_identifier().cloned());
        object.payload = Some(ObjectPayload::ExternalDeclaration(Box::new(declaration)));
        object
    }

    pub fn from_gather(mut gather: Gather) -> Self {
        let identifier = gather.get_identifier().cloned();
        let indentationDepth = gather.get_indentationDepth();
        let runtimeObject = Self::container_from_content_item(gather.GenerateRuntimeObject());

        let mut object = Object::with_kind(ObjectKind::WeavePoint);
        object.set_identifier(identifier);
        object.set_indentationDepth(indentationDepth);
        object.set_runtimeObject(runtimeObject);
        object.payload = Some(ObjectPayload::Gather(Box::new(gather)));
        object
    }

    pub fn from_included_file(included: IncludedFile) -> Self {
        let mut object = Object::with_kind(ObjectKind::Plain);
        object.content = included.get_includedStory().content.clone();
        object.payload = Some(ObjectPayload::IncludedFile(Box::new(included)));
        object
    }

    pub fn from_list_definition(list_definition: ListDefinition) -> Self {
        let mut object = Object::with_kind(ObjectKind::Plain);
        object.set_identifier(list_definition.identifier.clone());
        object.payload = Some(ObjectPayload::ListDefinition(Box::new(list_definition)));
        object
    }

    pub fn from_knot(knot: Knot) -> Self {
        let mut object = Object::with_kind(ObjectKind::Knot);
        object.isFunction = knot.get_base().get_isFunction();
        object.set_identifier(knot.get_identifier().cloned());
        object.content = knot.get_base().base.content.clone();
        object.set_debugMetadata(
            knot.get_identifier()
                .and_then(|identifier| identifier.debugMetadata.clone()),
        );
        object.payload = Some(ObjectPayload::Knot(Box::new(knot)));
        object
    }

    pub fn from_return(returned: Return) -> Self {
        let runtimeObject = Some(returned.GenerateRuntimeObject());
        let mut object = Object::with_kind(ObjectKind::Plain);
        object.set_runtimeObject(runtimeObject);
        object.payload = Some(ObjectPayload::Return(Box::new(returned)));
        object
    }

    pub fn from_stitch(stitch: Stitch) -> Self {
        let mut object = Object::with_kind(ObjectKind::Stitch);
        object.isFunction = stitch.get_base().get_isFunction();
        object.set_identifier(stitch.get_identifier().cloned());
        object.content = stitch.get_base().base.content.clone();
        object.set_debugMetadata(
            stitch
                .get_identifier()
                .and_then(|identifier| identifier.debugMetadata.clone()),
        );
        object.payload = Some(ObjectPayload::Stitch(Box::new(stitch)));
        object
    }

    pub fn from_variable_assignment(mut assignment: VariableAssignment) -> Self {
        let runtimeObject = assignment
            .GenerateRuntimeObject()
            .and_then(Self::container_from_content_item);
        let mut object = Object::with_kind(ObjectKind::Plain);
        object.set_identifier(assignment.get_variableIdentifier().cloned());
        object.content = assignment.get_base().content.clone();
        object.set_runtimeObject(runtimeObject);
        object.payload = Some(ObjectPayload::VariableAssignment(Box::new(assignment)));
        object
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
        self.EnsureRuntimeObject().unwrap_or_else(|| {
            panic!("Object::GenerateRuntimeObject called for parsed object with no runtime output")
        })
    }

    pub fn EnsureRuntimeObject(&mut self) -> Option<Container> {
        if self.runtimeObject.is_some() {
            return self.runtimeObject.clone();
        }

        let generated = match self.payload.as_mut() {
            Some(ObjectPayload::AuthorWarning(warning)) => {
                let _ = warning.GenerateRuntimeObject();
                None
            }
            Some(ObjectPayload::Choice(choice)) => {
                Self::container_from_content_item(choice.GenerateRuntimeObject())
            }
            Some(ObjectPayload::ConstantDeclaration(declaration)) => {
                let _ = declaration.GenerateRuntimeObject();
                None
            }
            Some(ObjectPayload::ContentList(content_list)) => {
                Some(content_list.GenerateRuntimeObject())
            }
            Some(ObjectPayload::Expression(expression)) => Some(expression.GenerateRuntimeObject()),
            Some(ObjectPayload::ExternalDeclaration(declaration)) => {
                let _ = declaration.GenerateRuntimeObject();
                None
            }
            Some(ObjectPayload::Gather(gather)) => {
                Self::container_from_content_item(gather.GenerateRuntimeObject())
            }
            Some(ObjectPayload::IncludedFile(included)) => {
                let _ = included.GenerateRuntimeObject();
                None
            }
            Some(ObjectPayload::Knot(knot)) => Some(knot.GenerateRuntimeObject()),
            Some(ObjectPayload::ListDefinition(list_definition)) => {
                let _ = list_definition.GenerateRuntimeObject();
                None
            }
            Some(ObjectPayload::Return(returned)) => Some(returned.GenerateRuntimeObject()),
            Some(ObjectPayload::Stitch(stitch)) => Some(stitch.GenerateRuntimeObject()),
            Some(ObjectPayload::VariableAssignment(assignment)) => assignment
                .GenerateRuntimeObject()
                .and_then(Self::container_from_content_item),
            Some(ObjectPayload::Weave(weave)) => {
                Self::container_from_content_item(weave.GenerateRuntimeObject())
            }
            None => None,
        };

        if let Some(mut generated) = generated {
            if generated.get_debugMetadata().is_none() {
                generated.set_debugMetadata(self.get_debugMetadata().cloned());
            }
            self.runtimeObject = Some(generated);
        }

        self.runtimeObject.clone()
    }

    pub fn ResolveReferences(&mut self, context: &mut crate::ParsedHierarchy::Story::Story) {
        let handled_by_payload = match self.payload.as_mut() {
            Some(ObjectPayload::AuthorWarning(_)) => true,
            Some(ObjectPayload::Choice(choice)) => {
                choice.ResolveReferences(context);
                true
            }
            Some(ObjectPayload::ConstantDeclaration(declaration)) => {
                declaration.ResolveReferences(context);
                true
            }
            Some(ObjectPayload::ContentList(content_list)) => {
                content_list.ResolveReferences(context);
                true
            }
            Some(ObjectPayload::Expression(expression)) => {
                let parent_context = match self.parent.as_deref() {
                    Some(parent)
                        if matches!(
                            parent.kind,
                            ObjectKind::Weave
                                | ObjectKind::FlowBase
                                | ObjectKind::Knot
                                | ObjectKind::Stitch
                                | ObjectKind::Story
                        ) =>
                    {
                        Some(ExpressionParentContext::FlowBase)
                    }
                    Some(parent)
                        if parent
                            .payload
                            .as_ref()
                            .map(|payload| matches!(payload, ObjectPayload::ContentList(_)))
                            .unwrap_or(false) =>
                    {
                        Some(ExpressionParentContext::ContentList)
                    }
                    _ => Some(ExpressionParentContext::Other),
                };
                expression.set_parentContext(parent_context);
                expression.ResolveReferences(context);
                true
            }
            Some(ObjectPayload::ExternalDeclaration(_)) => true,
            Some(ObjectPayload::Gather(gather)) => {
                gather.ResolveReferences(context);
                true
            }
            Some(ObjectPayload::IncludedFile(_)) => true,
            Some(ObjectPayload::Knot(knot)) => {
                knot.ResolveReferences(context);
                true
            }
            Some(ObjectPayload::ListDefinition(list_definition)) => {
                list_definition.ResolveReferences(context);
                true
            }
            Some(ObjectPayload::Return(_)) => true,
            Some(ObjectPayload::Stitch(stitch)) => {
                stitch.ResolveReferences(context);
                true
            }
            Some(ObjectPayload::VariableAssignment(assignment)) => {
                assignment.ResolveReferences(context);
                true
            }
            Some(ObjectPayload::Weave(weave)) => {
                weave.ResolveReferences(context);
                true
            }
            None => false,
        };

        if !handled_by_payload {
            for obj in &mut self.content {
                obj.ResolveReferences(context);
            }
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
        match self.payload.as_ref() {
            Some(ObjectPayload::ListDefinition(_)) => "List definition".to_string(),
            _ => match self.kind {
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
            .to_string(),
        }
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

    fn container_from_content_item(content_item: ContentItem) -> Option<Container> {
        match content_item {
            ContentItem::Container(container) => Some(container.as_ref().clone()),
            other => {
                let mut container = Container::new();
                container.AddContent(other);
                Some(container)
            }
        }
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

impl From<ContentList> for Object {
    fn from(value: ContentList) -> Self {
        Object::from_content_list(value)
    }
}

impl From<Weave> for Object {
    fn from(mut value: Weave) -> Self {
        let runtimeObject = match value.GenerateRuntimeObject() {
            ContentItem::Container(container) => Some(container.as_ref().clone()),
            _ => None,
        };

        let mut object = value.base.clone();
        object.set_runtimeObject(runtimeObject);
        object.payload = Some(ObjectPayload::Weave(Box::new(value)));
        object
    }
}

#[cfg(test)]
mod tests {
    use super::{Object, ObjectKind};
    use crate::ParsedHierarchy::ContentList::{ContentList, ContentListItem};
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::Text::Text;
    use crate::ParsedHierarchy::Weave::Weave;

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

    #[test]
    fn from_content_list_and_weave_preserves_runtime_object() {
        let content_list =
            ContentList::new(vec![ContentListItem::from(Text::new("x".to_string()))]);
        let mut content_object = Object::from(content_list);
        assert!(content_object.get_runtimeObject().is_none());
        assert!(content_object.EnsureRuntimeObject().is_some());
        assert_eq!(content_object.get_typeName(), "Object");

        let weave = Weave::new(vec![], -1);
        let weave_object = Object::from(weave);
        assert!(weave_object.get_runtimeObject().is_some());
        assert_eq!(weave_object.get_typeName(), "Weave");
    }
}
