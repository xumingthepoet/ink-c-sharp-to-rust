// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Sequence.cs

use crate::stub::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SequenceType {
    PortPlaceholder,
}

impl Default for SequenceType {
    fn default() -> Self {
        Self::PortPlaceholder
    }
}

#[derive(Clone, Debug, Default)]
pub struct Sequence {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct SequenceDivertToResolve {
    pub _port_marker: (),
}

impl Sequence {
    // C# signature: public Sequence (List<ContentList> elementContentLists, SequenceType sequenceType)
    pub fn new(
        _elementContentLists: Vec<crate::stub::ContentList>,
        _sequenceType: crate::stub::SequenceType,
    ) -> Self {
        Default::default()
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public override void ResolveReferences(Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}
}
