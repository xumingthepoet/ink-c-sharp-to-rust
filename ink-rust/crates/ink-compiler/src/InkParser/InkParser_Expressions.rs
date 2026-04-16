// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/InkParser/InkParser_Expressions.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct InkParser {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct InfixOperator {
    pub _port_marker: (),
}

impl InkParser {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: protected Parsed.Object TempDeclarationOrAssignment()
    pub fn TempDeclarationOrAssignment(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected void DisallowIncrement (Parsed.Object expr)
    pub fn DisallowIncrement(&mut self, _expr: crate::stub::PortStub) {}

    // C# signature: protected bool ParseTempKeyword()
    pub fn ParseTempKeyword(&mut self) -> bool {
        Default::default()
    }

    // C# signature: protected Parsed.Return ReturnStatement()
    pub fn ReturnStatement(&mut self) -> crate::stub::Return {
        Default::default()
    }

    // C# signature: protected Expression Expression()
    pub fn Expression(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: protected Expression Expression(int minimumPrecedence)
    pub fn Expression_overload_2(&mut self, _minimumPrecedence: i32) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: protected Expression ExpressionUnary()
    pub fn ExpressionUnary(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: protected string ExpressionNot()
    pub fn ExpressionNot(&mut self) -> String {
        Default::default()
    }

    // C# signature: protected Expression ExpressionLiteral()
    pub fn ExpressionLiteral(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: protected Expression ExpressionDivertTarget()
    pub fn ExpressionDivertTarget(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: protected Number ExpressionInt()
    pub fn ExpressionInt(&mut self) -> crate::stub::Number {
        Default::default()
    }

    // C# signature: protected Number ExpressionFloat()
    pub fn ExpressionFloat(&mut self) -> crate::stub::Number {
        Default::default()
    }

    // C# signature: protected StringExpression ExpressionString()
    pub fn ExpressionString(&mut self) -> crate::stub::StringExpression {
        Default::default()
    }

    // C# signature: protected Number ExpressionBool()
    pub fn ExpressionBool(&mut self) -> crate::stub::Number {
        Default::default()
    }

    // C# signature: protected Expression ExpressionFunctionCall()
    pub fn ExpressionFunctionCall(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: protected List<Expression> ExpressionFunctionCallArguments()
    pub fn ExpressionFunctionCallArguments(&mut self) -> Vec<crate::stub::Expression> {
        Default::default()
    }

    // C# signature: protected Expression ExpressionVariableName()
    pub fn ExpressionVariableName(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: protected Expression ExpressionParen()
    pub fn ExpressionParen(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: protected Expression ExpressionInfixRight(Parsed.Expression left, InfixOperator op)
    pub fn ExpressionInfixRight(
        &mut self,
        _left: crate::stub::Expression,
        _op: crate::stub::InfixOperator,
    ) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: private InfixOperator ParseInfixOperator()
    pub fn ParseInfixOperator(&mut self) -> crate::stub::InfixOperator {
        Default::default()
    }

    // C# signature: protected Parsed.List ExpressionList ()
    pub fn ExpressionList(&mut self) -> crate::stub::List {
        Default::default()
    }

    // C# signature: protected Identifier ListMember ()
    pub fn ListMember(&mut self) -> crate::stub::Identifier {
        Default::default()
    }
}
