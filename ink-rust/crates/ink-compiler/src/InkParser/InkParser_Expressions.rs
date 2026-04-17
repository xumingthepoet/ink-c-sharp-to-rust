// Source: ink-c-sharp/compiler/InkParser/InkParser_Expressions.cs

use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::Expression::{
    BinaryExpression, Expression, ExpressionKind, IncDecExpression, MultipleConditionExpression,
    UnaryExpression,
};
use crate::ParsedHierarchy::FunctionCall::FunctionCall;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::List::List;
use crate::ParsedHierarchy::Number::{Number, NumberValue};
use crate::ParsedHierarchy::StringExpression::StringExpression;
use crate::ParsedHierarchy::VariableReference::VariableReference;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InfixOperator {
    pub type_: String,
    pub precedence: i32,
    pub requireWhitespace: bool,
}

impl InfixOperator {
    pub fn new(type_: String, precedence: i32, requireWhitespace: bool) -> Self {
        Self {
            type_,
            precedence,
            requireWhitespace,
        }
    }
}

impl InkParser {
    pub fn TempDeclarationOrAssignment(
        &mut self,
    ) -> Option<crate::ParsedHierarchy::Object::Object> {
        todo!("temp declarations still depend on ParsedHierarchy.Object and VariableAssignment");
    }

    pub fn DisallowIncrement(&mut self, _expr: &crate::ParsedHierarchy::Object::Object) {
        todo!("increment/decrement line handling is still pending the statement parser");
    }

    pub fn ParseTempKeyword(&mut self) -> bool {
        matches!(self.Peek(|parser| parser.Identifier()), Some(ref identifier) if identifier == "temp")
            && self.ParseObject(|parser| parser.Identifier()).as_deref() == Some("temp")
    }

    pub fn ReturnStatement(&mut self) -> Option<crate::ParsedHierarchy::Return::Return> {
        self.Whitespace();
        let return_or_done = self.ParseObject(|parser| parser.Identifier())?;
        if return_or_done != "return" {
            return None;
        }

        self.Whitespace();
        let expr = self.ParseObject(|parser| parser.Expression());
        Some(crate::ParsedHierarchy::Return::Return::new(expr))
    }

    pub fn Expression(&mut self) -> Option<Expression> {
        self.Expression_overload_2(0)
    }

    pub fn Expression_overload_2(&mut self, minimumPrecedence: i32) -> Option<Expression> {
        self.Whitespace();

        let mut expr = self.ExpressionUnary()?;
        self.Whitespace();

        loop {
            let Some(infix_op) = self.ParseInfixOperator() else {
                break;
            };

            if infix_op.precedence <= minimumPrecedence {
                break;
            }

            let expectationMessage = format!("right side of '{}' expression", infix_op.type_);
            let Some(multiary_expr) = self.ExpressionInfixRight(expr.clone(), infix_op.clone())
            else {
                self.Error(expectationMessage);
                return None;
            };

            expr = multiary_expr;
        }

        self.Whitespace();
        Some(expr)
    }

    pub fn ExpressionUnary(&mut self) -> Option<Expression> {
        if let Some(divertTarget) = self.ExpressionDivertTarget() {
            return Some(divertTarget);
        }

        let prefixOp = self
            .OneOf(vec![
                Box::new(|parser: &mut InkParser| parser.ParseString("-".to_string())),
                Box::new(|parser: &mut InkParser| parser.ParseString("!".to_string())),
            ])
            .or_else(|| self.ExpressionNot());

        self.Whitespace();

        let mut expr = self
            .OneOf(vec![
                Box::new(|parser: &mut InkParser| parser.ExpressionList()),
                Box::new(|parser: &mut InkParser| parser.ExpressionParen()),
                Box::new(|parser: &mut InkParser| parser.ExpressionFunctionCall()),
                Box::new(|parser: &mut InkParser| parser.ExpressionVariableName()),
                Box::new(|parser: &mut InkParser| parser.ExpressionLiteral()),
            ])
            .or_else(|| {
                if prefixOp.is_some() {
                    self.ExpressionUnary()
                } else {
                    None
                }
            })?;

        if let Some(prefixOp) = prefixOp {
            expr = UnaryExpression::WithInner(expr, prefixOp);
        }

        self.Whitespace();

        if let Some(postfixOp) = self.OneOf(vec![
            Box::new(|parser: &mut InkParser| parser.ParseString("++".to_string())),
            Box::new(|parser: &mut InkParser| parser.ParseString("--".to_string())),
        ]) {
            let isInc = postfixOp == "++";

            if let ExpressionKind::VariableReference(variable_reference) = &expr.kind {
                let var_ref = variable_reference.as_ref();
                let identifier = var_ref.get_identifier().unwrap_or_default();
                expr = Expression::from_kind(ExpressionKind::IncDec(IncDecExpression::new(
                    identifier, isInc,
                )));
            } else {
                self.Error(format!(
                    "can only increment and decrement variables, but saw '{}'",
                    expr.ToString()
                ));
            }
        }

        Some(expr)
    }

