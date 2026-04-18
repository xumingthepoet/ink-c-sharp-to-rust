// Source: ink-c-sharp/ink-engine-runtime/NativeFunctionCall.cs

use crate::InkList::{InkList, InkListItem};
use crate::Path::Path;
use crate::StoryException::StoryException;
use crate::Value::{BoolValue, FloatValue, IntValue, ListValue, StringValue, Value, ValueType};
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NativeFunctionCall {
    name: String,
    numberOfParameters: i32,
}

fn native_functions() -> &'static HashMap<&'static str, i32> {
    static FUNCTIONS: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
    FUNCTIONS.get_or_init(|| {
        let mut functions = HashMap::new();
        for (name, count) in [
            ("+", 2),
            ("-", 2),
            ("/", 2),
            ("*", 2),
            ("%", 2),
            ("_", 1),
            ("==", 2),
            (">", 2),
            ("<", 2),
            (">=", 2),
            ("<=", 2),
            ("!=", 2),
            ("!", 1),
            ("&&", 2),
            ("||", 2),
            ("MIN", 2),
            ("MAX", 2),
            ("POW", 2),
            ("FLOOR", 1),
            ("CEILING", 1),
            ("INT", 1),
            ("FLOAT", 1),
            ("?", 2),
            ("!?", 2),
            ("^", 2),
            ("LIST_MIN", 1),
            ("LIST_MAX", 1),
            ("LIST_ALL", 1),
            ("LIST_COUNT", 1),
            ("LIST_VALUE", 1),
            ("LIST_INVERT", 1),
        ] {
            functions.insert(name, count);
        }
        functions
    })
}

fn value_type_rank(value_type: ValueType) -> i32 {
    match value_type {
        ValueType::Bool => -1,
        ValueType::Int => 0,
        ValueType::Float => 1,
        ValueType::List => 2,
        ValueType::String => 3,
        ValueType::DivertTarget => 4,
        ValueType::VariablePointer => 5,
    }
}

fn expect_value_type(value: &Value, expected: ValueType) -> Value {
    match value.Cast(expected) {
        Ok(Some(casted)) => casted,
        Ok(None) => panic!("Value could not be cast to {:?}", expected),
        Err(err) => panic!("{}", err),
    }
}

fn to_int(value: &Value) -> i32 {
    match value {
        Value::Bool(BoolValue { value }) => i32::from(*value),
        Value::Int(IntValue { value }) => *value,
        Value::Float(FloatValue { value }) => *value as i32,
        Value::String(StringValue { value, .. }) => value.parse().unwrap_or(0),
        _ => panic!(
            "Unexpected value type for int coercion: {:?}",
            value.value_type()
        ),
    }
}

fn to_float(value: &Value) -> f32 {
    match value {
        Value::Bool(BoolValue { value }) => {
            if *value {
                1.0
            } else {
                0.0
            }
        }
        Value::Int(IntValue { value }) => *value as f32,
        Value::Float(FloatValue { value }) => *value,
        Value::String(StringValue { value, .. }) => value.parse().unwrap_or(0.0),
        _ => panic!(
            "Unexpected value type for float coercion: {:?}",
            value.value_type()
        ),
    }
}

fn to_string_value(value: &Value) -> String {
    match value {
        Value::String(StringValue { value, .. }) => value.clone(),
        Value::Bool(BoolValue { value }) => value.to_string(),
        Value::Int(IntValue { value }) => value.to_string(),
        Value::Float(FloatValue { value }) => value.to_string(),
        _ => value.ToString(),
    }
}

fn list_value_to_ink_list(list_value: &ListValue) -> InkList {
    let mut ink_list = InkList::new();
    for (item, value) in &list_value.value {
        ink_list.insert_entry(item.clone(), *value);
    }
    if let Some(origin_names) = &list_value.originNames {
        ink_list.SetInitialOriginNames(origin_names.clone());
    }
    if let Some(origins) = &list_value.origins {
        ink_list.origins = Some(origins.clone());
    }
    ink_list
}

fn ink_list_to_list_value(mut ink_list: InkList) -> ListValue {
    ListValue {
        value: ink_list.get_entries().clone(),
        originNames: ink_list.get_originNames().map(|names| names.to_vec()),
        origins: ink_list.get_origins().map(|origins| origins.to_vec()),
    }
}

