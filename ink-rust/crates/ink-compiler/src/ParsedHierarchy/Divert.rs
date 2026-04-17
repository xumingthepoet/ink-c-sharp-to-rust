// Source: ink-c-sharp/compiler/ParsedHierarchy/Divert.cs

use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::Path::Path;
use ink_runtime::Container::{Container as RuntimeContainer, ContentItem};
use ink_runtime::ControlCommand::ControlCommand;
use ink_runtime::Divert::Divert as RuntimeDivert;
use ink_runtime::PushPop::PushPopType;
use ink_runtime::Value::VariablePointerValue;

#[derive(Clone, Debug, Default)]
pub struct Divert {
    pub target: Option<Path>,
    pub targetContent: Option<crate::stub::PortStub>,
    pub arguments: Vec<Expression>,
    pub runtimeDivert: Option<RuntimeDivert>,
    pub isFunctionCall: bool,
    pub isEmpty: bool,
    pub isTunnel: bool,
    pub isThread: bool,
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
    pub fn new_overload_2(targetContent: crate::stub::PortStub) -> Self {
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

        self.runtimeDivert = Some(if self.isFunctionCall {
            RuntimeDivert::new_overload_2(PushPopType::Function)
        } else if self.isTunnel {
            RuntimeDivert::new_overload_2(PushPopType::Tunnel)
        } else {
            RuntimeDivert::new()
        });

        let mut container = RuntimeContainer::new();
        let needs_arg_codegen = !self.arguments.is_empty();

        if needs_arg_codegen || self.isFunctionCall || self.isTunnel || self.isThread {
            if needs_arg_codegen && !self.isFunctionCall {
                container.AddContent(ControlCommand::EvalStart());
            }

            for arg in &mut self.arguments {
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

            ContentItem::Container(Box::new(container))
        } else if let Some(runtime_divert) = &self.runtimeDivert {
            ContentItem::Divert(runtime_divert.clone())
        } else {
            ContentItem::Container(Box::new(RuntimeContainer::new()))
        }
    }

    // C# signature: public string PathAsVariableName()
    pub fn PathAsVariableName(&mut self) -> String {
        self.target
            .as_ref()
            .and_then(|path| path.get_firstComponent().map(|s| s.to_string()))
            .unwrap_or_default()
    }

    // C# signature: public override void ResolveReferences(Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {
        if self.get_isEmpty() || self.get_isEnd() || self.get_isDone() {
            return;
        }

        // Full ancestry/path resolution is still waiting on the parser object tree.
    }

    // C# signature: public override void Error (string message, Object source = null, bool isWarning = false)
    pub fn Error(&mut self, _message: String, _source: crate::stub::PortStub, _isWarning: bool) {}

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        if let Some(target) = &self.target {
            target.ToString()
        } else {
            "-> <empty divert>".to_string()
        }
    }

    // C# signature: Parsed.Path target { get; }
    pub fn get_target(&mut self) -> Option<&Path> {
        self.target.as_ref()
    }

    // C# signature: Parsed.Object targetContent { get; }
    pub fn get_targetContent(&mut self) -> Option<&crate::stub::PortStub> {
        self.targetContent.as_ref()
    }

    // C# signature: List<Expression> arguments { get; }
    pub fn get_arguments(&mut self) -> &[Expression] {
        &self.arguments
    }

    // C# signature: Runtime.Divert runtimeDivert { get; }
    pub fn get_runtimeDivert(&mut self) -> Option<&RuntimeDivert> {
        self.runtimeDivert.as_ref()
    }

    // C# signature: bool isFunctionCall { get; }
    pub fn get_isFunctionCall(&mut self) -> bool {
        self.isFunctionCall
    }

    // C# signature: bool isEmpty { get; }
    pub fn get_isEmpty(&mut self) -> bool {
        self.isEmpty
    }

    // C# signature: bool isTunnel { get; }
    pub fn get_isTunnel(&mut self) -> bool {
        self.isTunnel
    }

    // C# signature: bool isThread { get; }
    pub fn get_isThread(&mut self) -> bool {
        self.isThread
    }

    // C# signature: bool isEnd { get; }
    pub fn get_isEnd(&mut self) -> bool {
        self.target
            .as_ref()
            .map(|target| target.get_dotSeparatedComponents() == "END")
            .unwrap_or(false)
    }

    // C# signature: bool isDone { get; }
    pub fn get_isDone(&mut self) -> bool {
        self.target
            .as_ref()
            .map(|target| target.get_dotSeparatedComponents() == "DONE")
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::Divert;
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::Path::Path;

    #[test]
    fn stringifies_variable_and_stack_diverts() {
        let mut divert = Divert::new(
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
}
