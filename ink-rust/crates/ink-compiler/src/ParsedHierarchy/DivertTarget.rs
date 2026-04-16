// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/DivertTarget.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct DivertTarget {
    pub _port_marker: (),
}

impl DivertTarget {
    // C# signature: public DivertTarget (Divert divert)
    pub fn new(_divert: crate::stub::Divert) -> Self {
        Default::default()
    }

    // C# signature: public override void GenerateIntoContainer (Runtime.Container container)
    pub fn GenerateIntoContainer(&mut self, _container: crate::stub::Container) {}

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: public override bool Equals (object obj)
    pub fn Equals(&mut self, _obj: crate::stub::PortStub) -> bool {
        Default::default()
    }

    // C# signature: public override int GetHashCode ()
    pub fn GetHashCode(&mut self) -> i32 {
        Default::default()
    }
}
