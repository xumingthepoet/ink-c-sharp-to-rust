// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/CharacterSet.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct CharacterSet {
    pub _port_marker: (),
}

impl CharacterSet {
    // C# signature: public CharacterSet ()
    pub fn new() -> Self {
        Default::default()
    }

    // C# signature: public CharacterSet(string str)
    pub fn new_overload_2(_str: String) -> Self {
        Default::default()
    }

    // C# signature: public CharacterSet(CharacterSet charSetToCopy)
    pub fn new_overload_3(_charSetToCopy: crate::stub::CharacterSet) -> Self {
        Default::default()
    }

    // C# signature: public static CharacterSet FromRange(char start, char end)
    pub fn FromRange(_start: char, _end: char) -> crate::stub::CharacterSet {
        Default::default()
    }

    // C# signature: public CharacterSet AddRange(char start, char end)
    pub fn AddRange(&mut self, _start: char, _end: char) -> crate::stub::CharacterSet {
        Default::default()
    }

    // C# signature: public CharacterSet AddCharacters(IEnumerable<char> chars)
    pub fn AddCharacters(&mut self, _chars: Vec<char>) -> crate::stub::CharacterSet {
        Default::default()
    }

    // C# signature: public CharacterSet AddCharacters (string chars)
    pub fn AddCharacters_overload_2(&mut self, _chars: String) -> crate::stub::CharacterSet {
        Default::default()
    }
}
