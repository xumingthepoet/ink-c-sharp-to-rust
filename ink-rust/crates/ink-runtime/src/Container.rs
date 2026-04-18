// Source: ink-c-sharp/ink-engine-runtime/Container.cs

use crate::Choice::Choice;
use crate::ChoicePoint::ChoicePoint;
use crate::ControlCommand::ControlCommand;
use crate::DebugMetadata::DebugMetadata;
use crate::Divert::Divert;
use crate::Glue::Glue;
use crate::NativeFunctionCall::NativeFunctionCall;
use crate::Path::{Component, Path};
use crate::SearchResult::SearchResult;
use crate::Tag::Tag;
use crate::Value::{ListValue, StringValue, Value};
use crate::VariableAssignment::VariableAssignment;
use crate::VariableReference::VariableReference;
use crate::Void::Void;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_CONTAINER_UID: AtomicUsize = AtomicUsize::new(1);

fn next_container_uid() -> usize {
    NEXT_CONTAINER_UID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContentItem {
    Value(Value),
    ControlCommand(ControlCommand),
    Void(Void),
    Container(Rc<Container>),
    VariableReference(VariableReference),
    Divert(Divert),
    ChoicePoint(ChoicePoint),
    Glue(Glue),
    NativeFunctionCall(NativeFunctionCall),
    VariableAssignment(VariableAssignment),
    Tag(Tag),
    Choice(Choice),
}

impl From<Value> for ContentItem {
    fn from(value: Value) -> Self {
        Self::Value(value)
    }
}

impl From<StringValue> for ContentItem {
    fn from(value: StringValue) -> Self {
        Self::Value(Value::String(value))
    }
}

impl From<ListValue> for ContentItem {
    fn from(value: ListValue) -> Self {
        Self::Value(Value::List(value))
    }
}

impl From<ControlCommand> for ContentItem {
    fn from(value: ControlCommand) -> Self {
        Self::ControlCommand(value)
    }
}

impl From<Void> for ContentItem {
    fn from(value: Void) -> Self {
        Self::Void(value)
    }
}

impl From<Container> for ContentItem {
    fn from(value: Container) -> Self {
        Self::Container(Rc::new(value))
    }
}

impl From<VariableReference> for ContentItem {
    fn from(value: VariableReference) -> Self {
        Self::VariableReference(value)
    }
}

impl From<Divert> for ContentItem {
    fn from(value: Divert) -> Self {
        Self::Divert(value)
    }
}

impl From<ChoicePoint> for ContentItem {
    fn from(value: ChoicePoint) -> Self {
        Self::ChoicePoint(value)
    }
}

impl From<Glue> for ContentItem {
    fn from(value: Glue) -> Self {
        Self::Glue(value)
    }
}

impl From<NativeFunctionCall> for ContentItem {
    fn from(value: NativeFunctionCall) -> Self {
        Self::NativeFunctionCall(value)
    }
}

impl From<VariableAssignment> for ContentItem {
    fn from(value: VariableAssignment) -> Self {
        Self::VariableAssignment(value)
    }
}

impl From<Tag> for ContentItem {
    fn from(value: Tag) -> Self {
        Self::Tag(value)
    }
}

impl From<Choice> for ContentItem {
    fn from(value: Choice) -> Self {
        Self::Choice(value)
    }
}

impl std::fmt::Display for ContentItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentItem::Value(value) => write!(f, "{}", value),
            ContentItem::ControlCommand(command) => write!(f, "{}", command),
            ContentItem::Void(_) => write!(f, "void"),
            ContentItem::Container(container) => write!(f, "{container:?}"),
            ContentItem::VariableReference(reference) => write!(f, "{reference:?}"),
            ContentItem::Divert(divert) => write!(f, "{divert:?}"),
            ContentItem::ChoicePoint(choice) => write!(f, "{choice:?}"),
            ContentItem::Glue(glue) => write!(f, "{glue:?}"),
            ContentItem::NativeFunctionCall(call) => write!(f, "{call:?}"),
            ContentItem::VariableAssignment(var) => write!(f, "{var:?}"),
            ContentItem::Tag(tag) => write!(f, "{tag:?}"),
            ContentItem::Choice(choice) => write!(f, "{choice:?}"),
        }
    }
}

