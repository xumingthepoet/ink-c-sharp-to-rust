// Source: ink-c-sharp/ink-engine-runtime/NativeFunctionCall.cs

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
        for name in [
            "+",
            "-",
            "/",
            "*",
            "%",
            "_",
            "==",
            ">",
            "<",
            ">=",
            "<=",
            "!=",
            "!",
            "&&",
            "||",
            "MIN",
            "MAX",
            "POW",
            "FLOOR",
            "CEILING",
            "INT",
            "FLOAT",
            "?",
            "!?",
            "^",
            "LIST_MIN",
            "LIST_MAX",
            "LIST_ALL",
            "LIST_COUNT",
            "LIST_VALUE",
            "LIST_INVERT",
        ] {
            functions.insert(
                name,
                if matches!(
                    name,
                    "_" | "!"
                        | "FLOOR"
                        | "CEILING"
                        | "INT"
                        | "FLOAT"
                        | "LIST_MIN"
                        | "LIST_MAX"
                        | "LIST_ALL"
                        | "LIST_COUNT"
                        | "LIST_VALUE"
                        | "LIST_INVERT"
                ) {
                    1
                } else {
                    2
                },
            );
        }
        functions
    })
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
    pub fn Call(&mut self, _parameters: Vec<crate::stub::PortStub>) -> crate::stub::PortStub {
        todo!("port NativeFunctionCall.Call after native operation dispatch is translated");
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
}
