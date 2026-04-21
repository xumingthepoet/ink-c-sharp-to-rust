// Source: ink-c-sharp/compiler/ParsedHierarchy/ConditionalSingleBranch.cs

use crate::ParsedHierarchy::ContentList::ContentListItem;
use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::Object::{Object, ObjectKind, ObjectPayload};
use crate::ParsedHierarchy::Story::Story;
use crate::ParsedHierarchy::Weave::Weave;
use ink_runtime::Container::{Container as RuntimeContainer, ContentItem};
use ink_runtime::ControlCommand::ControlCommand;
use ink_runtime::Divert::Divert as RuntimeDivert;
use ink_runtime::NativeFunctionCall::NativeFunctionCall as RuntimeNativeFunctionCall;
use ink_runtime::Path::Component;
use ink_runtime::Value::StringValue;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConditionalSingleBranch {
    base: Object,
    pub isTrueBranch: bool,
    ownExpression: Option<Expression>,
    pub matchingEquality: bool,
    pub isElse: bool,
    pub isInline: bool,
    returnDivert: Option<RuntimeDivert>,
    contentContainer: Option<RuntimeContainer>,
    conditionalDivert: Option<RuntimeDivert>,
    innerWeave: Option<Weave>,
}

impl ConditionalSingleBranch {
    // C# signature: public ConditionalSingleBranch (List<Parsed.Object> content)
    pub fn new(content: Vec<Object>) -> Self {
        let mut base = Object::new();
        let innerWeave = {
            let weave = Weave::new(content, -1);
            let mut weave_object = Object::with_kind(ObjectKind::Weave);
            weave_object.content = weave.base.content.clone();
            weave_object.set_identifier(weave.base.identifier.clone());
            weave_object.set_indentationDepth(weave.base.indentationDepth);
            weave_object.set_debugMetadata(weave.base.get_debugMetadata().cloned());
            weave_object.payload = Some(ObjectPayload::Weave(Box::new(weave.clone())));
            base.AddContent(weave_object);
            Some(weave)
        };

        Self {
            base,
            innerWeave,
            ..Default::default()
        }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> RuntimeContainer {
        if let Some(inner_weave) = &self.innerWeave {
            for obj in &inner_weave.base.content {
                if Self::object_starts_with_else_text(obj) {
                    self.base.Warning(
                        "Saw the text 'else:' which is being treated as content. Did you mean '- else:'?"
                            .to_string(),
                        Some(obj.clone()),
                    );
                }
            }
        }

        let mut container = RuntimeContainer::new();

        let duplicates_stack_value = self.matchingEquality && !self.isElse;
        if duplicates_stack_value {
            container.AddContent(ControlCommand::Duplicate());
        }

        let mut conditional_divert = RuntimeDivert::new();
        conditional_divert.set_isConditional(!self.isElse);

        if !self.isTrueBranch && !self.isElse {
            let needs_eval = self.ownExpression.is_some();
            if needs_eval {
                container.AddContent(ControlCommand::EvalStart());
            }

            if let Some(expr) = &self.ownExpression {
                expr.GenerateIntoContainer(&mut container);
            }

            if self.matchingEquality {
                container.AddContent(RuntimeNativeFunctionCall::CallWithName("==".to_string()));
            }

            if needs_eval {
                container.AddContent(ControlCommand::EvalEnd());
            }
        }

        container.AddContent(conditional_divert.clone());

        let mut content_container = self.GenerateRuntimeForContent();
        content_container.set_name(Some("b".to_string()));

        if !self.isInline {
            content_container.InsertContent(StringValue::new("\n".to_string()), 0);
        }

        if duplicates_stack_value || (self.isElse && self.matchingEquality) {
            content_container.InsertContent(ControlCommand::PopEvaluatedValue(), 0);
        }

        let mut content_path = content_container.get_path().clone();
        if content_path.get_length() == 0 {
            content_path = content_path.PathByAppendingComponent(Component::new_overload_2(
                content_container.get_name().to_string(),
            ));
        }
        content_container.set_path(content_path.clone());

        container.AddToNamedContentOnly(content_container.clone());

        let mut return_divert = RuntimeDivert::new();
        return_divert.set_targetPath(Some(content_path.clone()));
        content_container.AddContent(return_divert.clone());

        self.returnDivert = Some(return_divert);
        self.contentContainer = Some(content_container);
        self.conditionalDivert = Some(conditional_divert);

        container
    }

    fn GenerateRuntimeForContent(&mut self) -> RuntimeContainer {
        self.innerWeave
            .as_mut()
            .and_then(|weave| weave.get_rootContainer())
            .unwrap_or_else(RuntimeContainer::new)
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        if let Some(own_expression) = &mut self.ownExpression {
            own_expression.ResolveReferences(context);
        }

        if let Some(inner_weave) = &mut self.innerWeave {
            inner_weave.ResolveReferences(context);
        }

        if let Some(content_container) = &self.contentContainer {
            if let Some(conditional_divert) = &mut self.conditionalDivert {
                conditional_divert.set_targetPath(Some(content_container.get_path()));
            }
        }

        self.base.ResolveReferences(context);
    }

    // C# signature: bool isTrueBranch { get; }
    pub fn get_isTrueBranch(&self) -> bool {
        self.isTrueBranch
    }

    pub fn set_isTrueBranch(&mut self, value: bool) {
        self.isTrueBranch = value;
    }

    // C# signature: Expression ownExpression { get; }
    pub fn get_ownExpression(&self) -> Option<&Expression> {
        self.ownExpression.as_ref()
    }

    pub fn set_ownExpression(&mut self, value: Option<Expression>) {
        self.ownExpression = value;
        if let Some(own_expression) = self.ownExpression.clone() {
            self.base
                .AddContent(Object::from_expression(own_expression));
        }
    }

    // C# signature: bool matchingEquality { get; }
    pub fn get_matchingEquality(&self) -> bool {
        self.matchingEquality
    }

    pub fn set_matchingEquality(&mut self, value: bool) {
        self.matchingEquality = value;
    }

    // C# signature: bool isElse { get; }
    pub fn get_isElse(&self) -> bool {
        self.isElse
    }

    pub fn set_isElse(&mut self, value: bool) {
        self.isElse = value;
    }

    // C# signature: bool isInline { get; }
    pub fn get_isInline(&self) -> bool {
        self.isInline
    }

    pub fn set_isInline(&mut self, value: bool) {
        self.isInline = value;
    }

    // C# signature: Runtime.Divert returnDivert { get; }
    pub fn get_returnDivert(&self) -> Option<&RuntimeDivert> {
        self.returnDivert.as_ref()
    }

    pub fn set_returnDivertTargetPath(&mut self, path: Option<ink_runtime::Path::Path>) {
        if let Some(return_divert) = &mut self.returnDivert {
            return_divert.set_targetPath(path);
        }
    }

    pub fn get_base(&self) -> &Object {
        &self.base
    }

    fn object_starts_with_else_text(obj: &Object) -> bool {
        match obj.payload.as_ref() {
            Some(ObjectPayload::ContentList(content_list)) => {
                Self::content_list_starts_with_else_text(content_list.as_ref())
            }
            _ => false,
        }
    }

    fn content_list_starts_with_else_text(
        content_list: &crate::ParsedHierarchy::ContentList::ContentList,
    ) -> bool {
        content_list.get_content().first().is_some_and(
            |item| matches!(item, ContentListItem::Text(text) if text.text.starts_with("else:")),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ConditionalSingleBranch;
    use crate::ParsedHierarchy::ContentList::ContentListItem;
    use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::Story::Story;
    use crate::ParsedHierarchy::VariableReference::VariableReference;

    #[test]
    fn resolve_references_visits_branch_own_expression() {
        let mut branch = ConditionalSingleBranch::new(vec![]);
        branch.set_ownExpression(Some(Expression::from_kind(
            ExpressionKind::VariableReference(Box::new(VariableReference::new(vec![Identifier {
                name: Some("score".to_string()),
                debugMetadata: None,
            }]))),
        )));

        let mut story = Story::new(vec![], false);
        branch.ResolveReferences(&mut story);

        let resolved = branch
            .get_ownExpression()
            .and_then(|expression| match &expression.kind {
                ExpressionKind::VariableReference(reference) => {
                    Some(reference.get_runtimeVarRef().is_some())
                }
                _ => None,
            })
            .unwrap_or(false);

        assert!(resolved);
    }

    #[test]
    fn content_list_helper_detects_else_prefix_text() {
        let content_list =
            crate::ParsedHierarchy::ContentList::ContentList::new(vec![ContentListItem::from(
                crate::ParsedHierarchy::Text::Text::new("else: content".to_string()),
            )]);

        assert!(ConditionalSingleBranch::content_list_starts_with_else_text(
            &content_list
        ));
    }
}