impl NativeFunctionCall {
    // C# signature: public NativeFunctionCall(string name)
    pub fn new(name: String) -> Self {
        let numberOfParameters = native_functions()
            .get(name.as_str())
            .copied()
            .unwrap_or_default();

        Self {
            name,
            numberOfParameters,
        }
    }

    // C# signature: public NativeFunctionCall()
    pub fn new_overload_2() -> Self {
        Self::default()
    }

    // C# signature: public static NativeFunctionCall CallWithName(string functionName)
    pub fn CallWithName(functionName: String) -> NativeFunctionCall {
        Self::new(functionName)
    }

    // C# signature: public static bool CallExistsWithName(string functionName)
    pub fn CallExistsWithName(functionName: String) -> bool {
        native_functions().contains_key(functionName.as_str())
    }

    // C# signature: public Runtime.Object Call(List<Runtime.Object> parameters)
    pub fn Call(&mut self, parameters: Vec<Value>) -> Value {
        if self.numberOfParameters as usize != parameters.len() {
            panic!("Unexpected number of parameters");
        }

        if std::env::var_os("INK_DEBUG_RUNTIME").is_some() {
            eprintln!(
                "native call {} params={:?}",
                self.name,
                parameters
                    .iter()
                    .map(|p| p.value_type())
                    .collect::<Vec<_>>()
            );
        }

        if parameters.iter().any(|p| matches!(p, Value::String(_))) {
            return self.call_for_string(parameters);
        }
        if parameters.iter().any(|p| matches!(p, Value::List(_))) {
            return self.call_for_list(parameters);
        }
        if parameters
            .iter()
            .any(|p| matches!(p, Value::DivertTarget(_)))
        {
            return self.call_for_divert_target(parameters);
        }

        let coerced = self.coerce_values_to_single_type(parameters);
        match coerced[0].value_type() {
            ValueType::Int | ValueType::Bool => self.call_for_int(coerced),
            ValueType::Float => self.call_for_float(coerced),
            ValueType::String => self.call_for_string(coerced),
            ValueType::List => self.call_for_list(coerced),
            ValueType::DivertTarget => self.call_for_divert_target(coerced),
            ValueType::VariablePointer => {
                panic!("Cannot perform native operations on variable pointers")
            }
        }
    }

    fn call_for_int(&self, params: Vec<Value>) -> Value {
        let unary = params.len() == 1;
        let a = to_int(&params[0]);
        let b = if unary { 0 } else { to_int(&params[1]) };
        match self.name.as_str() {
            "+" => Value::new_int(a + b),
            "_" if unary => Value::new_int(-a),
            "-" if unary => Value::new_int(-a),
            "-" => Value::new_int(a - b),
            "*" => Value::new_int(a * b),
            "/" => Value::new_int(a / b),
            "%" => Value::new_int(a % b),
            "==" => Value::new_bool(a == b),
            ">" => Value::new_bool(a > b),
            "<" => Value::new_bool(a < b),
            ">=" => Value::new_bool(a >= b),
            "<=" => Value::new_bool(a <= b),
            "!=" => Value::new_bool(a != b),
            "!" => Value::new_bool(a == 0),
            "&&" => Value::new_bool(a != 0 && b != 0),
            "||" => Value::new_bool(a != 0 || b != 0),
            "MIN" => Value::new_int(a.min(b)),
            "MAX" => Value::new_int(a.max(b)),
            "POW" => Value::new_float((a as f32).powf(b as f32)),
            "FLOOR" => Value::new_int(a),
            "CEILING" => Value::new_int(a),
            "INT" => Value::new_int(a),
            "FLOAT" => Value::new_float(a as f32),
            _ => panic!("Cannot perform operation '{}' on int", self.name),
        }
    }

