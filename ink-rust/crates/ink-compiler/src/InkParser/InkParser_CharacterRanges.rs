// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/InkParser/InkParser_CharacterRanges.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct InkParser {
    pub _port_marker: (),
}

impl InkParser {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: private void ExtendIdentifierCharacterRanges(CharacterSet identifierCharSet)
    pub fn ExtendIdentifierCharacterRanges(
        &mut self,
        _identifierCharSet: crate::stub::CharacterSet,
    ) {
    }

    // C# signature: public static CharacterRange[] ListAllCharacterRanges()
    pub fn ListAllCharacterRanges() -> Vec<crate::stub::CharacterRange> {
        Default::default()
    }
}
