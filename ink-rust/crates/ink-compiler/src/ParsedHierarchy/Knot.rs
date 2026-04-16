// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Knot.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Knot {
    pub _port_marker: (),
}

impl Knot {
    // C# signature: public Knot (Identifier name, List<Parsed.Object> topLevelObjects, List<Argument> arguments, bool isFunction)
    pub fn new(
        _name: crate::stub::Identifier,
        _topLevelObjects: Vec<crate::stub::PortStub>,
        _arguments: Vec<crate::stub::Argument>,
        _isFunction: bool,
    ) -> Self {
        Default::default()
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: FlowLevel flowLevel { get; }
    pub fn get_flowLevel(&mut self) -> crate::stub::FlowLevel {
        Default::default()
    }
}
