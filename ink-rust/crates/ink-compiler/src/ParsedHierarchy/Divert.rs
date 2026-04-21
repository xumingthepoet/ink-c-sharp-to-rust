// Source: ink-c-sharp/compiler/ParsedHierarchy/Divert.cs

use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
use crate::ParsedHierarchy::FlowBase::{Argument, FlowBase};
use crate::ParsedHierarchy::Object::{Object, ObjectKind};
use crate::ParsedHierarchy::Path::Path;
use ink_runtime::Container::{Container as RuntimeContainer, ContentItem};
use ink_runtime::ControlCommand::ControlCommand;
use ink_runtime::Divert::Divert as RuntimeDivert;
use ink_runtime::PushPop::PushPopType;
use ink_runtime::Value::Value;
use std::rc::Rc;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Divert {
    pub target: Option<Path>,
    pub targetContent: Option<Object>,
    pub arguments: Vec<Expression>,
    pub runtimeDivert: Option<RuntimeDivert>,
    pub isFunctionCall: bool,
    pub isEmpty: bool,
    pub isTunnel: bool,
    pub isThread: bool,
    resolvedTargetPathString: Option<String>,
    resolvedVariableDivertName: Option<String>,
}

impl Divert {
    // C# signature: public Divert (Parsed.Path target, List<Expression> arguments = null)
    pub fn new(target: Path, arguments: Vec<Expression>) -> Self {
        Self {
            target: Some(target),
            arguments,
            ..Default::default()
        }
    }