    pub fn ExpressionNot(&mut self) -> Option<String> {
        let id = self.Identifier()?;
        if id == "not" {
            Some(id)
        } else {
            None
        }
    }

    pub fn ExpressionLiteral(&mut self) -> Option<Expression> {
        self.OneOf(vec![
            Box::new(|parser: &mut InkParser| parser.ExpressionFloat()),
            Box::new(|parser: &mut InkParser| parser.ExpressionInt()),
            Box::new(|parser: &mut InkParser| parser.ExpressionBool()),
            Box::new(|parser: &mut InkParser| parser.ExpressionString()),
        ])
    }

    pub fn ExpressionDivertTarget(&mut self) -> Option<Expression> {
        todo!("divert target expressions still depend on the unported divert parser");
    }

    pub fn ExpressionInt(&mut self) -> Option<Expression> {
        self.ParseInt().map(|value| {
            Expression::from_kind(ExpressionKind::Number(Number::new(NumberValue::Int(value))))
        })
    }

    pub fn ExpressionFloat(&mut self) -> Option<Expression> {
        self.ParseFloat().map(|value| {
            Expression::from_kind(ExpressionKind::Number(Number::new(NumberValue::Float(
                value,
            ))))
        })
    }

    pub fn ExpressionString(&mut self) -> Option<Expression> {
        todo!("string expression parsing still depends on MixedTextAndLogic");
    }

    pub fn ExpressionBool(&mut self) -> Option<Expression> {
        let id = self.ParseObject(|parser| parser.Identifier())?;
        match id.as_str() {
            "true" => Some(Expression::from_kind(ExpressionKind::Number(Number::new(
                NumberValue::Bool(true),
            )))),
            "false" => Some(Expression::from_kind(ExpressionKind::Number(Number::new(
                NumberValue::Bool(false),
            )))),
            _ => None,
        }
    }

    pub fn ExpressionFunctionCall(&mut self) -> Option<Expression> {
        let iden = self.IdentifierWithMetadata()?;
        self.Whitespace();
        let arguments = self.ExpressionFunctionCallArguments()?;
        Some(Expression::from(FunctionCall::new(iden, arguments)))
    }

    pub fn ExpressionFunctionCallArguments(&mut self) -> Option<Vec<Expression>> {
        if self.ParseString("(".to_string()).is_none() {
            return None;
        }

        self.Whitespace();
        let mut arguments = Vec::new();
        if self.ParseString(")".to_string()).is_none() {
            loop {
                let argument = self.ParseObject(|parser| parser.Expression())?;
                arguments.push(argument);
                self.Whitespace();
                if self.ParseString(",".to_string()).is_none() {
                    break;
                }
                self.Whitespace();
            }

            self.Whitespace();
            if self.ParseString(")".to_string()).is_none() {
                self.Error("closing ')' for function call".to_string());
                return None;
            }
        }

        Some(arguments)
    }

    pub fn ExpressionVariableName(&mut self) -> Option<Expression> {
        let mut path = Vec::new();
        let first = self.IdentifierWithMetadata()?;
        path.push(first);

        loop {
            self.Whitespace();
            if self.ParseString(".".to_string()).is_none() {
                break;
            }
            self.Whitespace();
            let next = self.IdentifierWithMetadata()?;
            path.push(next);
        }

        if path.is_empty()
            || crate::ParsedHierarchy::Story::Story::IsReservedKeyword(
                path[0].name.clone().unwrap_or_default(),
            )
        {
            return None;
        }

        Some(Expression::from(VariableReference::new(path)))
    }

