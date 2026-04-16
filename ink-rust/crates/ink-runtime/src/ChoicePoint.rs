// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/ChoicePoint.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct ChoicePoint {
    pub _port_marker: (),
}

impl ChoicePoint {
    // C# signature: public ChoicePoint (bool onceOnly)
    pub fn new(_onceOnly: bool) -> Self {
        Default::default()
    }

    // C# signature: public ChoicePoint()
    pub fn new_overload_2() -> Self {
        Default::default()
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: Path pathOnChoice { get; }
    pub fn get_pathOnChoice(&mut self) -> crate::stub::Path {
        Default::default()
    }

    // C# signature: Container choiceTarget { get; }
    pub fn get_choiceTarget(&mut self) -> crate::stub::Container {
        Default::default()
    }

    // C# signature: string pathStringOnChoice { get; }
    pub fn get_pathStringOnChoice(&mut self) -> String {
        Default::default()
    }

    // C# signature: bool hasCondition { get; }
    pub fn get_hasCondition(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool hasStartContent { get; }
    pub fn get_hasStartContent(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool hasChoiceOnlyContent { get; }
    pub fn get_hasChoiceOnlyContent(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool onceOnly { get; }
    pub fn get_onceOnly(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isInvisibleDefault { get; }
    pub fn get_isInvisibleDefault(&mut self) -> bool {
        Default::default()
    }

    // C# signature: int flags { get; }
    pub fn get_flags(&mut self) -> i32 {
        Default::default()
    }
}
