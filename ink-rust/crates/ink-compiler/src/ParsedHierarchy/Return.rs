// Source: ink-c-sharp/compiler/ParsedHierarchy/Return.cs

use crate::ParsedHierarchy::Expression::Expression;
use ink_runtime::Container::Container;
use ink_runtime::ControlCommand::ControlCommand;
use ink_runtime::Void::Void;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Return {
    pub returnedExpression: Option<Expression>,
}

impl Return {
    // C# signature: public Return (Expression returnedExpression = null)
    pub fn new(returnedExpression: Option<Expression>) -> Self {
        Self { returnedExpression }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&self) -> Container {
        let mut container = Container::new();

        if let Some(returnedExpression) = &self.returnedExpression {
            container.AddContent(returnedExpression.GenerateRuntimeObject());
        } else {
            container.AddContent(ControlCommand::EvalStart());
            container.AddContent(Void::new());
            container.AddContent(ControlCommand::EvalEnd());
        }

        container.AddContent(ControlCommand::PopFunction());
        container
    }

    // C# signature: Expression returnedExpression { get; }
    pub fn get_returnedExpression(&self) -> Option<&Expression> {
        self.returnedExpression.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::Return;
    use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
    use crate::ParsedHierarchy::Number::{Number, NumberValue};
    use ink_runtime::Container::ContentItem;
    use ink_runtime::ControlCommand::CommandType;

    #[test]
    fn generates_eval_wrapping_for_void_return() {
        let runtime = Return::new(None).GenerateRuntimeObject();
        let commands = runtime.get_content();

        assert!(matches!(commands[0], ContentItem::ControlCommand(_)));
        assert_eq!(
            match &commands[commands.len() - 1] {
                ContentItem::ControlCommand(cmd) => cmd.get_commandType(),
                _ => panic!("expected command"),
            },
            CommandType::PopFunction
        );
    }

    #[test]
    fn embeds_returned_expression_runtime() {
        let expr = Expression::from_kind(ExpressionKind::Number(Number::new(NumberValue::Int(7))));
        let runtime = Return::new(Some(expr)).GenerateRuntimeObject();

        assert!(!runtime.get_content().is_empty());
    }
}
