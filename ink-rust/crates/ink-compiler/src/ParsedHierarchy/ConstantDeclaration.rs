// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/ConstantDeclaration.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct ConstantDeclaration {
    pub _port_marker: (),
}

impl ConstantDeclaration {
    // C# signature: public ConstantDeclaration (Identifier name, Expression assignedExpression)
    pub fn new(
        _name: crate::stub::Identifier,
        _assignedExpression: crate::stub::Expression,
    ) -> Self {
        Default::default()
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: string constantName { get; }
    pub fn get_constantName(&mut self) -> String {
        Default::default()
    }

    // C# signature: Identifier constantIdentifier { get; }
    pub fn get_constantIdentifier(&mut self) -> crate::stub::Identifier {
        Default::default()
    }

    // C# signature: Expression expression { get; }
    pub fn get_expression(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: string typeName { get; }
    pub fn get_typeName(&mut self) -> String {
        Default::default()
    }
}
