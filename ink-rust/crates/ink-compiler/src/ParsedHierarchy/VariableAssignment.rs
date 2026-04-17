// Source: ink-c-sharp/compiler/ParsedHierarchy/VariableAssignment.cs

use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::ListDefinition::ListDefinition;
use crate::ParsedHierarchy::Story::Story;
use crate::ParsedHierarchy::VariableReference::VariableReference;
use ink_runtime::Container::{Container, ContentItem};
use ink_runtime::VariableAssignment::VariableAssignment as RuntimeVariableAssignment;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VariableAssignment {
    variableIdentifier: Option<crate::ParsedHierarchy::Identifier::Identifier>,
    expression: Option<Expression>,
    listDefinition: Option<Box<ListDefinition>>,
    isGlobalDeclaration: bool,
    isNewTemporaryDeclaration: bool,
    runtimeAssignment: Option<RuntimeVariableAssignment>,
}

impl VariableAssignment {
    // C# signature: public VariableAssignment (Identifier identifier, Expression assignedExpression)
    pub fn new(
        identifier: crate::ParsedHierarchy::Identifier::Identifier,
        assignedExpression: Expression,
    ) -> Self {
        Self {
            variableIdentifier: Some(identifier),
            expression: Some(assignedExpression),
            ..Default::default()
        }
    }

    // C# signature: public VariableAssignment (Identifier identifier, ListDefinition listDef)
    pub fn new_overload_2(
        identifier: crate::ParsedHierarchy::Identifier::Identifier,
        mut listDef: ListDefinition,
    ) -> Self {
        listDef.variableAssignment = None;
        Self {
            variableIdentifier: Some(identifier),
            listDefinition: Some(Box::new(listDef)),
            isGlobalDeclaration: true,
            ..Default::default()
        }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> Option<ContentItem> {
        if self.isGlobalDeclaration {
            return None;
        }

        let mut container = Container::new();

        if let Some(expression) = &self.expression {
            container.AddContent(expression.GenerateRuntimeObject());
        } else if let Some(listDefinition) = &mut self.listDefinition {
            container.AddContent(listDefinition.GenerateRuntimeObject());
        }

        let runtime_assignment =
            RuntimeVariableAssignment::new(self.get_variableName(), self.isNewTemporaryDeclaration);
        container.AddContent(runtime_assignment.clone());
        self.runtimeAssignment = Some(runtime_assignment);

        Some(ContentItem::Container(Box::new(container)))
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        if self.isDeclaration() && self.listDefinition.is_none() {
            context.CheckForNamingCollisions(
                Default::default(),
                self.variableIdentifier.clone().unwrap_or_default(),
                if self.isGlobalDeclaration {
                    crate::ParsedHierarchy::Story::SymbolType::Var
                } else {
                    crate::ParsedHierarchy::Story::SymbolType::Temp
                },
                String::new(),
            );
        }

        if self.isGlobalDeclaration {
            if let Some(variableReference) =
                self.expression
                    .as_ref()
                    .and_then(|expression| match &expression.kind {
                        crate::ParsedHierarchy::Expression::ExpressionKind::VariableReference(
                            reference,
                        ) => Some(reference.as_ref()),
                        _ => None,
                    })
            {
                if !variableReference.isConstantReference && !variableReference.isListItemReference
                {
                    context.Error(
                        "global variable assignments cannot refer to other variables, only literal values, constants and list items".to_string(),
                        Default::default(),
                        false,
                    );
                }
            }
        }

        if !self.isNewTemporaryDeclaration {
            let resolved =
                context.ResolveVariableWithName(self.get_variableName(), Default::default());
            if !resolved.found {
                if context.constants.contains_key(&self.get_variableName()) {
                    context.Error(
                        format!(
                            "Can't re-assign to a constant (do you need to use VAR when declaring '{}'?)",
                            self.get_variableName()
                        ),
                        Default::default(),
                        false,
                    );
                } else {
                    context.Error(
                        format!(
                            "Variable could not be found to assign to: '{}'",
                            self.get_variableName()
                        ),
                        Default::default(),
                        false,
                    );
                }
            }

            if let Some(runtimeAssignment) = &mut self.runtimeAssignment {
                runtimeAssignment.set_isGlobal(resolved.isGlobal);
            }
        }
    }

    // C# signature: string variableName { get; }
    pub fn get_variableName(&self) -> String {
        self.variableIdentifier
            .as_ref()
            .and_then(|identifier| identifier.name.clone())
            .unwrap_or_default()
    }

    // C# signature: Identifier variableIdentifier { get; }
    pub fn get_variableIdentifier(
        &self,
    ) -> Option<&crate::ParsedHierarchy::Identifier::Identifier> {
        self.variableIdentifier.as_ref()
    }

    // C# signature: Expression expression { get; }
    pub fn get_expression(&self) -> Option<&Expression> {
        self.expression.as_ref()
    }

    // C# signature: ListDefinition listDefinition { get; }
    pub fn get_listDefinition(&self) -> Option<&ListDefinition> {
        self.listDefinition.as_deref()
    }

    // C# signature: bool isGlobalDeclaration { get; }
    pub fn get_isGlobalDeclaration(&self) -> bool {
        self.isGlobalDeclaration
    }

    pub fn set_isGlobalDeclaration(&mut self, value: bool) {
        self.isGlobalDeclaration = value;
    }

    // C# signature: bool isNewTemporaryDeclaration { get; }
    pub fn get_isNewTemporaryDeclaration(&self) -> bool {
        self.isNewTemporaryDeclaration
    }

    pub fn set_isNewTemporaryDeclaration(&mut self, value: bool) {
        self.isNewTemporaryDeclaration = value;
    }

    // C# signature: bool isDeclaration { get; }
    pub fn get_isDeclaration(&self) -> bool {
        self.isGlobalDeclaration || self.isNewTemporaryDeclaration
    }

    pub fn isDeclaration(&self) -> bool {
        self.get_isDeclaration()
    }

    // C# signature: string typeName { get; }
    pub fn get_typeName(&self) -> String {
        if self.isNewTemporaryDeclaration {
            "temp".to_string()
        } else if self.isGlobalDeclaration {
            "VAR".to_string()
        } else {
            "variable assignment".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VariableAssignment;
    use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::Number::{Number, NumberValue};

    #[test]
    fn declares_variable_and_reports_type_name() {
        let identifier = Identifier {
            name: Some("score".to_string()),
            debugMetadata: None,
        };
        let expression =
            Expression::from_kind(ExpressionKind::Number(Number::new(NumberValue::Int(3))));

        let assignment = VariableAssignment::new(identifier, expression);
        assert_eq!(assignment.get_variableName(), "score");
        assert_eq!(assignment.get_typeName(), "variable assignment");
    }
}