    fn call_for_float(&self, params: Vec<Value>) -> Value {
        let unary = params.len() == 1;
        let a = to_float(&params[0]);
        let b = if unary { 0.0 } else { to_float(&params[1]) };
        match self.name.as_str() {
            "+" => Value::new_float(a + b),
            "_" if unary => Value::new_float(-a),
            "-" if unary => Value::new_float(-a),
            "-" => Value::new_float(a - b),
            "*" => Value::new_float(a * b),
            "/" => Value::new_float(a / b),
            "%" => Value::new_float(a % b),
            "==" => Value::new_bool(a == b),
            ">" => Value::new_bool(a > b),
            "<" => Value::new_bool(a < b),
            ">=" => Value::new_bool(a >= b),
            "<=" => Value::new_bool(a <= b),
            "!=" => Value::new_bool(a != b),
            "!" => Value::new_bool(a == 0.0),
            "&&" => Value::new_bool(a != 0.0 && b != 0.0),
            "||" => Value::new_bool(a != 0.0 || b != 0.0),
            "MIN" => Value::new_float(a.min(b)),
            "MAX" => Value::new_float(a.max(b)),
            "POW" => Value::new_float(a.powf(b)),
            "FLOOR" => Value::new_float(a.floor()),
            "CEILING" => Value::new_float(a.ceil()),
            "INT" => Value::new_int(a as i32),
            "FLOAT" => Value::new_float(a),
            _ => panic!("Cannot perform operation '{}' on float", self.name),
        }
    }

    fn call_for_string(&self, params: Vec<Value>) -> Value {
        let a = to_string_value(&params[0]);
        let b = if params.len() > 1 {
            to_string_value(&params[1])
        } else {
            String::new()
        };
        match self.name.as_str() {
            "+" => Value::new_string(format!("{}{}", a, b)),
            "==" => Value::new_bool(a == b),
            "!=" => Value::new_bool(a != b),
            "?" => Value::new_bool(a.contains(&b)),
            "!?" => Value::new_bool(!a.contains(&b)),
            _ => panic!("Cannot perform operation '{}' on string", self.name),
        }
    }

    fn call_for_divert_target(&self, params: Vec<Value>) -> Value {
        let a = match &params[0] {
            Value::DivertTarget(value) => value.value.clone().unwrap_or_default(),
            _ => panic!("Expected divert target value"),
        };
        let b = match &params[1] {
            Value::DivertTarget(value) => value.value.clone().unwrap_or_default(),
            _ => panic!("Expected divert target value"),
        };
        match self.name.as_str() {
            "==" => Value::new_bool(a == b),
            "!=" => Value::new_bool(a != b),
            _ => panic!("Cannot perform operation '{}' on divert target", self.name),
        }
    }

    fn call_for_list(&self, params: Vec<Value>) -> Value {
        if params.len() == 2 {
            if let (Value::List(list), Value::Int(int_val)) = (&params[0], &params[1]) {
                if self.name == "+" || self.name == "-" {
                    return self.call_list_increment_operation(list.clone(), int_val.value);
                }
            }
        }

        let unary = params.len() == 1;
        let a = match &params[0] {
            Value::List(value) => list_value_to_ink_list(value),
            _ => panic!("Expected list value"),
        };

        if unary {
            return match self.name.as_str() {
                "!" => Value::new_int(if a.get_entries().is_empty() { 1 } else { 0 }),
                "LIST_INVERT" => Value::new_list(ink_list_to_list_value(a.get_inverse())),
                "LIST_ALL" => Value::new_list(ink_list_to_list_value(a.get_all())),
                "LIST_MIN" => Value::new_list(ink_list_to_list_value(a.MinAsList())),
                "LIST_MAX" => Value::new_list(ink_list_to_list_value(a.MaxAsList())),
                "LIST_COUNT" => Value::new_int(a.get_entries().len() as i32),
                "LIST_VALUE" => Value::new_int(a.get_maxItem().1),
                _ => panic!("Cannot perform unary operation '{}' on list", self.name),
            };
        }

        let b = match &params[1] {
            Value::List(value) => list_value_to_ink_list(value),
            _ => panic!("Expected list value"),
        };

        match self.name.as_str() {
            "+" => Value::new_list(ink_list_to_list_value(a.Union(b))),
            "-" => Value::new_list(ink_list_to_list_value(a.Without(b))),
            "?" => Value::new_bool(a.Contains(b)),
            "!?" => Value::new_bool(!a.Contains(b)),
            "^" => Value::new_list(ink_list_to_list_value(a.Intersect(b))),
            "==" => Value::new_bool(a.Equals(&b)),
            ">" => Value::new_bool(a.GreaterThan(b)),
            "<" => Value::new_bool(a.LessThan(b)),
            ">=" => Value::new_bool(a.GreaterThanOrEquals(b)),
            "<=" => Value::new_bool(a.LessThanOrEquals(b)),
            "!=" => Value::new_bool(!a.Equals(&b)),
            "&&" => Value::new_bool(a.get_entries().len() > 0 && b.get_entries().len() > 0),
            "||" => Value::new_bool(a.get_entries().len() > 0 || b.get_entries().len() > 0),
            _ => panic!("Cannot perform operation '{}' on lists", self.name),
        }
    }

