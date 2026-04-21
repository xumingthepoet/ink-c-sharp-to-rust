// Source: ink-c-sharp/compiler/ParsedHierarchy/VariableAssignment.cs

use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::ListDefinition::ListDefinition;
use crate::ParsedHierarchy::Object::Object;
use crate::ParsedHierarchy::Story::Story;
use crate::ParsedHierarchy::VariableReference::VariableReference;
use ink_runtime::Container::{Container, ContentItem};
use ink_runtime::VariableAssignment::VariableAssignment as RuntimeVariableAssignment;
use std::rc::Rc;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VariableAssignment {
    base: Object,
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
        let mut base = Object::new();
        base.AddContent(Object::from_expression(assignedExpression.clone()));

        Self {
            base,
            variableIdentifier: Some(identifier),
            expression: Some(assignedExpression),
            ..Default::default()
        }
    }

    // C# signature: public VariableAssignment (Identifier identifier, ListDefinition listDef)
    pub fn new_overload_2(
        identifier: crate::ParsedHierarchy::Identifier::Identifier,
        listDef: ListDefinition,
    ) -> Self {
        let base = Object::new();
        let mut assignment = Self {
            base,
            variableIdentifier: Some(identifier),
            listDefinition: Some(Box::new(listDef)),
            isGlobalDeclaration: true,
            ..Default::default()
        };

        let self_clone = assignment.clone();
        if let Some(list_definition) = &mut assignment.listDefinition {
            list_definition.variableAssignment = Some(self_clone);
            assignment
                .base
                .AddContent(Object::from_list_definition((**list_definition).clone()));
        }

        assignment
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

        Some(ContentItem::Container(Rc::new(container)))
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        if let Some(expression) = &self.expression {
            self.base.content.clear();
            self.base
                .AddContent(Object::from_expression(expression.clone()));
        } else if let Some(list_definition) = &self.listDefinition {
            self.base.content.clear();
            self.base
                .AddContent(Object::from_list_definition((**list_definition).clone()));
        }

        if self.listDefinition.is_some() {
            let self_clone = self.clone();
            if let Some(list_definition) = &mut self.listDefinition {
                list_definition.variableAssignment = Some(self_clone);
            }
        }

        self.base.ResolveReferences(context);

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

    pub fn get_base(&self) -> &Object {
        &self.base
    }
}

#[cfg(test)]
mod tests {
    use super::{ListDefinition, VariableAssignment};
    use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::ListDefinition::ListElementDefinition;
    use crate::ParsedHierarchy::Number::{Number, NumberValue};
    use crate::ParsedHierarchy::Story::Story;

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

    #[test]
    fn resolve_references_threads_list_backref() {
        let list_definition = ListDefinition::new(vec![ListElementDefinition::new(
            Identifier {
                name: Some("item".to_string()),
                debugMetadata: None,
            },
            true,
            None,
        )]);

        let mut assignment = VariableAssignment::new_overload_2(
            Identifier {
                name: Some("food".to_string()),
                debugMetadata: None,
            },
            list_definition,
        );

        let mut story = Story::new(vec![], false);
        let mut registry_list = ListDefinition::new(vec![ListElementDefinition::new(
            Identifier {
                name: Some("item".to_string()),
                debugMetadata: None,
            },
            true,
            None,
        )]);
        registry_list.identifier = Some(Identifier {
            name: Some("food".to_string()),
            debugMetadata: None,
        });
        story.register_list_definition(registry_list);

        assignment.ResolveReferences(&mut story);

        let backref = assignment
            .get_listDefinition()
            .unwrap()
            .variableAssignment
            .as_ref();
        assert!(backref.is_some());
        assert_eq!(backref.unwrap().get_variableName(), "food");
    }

    #[test]
    fn list_assignment_constructor_populates_backref() {
        let list_definition = ListDefinition::new(vec![ListElementDefinition::new(
            Identifier {
                name: Some("item".to_string()),
                debugMetadata: None,
            },
            true,
            None,
        )]);

        let assignment = VariableAssignment::new_overload_2(
            Identifier {
                name: Some("food".to_string()),
                debugMetadata: None,
            },
            list_definition,
        );

        assert_eq!(
            assignment
                .get_listDefinition()
                .and_then(|list_def| list_def.variableAssignment.as_ref())
                .map(|backref| backref.get_variableName().to_string()),
            Some("food".to_string())
        );
    }

    #[test]
    fn expression_assignment_embeds_expression_in_base_tree() {
        let identifier = Identifier {
            name: Some("score".to_string()),
            debugMetadata: None,
        };
        let expression =
            Expression::from_kind(ExpressionKind::Number(Number::new(NumberValue::Int(3))));

        let assignment = VariableAssignment::new(identifier, expression);

        assert_eq!(assignment.get_base().content.len(), 1);
        assert!(matches!(
            assignment.get_base().content[0].payload.as_ref(),
            Some(crate::ParsedHierarchy::Object::ObjectPayload::Expression(_))
        ));
    }

    #[test]
    fn list_assignment_embeds_list_definition_in_base_tree() {
        let list_definition = ListDefinition::new(vec![ListElementDefinition::new(
            Identifier {
                name: Some("item".to_string()),
                debugMetadata: None,
            },
            true,
            None,
        )]);

        let assignment = VariableAssignment::new_overload_2(
            Identifier {
                name: Some("food".to_string()),
                debugMetadata: None,
            },
            list_definition,
        );

        assert_eq!(assignment.get_base().content.len(), 1);
        assert!(matches!(
            assignment.get_base().content[0].payload.as_ref(),
            Some(crate::ParsedHierarchy::Object::ObjectPayload::ListDefinition(_))
        ));
    }
}
