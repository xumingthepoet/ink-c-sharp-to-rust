// Source: ink-c-sharp/compiler/ParsedHierarchy/Expression.cs

use crate::ParsedHierarchy::DivertTarget::DivertTarget;
use crate::ParsedHierarchy::FunctionCall::FunctionCall;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::List::List;
use crate::ParsedHierarchy::Number::{Number, NumberValue};
use crate::ParsedHierarchy::StringExpression::StringExpression;
use crate::ParsedHierarchy::Text::Text;
use crate::ParsedHierarchy::VariableReference::VariableReference;
use ink_runtime::Container::{Container as RuntimeContainer, ContentItem};
use ink_runtime::ControlCommand::ControlCommand;
use ink_runtime::NativeFunctionCall::NativeFunctionCall as RuntimeNativeFunctionCall;
use ink_runtime::Value::{IntValue, Value};
use ink_runtime::VariableAssignment::VariableAssignment as RuntimeVariableAssignment;
use ink_runtime::VariableReference::VariableReference as RuntimeVariableReference;
use std::cell::RefCell;

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    pub outputWhenComplete: bool,
    pub kind: ExpressionKind,
    prototypeRuntimeConstantExpression: RefCell<Option<RuntimeContainer>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionKind {
    Empty,
    Number(Number),
    Text(Text),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    IncDec(IncDecExpression),
    MultipleCondition(MultipleConditionExpression),
    FunctionCall(Box<FunctionCall>),
    DivertTarget(Box<DivertTarget>),
    VariableReference(Box<VariableReference>),
    List(List),
    StringExpression(Box<StringExpression>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpression {
    pub leftExpression: Box<Expression>,
    pub rightExpression: Box<Expression>,
    pub opName: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExpression {
    pub innerExpression: Box<Expression>,
    pub op: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IncDecExpression {
    pub varIdentifier: Identifier,
    pub isInc: bool,
    pub expression: Option<Box<Expression>>,
    pub runtimeAssignment: Option<RuntimeVariableAssignment>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MultipleConditionExpression {
    pub subExpressions: Vec<Expression>,
}

impl Expression {
    pub fn new() -> Self {
        Self {
            outputWhenComplete: false,
            kind: ExpressionKind::Empty,
            prototypeRuntimeConstantExpression: RefCell::new(None),
        }
    }

    pub fn from_kind(kind: ExpressionKind) -> Self {
        Self {
            outputWhenComplete: false,
            kind,
            prototypeRuntimeConstantExpression: RefCell::new(None),
        }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&self) -> RuntimeContainer {
        match &self.kind {
            ExpressionKind::Empty => panic!("Empty expression cannot be generated"),
            _ => {
                let mut container = RuntimeContainer::new();
                container.AddContent(ControlCommand::EvalStart());
                self.GenerateIntoContainer(&mut container);
                if self.outputWhenComplete {
                    container.AddContent(ControlCommand::EvalOutput());
                }
                container.AddContent(ControlCommand::EvalEnd());
                container
            }
        }
    }

    // C# signature: public void GenerateConstantIntoContainer(Runtime.Container container)
    pub fn GenerateConstantIntoContainer(&self, container: &mut RuntimeContainer) {
        if self.prototypeRuntimeConstantExpression.borrow().is_none() {
            let mut prototype = RuntimeContainer::new();
            self.GenerateIntoContainer(&mut prototype);
            *self.prototypeRuntimeConstantExpression.borrow_mut() = Some(prototype);
        }

        if let Some(prototype) = self.prototypeRuntimeConstantExpression.borrow().as_ref() {
            for runtime_obj in prototype.get_content().iter().cloned() {
                container.AddContent(runtime_obj);
            }
        }
    }

    // C# signature: public abstract void GenerateIntoContainer (Runtime.Container container)
    pub fn GenerateIntoContainer(&self, container: &mut RuntimeContainer) {
        match &self.kind {
            ExpressionKind::Empty => panic!("Empty expression cannot be generated"),
            ExpressionKind::Number(number) => number.GenerateIntoContainer(container),
            ExpressionKind::Text(text) => {
                container.AddContent(text.GenerateRuntimeObject());
            }
            ExpressionKind::Binary(binary) => binary.GenerateIntoContainer(container),
            ExpressionKind::Unary(unary) => unary.GenerateIntoContainer(container),
            ExpressionKind::IncDec(inc_dec) => inc_dec.GenerateIntoContainer(container),
            ExpressionKind::MultipleCondition(multiple) => {
                multiple.GenerateIntoContainer(container)
            }
            ExpressionKind::FunctionCall(function_call) => {
                function_call.GenerateIntoContainer(container)
            }
            ExpressionKind::DivertTarget(divert_target) => {
                divert_target.GenerateIntoContainer(container)
            }
            ExpressionKind::VariableReference(variable_reference) => {
                variable_reference.GenerateIntoContainer(container)
            }
            ExpressionKind::List(list) => list.GenerateIntoContainer(container),
            ExpressionKind::StringExpression(string_expression) => {
                string_expression.GenerateIntoContainer(container)
            }
        }
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: &mut crate::ParsedHierarchy::Story::Story) {
        match &mut self.kind {
            ExpressionKind::Empty => {}
            ExpressionKind::Number(_) | ExpressionKind::Text(_) => {}
            ExpressionKind::Binary(binary) => binary.ResolveReferences(_context),
            ExpressionKind::Unary(unary) => unary.ResolveReferences(_context),
            ExpressionKind::IncDec(inc_dec) => inc_dec.ResolveReferences(_context),
            ExpressionKind::MultipleCondition(multiple) => multiple.ResolveReferences(_context),
            ExpressionKind::FunctionCall(function_call) => {
                function_call.ResolveReferences(_context)
            }
            ExpressionKind::DivertTarget(divert_target) => {
                divert_target.ResolveReferences(_context)
            }
            ExpressionKind::VariableReference(variable_reference) => {
                variable_reference.ResolveReferences(_context)
            }
            ExpressionKind::List(list) => list.ResolveReferences(_context),
            ExpressionKind::StringExpression(string_expression) => {
                string_expression.ResolveReferences(_context)
            }
        }
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        match &self.kind {
            ExpressionKind::Empty => String::new(),
            ExpressionKind::Number(number) => number.ToString(),
            ExpressionKind::Text(text) => text.ToString(),
            ExpressionKind::Binary(binary) => binary.ToString(),
            ExpressionKind::Unary(unary) => unary.ToString(),
            ExpressionKind::IncDec(inc_dec) => inc_dec.ToString(),
            ExpressionKind::MultipleCondition(multiple) => multiple.ToString(),
            ExpressionKind::FunctionCall(function_call) => function_call.ToString(),
            ExpressionKind::DivertTarget(divert_target) => divert_target.ToString(),
            ExpressionKind::VariableReference(variable_reference) => variable_reference.ToString(),
            ExpressionKind::List(list) => list.ToString(),
            ExpressionKind::StringExpression(string_expression) => string_expression.ToString(),
        }
    }

    // C# signature: public static Expression WithInner(Expression inner, string op)
    pub fn WithInner(inner: Expression, op: String) -> Expression {
        if let ExpressionKind::Number(number) = &inner.kind {
            match op.as_str() {
                "-" => match number.value {
                    NumberValue::Int(value) => {
                        return Expression::from_kind(ExpressionKind::Number(Number::new(
                            NumberValue::Int(-value),
                        )))
                    }
                    NumberValue::Float(value) => {
                        return Expression::from_kind(ExpressionKind::Number(Number::new(
                            NumberValue::Float(-value),
                        )))
                    }
                    _ => {}
                },
                "!" | "not" => match number.value {
                    NumberValue::Int(value) => {
                        return Expression::from_kind(ExpressionKind::Number(Number::new(
                            NumberValue::Bool(value == 0),
                        )))
                    }
                    NumberValue::Float(value) => {
                        return Expression::from_kind(ExpressionKind::Number(Number::new(
                            NumberValue::Bool(value == 0.0),
                        )))
                    }
                    NumberValue::Bool(value) => {
                        return Expression::from_kind(ExpressionKind::Number(Number::new(
                            NumberValue::Bool(!value),
                        )))
                    }
                },
                _ => {}
            }
            panic!("Unexpected operation or number type");
        }

        Expression::from_kind(ExpressionKind::Unary(UnaryExpression::new(inner, op)))
    }

    // C# signature: bool outputWhenComplete { get; }
    pub fn get_outputWhenComplete(&self) -> bool {
        self.outputWhenComplete
    }

    // C# signature: List<Expression> subExpressions { get; }
    pub fn get_subExpressions(&self) -> Vec<Expression> {
        match &self.kind {
            ExpressionKind::Binary(binary) => vec![
                (*binary.leftExpression).clone(),
                (*binary.rightExpression).clone(),
            ],
            ExpressionKind::Unary(unary) => vec![(*unary.innerExpression).clone()],
            ExpressionKind::IncDec(inc_dec) => {
                let mut sub_expressions = Vec::new();
                if let Some(expression) = &inc_dec.expression {
                    sub_expressions.push((**expression).clone());
                }
                sub_expressions
            }
            ExpressionKind::MultipleCondition(multiple) => multiple.subExpressions.clone(),
            ExpressionKind::FunctionCall(function_call) => function_call.get_arguments().to_vec(),
            ExpressionKind::DivertTarget(_) => Vec::new(),
            ExpressionKind::VariableReference(_) => Vec::new(),
            ExpressionKind::List(_) => Vec::new(),
            ExpressionKind::StringExpression(_) => Vec::new(),
            _ => Vec::new(),
        }
    }
}

impl BinaryExpression {
    pub fn new(left: Expression, right: Expression, opName: String) -> Self {
        Self {
            leftExpression: Box::new(left),
            rightExpression: Box::new(right),
            opName,
        }
    }

    pub fn GenerateIntoContainer(&self, container: &mut RuntimeContainer) {
        self.leftExpression.GenerateIntoContainer(container);
        self.rightExpression.GenerateIntoContainer(container);
        container.AddContent(RuntimeNativeFunctionCall::CallWithName(
            self.native_name_for_op(),
        ));
    }

    pub fn ResolveReferences(&mut self, context: &mut crate::ParsedHierarchy::Story::Story) {
        self.leftExpression.ResolveReferences(context);
        self.rightExpression.ResolveReferences(context);
        if self.native_name_for_op() == "?" {
            if let ExpressionKind::Unary(left_unary) = &self.leftExpression.kind {
                if left_unary.op == "not" || left_unary.op == "!" {
                    panic!("Using 'not' or '!' here negates the left expression rather than the result of '?' or 'has'. Add parentheses around (A ? B).");
                }
            }
        }
    }

    fn native_name_for_op(&self) -> String {
        match self.opName.as_str() {
            "and" => "&&".to_string(),
            "or" => "||".to_string(),
            "mod" => "%".to_string(),
            "has" => "?".to_string(),
            "hasnt" => "!?".to_string(),
            other => other.to_string(),
        }
    }

    pub fn ToString(&self) -> String {
        format!(
            "({} {} {})",
            self.leftExpression.ToString(),
            self.opName,
            self.rightExpression.ToString()
        )
    }
}

impl UnaryExpression {
    pub fn new(inner: Expression, op: String) -> Self {
        Self {
            innerExpression: Box::new(inner),
            op,
        }
    }

    pub fn WithInner(inner: Expression, op: String) -> Expression {
        Expression::WithInner(inner, op)
    }

    pub fn GenerateIntoContainer(&self, container: &mut RuntimeContainer) {
        self.innerExpression.GenerateIntoContainer(container);
        container.AddContent(RuntimeNativeFunctionCall::CallWithName(
            self.native_name_for_op(),
        ));
    }

    pub fn ResolveReferences(&mut self, context: &mut crate::ParsedHierarchy::Story::Story) {
        self.innerExpression.ResolveReferences(context);
    }

    pub fn ToString(&self) -> String {
        format!(
            "{}{}",
            self.native_name_for_op(),
            self.innerExpression.ToString()
        )
    }

    fn native_name_for_op(&self) -> String {
        match self.op.as_str() {
            "-" => "_".to_string(),
            "not" => "!".to_string(),
            other => other.to_string(),
        }
    }
}

impl IncDecExpression {
    pub fn new(varIdentifier: Identifier, isInc: bool) -> Self {
        Self {
            varIdentifier,
            isInc,
            expression: None,
            runtimeAssignment: None,
        }
    }

    pub fn new_with_expression(
        varIdentifier: Identifier,
        expression: Expression,
        isInc: bool,
    ) -> Self {
        Self {
            varIdentifier,
            isInc,
            expression: Some(Box::new(expression)),
            runtimeAssignment: None,
        }
    }

    pub fn GenerateIntoContainer(&self, container: &mut RuntimeContainer) {
        container.AddContent(RuntimeVariableReference::new(
            self.varIdentifier.name.clone().unwrap_or_default(),
        ));
        if let Some(expression) = &self.expression {
            expression.GenerateIntoContainer(container);
        } else {
            container.AddContent(Value::new_int(1));
        }
        container.AddContent(RuntimeNativeFunctionCall::CallWithName(if self.isInc {
            "+".to_string()
        } else {
            "-".to_string()
        }));
        let runtime_assignment = RuntimeVariableAssignment::new(
            self.varIdentifier.name.clone().unwrap_or_default(),
            false,
        );
        container.AddContent(runtime_assignment);
    }

    pub fn ResolveReferences(&mut self, context: &mut crate::ParsedHierarchy::Story::Story) {
        if let Some(expression) = &mut self.expression {
            expression.ResolveReferences(context);
        }
    }

    pub fn ToString(&self) -> String {
        if let Some(expression) = &self.expression {
            format!(
                "{} {} {}",
                self.varIdentifier,
                if self.isInc { "+=" } else { "-=" },
                expression.ToString()
            )
        } else {
            format!(
                "{}{}",
                self.varIdentifier,
                if self.isInc { "++" } else { "--" }
            )
        }
    }
}

impl MultipleConditionExpression {
    pub fn new(conditionExpressions: Vec<Expression>) -> Self {
        Self {
            subExpressions: conditionExpressions,
        }
    }

    pub fn GenerateIntoContainer(&self, container: &mut RuntimeContainer) {
        let mut is_first = true;
        for condition_expr in &self.subExpressions {
            condition_expr.GenerateIntoContainer(container);
            if !is_first {
                container.AddContent(RuntimeNativeFunctionCall::CallWithName("&&".to_string()));
            }
            is_first = false;
        }
    }

    pub fn ResolveReferences(&mut self, context: &mut crate::ParsedHierarchy::Story::Story) {
        for expression in &mut self.subExpressions {
            expression.ResolveReferences(context);
        }
    }

    pub fn ToString(&self) -> String {
        self.subExpressions
            .iter()
            .map(|expr| expr.ToString())
            .collect::<Vec<_>>()
            .join(" && ")
    }
}

impl From<Number> for Expression {
    fn from(value: Number) -> Self {
        Expression::from_kind(ExpressionKind::Number(value))
    }
}

impl From<Text> for Expression {
    fn from(value: Text) -> Self {
        Expression::from_kind(ExpressionKind::Text(value))
    }
}

impl From<FunctionCall> for Expression {
    fn from(value: FunctionCall) -> Self {
        Expression::from_kind(ExpressionKind::FunctionCall(Box::new(value)))
    }
}

impl From<DivertTarget> for Expression {
    fn from(value: DivertTarget) -> Self {
        Expression::from_kind(ExpressionKind::DivertTarget(Box::new(value)))
    }
}

impl From<VariableReference> for Expression {
    fn from(value: VariableReference) -> Self {
        Expression::from_kind(ExpressionKind::VariableReference(Box::new(value)))
    }
}

impl From<List> for Expression {
    fn from(value: List) -> Self {
        Expression::from_kind(ExpressionKind::List(value))
    }
}

impl From<StringExpression> for Expression {
    fn from(value: StringExpression) -> Self {
        Expression::from_kind(ExpressionKind::StringExpression(Box::new(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Expression, ExpressionKind, MultipleConditionExpression, Number, NumberValue,
        UnaryExpression,
    };
    use ink_runtime::Container::Container;

    #[test]
    fn generates_constant_number_into_container() {
        let expr = Expression::from_kind(ExpressionKind::Number(Number::new(NumberValue::Int(5))));
        let mut container = Container::new();

        expr.GenerateConstantIntoContainer(&mut container);

        assert_eq!(container.get_content().len(), 1);
    }

    #[test]
    fn flattens_unary_number_inner() {
        let expr = Expression::WithInner(
            Expression::from_kind(ExpressionKind::Number(Number::new(NumberValue::Int(5)))),
            "-".to_string(),
        );

        assert_eq!(expr.ToString(), "-5");
    }

    #[test]
    fn joins_multiple_conditions_in_order() {
        let expr = MultipleConditionExpression::new(vec![
            Expression::from_kind(ExpressionKind::Number(Number::new(NumberValue::Int(1)))),
            Expression::from_kind(ExpressionKind::Number(Number::new(NumberValue::Int(2)))),
        ]);

        assert_eq!(expr.ToString(), "1 && 2");
    }
}
