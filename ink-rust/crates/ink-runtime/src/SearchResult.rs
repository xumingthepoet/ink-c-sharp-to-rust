// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/SearchResult.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct SearchResult {
    pub _port_marker: (),
}

impl SearchResult {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: Runtime.Object correctObj { get; }
    pub fn get_correctObj(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: Container container { get; }
    pub fn get_container(&mut self) -> crate::stub::Container {
        Default::default()
    }
}