    fn call_list_increment_operation(&self, listVal: ListValue, intVal: i32) -> Value {
        let listVal = list_value_to_ink_list(&listVal);
        let mut result_raw_list = InkList::new();
        let int_op = match self.name.as_str() {
            "+" => |x: i32, y: i32| x + y,
            "-" => |x: i32, y: i32| x - y,
            _ => unreachable!(),
        };

        for (list_item, list_item_value) in listVal.get_entries() {
            let target_int = int_op(*list_item_value, intVal);
            if let Some(item_origin) = listVal.get_originOfMaxItem().or_else(|| {
                listVal.get_origins().and_then(|origins| {
                    origins
                        .iter()
                        .find(|origin| {
                            origin.get_name() == list_item.originName.as_deref().unwrap_or("")
                        })
                        .cloned()
                })
            }) {
                if let Some(incremented_item) = item_origin.TryGetItemWithValue(target_int) {
                    result_raw_list.insert_entry(incremented_item, target_int);
                }
            }
        }

        Value::new_list(ink_list_to_list_value(result_raw_list))
    }

    fn coerce_values_to_single_type(&self, parametersIn: Vec<Value>) -> Vec<Value> {
        let mut valType = ValueType::Int;
        let mut specialCaseList: Option<ListValue> = None;

        for val in &parametersIn {
            if value_type_rank(val.value_type()) > value_type_rank(valType) {
                valType = val.value_type();
            }

            if let Value::List(list) = val {
                specialCaseList = Some(list.clone());
            }
        }

        let mut parametersOut = Vec::with_capacity(parametersIn.len());

        if valType == ValueType::List {
            let specialCaseList = specialCaseList.expect("list coercion requires a list parameter");
            let specialCaseInkList = list_value_to_ink_list(&specialCaseList);

            for val in parametersIn {
                match val {
                    Value::List(list) => parametersOut.push(Value::List(list)),
                    Value::Int(intVal) => {
                        let originDef = specialCaseInkList.get_originOfMaxItem().or_else(|| {
                            specialCaseInkList
                                .get_origins()
                                .and_then(|origins| origins.first().cloned())
                        });
                        if let Some(originDef) = originDef {
                            if let Some(item) = originDef.TryGetItemWithValue(intVal.value) {
                                parametersOut.push(Value::new_list(ListValue::new_overload_3(
                                    item,
                                    intVal.value,
                                )));
                            } else {
                                panic!(
                                    "Could not find List item with the value {} in {}",
                                    intVal.value,
                                    originDef.get_name()
                                );
                            }
                        } else {
                            panic!("Cannot mix Lists and Int values in this operation");
                        }
                    }
                    other => panic!(
                        "Cannot mix Lists and {:?} values in this operation",
                        other.value_type()
                    ),
                }
            }
        } else {
            for val in parametersIn {
                parametersOut.push(expect_value_type(&val, valType));
            }
        }

        parametersOut
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        format!("Native '{}'", self.name)
    }

    // C# signature: string name { get; }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    // C# signature: int numberOfParameters { get; }
    pub fn get_numberOfParameters(&self) -> i32 {
        self.numberOfParameters
    }
}

#[cfg(test)]
mod tests {
    use super::NativeFunctionCall;
    use crate::Value::Value;

    #[test]
    fn recognizes_native_function_names() {
        assert!(NativeFunctionCall::CallExistsWithName("+".to_string()));
        assert!(NativeFunctionCall::CallExistsWithName(
            "LIST_VALUE".to_string()
        ));
        assert!(!NativeFunctionCall::CallExistsWithName(
            "CHOICE_COUNT".to_string()
        ));
    }

    #[test]
    fn stores_name_and_parameter_count() {
        let call = NativeFunctionCall::CallWithName("!".to_string());
        assert_eq!(call.get_name(), "!");
        assert_eq!(call.get_numberOfParameters(), 1);
        assert_eq!(call.ToString(), "Native '!'");
    }

    #[test]
    fn adds_ints() {
        let mut call = NativeFunctionCall::CallWithName("+".to_string());
        let result = call.Call(vec![Value::new_int(2), Value::new_int(3)]);
        assert_eq!(result.ToString(), "5");
    }
}
