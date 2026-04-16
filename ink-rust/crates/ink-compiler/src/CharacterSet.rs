// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/CharacterSet.cs

use std::collections::HashSet;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CharacterSet {
    pub characters: HashSet<char>,
}

impl CharacterSet {
    // C# signature: public CharacterSet ()
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public CharacterSet(string str)
    pub fn new_overload_2(_str: String) -> Self {
        let mut set = Self::new();
        set.AddCharacters_overload_2(_str);
        set
    }

    // C# signature: public CharacterSet(CharacterSet charSetToCopy)
    pub fn new_overload_3(_charSetToCopy: &CharacterSet) -> Self {
        _charSetToCopy.clone()
    }

    // C# signature: public static CharacterSet FromRange(char start, char end)
    pub fn FromRange(_start: char, _end: char) -> CharacterSet {
        let mut set = Self::new();
        set.AddRange(_start, _end);
        set
    }

    // C# signature: public CharacterSet AddRange(char start, char end)
    pub fn AddRange(&mut self, _start: char, _end: char) -> &mut CharacterSet {
        for c in (_start as u32)..=(_end as u32) {
            if let Some(c) = char::from_u32(c) {
                self.characters.insert(c);
            }
        }
        self
    }

    // C# signature: public CharacterSet AddCharacters(IEnumerable<char> chars)
    pub fn AddCharacters<I>(&mut self, _chars: I) -> &mut CharacterSet
    where
        I: IntoIterator<Item = char>,
    {
        self.characters.extend(_chars);
        self
    }

    // C# signature: public CharacterSet AddCharacters (string chars)
    pub fn AddCharacters_overload_2(&mut self, _chars: String) -> &mut CharacterSet {
        self.AddCharacters(_chars.chars())
    }

    pub fn Contains(&self, c: char) -> bool {
        self.characters.contains(&c)
    }
}

#[cfg(test)]
mod tests {
    use super::CharacterSet;

    #[test]
    fn builds_sets_from_ranges_and_strings() {
        let mut from_range = CharacterSet::FromRange('a', 'c');
        assert!(from_range.Contains('a'));
        assert!(from_range.Contains('b'));
        assert!(from_range.Contains('c'));
        assert!(!from_range.Contains('d'));

        from_range.AddCharacters_overload_2("de".to_string());
        assert!(from_range.Contains('d'));
        assert!(from_range.Contains('e'));

        let copied = CharacterSet::new_overload_3(&from_range);
        assert_eq!(copied, from_range);
    }
}
