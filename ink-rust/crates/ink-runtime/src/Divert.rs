// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Divert.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Divert {
    pub _port_marker: (),
}

impl Divert {
    // C# signature: public Divert ()
    pub fn new() -> Self {
        Default::default()
    }

    // C# signature: public Divert(PushPopType stackPushType)
    pub fn new_overload_2(_stackPushType: crate::stub::PushPopType) -> Self {
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

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: Path targetPath { get; }
    pub fn get_targetPath(&mut self) -> crate::stub::Path {
        Default::default()
    }

    // C# signature: Pointer targetPointer { get; }
    pub fn get_targetPointer(&mut self) -> crate::stub::Pointer {
        Default::default()
    }

    // C# signature: string targetPathString { get; }
    pub fn get_targetPathString(&mut self) -> String {
        Default::default()
    }

    // C# signature: string variableDivertName { get; }
    pub fn get_variableDivertName(&mut self) -> String {
        Default::default()
    }

    // C# signature: bool hasVariableTarget { get; }
    pub fn get_hasVariableTarget(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool pushesToStack { get; }
    pub fn get_pushesToStack(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isExternal { get; }
    pub fn get_isExternal(&mut self) -> bool {
        Default::default()
    }

    // C# signature: int externalArgs { get; }
    pub fn get_externalArgs(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: bool isConditional { get; }
    pub fn get_isConditional(&mut self) -> bool {
        Default::default()
    }
}
