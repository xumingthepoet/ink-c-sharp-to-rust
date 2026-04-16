// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Conditional.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Conditional {
    pub _port_marker: (),
}

impl Conditional {
    // C# signature: public Conditional (Expression condition, List<ConditionalSingleBranch> branches)
    pub fn new(
        _condition: crate::stub::Expression,
        _branches: Vec<crate::stub::ConditionalSingleBranch>,
    ) -> Self {
        Default::default()
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: Expression initialCondition { get; }
    pub fn get_initialCondition(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: List<ConditionalSingleBranch> branches { get; }
    pub fn get_branches(&mut self) -> Vec<crate::stub::ConditionalSingleBranch> {
        Default::default()
    }
}
