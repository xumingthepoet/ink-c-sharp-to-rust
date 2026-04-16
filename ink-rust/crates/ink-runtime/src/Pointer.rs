// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Pointer.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Pointer {
    pub _port_marker: (),
}

impl Pointer {
    // C# signature: public Pointer (Container container, int index)
    pub fn new(_container: crate::stub::Container, _index: i32) -> Self {
        Default::default()
    }

    // C# signature: public Runtime.Object Resolve ()
    pub fn Resolve(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: public static Pointer StartOf (Container container)
    pub fn StartOf(_container: crate::stub::Container) -> crate::stub::Pointer {
        Default::default()
    }

    // C# signature: bool isNull { get; }
    pub fn get_isNull(&mut self) -> bool {
        Default::default()
    }

    // C# signature: Path path { get; }
    pub fn get_path(&mut self) -> crate::stub::Path {
        Default::default()
    }
}
