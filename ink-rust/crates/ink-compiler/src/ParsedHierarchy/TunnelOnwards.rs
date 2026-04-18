// Source: ink-c-sharp/compiler/ParsedHierarchy/TunnelOnwards.cs

use crate::ParsedHierarchy::Divert::Divert;
use crate::ParsedHierarchy::Story::Story;
use ink_runtime::Container::{Container, ContentItem};
use ink_runtime::ControlCommand::{CommandType, ControlCommand};
use ink_runtime::Value::{DivertTargetValue, Value};
use ink_runtime::VariableReference::VariableReference as RuntimeVariableReference;
use ink_runtime::Void::Void as RuntimeVoid;
use std::rc::Rc;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TunnelOnwards {
    divertAfter: Option<Divert>,
    overrideDivertTarget: Option<DivertTargetValue>,
}

impl TunnelOnwards {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_divertAfter(&mut self, divertAfter: Option<Divert>) {
        self.divertAfter = divertAfter;
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> ContentItem {
        let mut container = Container::new();
        container.AddContent(ControlCommand::EvalStart());

        if let Some(divertAfter) = &self.divertAfter {
            let mut return_divert = divertAfter.clone();
            let return_runtime_obj = return_divert.GenerateRuntimeObject();

            if let ContentItem::Container(return_runtime_container) = return_runtime_obj {
                if let Some(end_idx) = find_eval_end(&return_runtime_container) {
                    if let Some(start_idx) = find_eval_start(&return_runtime_container) {
                        for item in return_runtime_container.get_content()[start_idx + 1..end_idx]
                            .iter()
                            .cloned()
                        {
                            container.AddContent(item);
                        }
                    }
                }

                if let Some(runtime_divert) = return_divert.get_runtimeDivert() {
                    if let Some(name) = runtime_divert.get_variableDivertName() {
                        container.AddContent(RuntimeVariableReference::new(name.to_string()));
                    } else {
                        self.overrideDivertTarget
                            .get_or_insert_with(DivertTargetValue::default);
                        container.AddContent(ContentItem::Value(Value::DivertTarget(
                            self.overrideDivertTarget
                                .clone()
                                .unwrap_or_else(DivertTargetValue::default),
                        )));
                    }
                }
            } else if let ContentItem::Divert(runtime_divert) = return_runtime_obj {
                if let Some(name) = runtime_divert.get_variableDivertName() {
                    container.AddContent(RuntimeVariableReference::new(name.to_string()));
                } else {
                    self.overrideDivertTarget
                        .get_or_insert_with(DivertTargetValue::default);
                    container.AddContent(ContentItem::Value(Value::DivertTarget(
                        self.overrideDivertTarget
                            .clone()
                            .unwrap_or_else(DivertTargetValue::default),
                    )));
                }
            }
        } else {
            container.AddContent(ContentItem::Void(RuntimeVoid::new()));
        }

        container.AddContent(ControlCommand::EvalEnd());
        container.AddContent(ControlCommand::PopTunnel());

        ContentItem::Container(Rc::new(container))
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        if let Some(divertAfter) = &mut self.divertAfter {
            divertAfter.ResolveReferences(context);

            if let Some(targetContent) = divertAfter.get_targetContent() {
                self.overrideDivertTarget
                    .get_or_insert_with(DivertTargetValue::default)
                    .value = Some(targetContent.get_runtimePath());
            }
        }
    }

    // C# signature: Divert divertAfter { get; }
    pub fn get_divertAfter(&self) -> Option<&Divert> {
        self.divertAfter.as_ref()
    }
}

fn find_eval_start(container: &Container) -> Option<usize> {
    container.get_content().iter().position(|item| {
        matches!(
            item,
            ContentItem::ControlCommand(command)
                if command.get_commandType() == CommandType::EvalStart
        )
    })
}

fn find_eval_end(container: &Container) -> Option<usize> {
    container.get_content().iter().rposition(|item| {
        matches!(
            item,
            ContentItem::ControlCommand(command)
                if command.get_commandType() == CommandType::EvalEnd
        )
    })
}

#[cfg(test)]
mod tests {
    use super::TunnelOnwards;
    use crate::ParsedHierarchy::Divert::Divert;
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::Object::{Object, ObjectKind};
    use crate::ParsedHierarchy::Path::Path;
    use crate::ParsedHierarchy::Story::Story;
    use ink_runtime::Container::Container;
    use ink_runtime::Container::ContentItem;
    use ink_runtime::Path::Path as RuntimePath;
    use ink_runtime::Value::Value as RuntimeValue;

    #[test]
    fn generates_tunnel_shell_when_divert_exists() {
        let mut tunnel = TunnelOnwards::new();
        tunnel.set_divertAfter(Some(Divert::new(
            Path::new_overload_2(vec![Identifier {
                name: Some("knot".to_string()),
                debugMetadata: None,
            }]),
            Vec::new(),
        )));

        let generated = tunnel.GenerateRuntimeObject();
        assert!(matches!(generated, ContentItem::Container(_)));
    }

    #[test]
    fn resolve_references_updates_override_target_path() {
        let mut tunnel = TunnelOnwards::new();
        let mut target_container = Container::new();
        target_container.set_path(RuntimePath::new_overload_4("knot".to_string()));

        let mut target_object = Object::with_kind(ObjectKind::Plain);
        target_object.set_runtimeObject(Some(target_container));

        tunnel.set_divertAfter(Some(Divert::new_overload_2(target_object.clone())));
        let mut generated = tunnel.GenerateRuntimeObject();
        let mut story = Story::new(Vec::new(), false);
        tunnel.ResolveReferences(&mut story);

        if let ContentItem::Container(container) = &mut generated {
            assert!(container
                .get_content()
                .iter()
                .any(|item| matches!(item, ContentItem::Value(RuntimeValue::DivertTarget(_)))));
        }

        assert_eq!(
            tunnel
                .overrideDivertTarget
                .as_ref()
                .and_then(|value| value.get_targetPath())
                .map(|path| path.ToString()),
            Some("knot".to_string())
        );
    }
}
