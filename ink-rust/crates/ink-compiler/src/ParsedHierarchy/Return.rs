// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Return.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Return {
    pub _port_marker: (),
}

impl Return {
    // C# signature: public Return (Expression returnedExpression = null)
    pub fn new(_returnedExpression: crate::stub::Expression) -> Self {
        Default::default()
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: Expression returnedExpression { get; }
    pub fn get_returnedExpression(&mut self) -> crate::stub::Expression {
        Default::default()
    }
}
