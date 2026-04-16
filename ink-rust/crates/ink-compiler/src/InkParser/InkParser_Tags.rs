// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/InkParser/InkParser_Tags.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct InkParser {
    pub _port_marker: (),
}

impl InkParser {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: protected Parsed.Object StartTag ()
    pub fn StartTag(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected void EndTagIfNecessary(List<Parsed.Object> outputContentList)
    pub fn EndTagIfNecessary(&mut self, _outputContentList: Vec<crate::stub::PortStub>) {}

    // C# signature: protected void EndTagIfNecessary(Parsed.ContentList outputContentList)
    pub fn EndTagIfNecessary_overload_2(&mut self, _outputContentList: crate::stub::ContentList) {}
}
