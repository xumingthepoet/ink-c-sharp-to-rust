// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/StringExpression.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct StringExpression {
    pub _port_marker: (),
}

impl StringExpression {
    // C# signature: public StringExpression (List<Parsed.Object> content)
    pub fn new(_content: Vec<crate::stub::PortStub>) -> Self {
        Default::default()
    }

    // C# signature: public override void GenerateIntoContainer (Runtime.Container container)
    pub fn GenerateIntoContainer(&mut self, _container: crate::stub::Container) {}

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: public override bool Equals (object obj)
    pub fn Equals(&mut self, _obj: crate::stub::PortStub) -> bool {
        Default::default()
    }

    // C# signature: public override int GetHashCode ()
    pub fn GetHashCode(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: bool isSingleString { get; }
    pub fn get_isSingleString(&mut self) -> bool {
        Default::default()
    }
}
