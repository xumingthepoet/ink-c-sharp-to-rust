// Source: ink-c-sharp/ink-engine-runtime/Container.cs

use crate::ControlCommand::ControlCommand;
use crate::Path::Path;
use crate::Value::{ListValue, StringValue, Value};
use crate::Void::Void;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum ContentItem {
    Value(Value),
    ControlCommand(ControlCommand),
    Void(Void),
    Container(Box<Container>),
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
    pub fn ContentWithPathComponent(
        &mut self,
        _component: crate::stub::Component,
    ) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public SearchResult ContentAtPath(Path path, int partialPathStart = 0, int partialPathLength = -1)
    pub fn ContentAtPath(
        &mut self,
        _path: crate::stub::Path,
        _partialPathStart: i32,
        _partialPathLength: i32,
    ) -> crate::stub::SearchResult {
        Default::default()
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
        self.get_name().to_string()
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

    // C# signature: Path pathToFirstLeafContent { get; }
    pub fn get_pathToFirstLeafContent(&self) -> Path {
        Path::new()
    }
}
