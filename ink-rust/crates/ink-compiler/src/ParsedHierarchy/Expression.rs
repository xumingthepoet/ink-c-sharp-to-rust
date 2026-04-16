// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Expression.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Expression {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct BinaryExpression {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct UnaryExpression {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct IncDecExpression {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct MultipleConditionExpression {
    pub _port_marker: (),
}

impl Expression {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public void GenerateConstantIntoContainer(Runtime.Container container)
    pub fn GenerateConstantIntoContainer(&mut self, _container: crate::stub::Container) {}

    // C# signature: public abstract void GenerateIntoContainer (Runtime.Container container)
    pub fn GenerateIntoContainer(&mut self, _container: crate::stub::Container) {}

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: public static Expression WithInner(Expression inner, string op)
    pub fn WithInner(_inner: crate::stub::Expression, _op: String) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: bool outputWhenComplete { get; }
    pub fn get_outputWhenComplete(&mut self) -> bool {
        Default::default()
    }

    // C# signature: List<Expression> subExpressions { get; }
    pub fn get_subExpressions(&mut self) -> Vec<crate::stub::Expression> {
        Default::default()
    }
}
