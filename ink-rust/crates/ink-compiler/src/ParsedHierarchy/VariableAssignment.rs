// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/VariableAssignment.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct VariableAssignment {
    pub _port_marker: (),
}

impl VariableAssignment {
    // C# signature: public VariableAssignment (Identifier identifier, Expression assignedExpression)
    pub fn new(
        _identifier: crate::stub::Identifier,
        _assignedExpression: crate::stub::Expression,
    ) -> Self {
        Default::default()
    }

    // C# signature: public VariableAssignment (Identifier identifier, ListDefinition listDef)
    pub fn new_overload_2(
        _identifier: crate::stub::Identifier,
        _listDef: crate::stub::ListDefinition,
    ) -> Self {
        Default::default()
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: string variableName { get; }
    pub fn get_variableName(&mut self) -> String {
        Default::default()
    }

    // C# signature: Identifier variableIdentifier { get; }
    pub fn get_variableIdentifier(&mut self) -> crate::stub::Identifier {
        Default::default()
    }

    // C# signature: Expression expression { get; }
    pub fn get_expression(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: ListDefinition listDefinition { get; }
    pub fn get_listDefinition(&mut self) -> crate::stub::ListDefinition {
        Default::default()
    }

    // C# signature: bool isGlobalDeclaration { get; }
    pub fn get_isGlobalDeclaration(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isNewTemporaryDeclaration { get; }
    pub fn get_isNewTemporaryDeclaration(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isDeclaration { get; }
    pub fn get_isDeclaration(&mut self) -> bool {
        Default::default()
    }

    // C# signature: string typeName { get; }
    pub fn get_typeName(&mut self) -> String {
        Default::default()
    }
}
