// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/VariableReference.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct VariableReference {
    pub _port_marker: (),
}

impl VariableReference {
    // C# signature: public VariableReference (string name)
    pub fn new(_name: String) -> Self {
        Default::default()
    }

    // C# signature: public VariableReference()
    pub fn new_overload_2() -> Self {
        Default::default()
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: string name { get; }
    pub fn get_name(&mut self) -> String {
        Default::default()
    }

    // C# signature: Path pathForCount { get; }
    pub fn get_pathForCount(&mut self) -> crate::stub::Path {
        Default::default()
    }

    // C# signature: Container containerForCount { get; }
    pub fn get_containerForCount(&mut self) -> crate::stub::Container {
        Default::default()
    }

    // C# signature: string pathStringForCount { get; }
    pub fn get_pathStringForCount(&mut self) -> String {
        Default::default()
    }
}
