// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Object.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Object {
    pub _port_marker: (),
}

impl Object {
    // C# signature: public Object ()
    pub fn new() -> Self {
        Default::default()
    }

    // C# signature: public int? DebugLineNumberOfPath(Path path)
    pub fn DebugLineNumberOfPath(&mut self, _path: crate::stub::Path) -> i32 {
        Default::default()
    }

    // C# signature: public SearchResult ResolvePath(Path path)
    pub fn ResolvePath(&mut self, _path: crate::stub::Path) -> crate::stub::SearchResult {
        Default::default()
    }

    // C# signature: public Path ConvertPathToRelative(Path globalPath)
    pub fn ConvertPathToRelative(&mut self, _globalPath: crate::stub::Path) -> crate::stub::Path {
        Default::default()
    }

    // C# signature: public string CompactPathString(Path otherPath)
    pub fn CompactPathString(&mut self, _otherPath: crate::stub::Path) -> String {
        Default::default()
    }

    // C# signature: public virtual Object Copy()
    pub fn Copy(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public void SetChild<T>(ref T obj, T value)
    pub fn SetChild(&mut self, _obj: &mut crate::stub::PortStub, _value: crate::stub::PortStub) {}

    // C# signature: public static implicit operator bool (Object obj)
    pub fn operator_stub(_obj: crate::stub::PortStub) -> crate::stub::implicit {
        Default::default()
    }

    // C# signature: public static bool operator ==(Object a, Object b)
    pub fn operator_stub_overload_2(_a: crate::stub::PortStub, _b: crate::stub::PortStub) -> bool {
        Default::default()
    }

    // C# signature: public override bool Equals (object obj)
    pub fn Equals(&mut self, _obj: crate::stub::PortStub) -> bool {
        Default::default()
    }

    // C# signature: public override int GetHashCode ()
    pub fn GetHashCode(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: Runtime.Object parent { get; }
    pub fn get_parent(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: Runtime.DebugMetadata debugMetadata { get; }
    pub fn get_debugMetadata(&mut self) -> crate::stub::DebugMetadata {
        Default::default()
    }

    // C# signature: Runtime.DebugMetadata ownDebugMetadata { get; }
    pub fn get_ownDebugMetadata(&mut self) -> crate::stub::DebugMetadata {
        Default::default()
    }

    // C# signature: Path path { get; }
    pub fn get_path(&mut self) -> crate::stub::Path {
        Default::default()
    }

    // C# signature: Container rootContentContainer { get; }
    pub fn get_rootContentContainer(&mut self) -> crate::stub::Container {
        Default::default()
    }
}