    pub fn ExpressionParen(&mut self) -> Option<Expression> {
        if self.ParseString("(".to_string()).is_none() {
            return None;
        }

        let inner_expr = self.ParseObject(|parser| parser.Expression())?;
        self.Whitespace();
        if self.ParseString(")".to_string()).is_none() {
            self.Error("closing parenthesis ')' for expression".to_string());
            return None;
        }
        Some(inner_expr)
    }

    pub fn ExpressionInfixRight(
        &mut self,
        left: Expression,
        op: InfixOperator,
    ) -> Option<Expression> {
        self.Whitespace();
        let right = self.Expression_overload_2(op.precedence)?;
        Some(Expression::from_kind(ExpressionKind::Binary(
            BinaryExpression::new(left, right, op.type_),
        )))
    }

    pub fn ParseInfixOperator(&mut self) -> Option<InfixOperator> {
        for op in Self::binary_operators().into_iter() {
            let parsed = self.ParseObject(|parser| {
                parser.ParseString(op.type_.clone())?;
                if op.requireWhitespace && parser.Whitespace().is_none() {
                    return None;
                }
                Some(op.clone())
            });
            if parsed.is_some() {
                return parsed;
            }
        }
        None
    }

    pub fn ExpressionList(&mut self) -> Option<Expression> {
        self.Whitespace();
        if self.ParseString("(".to_string()).is_none() {
            return None;
        }
        self.Whitespace();

        let mut memberNames = Vec::new();
        if self.ParseString(")".to_string()).is_none() {
            loop {
                let member = self.ListMember()?;
                memberNames.push(member);
                self.Whitespace();
                if self.ParseString(",".to_string()).is_none() {
                    break;
                }
                self.Whitespace();
            }

            self.Whitespace();
            if self.ParseString(")".to_string()).is_none() {
                self.Error("closing ')' for list expression".to_string());
                return None;
            }
        }

        Some(Expression::from(List::new(memberNames)))
    }

    pub fn ListMember(&mut self) -> Option<Identifier> {
        self.Whitespace();

        let mut identifier = self.IdentifierWithMetadata()?;
        if self.ParseString(".".to_string()).is_some() {
            let Some(identifier2) = self.IdentifierWithMetadata() else {
                self.Error(format!(
                    "element name within the set {}",
                    identifier.name.clone().unwrap_or_default()
                ));
                return None;
            };
            identifier.name = Some(format!(
                "{}.{}",
                identifier.name.unwrap_or_default(),
                identifier2.name.unwrap_or_default()
            ));
        }

        self.Whitespace();
        Some(identifier)
    }

    fn binary_operators() -> Vec<InfixOperator> {
        vec![
            InfixOperator::new("&&".to_string(), 1, false),
            InfixOperator::new("||".to_string(), 1, false),
            InfixOperator::new("and".to_string(), 1, true),
            InfixOperator::new("or".to_string(), 1, true),
            InfixOperator::new("==".to_string(), 2, false),
            InfixOperator::new(">=".to_string(), 2, false),
            InfixOperator::new("<=".to_string(), 2, false),
            InfixOperator::new("<".to_string(), 2, false),
            InfixOperator::new(">".to_string(), 2, false),
            InfixOperator::new("!=".to_string(), 2, false),
            InfixOperator::new("?".to_string(), 3, false),
            InfixOperator::new("has".to_string(), 3, true),
            InfixOperator::new("!?".to_string(), 3, false),
            InfixOperator::new("hasnt".to_string(), 3, true),
            InfixOperator::new("^".to_string(), 3, false),
            InfixOperator::new("+".to_string(), 4, false),
            InfixOperator::new("-".to_string(), 5, false),
            InfixOperator::new("*".to_string(), 6, false),
            InfixOperator::new("/".to_string(), 7, false),
            InfixOperator::new("%".to_string(), 8, false),
            InfixOperator::new("mod".to_string(), 8, true),
        ]
    }
}
