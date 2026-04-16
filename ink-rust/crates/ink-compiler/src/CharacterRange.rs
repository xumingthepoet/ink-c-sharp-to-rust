// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/CharacterRange.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct CharacterRange {
    pub _port_marker: (),
}

impl CharacterRange {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public static CharacterRange Define(char start, char end, IEnumerable<char> excludes = null)
    pub fn Define(_start: char, _end: char, _excludes: Vec<char>) -> crate::stub::CharacterRange {
        Default::default()
    }

    // C# signature: public CharacterSet ToCharacterSet ()
    pub fn ToCharacterSet(&mut self) -> crate::stub::CharacterSet {
        Default::default()
    }

    // C# signature: char start { get; }
    pub fn get_start(&mut self) -> char {
        Default::default()
    }

    // C# signature: char end { get; }
    pub fn get_end(&mut self) -> char {
        Default::default()
    }
}
