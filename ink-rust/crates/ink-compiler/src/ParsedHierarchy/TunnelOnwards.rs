// Source: ink-c-sharp/compiler/ParsedHierarchy/TunnelOnwards.cs

use crate::ParsedHierarchy::Divert::Divert;
use ink_runtime::Container::{Container, ContentItem};
use ink_runtime::ControlCommand::{CommandType, ControlCommand};
use ink_runtime::Value::{DivertTargetValue, Value};
use ink_runtime::VariableReference::VariableReference as RuntimeVariableReference;
use ink_runtime::Void::Void as RuntimeVoid;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TunnelOnwards {
    divertAfter: Option<Divert>,
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
                        container.AddContent(ContentItem::Value(Value::DivertTarget(
                            DivertTargetValue::new(None),
                        )));
                    }
                }
            } else if let ContentItem::Divert(runtime_divert) = return_runtime_obj {
                if let Some(name) = runtime_divert.get_variableDivertName() {
                    container.AddContent(RuntimeVariableReference::new(name.to_string()));
                } else {
                    container.AddContent(ContentItem::Value(Value::DivertTarget(
                        DivertTargetValue::new(None),
                    )));
                }
            }
        } else {
            container.AddContent(ContentItem::Void(RuntimeVoid::new()));
        }

        container.AddContent(ControlCommand::EvalEnd());
        container.AddContent(ControlCommand::PopTunnel());

        ContentItem::Container(Box::new(container))
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {
        // Full target resolution depends on the Parsed.Object ancestry chain,
        // which is not ported yet. Keep this as a visible partial.
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
    use crate::ParsedHierarchy::Path::Path;
    use ink_runtime::Container::ContentItem;

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
}
