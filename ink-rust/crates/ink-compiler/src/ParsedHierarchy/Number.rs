// Source: ink-c-sharp/compiler/ParsedHierarchy/Number.cs

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NumberValue {
    Int(i32),
    Float(f32),
    Bool(bool),
}

impl Default for NumberValue {
    fn default() -> Self {
        Self::Int(0)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Number {
    pub value: NumberValue,
}

impl Number {
    // C# signature: public Number(object value)
    pub fn new(value: NumberValue) -> Self {
        Self { value }
    }

    pub fn new_int(value: i32) -> Self {
        Self::new(NumberValue::Int(value))
    }

    pub fn new_float(value: f32) -> Self {
        Self::new(NumberValue::Float(value))
    }

    pub fn new_bool(value: bool) -> Self {
        Self::new(NumberValue::Bool(value))
    }

    // C# signature: public override void GenerateIntoContainer (Runtime.Container container)
    pub fn GenerateIntoContainer(&self, _container: &mut ink_runtime::Container::Container) {
        // Runtime value insertion depends on Runtime.Value and Container being ported.
        // The parsed number itself is fully represented here.
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
        self == obj
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

#[cfg(test)]
mod tests {
    use super::Number;

    #[test]
    fn formats_supported_number_values() {
        assert_eq!(Number::new_int(3).ToString(), "3");
        assert_eq!(Number::new_float(2.5).ToString(), "2.5");
        assert_eq!(Number::new_bool(true).ToString(), "true");
        assert!(Number::new_int(3).Equals(&Number::new_int(3)));
    }
}
