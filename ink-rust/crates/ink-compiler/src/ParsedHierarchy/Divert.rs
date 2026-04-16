// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Divert.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Divert {
    pub _port_marker: (),
}

impl Divert {
    // C# signature: public Divert (Parsed.Path target, List<Expression> arguments = null)
    pub fn new(_target: crate::stub::Path, _arguments: Vec<crate::stub::Expression>) -> Self {
        Default::default()
    }

    // C# signature: public Divert (Parsed.Object targetContent)
    pub fn new_overload_2(_targetContent: crate::stub::PortStub) -> Self {
        Default::default()
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public string PathAsVariableName()
    pub fn PathAsVariableName(&mut self) -> String {
        Default::default()
    }

    // C# signature: public override void ResolveReferences(Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: public override void Error (string message, Object source = null, bool isWarning = false)
    pub fn Error(&mut self, _message: String, _source: crate::stub::PortStub, _isWarning: bool) {}

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: Parsed.Path target { get; }
    pub fn get_target(&mut self) -> crate::stub::Path {
        Default::default()
    }

    // C# signature: Parsed.Object targetContent { get; }
    pub fn get_targetContent(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: List<Expression> arguments { get; }
    pub fn get_arguments(&mut self) -> Vec<crate::stub::Expression> {
        Default::default()
    }

    // C# signature: Runtime.Divert runtimeDivert { get; }
    pub fn get_runtimeDivert(&mut self) -> crate::stub::Divert {
        Default::default()
    }

    // C# signature: bool isFunctionCall { get; }
    pub fn get_isFunctionCall(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isEmpty { get; }
    pub fn get_isEmpty(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isTunnel { get; }
    pub fn get_isTunnel(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isThread { get; }
    pub fn get_isThread(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isEnd { get; }
    pub fn get_isEnd(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isDone { get; }
    pub fn get_isDone(&mut self) -> bool {
        Default::default()
    }
}
