// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/ConditionalSingleBranch.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct ConditionalSingleBranch {
    pub _port_marker: (),
}

impl ConditionalSingleBranch {
    // C# signature: public ConditionalSingleBranch (List<Parsed.Object> content)
    pub fn new(_content: Vec<crate::stub::PortStub>) -> Self {
        Default::default()
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: bool isTrueBranch { get; }
    pub fn get_isTrueBranch(&mut self) -> bool {
        Default::default()
    }

    // C# signature: Expression ownExpression { get; }
    pub fn get_ownExpression(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: bool matchingEquality { get; }
    pub fn get_matchingEquality(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isElse { get; }
    pub fn get_isElse(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isInline { get; }
    pub fn get_isInline(&mut self) -> bool {
        Default::default()
    }

    // C# signature: Runtime.Divert returnDivert { get; }
    pub fn get_returnDivert(&mut self) -> crate::stub::Divert {
        Default::default()
    }
}
