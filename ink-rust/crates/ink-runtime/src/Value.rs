// Source: ink-c-sharp/ink-engine-runtime/Value.cs

use crate::InkList::InkListItem;
use crate::ListDefinition::ListDefinition;
use crate::Path::Path;
use crate::StoryException::StoryException;
use std::collections::HashMap;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueType {
    Bool = -1,
    Int = 0,
    Float = 1,
    List = 2,
    String = 3,
    DivertTarget = 4,
    VariablePointer = 5,
}

impl Default for ValueType {
    fn default() -> Self {
        Self::Int
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(BoolValue),
    Int(IntValue),
    Float(FloatValue),
    String(StringValue),
    DivertTarget(DivertTargetValue),
    VariablePointer(VariablePointerValue),
    List(ListValue),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueInput {
    Bool(bool),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Path(Path),
    ListValue(ListValue),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BoolValue {
    pub value: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct IntValue {
    pub value: i32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FloatValue {
    pub value: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringValue {
    pub value: String,
    pub isNewline: bool,
    pub isInlineWhitespace: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DivertTargetValue {
    pub value: Option<Path>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariablePointerValue {
    pub value: Option<String>,
    pub contextIndex: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListValue {
    pub value: HashMap<InkListItem, i32>,
    pub originNames: Option<Vec<String>>,
    pub origins: Option<Vec<ListDefinition>>,
}

impl Default for StringValue {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl Default for DivertTargetValue {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Default for VariablePointerValue {
    fn default() -> Self {
        Self::new(None, -1)
    }
}

impl Default for ListValue {
    fn default() -> Self {
        Self::new()
    }
}

impl Value {
    pub fn new_bool(value: bool) -> Self {
        Self::Bool(BoolValue::new(value))
    }

    pub fn new_int(value: i32) -> Self {
        Self::Int(IntValue::new(value))
    }

    pub fn new_float(value: f32) -> Self {
        Self::Float(FloatValue::new(value))
    }

    pub fn new_string(value: String) -> Self {
        Self::String(StringValue::new(value))
    }

    pub fn new_divert_target(value: Option<Path>) -> Self {
        Self::DivertTarget(DivertTargetValue::new(value))
    }

    pub fn new_variable_pointer(value: Option<String>, contextIndex: i32) -> Self {
        Self::VariablePointer(VariablePointerValue::new(value, contextIndex))
    }

    pub fn new_list(value: ListValue) -> Self {
        Self::List(value)
    }

    // C# signature: public static Value Create(object val)
    pub fn Create(val: ValueInput) -> Option<Self> {
        match val {
            ValueInput::Bool(v) => Some(Self::new_bool(v)),
            ValueInput::Int(v) => Some(Self::new_int(v)),
            ValueInput::Long(v) => Some(Self::new_int(v as i32)),
            ValueInput::Float(v) => Some(Self::new_float(v)),
            ValueInput::Double(v) => Some(Self::new_float(v as f32)),
            ValueInput::String(v) => Some(Self::new_string(v)),
            ValueInput::Path(v) => Some(Self::new_divert_target(Some(v))),
            ValueInput::ListValue(v) => Some(Self::new_list(v)),
        }
    }

    // C# signature: public override Object Copy()
    pub fn Copy(&self) -> Self {
        self.clone()
    }

    // C# signature: protected StoryException BadCastException (ValueType targetType)
    pub fn BadCastException(&self, targetType: ValueType) -> StoryException {
        StoryException::new_overload_2(format!(
            "Can't cast {} from {:?} to {:?}",
            self,
            self.value_type(),
            targetType
        ))
    }

    pub fn Cast(&self, newType: ValueType) -> Result<Option<Self>, StoryException> {
        match self {
            Self::Bool(value) => value.Cast(newType),
            Self::Int(value) => value.Cast(newType),
            Self::Float(value) => value.Cast(newType),
            Self::String(value) => value.Cast(newType),
            Self::DivertTarget(value) => value.Cast(newType),
            Self::VariablePointer(value) => value.Cast(newType),
            Self::List(value) => value.Cast(newType),
        }
    }

    pub fn value_type(&self) -> ValueType {
        match self {
            Self::Bool(_) => ValueType::Bool,
            Self::Int(_) => ValueType::Int,
            Self::Float(_) => ValueType::Float,
            Self::String(_) => ValueType::String,
            Self::DivertTarget(_) => ValueType::DivertTarget,
            Self::VariablePointer(_) => ValueType::VariablePointer,
            Self::List(_) => ValueType::List,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Bool(value) => value.get_isTruthy(),
            Self::Int(value) => value.get_isTruthy(),
            Self::Float(value) => value.get_isTruthy(),
            Self::String(value) => value.get_isTruthy(),
            Self::DivertTarget(value) => value.get_isTruthy(),
            Self::VariablePointer(value) => value.get_isTruthy(),
            Self::List(value) => value.get_isTruthy(),
        }
    }

    pub fn value_object(&self) -> ValueInput {
        match self {
            Self::Bool(value) => ValueInput::Bool(value.value),
            Self::Int(value) => ValueInput::Int(value.value),
            Self::Float(value) => ValueInput::Float(value.value),
            Self::String(value) => ValueInput::String(value.value.clone()),
            Self::DivertTarget(value) => ValueInput::Path(value.value.clone().unwrap_or_default()),
            Self::VariablePointer(value) => {
                ValueInput::String(value.value.clone().unwrap_or_default())
            }
            Self::List(value) => ValueInput::ListValue(value.clone()),
        }
    }

    pub fn ToString(&self) -> String {
        self.to_string()
    }
}

impl BoolValue {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn Cast(&self, newType: ValueType) -> Result<Option<Value>, StoryException> {
        let result = match newType {
            ValueType::Bool => Some(Value::Bool(self.clone())),
            ValueType::Int => Some(Value::new_int(if self.value { 1 } else { 0 })),
            ValueType::Float => Some(Value::new_float(if self.value { 1.0 } else { 0.0 })),
            ValueType::String => Some(Value::new_string(if self.value {
                "true".to_string()
            } else {
                "false".to_string()
            })),
            _ => return Err(Value::new_bool(self.value).BadCastException(newType)),
        };

        Ok(result)
    }

    pub fn get_valueType(&self) -> ValueType {
        ValueType::Bool
    }

    pub fn get_isTruthy(&self) -> bool {
        self.value
    }
}

impl IntValue {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn Cast(&self, newType: ValueType) -> Result<Option<Value>, StoryException> {
        let result = match newType {
            ValueType::Bool => Some(Value::new_bool(self.value != 0)),
            ValueType::Int => Some(Value::Int(self.clone())),
            ValueType::Float => Some(Value::new_float(self.value as f32)),
            ValueType::String => Some(Value::new_string(self.value.to_string())),
            _ => return Err(Value::new_int(self.value).BadCastException(newType)),
        };

        Ok(result)
    }

    pub fn get_valueType(&self) -> ValueType {
        ValueType::Int
    }

    pub fn get_isTruthy(&self) -> bool {
        self.value != 0
    }
}

impl FloatValue {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn Cast(&self, newType: ValueType) -> Result<Option<Value>, StoryException> {
        let result = match newType {
            ValueType::Bool => Some(Value::new_bool(self.value != 0.0)),
            ValueType::Int => Some(Value::new_int(self.value as i32)),
            ValueType::Float => Some(Value::Float(self.clone())),
            ValueType::String => Some(Value::new_string(self.value.to_string())),
            _ => return Err(Value::new_float(self.value).BadCastException(newType)),
        };

        Ok(result)
    }

    pub fn get_valueType(&self) -> ValueType {
        ValueType::Float
    }

    pub fn get_isTruthy(&self) -> bool {
        self.value != 0.0
    }
}

impl StringValue {
    pub fn new(value: String) -> Self {
        let isNewline = value == "\n";
        let isInlineWhitespace = value.chars().all(|c| c == ' ' || c == '\t');
        Self {
            value,
            isNewline,
            isInlineWhitespace,
        }
    }

    pub fn Cast(&self, newType: ValueType) -> Result<Option<Value>, StoryException> {
        let result = match newType {
            ValueType::String => Some(Value::String(self.clone())),
            ValueType::Int => match self.value.parse::<i32>() {
                Ok(parsed) => Some(Value::new_int(parsed)),
                Err(_) => None,
            },
            ValueType::Float => match self.value.parse::<f32>() {
                Ok(parsed) => Some(Value::new_float(parsed)),
                Err(_) => None,
            },
            _ => return Err(Value::new_string(self.value.clone()).BadCastException(newType)),
        };

        Ok(result)
    }

    pub fn get_valueType(&self) -> ValueType {
        ValueType::String
    }

    pub fn get_isTruthy(&self) -> bool {
        !self.value.is_empty()
    }

    pub fn get_isNewline(&self) -> bool {
        self.isNewline
    }

    pub fn get_isInlineWhitespace(&self) -> bool {
        self.isInlineWhitespace
    }

    pub fn get_isNonWhitespace(&self) -> bool {
        !self.isNewline && !self.isInlineWhitespace
    }
}

impl DivertTargetValue {
    pub fn new(value: Option<Path>) -> Self {
        Self { value }
    }

    pub fn Cast(&self, newType: ValueType) -> Result<Option<Value>, StoryException> {
        if newType == ValueType::DivertTarget {
            return Ok(Some(Value::DivertTarget(self.clone())));
        }

        Err(Value::DivertTarget(self.clone()).BadCastException(newType))
    }

    pub fn get_valueType(&self) -> ValueType {
        ValueType::DivertTarget
    }

    pub fn get_isTruthy(&self) -> bool {
        panic!("Shouldn't be checking the truthiness of a divert target")
    }

    pub fn get_targetPath(&self) -> Option<&Path> {
        self.value.as_ref()
    }
}

impl VariablePointerValue {
    pub fn new(value: Option<String>, contextIndex: i32) -> Self {
        Self {
            value,
            contextIndex,
        }
    }

    pub fn Cast(&self, newType: ValueType) -> Result<Option<Value>, StoryException> {
        if newType == ValueType::VariablePointer {
            return Ok(Some(Value::VariablePointer(self.clone())));
        }

        Err(Value::VariablePointer(self.clone()).BadCastException(newType))
    }

    pub fn get_valueType(&self) -> ValueType {
        ValueType::VariablePointer
    }

    pub fn get_isTruthy(&self) -> bool {
        panic!("Shouldn't be checking the truthiness of a variable pointer")
    }

    pub fn get_variableName(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub fn get_contextIndex(&self) -> i32 {
        self.contextIndex
    }
}

impl ListValue {
    pub fn new() -> Self {
        Self {
            value: HashMap::new(),
            originNames: None,
            origins: None,
        }
    }

    pub fn new_overload_2(value: HashMap<InkListItem, i32>) -> Self {
        Self {
            value,
            originNames: None,
            origins: None,
        }
    }

    pub fn new_overload_3(singleItem: InkListItem, singleValue: i32) -> Self {
        let mut value = HashMap::new();
        value.insert(singleItem, singleValue);
        Self {
            value,
            originNames: None,
            origins: None,
        }
    }

    fn max_item(&self) -> Option<(&InkListItem, i32)> {
        self.value
            .iter()
            .max_by_key(|(_, value)| **value)
            .map(|(item, value)| (item, *value))
    }

    pub fn Cast(&self, newType: ValueType) -> Result<Option<Value>, StoryException> {
        let result = match newType {
            ValueType::List => Some(Value::List(self.clone())),
            ValueType::Int => Some(Value::new_int(
                self.max_item().map(|(_, value)| value).unwrap_or(0),
            )),
            ValueType::Float => Some(Value::new_float(
                self.max_item()
                    .map(|(_, value)| value as f32)
                    .unwrap_or(0.0),
            )),
            ValueType::String => Some(Value::new_string(
                self.max_item()
                    .map(|(item, _)| item.to_string())
                    .unwrap_or_default(),
            )),
            _ => return Err(Value::List(self.clone()).BadCastException(newType)),
        };

        Ok(result)
    }

    pub fn get_valueType(&self) -> ValueType {
        ValueType::List
    }

    pub fn get_isTruthy(&self) -> bool {
        !self.value.is_empty()
    }

    pub fn set_initial_origin_names(&mut self, initialOriginNames: Option<Vec<String>>) {
        self.originNames = initialOriginNames;
    }

    pub fn get_originNames(&self) -> Option<&[String]> {
        self.originNames.as_deref()
    }

    pub fn get_origins(&self) -> Option<&[ListDefinition]> {
        self.origins.as_deref()
    }

    pub fn get_maxItem(&self) -> Option<(&InkListItem, i32)> {
        self.max_item()
    }

    pub fn RetainListOriginsForAssignment(oldValue: &Value, newValue: &mut Value) {
        if let Value::List(oldList) = oldValue {
            if let Value::List(newList) = newValue {
                if newList.value.is_empty() {
                    newList.originNames = oldList.originNames.clone();
                    newList.origins = oldList.origins.clone();
                }
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(value) => write!(f, "{}", value),
            Value::Int(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
            Value::DivertTarget(value) => write!(f, "{}", value),
            Value::VariablePointer(value) => write!(f, "{}", value),
            Value::List(value) => write!(f, "{}", value),
        }
    }
}

impl std::fmt::Display for BoolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.value { "true" } else { "false" })
    }
}

impl std::fmt::Display for IntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Display for FloatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Display for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

impl std::fmt::Display for DivertTargetValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DivertTargetValue({})",
            self.value
                .as_ref()
                .map_or(String::new(), ToString::to_string)
        )
    }
}

impl std::fmt::Display for VariablePointerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VariablePointerValue({})",
            self.value.as_deref().unwrap_or_default()
        )
    }
}

impl std::fmt::Display for ListValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ordered = self
            .value
            .iter()
            .map(|(item, value)| (item.clone(), *value))
            .collect::<Vec<_>>();
        ordered.sort_by(|left, right| match left.1.cmp(&right.1) {
            std::cmp::Ordering::Equal => left
                .0
                .originName
                .as_deref()
                .unwrap_or("")
                .cmp(right.0.originName.as_deref().unwrap_or("")),
            other => other,
        });

        let rendered = ordered
            .into_iter()
            .map(|(item, _)| item.itemName.unwrap_or_default())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "{rendered}")
    }
}

impl std::fmt::Display for ValueInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueInput::Bool(value) => write!(f, "{}", value),
            ValueInput::Int(value) => write!(f, "{}", value),
            ValueInput::Long(value) => write!(f, "{}", value),
            ValueInput::Float(value) => write!(f, "{}", value),
            ValueInput::Double(value) => write!(f, "{}", value),
            ValueInput::String(value) => write!(f, "{}", value),
            ValueInput::Path(value) => write!(f, "{}", value),
            ValueInput::ListValue(value) => write!(f, "{}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Path::{Component, Path};

    #[test]
    fn string_value_classifies_whitespace() {
        let newline = StringValue::new("\n".to_string());
        let whitespace = StringValue::new(" \t ".to_string());
        let text = StringValue::new("abc".to_string());

        assert!(newline.get_isNewline());
        assert!(whitespace.get_isInlineWhitespace());
        assert!(text.get_isNonWhitespace());
    }

    #[test]
    fn list_value_casts_to_scalar_types() {
        let mut items = HashMap::new();
        items.insert(
            InkListItem::new(Some("food".to_string()), Some("apples".to_string())),
            2,
        );
        let list = ListValue::new_overload_2(items);

        assert_eq!(list.Cast(ValueType::Int).unwrap().unwrap().to_string(), "2");
        assert_eq!(
            list.Cast(ValueType::Float).unwrap().unwrap().to_string(),
            "2"
        );
        assert_eq!(
            list.Cast(ValueType::String).unwrap().unwrap().to_string(),
            "food.apples"
        );
    }

    #[test]
    fn create_wraps_supported_inputs() {
        assert!(matches!(
            Value::Create(ValueInput::Bool(true)),
            Some(Value::Bool(_))
        ));
        assert!(matches!(
            Value::Create(ValueInput::Path(Path::new_overload_3(
                vec![Component::new(1)],
                false
            ))),
            Some(Value::DivertTarget(_))
        ));
    }
}
