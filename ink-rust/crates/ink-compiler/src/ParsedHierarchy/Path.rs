// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Path.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Path {
    pub _port_marker: (),
}

impl Path {
    // C# signature: public Path(FlowLevel baseFlowLevel, List<Identifier> components)
    pub fn new(
        _baseFlowLevel: crate::stub::FlowLevel,
        _components: Vec<crate::stub::Identifier>,
    ) -> Self {
        Default::default()
    }

    // C# signature: public Path(List<Identifier> components)
    pub fn new_overload_2(_components: Vec<crate::stub::Identifier>) -> Self {
        Default::default()
    }

    // C# signature: public Path(Identifier ambiguousName)
    pub fn new_overload_3(_ambiguousName: crate::stub::Identifier) -> Self {
        Default::default()
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: public Parsed.Object ResolveFromContext(Parsed.Object context)
    pub fn ResolveFromContext(&mut self, _context: crate::stub::PortStub) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: FlowLevel baseTargetLevel { get; }
    pub fn get_baseTargetLevel(&mut self) -> crate::stub::FlowLevel {
        Default::default()
    }

    // C# signature: bool baseLevelIsAmbiguous { get; }
    pub fn get_baseLevelIsAmbiguous(&mut self) -> bool {
        Default::default()
    }

    // C# signature: string firstComponent { get; }
    pub fn get_firstComponent(&mut self) -> String {
        Default::default()
    }

    // C# signature: int numberOfComponents { get; }
    pub fn get_numberOfComponents(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: string dotSeparatedComponents { get; }
    pub fn get_dotSeparatedComponents(&mut self) -> String {
        Default::default()
    }

    // C# signature: List<Identifier> components { get; }
    pub fn get_components(&mut self) -> Vec<crate::stub::Identifier> {
        Default::default()
    }
}
