// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/InkParser/InkParser_Content.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct InkParser {
    pub _port_marker: (),
}

impl InkParser {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: protected List<Parsed.Object> LineOfMixedTextAndLogic()
    pub fn LineOfMixedTextAndLogic(&mut self) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: protected List<Parsed.Object> MixedTextAndLogic()
    pub fn MixedTextAndLogic(&mut self) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: protected Parsed.Text ContentText()
    pub fn ContentText(&mut self) -> crate::stub::Text {
        Default::default()
    }

    // C# signature: protected Parsed.Text ContentTextAllowingEcapeChar()
    pub fn ContentTextAllowingEcapeChar(&mut self) -> crate::stub::Text {
        Default::default()
    }

    // C# signature: protected string ContentTextNoEscape()
    pub fn ContentTextNoEscape(&mut self) -> String {
        Default::default()
    }
}
