// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Value.cs

use crate::stub::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueType {
    PortPlaceholder,
}

impl Default for ValueType {
    fn default() -> Self {
        Self::PortPlaceholder
    }
}

#[derive(Clone, Debug, Default)]
pub struct Value {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct BoolValue {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct IntValue {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct FloatValue {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct StringValue {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct DivertTargetValue {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct VariablePointerValue {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct ListValue {
    pub _port_marker: (),
}

impl Value {
    // C# signature: public Value (T val)
    pub fn new(_val: crate::stub::PortStub) -> Self {
        Default::default()
    }

    // C# signature: public abstract Value Cast(ValueType newType)
    pub fn Cast(&mut self, _newType: crate::stub::ValueType) -> crate::stub::Value {
        Default::default()
    }

    // C# signature: public static Value Create(object val)
    pub fn Create(_val: crate::stub::PortStub) -> crate::stub::Value {
        Default::default()
    }

    // C# signature: public override Object Copy()
    pub fn Copy(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected StoryException BadCastException (ValueType targetType)
    pub fn BadCastException(
        &mut self,
        _targetType: crate::stub::ValueType,
    ) -> crate::stub::StoryException {
        Default::default()
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: public static void RetainListOriginsForAssignment (Runtime.Object oldValue, Runtime.Object newValue)
    pub fn RetainListOriginsForAssignment(
        _oldValue: crate::stub::PortStub,
        _newValue: crate::stub::PortStub,
    ) {
    }

    // C# signature: ValueType valueType { get; }
    pub fn get_valueType(&mut self) -> crate::stub::ValueType {
        Default::default()
    }

    // C# signature: bool isTruthy { get; }
    pub fn get_isTruthy(&mut self) -> bool {
        Default::default()
    }

    // C# signature: object valueObject { get; }
    pub fn get_valueObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: T value { get; }
    pub fn get_value(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: object valueObject { get; }
    pub fn get_valueObject_overload_2(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: ValueType valueType { get; }
    pub fn get_valueType_overload_2(&mut self) -> crate::stub::ValueType {
        Default::default()
    }

    // C# signature: bool isTruthy { get; }
    pub fn get_isTruthy_overload_2(&mut self) -> bool {
        Default::default()
    }

    // C# signature: ValueType valueType { get; }
    pub fn get_valueType_overload_3(&mut self) -> crate::stub::ValueType {
        Default::default()
    }

    // C# signature: bool isTruthy { get; }
    pub fn get_isTruthy_overload_3(&mut self) -> bool {
        Default::default()
    }

    // C# signature: ValueType valueType { get; }
    pub fn get_valueType_overload_4(&mut self) -> crate::stub::ValueType {
        Default::default()
    }

    // C# signature: bool isTruthy { get; }
    pub fn get_isTruthy_overload_4(&mut self) -> bool {
        Default::default()
    }

    // C# signature: ValueType valueType { get; }
    pub fn get_valueType_overload_5(&mut self) -> crate::stub::ValueType {
        Default::default()
    }

    // C# signature: bool isTruthy { get; }
    pub fn get_isTruthy_overload_5(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isNewline { get; }
    pub fn get_isNewline(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isInlineWhitespace { get; }
    pub fn get_isInlineWhitespace(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isNonWhitespace { get; }
    pub fn get_isNonWhitespace(&mut self) -> bool {
        Default::default()
    }

    // C# signature: Path targetPath { get; }
    pub fn get_targetPath(&mut self) -> crate::stub::Path {
        Default::default()
    }

    // C# signature: ValueType valueType { get; }
    pub fn get_valueType_overload_6(&mut self) -> crate::stub::ValueType {
        Default::default()
    }

    // C# signature: bool isTruthy { get; }
    pub fn get_isTruthy_overload_6(&mut self) -> bool {
        Default::default()
    }

    // C# signature: string variableName { get; }
    pub fn get_variableName(&mut self) -> String {
        Default::default()
    }

    // C# signature: ValueType valueType { get; }
    pub fn get_valueType_overload_7(&mut self) -> crate::stub::ValueType {
        Default::default()
    }

    // C# signature: bool isTruthy { get; }
    pub fn get_isTruthy_overload_7(&mut self) -> bool {
        Default::default()
    }

    // C# signature: int contextIndex { get; }
    pub fn get_contextIndex(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: ValueType valueType { get; }
    pub fn get_valueType_overload_8(&mut self) -> crate::stub::ValueType {
        Default::default()
    }

    // C# signature: bool isTruthy { get; }
    pub fn get_isTruthy_overload_8(&mut self) -> bool {
        Default::default()
    }
}