    // C# signature: public Divert (Parsed.Object targetContent)
    pub fn new_overload_2(targetContent: Object) -> Self {
        Self {
            targetContent: Some(targetContent),
            ..Default::default()
        }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> ContentItem {
        if self.get_isEnd() {
            return ContentItem::ControlCommand(ControlCommand::End());
        }

        if self.get_isDone() {
            return ContentItem::ControlCommand(ControlCommand::Done());
        }

        let mut runtime_divert = if self.isFunctionCall {
            RuntimeDivert::new_overload_2(PushPopType::Function)
        } else if self.isTunnel {
            RuntimeDivert::new_overload_2(PushPopType::Tunnel)
        } else {
            RuntimeDivert::new()
        };

        if let Some(target_path_string) = &self.resolvedTargetPathString {
            runtime_divert.set_targetPathString(Some(target_path_string.clone()));
        }

        if let Some(variable_name) = &self.resolvedVariableDivertName {
            runtime_divert.set_variableDivertName(Some(variable_name.clone()));
        }

        self.runtimeDivert = Some(runtime_divert);

        let mut container = RuntimeContainer::new();
        let needs_arg_codegen = !self.arguments.is_empty();
        let target_arguments = self.get_target_arguments();

        if needs_arg_codegen || self.isFunctionCall || self.isTunnel || self.isThread {
            if needs_arg_codegen && !self.isFunctionCall {
                container.AddContent(ControlCommand::EvalStart());
            }

            for index in 0..self.arguments.len() {
                let arg = &self.arguments[index];
                if let Some(expected_args) = target_arguments.as_ref() {
                    if let Some(expected_arg) = expected_args.get(index) {
                        if expected_arg.isByReference {
                            match &arg.kind {
                                ExpressionKind::VariableReference(variable_reference) => {
                                    if variable_reference
                                        .get_runtimeVarRef()
                                        .and_then(|runtime_var_ref| {
                                            runtime_var_ref.get_pathForCount().cloned()
                                        })
                                        .is_some()
                                    {
                                        self.Error(
                                            format!(
                                                "can't pass a read count by reference. '{}' is a knot/stitch/label, but '{}' requires the name of a VAR to be passed.",
                                                variable_reference.get_name(),
                                                self.target
                                                    .as_ref()
                                                    .map(|target| target.get_dotSeparatedComponents())
                                                    .unwrap_or_default()
                                            ),
                                            Default::default(),
                                            false,
                                        );
                                        break;
                                    }

                                    container.AddContent(ContentItem::Value(
                                        Value::new_variable_pointer(
                                            Some(variable_reference.get_name()),
                                            -1,
                                        ),
                                    ));
                                    continue;
                                }
                                _ => {
                                    self.Error(
                                        format!(
                                            "Expected variable name to pass by reference to 'ref {}' but saw {}",
                                            expected_arg
                                                .identifier
                                                .as_ref()
                                                .and_then(|identifier| identifier.name.as_deref())
                                                .unwrap_or_default(),
                                            arg.ToString()
                                        ),
                                        Default::default(),
                                        false,
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }

                arg.GenerateIntoContainer(&mut container);
            }

            if needs_arg_codegen && !self.isFunctionCall {
                container.AddContent(ControlCommand::EvalEnd());
            }

            if self.isThread {
                container.AddContent(ControlCommand::StartThread());
            } else if self.isFunctionCall || self.isTunnel {
                if let Some(runtime_divert) = &self.runtimeDivert {
                    container.AddContent(runtime_divert.clone());
                }
            }

            ContentItem::Container(Rc::new(container))
        } else if let Some(runtime_divert) = &self.runtimeDivert {
            ContentItem::Divert(runtime_divert.clone())
        } else {
            ContentItem::Container(Rc::new(RuntimeContainer::new()))
        }
    }

    // C# signature: public string PathAsVariableName()
    pub fn PathAsVariableName(&self) -> String {
        self.target
            .as_ref()
            .and_then(|path| path.get_firstComponent().map(|s| s.to_string()))
            .unwrap_or_default()
    }

    // C# signature: public override void ResolveReferences(Story context)
    pub fn ResolveReferences(&mut self, context: &mut crate::ParsedHierarchy::Story::Story) {
        if self.get_isEmpty() || self.get_isEnd() || self.get_isDone() {
            return;
        }

        if let Some(target_content) = &self.targetContent {
            let runtime_path = target_content.get_runtimePath().ToString();
            self.resolvedTargetPathString = if runtime_path.is_empty() {
                target_content
                    .identifier
                    .as_ref()
                    .and_then(|identifier| identifier.name.clone())
            } else {
                Some(runtime_path)
            };
        }

        for argument in &mut self.arguments {
            argument.ResolveReferences(context);
        }

        let Some(target) = self.target.as_ref() else {
            return;
        };

        let target_was_found = self.targetContent.is_some();
        let mut is_built_in = false;
        let mut is_external = false;
        let target_name = target.get_firstComponent().unwrap_or_default().to_string();
        let target_path_string = target.get_dotSeparatedComponents();

        if target.get_numberOfComponents() == 1 {
            is_built_in =
                crate::ParsedHierarchy::FunctionCall::FunctionCall::IsBuiltIn(target_name.clone());
            is_external = context.IsExternal(target_name.clone());

            if is_built_in || is_external {
                if !self.isFunctionCall {
                    self.Error(
                        format!(
                            "{} must be called as a function: ~ {}()",
                            target_name, target_name
                        ),
                        Default::default(),
                        false,
                    );
                }

                if is_external {
                    if let Some(runtime_divert) = &mut self.runtimeDivert {
                        runtime_divert.set_isExternal(true);
                        runtime_divert.set_externalArgs(self.arguments.len() as i32);
                        runtime_divert.set_pushesToStack(false);
                        runtime_divert.set_targetPathString(Some(target_path_string.clone()));
                    }
                    self.CheckExternalArgumentValidity(context);
                }

                return;
            }
        }

        if self.resolvedVariableDivertName.is_some() {
            return;
        }

        if !target_was_found {
            if let Some(target_content) =
                context.ContentWithNameAtLevel(target_name.clone(), None, true)
            {
                self.targetContent = Some(target_content);
            }
        }

        if self.targetContent.is_some() {
            self.resolvedTargetPathString = Some(target_path_string);
            return;
        }

        if !target_was_found && !is_built_in && !is_external {
            self.Error(
                format!("target not found: '{}'", target.ToString()),
                Default::default(),
                false,
            );
        }
    }

    // C# signature: public override void Error (string message, Object source = null, bool isWarning = false)
    pub fn Error(
        &mut self,
        _message: String,
        _source: crate::ParsedHierarchy::Object::Object,
        _isWarning: bool,
    ) {
    }

    fn CheckExternalArgumentValidity(
        &mut self,
        context: &mut crate::ParsedHierarchy::Story::Story,
    ) {
        if let Some(target) = &self.target {
            let externalName = target.get_firstComponent().unwrap_or_default().to_string();
            if let Some(external) = context.externals.get(&externalName) {
                let expected = external.argumentNames.len();
                let actual = self.arguments.len();
                if actual != expected {
                    self.Error(
                        format!(
                            "incorrect number of arguments sent to external function '{}'. Expected {} but got {}",
                            externalName, expected, actual
                        ),
                        Default::default(),
                        false,
                    );
                }
            }
        }
    }

    fn get_target_arguments(&self) -> Option<Vec<Argument>> {
        self.targetContent
            .as_ref()
            .and_then(|target_content| match &target_content.kind {
                ObjectKind::Knot | ObjectKind::Stitch | ObjectKind::Story => Some(
                    FlowBase::from_object(target_content)
                        .get_arguments()
                        .to_vec(),
                ),
                _ => None,
            })
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        if let Some(target) = &self.target {
            target.ToString()
        } else {
            "-> <empty divert>".to_string()
        }
    }

    // C# signature: Parsed.Path target { get; }
    pub fn get_target(&self) -> Option<&Path> {
        self.target.as_ref()
    }

    // C# signature: Parsed.Object targetContent { get; }
    pub fn get_targetContent(&self) -> Option<&Object> {
        self.targetContent.as_ref()
    }

    // C# signature: List<Expression> arguments { get; }
    pub fn get_arguments(&self) -> &[Expression] {
        &self.arguments
    }

    // C# signature: Runtime.Divert runtimeDivert { get; }
    pub fn get_runtimeDivert(&self) -> Option<&RuntimeDivert> {
        self.runtimeDivert.as_ref()
    }

    // C# signature: bool isFunctionCall { get; }
    pub fn get_isFunctionCall(&self) -> bool {
        self.isFunctionCall
    }

    // C# signature: bool isEmpty { get; }
    pub fn get_isEmpty(&self) -> bool {
        self.isEmpty
    }

    // C# signature: bool isTunnel { get; }
    pub fn get_isTunnel(&self) -> bool {
        self.isTunnel
    }

    // C# signature: bool isThread { get; }
    pub fn get_isThread(&self) -> bool {
        self.isThread
    }

    // C# signature: bool isEnd { get; }
    pub fn get_isEnd(&self) -> bool {
        self.target
            .as_ref()
            .map(|target| target.get_dotSeparatedComponents() == "END")
            .unwrap_or(false)
    }

    // C# signature: bool isDone { get; }
    pub fn get_isDone(&self) -> bool {
        self.target
            .as_ref()
            .map(|target| target.get_dotSeparatedComponents() == "DONE")
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::Divert;
    use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
    use crate::ParsedHierarchy::FlowBase::Argument;
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::Knot::Knot;
    use crate::ParsedHierarchy::Object::Object;
    use crate::ParsedHierarchy::Path::Path;
    use crate::ParsedHierarchy::VariableReference::VariableReference;
    use ink_runtime::Container::ContentItem;
    use ink_runtime::Value::Value;

    #[test]
    fn stringifies_variable_and_stack_diverts() {
        let divert = Divert::new(
            Path::new_overload_2(vec![Identifier {
                name: Some("foo".to_string()),
                debugMetadata: None,
            }]),
            Vec::new(),
        );

        assert_eq!(divert.ToString(), "-> foo");
        assert!(!divert.get_isEnd());
        assert!(!divert.get_isDone());
    }

    #[test]
    fn passes_by_reference_arguments_as_variable_pointers() {
        let target = Object::from_knot(Knot::new(
            Identifier {
                name: Some("callee".to_string()),
                debugMetadata: None,
            },
            vec![],
            vec![Argument {
                identifier: Some(Identifier {
                    name: Some("refArg".to_string()),
                    debugMetadata: None,
                }),
                isByReference: true,
                isDivertTarget: false,
            }],
            true,
        ));

        let mut divert = Divert::new_overload_2(target);
        divert.isFunctionCall = true;
        divert.arguments = vec![Expression::from_kind(ExpressionKind::VariableReference(
            Box::new(VariableReference::new(vec![Identifier {
                name: Some("score".to_string()),
                debugMetadata: None,
            }])),
        ))];

        let runtime = divert.GenerateRuntimeObject();

        match runtime {
            ContentItem::Container(container) => {
                assert!(matches!(
                    container.get_content()[0],
                    ContentItem::Value(Value::VariablePointer(_))
                ));
            }
            other => panic!("unexpected runtime item: {:?}", other),
        }
    }
}
