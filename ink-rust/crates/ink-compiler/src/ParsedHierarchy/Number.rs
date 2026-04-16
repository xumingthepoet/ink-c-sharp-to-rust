// Source: ink-c-sharp/compiler/ParsedHierarchy/Number.cs

use ink_runtime::Container::Container;
use ink_runtime::Value::Value;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq)]
pub enum NumberValue {
    Int(i32),
    Float(f32),
    Bool(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Number {
    pub value: NumberValue,
}

impl Number {
    // C# signature: public Number(object value)
    pub fn new(value: NumberValue) -> Self {
        Self { value }
    }

    // C# signature: public override void GenerateIntoContainer (Runtime.Container container)
    pub fn GenerateIntoContainer(&self, container: &mut Container) {
        match self.value {
            NumberValue::Int(value) => container.AddContent(Value::new_int(value)),
            NumberValue::Float(value) => container.AddContent(Value::new_float(value)),
            NumberValue::Bool(value) => container.AddContent(Value::new_bool(value)),
        }
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        match self.value {
            NumberValue::Int(value) => value.to_string(),
            NumberValue::Float(value) => value.to_string(),
            NumberValue::Bool(value) => value.to_string(),
        }
    }

    // C# signature: public override bool Equals (object obj)
    pub fn Equals(&self, obj: &Number) -> bool {
        self.value == obj.value
    }

    // C# signature: public override int GetHashCode ()
    pub fn GetHashCode(&self) -> i32 {
        let mut hasher = DefaultHasher::new();
        match self.value {
            NumberValue::Int(value) => value.hash(&mut hasher),
            NumberValue::Float(value) => value.to_bits().hash(&mut hasher),
            NumberValue::Bool(value) => value.hash(&mut hasher),
        }
        hasher.finish() as i32
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.ToString())
    }
}

#[cfg(test)]
mod tests {
    use super::{Number, NumberValue};
    use ink_runtime::Container::{Container, ContentItem};

    #[test]
    fn generates_runtime_values() {
        let mut container = Container::new();
        Number::new(NumberValue::Int(4)).GenerateIntoContainer(&mut container);
        Number::new(NumberValue::Float(2.5)).GenerateIntoContainer(&mut container);
        Number::new(NumberValue::Bool(true)).GenerateIntoContainer(&mut container);

        assert_eq!(container.get_content().len(), 3);
        assert!(matches!(container.get_content()[0], ContentItem::Value(_)));
        assert_eq!(Number::new(NumberValue::Float(2.5)).ToString(), "2.5");
    }
}
