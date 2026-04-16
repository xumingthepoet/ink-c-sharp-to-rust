// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Path.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Path {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct Component {
    pub _port_marker: (),
}

impl Path {
    // C# signature: public Path()
    pub fn new() -> Self {
        Default::default()
    }

    // C# signature: public Path(Component head, Path tail)
    pub fn new_overload_2(_head: crate::stub::Component, _tail: crate::stub::Path) -> Self {
        Default::default()
    }

    // C# signature: public Path(IEnumerable<Component> components, bool relative = false)
    pub fn new_overload_3(_components: Vec<crate::stub::Component>, _relative: bool) -> Self {
        Default::default()
    }

    // C# signature: public Path(string componentsString)
    pub fn new_overload_4(_componentsString: String) -> Self {
        Default::default()
    }

    // C# signature: public static Component ToParent()
    pub fn ToParent() -> crate::stub::Component {
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

    // C# signature: public bool Equals(Component otherComp)
    pub fn Equals_overload_2(&mut self, _otherComp: crate::stub::Component) -> bool {
        Default::default()
    }

    // C# signature: public override int GetHashCode ()
    pub fn GetHashCode(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: public Component GetComponent(int index)
    pub fn GetComponent(&mut self, _index: i32) -> crate::stub::Component {
        Default::default()
    }

    // C# signature: public Path PathByAppendingPath(Path pathToAppend)
    pub fn PathByAppendingPath(&mut self, _pathToAppend: crate::stub::Path) -> crate::stub::Path {
        Default::default()
    }

    // C# signature: public Path PathByAppendingComponent (Component c)
    pub fn PathByAppendingComponent(&mut self, _c: crate::stub::Component) -> crate::stub::Path {
        Default::default()
    }

    // C# signature: public bool Equals (Path otherPath)
    pub fn Equals_overload_3(&mut self, _otherPath: crate::stub::Path) -> bool {
        Default::default()
    }

    // C# signature: int index { get; }
    pub fn get_index(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: string name { get; }
    pub fn get_name(&mut self) -> String {
        Default::default()
    }

    // C# signature: bool isIndex { get; }
    pub fn get_isIndex(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isParent { get; }
    pub fn get_isParent(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isRelative { get; }
    pub fn get_isRelative(&mut self) -> bool {
        Default::default()
    }

    // C# signature: Component head { get; }
    pub fn get_head(&mut self) -> crate::stub::Component {
        Default::default()
    }

    // C# signature: Path tail { get; }
    pub fn get_tail(&mut self) -> crate::stub::Path {
        Default::default()
    }

    // C# signature: int length { get; }
    pub fn get_length(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: Component lastComponent { get; }
    pub fn get_lastComponent(&mut self) -> crate::stub::Component {
        Default::default()
    }

    // C# signature: bool containsNamedComponent { get; }
    pub fn get_containsNamedComponent(&mut self) -> bool {
        Default::default()
    }

    // C# signature: Path self { get; }
    pub fn get_self() -> crate::stub::Path {
        Default::default()
    }

    // C# signature: string componentsString { get; }
    pub fn get_componentsString(&mut self) -> String {
        Default::default()
    }
}
