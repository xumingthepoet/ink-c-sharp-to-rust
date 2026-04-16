// Source: ink-c-sharp/compiler/ParsedHierarchy/ConstantDeclaration.cs

use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Story::Story;

#[derive(Clone, Debug, Default)]
pub struct ConstantDeclaration {
    pub constantIdentifier: Option<Identifier>,
    pub expression: Option<Expression>,
}

impl ConstantDeclaration {
    // C# signature: public ConstantDeclaration (Identifier name, Expression assignedExpression)
    pub fn new(name: Identifier, assignedExpression: Option<Expression>) -> Self {
        Self {
            constantIdentifier: Some(name),
            expression: assignedExpression,
        }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        crate::stub::PortStub::default()
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: Story) {}

    // C# signature: string constantName { get; }
    pub fn get_constantName(&self) -> Option<&str> {
        self.constantIdentifier
            .as_ref()
            .and_then(|identifier| identifier.name.as_deref())
    }

    // C# signature: Identifier constantIdentifier { get; }
    pub fn get_constantIdentifier(&self) -> Option<&Identifier> {
        self.constantIdentifier.as_ref()
    }

    // C# signature: Expression expression { get; }
    pub fn get_expression(&self) -> Option<&Expression> {
        self.expression.as_ref()
    }

    // C# signature: string typeName { get; }
    pub fn get_typeName(&self) -> String {
        "Constant".to_string()
    }
}
