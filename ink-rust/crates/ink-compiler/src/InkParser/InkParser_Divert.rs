// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/InkParser/InkParser_Divert.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct InkParser {
    pub _port_marker: (),
}

impl InkParser {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: protected List<Parsed.Object> MultiDivert()
    pub fn MultiDivert(&mut self) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: protected Divert StartThread()
    pub fn StartThread(&mut self) -> crate::stub::Divert {
        Default::default()
    }

    // C# signature: protected Divert DivertIdentifierWithArguments()
    pub fn DivertIdentifierWithArguments(&mut self) -> crate::stub::Divert {
        Default::default()
    }

    // C# signature: protected Divert SingleDivert()
    pub fn SingleDivert(&mut self) -> crate::stub::Divert {
        Default::default()
    }

    // C# signature: protected string ParseDivertArrowOrTunnelOnwards()
    pub fn ParseDivertArrowOrTunnelOnwards(&mut self) -> String {
        Default::default()
    }

    // C# signature: protected string ParseDivertArrow()
    pub fn ParseDivertArrow(&mut self) -> String {
        Default::default()
    }

    // C# signature: protected string ParseThreadArrow()
    pub fn ParseThreadArrow(&mut self) -> String {
        Default::default()
    }
}
