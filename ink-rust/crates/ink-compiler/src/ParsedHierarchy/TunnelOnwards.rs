// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/TunnelOnwards.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct TunnelOnwards {
    pub _port_marker: (),
}

impl TunnelOnwards {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: Divert divertAfter { get; }
    pub fn get_divertAfter(&mut self) -> crate::stub::Divert {
        Default::default()
    }
}
