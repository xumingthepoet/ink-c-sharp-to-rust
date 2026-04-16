// Source: ink-c-sharp/ink-engine-runtime/Container.cs

use crate::Choice::Choice;
use crate::ChoicePoint::ChoicePoint;
use crate::ControlCommand::ControlCommand;
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
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum ContentItem {
    Value(Value),
    ControlCommand(ControlCommand),
    Void(Void),
    Container(Box<Container>),
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
        Self::Container(Box::new(value))
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Container {
    content: Vec<ContentItem>,
    named_content: HashMap<String, ContentItem>,
    name: Option<String>,
    path: Path,
    visitsShouldBeCounted: bool,
    turnIndexShouldBeCounted: bool,
    countingAtStartOnly: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CountFlags {
    PortPlaceholder,
}

impl Default for CountFlags {
    fn default() -> Self {
        Self::PortPlaceholder
    }
}

impl Container {
    // C# signature: public Container ()
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_content(content: Vec<ContentItem>) -> Self {
        Self {
            content,
            path: Path::new(),
            ..Default::default()
        }
    }

    // C# signature: public void AddContent(Runtime.Object contentObj)
    pub fn AddContent<T: Into<ContentItem>>(&mut self, contentObj: T) {
        self.content.push(contentObj.into());
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
        let content = contentObj.into();
        if index >= self.content.len() {
            self.content.push(content);
        } else {
            self.content.insert(index, content);
        }
    }

    // C# signature: public void TryAddNamedContent(Runtime.Object contentObj)
    pub fn TryAddNamedContent<T: Into<ContentItem>>(&mut self, contentObj: T) {
        self.AddContent(contentObj);
    }

    // C# signature: public void AddToNamedContentOnly(INamedContent namedContentObj)
    pub fn AddToNamedContentOnly<T: Into<ContentItem>>(&mut self, namedContentObj: T) {
        let content = namedContentObj.into();
        if let Some(name) = self.name.clone() {
            self.named_content.insert(name, content);
        }
    }

    // C# signature: public void AddContentsOfContainer(Container otherContainer)
    pub fn AddContentsOfContainer(&mut self, otherContainer: Container) {
        self.content.extend(otherContainer.content);
        self.named_content.extend(otherContainer.named_content);
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
            None
        } else {
            self.named_content
                .get(component.get_name().unwrap_or(""))
                .cloned()
        }
    }

    // C# signature: public SearchResult ContentAtPath(Path path, int partialPathStart = 0, int partialPathLength = -1)
    pub fn ContentAtPath(
        &mut self,
        path: Path,
        partialPathStart: i32,
        partialPathLength: i32,
    ) -> SearchResult {
        let partialPathLength = if partialPathLength == -1 {
            path.get_length()
        } else {
            partialPathLength
        };

        let mut result = SearchResult::new();
        let mut currentContainer: Option<Container> = Some(self.clone());
        let mut currentObj: Option<ContentItem> =
            Some(ContentItem::Container(Box::new(self.clone())));

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
        _sb: crate::stub::StringBuilder,
        _indentation: i32,
        _pointedObj: crate::stub::PortStub,
    ) {
    }

    // C# signature: public virtual string BuildStringOfHierarchy()
    pub fn BuildStringOfHierarchy_overload_2(&mut self) -> String {
        let mut result = String::from("[");
        if self.get_hasValidName() {
            result.push(' ');
            result.push('(');
            result.push_str(self.get_name());
            result.push(')');
        }

        result.push('\n');

        for content in &self.content {
            match content {
                ContentItem::Container(container) => {
                    let mut child_container = container.as_ref().clone();
                    let child = child_container.BuildStringOfHierarchy_overload_2();
                    for line in child.lines() {
                        result.push_str("    ");
                        result.push_str(line);
                        result.push('\n');
                    }
                }
                ContentItem::Value(value) => {
                    result.push_str("    ");
                    result.push_str(&value.ToString());
                    result.push('\n');
                }
                ContentItem::ControlCommand(command) => {
                    result.push_str("    ");
                    result.push_str(&command.ToString());
                    result.push('\n');
                }
                ContentItem::Void(value) => {
                    result.push_str("    ");
                    result.push_str(&format!("{:?}", value));
                    result.push('\n');
                }
                ContentItem::VariableReference(reference) => {
                    result.push_str("    ");
                    result.push_str(&reference.ToString());
                    result.push('\n');
                }
                ContentItem::Divert(divert) => {
                    result.push_str("    ");
                    result.push_str(&divert.ToString());
                    result.push('\n');
                }
                ContentItem::ChoicePoint(choice) => {
                    result.push_str("    ");
                    result.push_str(&choice.ToString());
                    result.push('\n');
                }
                ContentItem::Glue(glue) => {
                    result.push_str("    ");
                    result.push_str(&glue.ToString());
                    result.push('\n');
                }
                ContentItem::NativeFunctionCall(call) => {
                    result.push_str("    ");
                    result.push_str(&call.ToString());
                    result.push('\n');
                }
                ContentItem::VariableAssignment(var) => {
                    result.push_str("    ");
                    result.push_str(&var.ToString());
                    result.push('\n');
                }
                ContentItem::Tag(tag) => {
                    result.push_str("    ");
                    result.push_str(&tag.ToString());
                    result.push('\n');
                }
                ContentItem::Choice(choice) => {
                    result.push_str("    ");
                    result.push_str(choice.get_pathStringOnChoice().as_deref().unwrap_or(""));
                    result.push('\n');
                }
            }
        }

        result.push(']');
        result
    }

    // C# signature: string name { get; }
    pub fn get_name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }

    // C# signature: Path path { get; }
    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn set_path(&mut self, path: Path) {
        self.path = path;
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

    // C# signature: int countFlags { get; }
    pub fn get_countFlags(&self) -> i32 {
        (self.visitsShouldBeCounted as i32)
            | ((self.turnIndexShouldBeCounted as i32) << 1)
            | ((self.countingAtStartOnly as i32) << 2)
    }

    // C# signature: bool hasValidName { get; }
    pub fn get_hasValidName(&self) -> bool {
        !self.get_name().is_empty()
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    pub fn set_countFlags(&mut self, countFlags: i32) {
        self.visitsShouldBeCounted = (countFlags & 1) != 0;
        self.turnIndexShouldBeCounted = (countFlags & 2) != 0;
        self.countingAtStartOnly = (countFlags & 4) != 0;
    }

    // C# signature: Path pathToFirstLeafContent { get; }
    pub fn get_pathToFirstLeafContent(&self) -> Path {
        let mut path = self.get_path().clone();
        let mut container = self;
        while let Some(ContentItem::Container(first_child)) = container.content.first() {
            path = path.PathByAppendingComponent(crate::Path::Component::new(0));
            container = first_child.as_ref();
        }
        path
    }
}

#[cfg(test)]
mod tests {
    use super::{Container, ContentItem};
    use crate::ControlCommand::ControlCommand;
    use crate::Path::{Component, Path};

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
}
