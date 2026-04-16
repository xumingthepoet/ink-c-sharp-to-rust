// Source: ink-c-sharp/compiler/ParsedHierarchy/ConstantDeclaration.cs

use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Story::Story;

#[derive(Clone, Debug, Default)]
pub struct ConstantDeclaration {
    pub constantIdentifier: Option<Identifier>,
    pub expression: Option<Box<Expression>>,
}

impl ConstantDeclaration {
    // C# signature: public ConstantDeclaration (Identifier name, Expression assignedExpression)
    pub fn new(name: Identifier, assignedExpression: Option<Expression>) -> Self {
        Self {
            constantIdentifier: Some(name),
            expression: assignedExpression.map(Box::new),
        }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&self) -> Option<ink_runtime::Object::Object> {
        None
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: &mut Story) {
        // Collision checking belongs to Parsed.Story once that type is fully ported.
    }

    // C# signature: string constantName { get; }
    pub fn get_constantName(&self) -> Option<&str> {
        self.constantIdentifier
            .as_ref()
            .and_then(|identifier| identifier.name.as_ref().map(std::string::String::as_str))
    }

    // C# signature: Identifier constantIdentifier { get; }
    pub fn get_constantIdentifier(&self) -> Option<&Identifier> {
        self.constantIdentifier.as_ref()
    }

    // C# signature: Expression expression { get; }
    pub fn get_expression(&self) -> Option<&Expression> {
        self.expression.as_deref()
    }

    // C# signature: string typeName { get; }
    pub fn get_typeName(&self) -> &str {
        "Constant"
    }
}

#[cfg(test)]
mod tests {
    use super::ConstantDeclaration;
    use crate::ParsedHierarchy::Identifier::Identifier;

    #[test]
    fn exposes_constant_name_and_type() {
        let decl = ConstantDeclaration::new(
            Identifier {
                name: Some("MAX".to_string()),
                debugMetadata: None,
            },
            None,
        );

        assert_eq!(decl.get_constantName(), Some("MAX"));
        assert_eq!(decl.get_typeName(), "Constant");
        assert!(decl.GenerateRuntimeObject().is_none());
    }
}
