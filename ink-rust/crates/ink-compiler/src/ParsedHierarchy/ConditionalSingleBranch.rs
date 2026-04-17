// Source: ink-c-sharp/compiler/ParsedHierarchy/ConditionalSingleBranch.cs

use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::Object::Object;
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
}

impl ConditionalSingleBranch {
    // C# signature: public ConditionalSingleBranch (List<Parsed.Object> content)
    pub fn new(content: Vec<Object>) -> Self {
        let mut base = Object::new();
        base.content = content;
        Self {
            base,
            ..Default::default()
        }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> RuntimeContainer {
        if let Some(inner_text) = self.base.content.iter().find_map(|obj| {
            obj.get_runtimeObject().and_then(|runtime| {
                runtime.get_content().iter().find_map(|item| match item {
                    ContentItem::Value(ink_runtime::Value::Value::String(value)) => {
                        Some(value.clone())
                    }
                    _ => None,
                })
            })
        }) {
            if inner_text.value.starts_with("else:") {
                self.base.Warning(
                    "Saw the text 'else:' which is being treated as content. Did you mean '- else:'?"
                        .to_string(),
                    None,
                );
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
        return_divert.set_targetPathString(Some(content_path.ToString()));
        content_container.AddContent(return_divert.clone());

        self.returnDivert = Some(return_divert);
        self.contentContainer = Some(content_container);
        self.conditionalDivert = Some(conditional_divert);

        container
    }

    fn GenerateRuntimeForContent(&mut self) -> RuntimeContainer {
        if self.base.content.is_empty() {
            return RuntimeContainer::new();
        }

        let mut weave = Weave::new(self.base.content.clone(), -1);
        weave
            .get_rootContainer()
            .unwrap_or_else(RuntimeContainer::new)
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        if let Some(content_container) = &self.contentContainer {
            if let Some(conditional_divert) = &mut self.conditionalDivert {
                conditional_divert
                    .set_targetPathString(Some(content_container.get_path().ToString()));
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
            return_divert.set_targetPathString(path.map(|p| p.ToString()));
        }
    }

    pub fn get_base(&self) -> &Object {
        &self.base
    }
}
