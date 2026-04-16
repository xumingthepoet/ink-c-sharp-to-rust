// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/NativeFunctionCall.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct NativeFunctionCall {
    pub _port_marker: (),
}

impl NativeFunctionCall {
    // C# signature: public NativeFunctionCall(string name)
    pub fn new(_name: String) -> Self {
        Default::default()
    }

    // C# signature: public NativeFunctionCall()
    pub fn new_overload_2() -> Self {
        Default::default()
    }

    // C# signature: public static NativeFunctionCall CallWithName(string functionName)
    pub fn CallWithName(_functionName: String) -> crate::stub::NativeFunctionCall {
        Default::default()
    }

    // C# signature: public static bool CallExistsWithName(string functionName)
    pub fn CallExistsWithName(_functionName: String) -> bool {
        Default::default()
    }

    // C# signature: public Runtime.Object Call(List<Runtime.Object> parameters)
    pub fn Call(&mut self, _parameters: Vec<crate::stub::PortStub>) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: string name { get; }
    pub fn get_name(&mut self) -> String {
        Default::default()
    }

    // C# signature: int numberOfParameters { get; }
    pub fn get_numberOfParameters(&mut self) -> i32 {
        Default::default()
    }
}
