// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Container.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Container {
    pub _port_marker: (),
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
        Default::default()
    }

    // C# signature: public void AddContent(Runtime.Object contentObj)
    pub fn AddContent(&mut self, _contentObj: crate::stub::PortStub) {}

    // C# signature: public void AddContent(IList<Runtime.Object> contentList)
    pub fn AddContent_overload_2(&mut self, _contentList: Vec<crate::stub::PortStub>) {}

    // C# signature: public void InsertContent(Runtime.Object contentObj, int index)
    pub fn InsertContent(&mut self, _contentObj: crate::stub::PortStub, _index: i32) {}

    // C# signature: public void TryAddNamedContent(Runtime.Object contentObj)
    pub fn TryAddNamedContent(&mut self, _contentObj: crate::stub::PortStub) {}

    // C# signature: public void AddToNamedContentOnly(INamedContent namedContentObj)
    pub fn AddToNamedContentOnly(&mut self, _namedContentObj: crate::stub::INamedContent) {}

    // C# signature: public void AddContentsOfContainer(Container otherContainer)
    pub fn AddContentsOfContainer(&mut self, _otherContainer: crate::stub::Container) {}

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
        Default::default()
    }

    // C# signature: string name { get; }
    pub fn get_name(&mut self) -> String {
        Default::default()
    }

    // C# signature: List<Runtime.Object> content { get; }
    pub fn get_content(&mut self) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: bool visitsShouldBeCounted { get; }
    pub fn get_visitsShouldBeCounted(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool turnIndexShouldBeCounted { get; }
    pub fn get_turnIndexShouldBeCounted(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool countingAtStartOnly { get; }
    pub fn get_countingAtStartOnly(&mut self) -> bool {
        Default::default()
    }

    // C# signature: int countFlags { get; }
    pub fn get_countFlags(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: bool hasValidName { get; }
    pub fn get_hasValidName(&mut self) -> bool {
        Default::default()
    }

    // C# signature: Path pathToFirstLeafContent { get; }
    pub fn get_pathToFirstLeafContent(&mut self) -> crate::stub::Path {
        Default::default()
    }
}
