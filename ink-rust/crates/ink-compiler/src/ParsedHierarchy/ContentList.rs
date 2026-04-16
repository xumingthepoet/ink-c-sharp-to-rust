// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/ContentList.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct ContentList {
    pub _port_marker: (),
}

impl ContentList {
    // C# signature: public ContentList (List<Parsed.Object> objects)
    pub fn new(_objects: Vec<crate::stub::PortStub>) -> Self {
        Default::default()
    }

    // C# signature: public ContentList()
    pub fn new_overload_2() -> Self {
        Default::default()
    }

    // C# signature: public void TrimTrailingWhitespace()
    pub fn TrimTrailingWhitespace(&mut self) {}

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: bool dontFlatten { get; }
    pub fn get_dontFlatten(&mut self) -> bool {
        Default::default()
    }

    // C# signature: Runtime.Container runtimeContainer { get; }
    pub fn get_runtimeContainer(&mut self) -> crate::stub::Container {
        Default::default()
    }
}
