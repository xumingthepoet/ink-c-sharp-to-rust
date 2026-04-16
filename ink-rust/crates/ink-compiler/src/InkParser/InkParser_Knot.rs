// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/InkParser/InkParser_Knot.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct InkParser {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct NameWithMetadata {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct FlowDecl {
    pub _port_marker: (),
}

impl InkParser {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: protected Knot KnotDefinition()
    pub fn KnotDefinition(&mut self) -> crate::stub::Knot {
        Default::default()
    }

    // C# signature: protected FlowDecl KnotDeclaration()
    pub fn KnotDeclaration(&mut self) -> crate::stub::FlowDecl {
        Default::default()
    }

    // C# signature: protected string KnotTitleEquals()
    pub fn KnotTitleEquals(&mut self) -> String {
        Default::default()
    }

    // C# signature: protected object StitchDefinition()
    pub fn StitchDefinition(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected FlowDecl StitchDeclaration()
    pub fn StitchDeclaration(&mut self) -> crate::stub::FlowDecl {
        Default::default()
    }

    // C# signature: protected object KnotStitchNoContentRecoveryRule()
    pub fn KnotStitchNoContentRecoveryRule(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected List<FlowBase.Argument> BracketedKnotDeclArguments()
    pub fn BracketedKnotDeclArguments(&mut self) -> Vec<crate::stub::Argument> {
        Default::default()
    }

    // C# signature: protected FlowBase.Argument FlowDeclArgument()
    pub fn FlowDeclArgument(&mut self) -> crate::stub::Argument {
        Default::default()
    }

    // C# signature: protected ExternalDeclaration ExternalDeclaration()
    pub fn ExternalDeclaration(&mut self) -> crate::stub::ExternalDeclaration {
        Default::default()
    }
}
