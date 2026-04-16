// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/FunctionCall.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct FunctionCall {
    pub _port_marker: (),
}

impl FunctionCall {
    // C# signature: public FunctionCall (Identifier functionName, List<Expression> arguments)
    pub fn new(
        _functionName: crate::stub::Identifier,
        _arguments: Vec<crate::stub::Expression>,
    ) -> Self {
        Default::default()
    }

    // C# signature: public override void GenerateIntoContainer (Runtime.Container container)
    pub fn GenerateIntoContainer(&mut self, _container: crate::stub::Container) {}

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: public static bool IsBuiltIn(string name)
    pub fn IsBuiltIn(_name: String) -> bool {
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

    // C# signature: Divert proxyDivert { get; }
    pub fn get_proxyDivert(&mut self) -> crate::stub::Divert {
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

    // C# signature: bool isChoiceCount { get; }
    pub fn get_isChoiceCount(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isTurns { get; }
    pub fn get_isTurns(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isTurnsSince { get; }
    pub fn get_isTurnsSince(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isRandom { get; }
    pub fn get_isRandom(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isSeedRandom { get; }
    pub fn get_isSeedRandom(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isListRange { get; }
    pub fn get_isListRange(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isListRandom { get; }
    pub fn get_isListRandom(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isReadCount { get; }
    pub fn get_isReadCount(&mut self) -> bool {
        Default::default()
    }
}
