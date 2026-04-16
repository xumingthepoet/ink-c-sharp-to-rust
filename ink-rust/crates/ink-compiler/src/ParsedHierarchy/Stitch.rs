// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Stitch.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Stitch {
    pub _port_marker: (),
}

impl Stitch {
    // C# signature: public Stitch (Identifier name, List<Parsed.Object> topLevelObjects, List<Argument> arguments, bool isFunction)
    pub fn new(
        _name: crate::stub::Identifier,
        _topLevelObjects: Vec<crate::stub::PortStub>,
        _arguments: Vec<crate::stub::Argument>,
        _isFunction: bool,
    ) -> Self {
        Default::default()
    }

    // C# signature: FlowLevel flowLevel { get; }
    pub fn get_flowLevel(&mut self) -> crate::stub::FlowLevel {
        Default::default()
    }
}
