// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/InkParser/InkParser_Conditional.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct InkParser {
    pub _port_marker: (),
}

impl InkParser {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: protected Conditional InnerConditionalContent()
    pub fn InnerConditionalContent(&mut self) -> crate::stub::Conditional {
        Default::default()
    }

    // C# signature: protected Conditional InnerConditionalContent(Expression initialQueryExpression)
    pub fn InnerConditionalContent_overload_2(
        &mut self,
        _initialQueryExpression: crate::stub::Expression,
    ) -> crate::stub::Conditional {
        Default::default()
    }

    // C# signature: protected List<ConditionalSingleBranch> InlineConditionalBranches()
    pub fn InlineConditionalBranches(&mut self) -> Vec<crate::stub::ConditionalSingleBranch> {
        Default::default()
    }

    // C# signature: protected List<ConditionalSingleBranch> MultilineConditionalBranches()
    pub fn MultilineConditionalBranches(&mut self) -> Vec<crate::stub::ConditionalSingleBranch> {
        Default::default()
    }

    // C# signature: protected ConditionalSingleBranch SingleMultilineCondition()
    pub fn SingleMultilineCondition(&mut self) -> crate::stub::ConditionalSingleBranch {
        Default::default()
    }

    // C# signature: protected Expression ConditionExpression()
    pub fn ConditionExpression(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: protected object ElseExpression()
    pub fn ElseExpression(&mut self) -> crate::stub::PortStub {
        Default::default()
    }
}