fn content_item_name(content: &ContentItem) -> Option<String> {
    match content {
        ContentItem::Container(container) => {
            if container.get_hasValidName() {
                Some(container.get_name().to_string())
            } else {
                None
            }
        }
        _ => None,
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

#[derive(Clone, Debug)]
pub struct Container {
    parent: RefCell<Option<Rc<Container>>>,
    content: Vec<ContentItem>,
    named_content: HashMap<String, ContentItem>,
    named_only_content: HashMap<String, ContentItem>,
    name: Option<String>,
    path: RefCell<Path>,
    path_to_first_leaf_content: Option<Path>,
    debug_metadata: Option<DebugMetadata>,
    visitsShouldBeCounted: bool,
    turnIndexShouldBeCounted: bool,
    countingAtStartOnly: bool,
    uid: usize,
}

impl PartialEq for Container {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Eq for Container {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CountFlags {
    Visits = 1,
    Turns = 2,
    CountStartOnly = 4,
}

impl Container {
    // C# signature: public Container ()
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_content(content: Vec<ContentItem>) -> Self {
        let mut container = Self::new();
        for item in content {
            container.AddContent(item);
        }
        container
    }

    // C# signature: public void AddContent(Runtime.Object contentObj)
    pub fn AddContent<T: Into<ContentItem>>(&mut self, contentObj: T) {
        let mut content = contentObj.into();
        if Self::content_item_has_parent(&content) {
            panic!("content is already in a container");
        }
        let index = self.content.len() as i32;
        let child_path = match &content {
            ContentItem::Container(container) => Some(if container.get_hasValidName() {
                self.get_path()
                    .PathByAppendingComponent(Component::new_overload_2(
                        container.get_name().to_string(),
                    ))
            } else {
                self.get_path()
                    .PathByAppendingComponent(Component::new(index))
            }),
            ContentItem::Divert(_) => Some(
                self.get_path()
                    .PathByAppendingComponent(Component::new(index)),
            ),
            _ => None,
        };
        self.content.push(content.clone());
        let parent_snapshot = Rc::new(self.clone());
        Self::attach_parent_metadata(&mut content, parent_snapshot.clone(), child_path.clone());
        if let Some(stored) = self.content.last_mut() {
            Self::attach_parent_metadata(stored, parent_snapshot, child_path);
        }
        if let Some(name) = content_item_name(&content) {
            self.named_content.insert(name, content);
        }
    }

    // C# signature: public void AddContent(IList<Runtime.Object> contentList)
    pub fn AddContent_overload_2<T: Into<ContentItem>>(&mut self, contentList: Vec<T>) {
        for content in contentList {
            self.AddContent(content);
        }
    }

    // C# signature: public void InsertContent(Runtime.Object contentObj, int index)
    pub fn InsertContent<T: Into<ContentItem>>(&mut self, contentObj: T, index: i32) {
        let index = index.max(0) as usize;
        let mut content = contentObj.into();
        if Self::content_item_has_parent(&content) {
            panic!("content is already in a container");
        }
        if index >= self.content.len() {
            self.content.push(content.clone());
        } else {
            self.content.insert(index, content.clone());
        }
        let parent_snapshot = Rc::new(self.clone());
        let child_path = match &content {
            ContentItem::Container(container) => Some(if container.get_hasValidName() {
                self.get_path()
                    .PathByAppendingComponent(Component::new_overload_2(
                        container.get_name().to_string(),
                    ))
            } else {
                self.get_path()
                    .PathByAppendingComponent(Component::new(index as i32))
            }),
            ContentItem::Divert(_) => Some(
                self.get_path()
                    .PathByAppendingComponent(Component::new(index as i32)),
            ),
            _ => None,
        };
        Self::attach_parent_metadata(&mut content, parent_snapshot.clone(), child_path.clone());
        if let Some(stored) = self.content.get_mut(index) {
            Self::attach_parent_metadata(stored, parent_snapshot, child_path);
        }
        if let Some(name) = content_item_name(&content) {
            self.named_content.insert(name, content);
        }
    }

    // C# signature: public void TryAddNamedContent(Runtime.Object contentObj)
    pub fn TryAddNamedContent<T: Into<ContentItem>>(&mut self, contentObj: T) {
        self.AddContent(contentObj);
    }

    // C# signature: public void AddToNamedContentOnly(INamedContent namedContentObj)
    pub fn AddToNamedContentOnly<T: Into<ContentItem>>(&mut self, namedContentObj: T) {
        let mut content = namedContentObj.into();
        let parent_snapshot = Rc::new(self.clone());
        let child_path = match &content {
            ContentItem::Container(container) => {
                if container.get_hasValidName() {
                    Some(
                        self.get_path()
                            .PathByAppendingComponent(Component::new_overload_2(
                                container.get_name().to_string(),
                            )),
                    )
                } else {
                    None
                }
            }
            _ => None,
        };
        if let Some(name) = content_item_name(&content) {
            Self::attach_parent_metadata(&mut content, parent_snapshot.clone(), child_path.clone());
            self.named_content.insert(name.clone(), content.clone());
            self.named_only_content.insert(name, content);
        }
    }

    // C# signature: public void AddContentsOfContainer(Container otherContainer)
    pub fn AddContentsOfContainer(&mut self, otherContainer: Container) {
        for content in otherContainer.content {
            let mut content = content;
            Self::detach_parent(&mut content);
            self.AddContent(content);
        }
    }

    fn detach_parent(content: &mut ContentItem) {
        match content {
            ContentItem::Container(container) => {
                container.as_ref().set_parent(None);
            }
            ContentItem::ChoicePoint(choice_point) => {
                choice_point.set_parent(None);
            }
            ContentItem::Divert(divert) => {
                divert.set_parent(None);
            }
            ContentItem::VariableReference(variable_reference) => {
                variable_reference.set_parent(None);
            }
            _ => {}
        }
    }

    fn attach_parent_metadata(
        content: &mut ContentItem,
        parent: Rc<Container>,
        child_path: Option<Path>,
    ) {
        match content {
            ContentItem::Container(container) => {
                container.as_ref().set_parent(Some(parent));
                if let Some(path) = child_path {
                    container.as_ref().set_path(path);
                }
                container.as_ref().refresh_child_parents();
            }
            ContentItem::ChoicePoint(choice_point) => {
                choice_point.set_parent(Some(parent));
            }
            ContentItem::Divert(divert) => {
                divert.set_parent(Some(parent));
                if let Some(path) = child_path {
                    divert.set_path(path);
                }
            }
            ContentItem::VariableReference(variable_reference) => {
                variable_reference.set_parent(Some(parent));
            }
            _ => {}
        }
    }

    fn content_item_has_parent(content: &ContentItem) -> bool {
        match content {
            ContentItem::Container(container) => container.get_parent().is_some(),
            ContentItem::ChoicePoint(choice_point) => choice_point.get_parent().is_some(),
            ContentItem::Divert(divert) => divert.get_parent().is_some(),
            ContentItem::VariableReference(variable_reference) => {
                variable_reference.get_parent().is_some()
            }
            _ => false,
        }
    }

    // C# signature: protected Runtime.Object ContentWithPathComponent(Path.Component component)
    pub fn ContentWithPathComponent(&mut self, component: Component) -> Option<ContentItem> {
        if component.get_isIndex() {
            let index = component.get_index();
            if index >= 0 {
                self.content.get(index as usize).cloned()
            } else {
                None
            }
        } else if component.get_isParent() {
            if let Some(parent) = self.parent.borrow().as_ref() {
                Some(ContentItem::Container(parent.clone()))
            } else {
                None
            }
        } else {
            let name = component.get_name().unwrap_or("");
            let result = self
                .named_content
                .get(name)
                .cloned()
                .or_else(|| self.named_only_content.get(name).cloned());
            result
        }
    }

    // C# signature: public SearchResult ContentAtPath(Path path, int partialPathStart = 0, int partialPathLength = -1)
    pub fn ContentAtPath(
        &mut self,
        path: Path,
        partialPathStart: i32,
        partialPathLength: i32,
    ) -> SearchResult {
        let debug_path = path.ToString();
        let partialPathLength = if partialPathLength == -1 {
            path.get_length()
        } else {
            partialPathLength
        };

        let mut result = SearchResult::new();
        let mut currentContainer: Option<Container> = Some(self.clone());
        let mut currentObj: Option<ContentItem> =
            Some(ContentItem::Container(Rc::new(self.clone())));

        for i in partialPathStart..partialPathLength {
            let Some(mut container) = currentContainer.take() else {
                result.approximate = true;
                break;
            };

            let Some(component) = path.GetComponent(i) else {
                result.approximate = true;
                break;
            };
            let foundObj = container.ContentWithPathComponent(component.clone());
            let Some(foundObj) = foundObj else {
                result.approximate = true;
                break;
            };

            if i < partialPathLength - 1 {
                match &foundObj {
                    ContentItem::Container(nextContainer) => {
                        currentContainer = Some(nextContainer.as_ref().clone());
                    }
                    _ => {
                        result.approximate = true;
                        currentObj = Some(foundObj);
                        break;
                    }
                }
            } else {
                currentContainer = None;
            }

            currentObj = Some(foundObj);
        }

        result.obj = currentObj;
        result
    }

    // C# signature: public void BuildStringOfHierarchy(StringBuilder sb, int indentation, Runtime.Object pointedObj)
    pub fn BuildStringOfHierarchy(
        &mut self,
        mut sb: crate::StringBuilder::StringBuilder,
        indentation: i32,
        pointedPath: Option<Path>,
    ) {
        fn append_indentation(sb: &mut crate::StringBuilder::StringBuilder, indentation: i32) {
            for _ in 0..(indentation.max(0) as usize * 4) {
                sb.AppendChar(' ');
            }
        }

        fn append_content(
            sb: &mut crate::StringBuilder::StringBuilder,
            indentation: i32,
            content: &ContentItem,
            pointedPath: &Option<Path>,
        ) {
            match content {
                ContentItem::Container(container) => {
                    let mut child = container.as_ref().clone();
                    child.BuildStringOfHierarchy(sb.clone(), indentation, pointedPath.clone());
                }
                ContentItem::Value(value) => {
                    append_indentation(sb, indentation);
                    if matches!(value, Value::String(_)) {
                        sb.Append("\"");
                        sb.Append(value.ToString().replace('\n', "\\n"));
                        sb.Append("\"");
                    } else {
                        sb.Append(value.ToString());
                    }
                }
                ContentItem::ControlCommand(command) => {
                    append_indentation(sb, indentation);
                    sb.Append(command.ToString())
                }
                ContentItem::Void(value) => {
                    append_indentation(sb, indentation);
                    sb.Append("void")
                }
                ContentItem::VariableReference(reference) => {
                    append_indentation(sb, indentation);
                    sb.Append(reference.ToString())
                }
                ContentItem::Divert(divert) => {
                    append_indentation(sb, indentation);
                    sb.Append(divert.ToString())
                }
                ContentItem::ChoicePoint(choice) => {
                    append_indentation(sb, indentation);
                    sb.Append(choice.ToString())
                }
                ContentItem::Glue(glue) => {
                    append_indentation(sb, indentation);
                    sb.Append(glue.ToString())
                }
                ContentItem::NativeFunctionCall(call) => {
                    append_indentation(sb, indentation);
                    sb.Append(call.ToString())
                }
                ContentItem::VariableAssignment(var) => {
                    append_indentation(sb, indentation);
                    sb.Append(var.ToString())
                }
                ContentItem::Tag(tag) => {
                    append_indentation(sb, indentation);
                    sb.Append(tag.ToString())
                }
                ContentItem::Choice(choice) => {
                    append_indentation(sb, indentation);
                    sb.Append(choice.get_pathStringOnChoice().as_deref().unwrap_or(""))
                }
            }
        }
        append_indentation(&mut sb, indentation);
        sb.Append("[");

        if self.get_hasValidName() {
            sb.Append(" (");
            sb.Append(self.get_name());
            sb.Append(")");
        }

        let current_path = self.get_path();
        if pointedPath.as_ref() == Some(&current_path) {
            sb.Append("  <---");
        }

        sb.AppendLine("");

        for (index, content) in self.content.iter().enumerate() {
            let item_path = match content {
                ContentItem::Container(container) => container.get_path().clone(),
                _ => self
                    .get_path()
                    .PathByAppendingComponent(Component::new(index as i32)),
            };
            append_content(&mut sb, indentation + 1, content, &pointedPath);

            if Some(&item_path) == pointedPath.as_ref() {
                sb.Append("  <---");
            }

            if index != self.content.len() - 1 {
                sb.Append(",");
            }

            sb.AppendLine("");
        }

        let mut only_named = HashMap::<String, &ContentItem>::new();
        for (name, content) in &self.named_content {
            if !self.content.iter().any(|c| c == content) {
                only_named.insert(name.clone(), content);
            }
        }

        if !only_named.is_empty() {
            append_indentation(&mut sb, indentation);
            sb.AppendLine("-- named: --");

            for (_, content) in only_named {
                if let ContentItem::Container(container) = content {
                    let mut child = container.as_ref().clone();
                    child.BuildStringOfHierarchy(sb.clone(), indentation + 1, pointedPath.clone());
                    sb.AppendLine("");
                }
            }
        }

        append_indentation(&mut sb, indentation);
        sb.Append("]");
    }

    pub(crate) fn refresh_child_parents(&self) {
        let parent_rc = Rc::new(self.clone());

        for content in &self.content {
            match content {
                ContentItem::Container(container) => {
                    container.as_ref().set_parent(Some(parent_rc.clone()));
                    container.as_ref().refresh_child_parents();
                }
                ContentItem::ChoicePoint(choice_point) => {
                    choice_point.set_parent(Some(parent_rc.clone()));
                }
                ContentItem::Divert(divert) => {
                    divert.set_parent(Some(parent_rc.clone()));
                }
                ContentItem::VariableReference(variable_reference) => {
                    variable_reference.set_parent(Some(parent_rc.clone()));
                }
                _ => {}
            }
        }

        for content in self.named_only_content.values() {
            match content {
                ContentItem::Container(container) => {
                    container.as_ref().set_parent(Some(parent_rc.clone()));
                    container.as_ref().refresh_child_parents();
                }
                ContentItem::ChoicePoint(choice_point) => {
                    choice_point.set_parent(Some(parent_rc.clone()));
                }
                ContentItem::Divert(divert) => {
                    divert.set_parent(Some(parent_rc.clone()));
                }
                ContentItem::VariableReference(variable_reference) => {
                    variable_reference.set_parent(Some(parent_rc.clone()));
                }
                _ => {}
            }
        }
    }

    // C# signature: public virtual string BuildStringOfHierarchy()
    pub fn BuildStringOfHierarchy_overload_2(&mut self) -> String {
        let sb = crate::StringBuilder::StringBuilder::new();
        self.BuildStringOfHierarchy(sb.clone(), 0, None);
        sb.ToString()
    }

    // C# signature: string name { get; }
    pub fn get_name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }

    // C# signature: Path path { get; }
    pub fn get_path(&self) -> Path {
        if self.parent.borrow().is_none() {
            return self.path.borrow().clone();
        }

        let mut components = Vec::new();
        let mut child = self.clone();
        let mut container = child.parent.borrow().clone();

        while let Some(parent) = container {
            if child.get_hasValidName() {
                components.push(Component::new_overload_2(child.get_name().to_string()));
            } else {
                let index = parent
                    .content
                    .iter()
                    .position(|content| match content {
                        ContentItem::Container(candidate) => candidate.as_ref() == &child,
                        _ => false,
                    })
                    .unwrap_or(0);
                components.push(Component::new(index as i32));
            }

            child = (*parent).clone();
            container = child.parent.borrow().clone();
        }

        components.reverse();
        let path = Path::new_overload_3(components, false);
        self.path.replace(path.clone());
        path
    }

    pub fn set_path(&self, path: Path) {
        self.path.replace(path);
    }

    pub fn get_debugMetadata(&self) -> Option<&DebugMetadata> {
        self.debug_metadata.as_ref()
    }

    pub fn set_debugMetadata(&mut self, debug_metadata: Option<DebugMetadata>) {
        self.debug_metadata = debug_metadata;
    }

    // C# signature: List<Runtime.Object> content { get; }
    pub fn get_content(&self) -> &[ContentItem] {
        &self.content
    }

    // C# signature: Dictionary<string, Runtime.Object> namedContent { get; }
    pub fn get_namedContent(&self) -> &HashMap<String, ContentItem> {
        &self.named_content
    }

    pub fn set_namedContent(&mut self, named_content: HashMap<String, ContentItem>) {
        self.named_content = named_content;
    }

    // C# signature: Dictionary<string, Runtime.Object> namedOnlyContent { get; }
    pub fn get_namedOnlyContent(&self) -> Option<HashMap<String, ContentItem>> {
        if self.named_only_content.is_empty() {
            None
        } else {
            Some(self.named_only_content.clone())
        }
    }

    pub fn set_namedOnlyContent(
        &mut self,
        named_only_content: Option<HashMap<String, ContentItem>>,
    ) {
        for name in self.named_only_content.keys().cloned().collect::<Vec<_>>() {
            self.named_content.remove(&name);
        }
        self.named_only_content.clear();

        let Some(named_only_content) = named_only_content else {
            return;
        };

        for (name, mut content) in named_only_content {
            if let ContentItem::Container(ref mut container) = content {
                container.as_ref().set_parent(Some(Rc::new(self.clone())));
            } else if let ContentItem::ChoicePoint(ref mut choice_point) = content {
                choice_point.set_parent(Some(Rc::new(self.clone())));
            } else if let ContentItem::VariableReference(ref mut variable_reference) = content {
                variable_reference.set_parent(Some(Rc::new(self.clone())));
            } else if let ContentItem::Divert(ref mut divert) = content {
                divert.set_parent(Some(Rc::new(self.clone())));
            }

            self.named_content.insert(name.clone(), content.clone());
            self.named_only_content.insert(name, content);
        }

        self.refresh_child_parents();
    }

    // C# signature: bool visitsShouldBeCounted { get; }
    pub fn get_visitsShouldBeCounted(&self) -> bool {
        self.visitsShouldBeCounted
    }

    // C# signature: bool turnIndexShouldBeCounted { get; }
    pub fn get_turnIndexShouldBeCounted(&self) -> bool {
        self.turnIndexShouldBeCounted
    }

    // C# signature: bool countingAtStartOnly { get; }
    pub fn get_countingAtStartOnly(&self) -> bool {
        self.countingAtStartOnly
    }

    pub fn get_parent(&self) -> Option<Rc<Container>> {
        self.parent.borrow().clone()
    }

    pub fn set_parent(&self, parent: Option<Rc<Container>>) {
        self.parent.replace(parent);
    }

    pub fn get_rootContentContainer(&self) -> Option<Container> {
        let mut ancestor = self.parent.borrow().clone()?;
        while let Some(parent) = ancestor.get_parent() {
            ancestor = parent;
        }
        Some((*ancestor).clone())
    }

    // C# signature: int countFlags { get; }
    pub fn get_countFlags(&self) -> i32 {
        let mut flags = (self.visitsShouldBeCounted as i32)
            | ((self.turnIndexShouldBeCounted as i32) << 1)
            | ((self.countingAtStartOnly as i32) << 2);
        if flags == 4 {
            flags = 0;
        }
        flags
    }

    // C# signature: bool hasValidName { get; }
    pub fn get_hasValidName(&self) -> bool {
        !self.get_name().is_empty()
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    pub fn set_countFlags(&mut self, countFlags: i32) {
        let flag = countFlags;
        self.visitsShouldBeCounted = (flag & CountFlags::Visits as i32) != 0;
        self.turnIndexShouldBeCounted = (flag & CountFlags::Turns as i32) != 0;
        self.countingAtStartOnly = (flag & CountFlags::CountStartOnly as i32) != 0;
    }

    // C# signature: Path pathToFirstLeafContent { get; }
    pub fn get_pathToFirstLeafContent(&self) -> Path {
        if let Some(path) = &self.path_to_first_leaf_content {
            return path.clone();
        }

        let mut path = self.get_path().clone();
        let mut container = self;
        while let Some(ContentItem::Container(first_child)) = container.content.first() {
            path = path.PathByAppendingComponent(crate::Path::Component::new(0));
            container = first_child.as_ref();
        }
        path
    }

    pub fn get_uid(&self) -> usize {
        self.uid
    }
}

impl Default for Container {
    fn default() -> Self {
        Self {
            parent: RefCell::new(None),
            content: Vec::new(),
            named_content: HashMap::new(),
            named_only_content: HashMap::new(),
            name: None,
            path: RefCell::new(Path::new()),
            path_to_first_leaf_content: None,
            debug_metadata: None,
            visitsShouldBeCounted: false,
            turnIndexShouldBeCounted: false,
            countingAtStartOnly: false,
            uid: next_container_uid(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Container, ContentItem};
    use crate::ControlCommand::ControlCommand;
    use crate::Path::{Component, Path};
    use std::collections::HashMap;
    use std::rc::Rc;

    #[test]
    fn resolves_indexed_child_container_paths() {
        let mut child = Container::new();
        child.AddContent(ControlCommand::BeginString());

        let mut root = Container::new();
        root.AddContent(child.clone());

        let result =
            root.ContentAtPath(Path::new_overload_3(vec![Component::new(0)], false), 0, -1);
        assert!(!result.approximate);
        assert!(matches!(
            result.get_correctObj(),
            Some(ContentItem::Container(_))
        ));
        assert_eq!(root.get_pathToFirstLeafContent().ToString(), "0");
    }

    #[test]
    fn count_flags_round_trip_matches_csharp_bits() {
        let mut container = Container::new();

        container.set_countFlags(1 | 2 | 4);
        assert!(container.get_visitsShouldBeCounted());
        assert!(container.get_turnIndexShouldBeCounted());
        assert!(container.get_countingAtStartOnly());
        assert_eq!(container.get_countFlags(), 7);

        container.set_countFlags(4);
        assert!(!container.get_visitsShouldBeCounted());
        assert!(!container.get_turnIndexShouldBeCounted());
        assert!(container.get_countingAtStartOnly());
        assert_eq!(container.get_countFlags(), 0);
    }

    #[test]
    fn add_contents_of_container_reparents_children() {
        let mut inner = Container::new();
        inner.set_name(Some("inner".to_string()));
        inner.AddContent(ControlCommand::BeginString());

        let mut source = Container::new();
        source.AddContent(inner.clone());

        let mut root = Container::new();
        root.AddContentsOfContainer(source);

        assert_eq!(root.get_content().len(), 1);
        assert!(root.get_namedContent().contains_key("inner"));
        let inserted = match root.get_content().first() {
            Some(ContentItem::Container(container)) => container.as_ref().clone(),
            _ => panic!("child container missing"),
        };
        assert!(inserted.get_parent().is_some());
    }

    #[test]
    fn computes_path_from_parent_chain_when_cached_path_is_empty() {
        let mut child = Container::new();
        child.set_name(Some("child".to_string()));

        let mut root = Container::new();
        root.AddContent(child.clone());

        let inserted_child = match root.get_content().first() {
            Some(ContentItem::Container(container)) => container.as_ref().clone(),
            _ => panic!("child container missing"),
        };

        assert_eq!(inserted_child.get_path().ToString(), "child");
    }

    #[test]
    fn resolves_named_child_container_through_content_at_path() {
        let mut named_child = Container::new();
        named_child.set_name(Some("child".to_string()));
        named_child.AddContent(ControlCommand::BeginString());

        let mut parent = Container::new();
        parent.AddContent(named_child.clone());

        let mut root = Container::new();
        root.AddContent(parent.clone());

        let result = root.ContentAtPath(
            Path::new_overload_3(
                vec![
                    Component::new(0),
                    Component::new_overload_2("child".to_string()),
                ],
                false,
            ),
            0,
            -1,
        );

        assert!(!result.approximate);
        assert!(matches!(
            result.get_correctObj(),
            Some(ContentItem::Container(container)) if container.get_name() == "child"
        ));
    }

    #[test]
    fn resolves_named_only_child_container_through_content_at_path() {
        let mut named_only_child = Container::new();
        named_only_child.set_name(Some("child".to_string()));
        named_only_child.AddContent(ControlCommand::BeginString());

        let mut parent = Container::new();
        let mut named_only = HashMap::new();
        named_only.insert(
            "child".to_string(),
            ContentItem::Container(Rc::new(named_only_child.clone())),
        );
        parent.set_namedOnlyContent(Some(named_only));

        let result = parent.ContentAtPath(
            Path::new_overload_3(vec![Component::new_overload_2("child".to_string())], false),
            0,
            -1,
        );

        assert!(!result.approximate);
        assert!(matches!(
            result.get_correctObj(),
            Some(ContentItem::Container(container)) if container.get_name() == "child"
        ));
    }
}
