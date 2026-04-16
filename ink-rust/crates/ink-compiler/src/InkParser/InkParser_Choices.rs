// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/InkParser/InkParser_Choices.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct InkParser {
    pub _port_marker: (),
}

impl InkParser {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: protected Choice Choice()
    pub fn Choice(&mut self) -> crate::stub::Choice {
        Default::default()
    }

    // C# signature: protected Expression ChoiceCondition()
    pub fn ChoiceCondition(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: protected object ChoiceConditionsSpace()
    pub fn ChoiceConditionsSpace(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected Expression ChoiceSingleCondition()
    pub fn ChoiceSingleCondition(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: protected Gather Gather()
    pub fn Gather(&mut self) -> crate::stub::Gather {
        Default::default()
    }

    // C# signature: protected object GatherDashes()
    pub fn GatherDashes(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected object ParseDashNotArrow()
    pub fn ParseDashNotArrow(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected Identifier BracketedName()
    pub fn BracketedName(&mut self) -> crate::stub::Identifier {
        Default::default()
    }
}
