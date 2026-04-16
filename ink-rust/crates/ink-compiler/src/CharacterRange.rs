// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/CharacterRange.cs

use std::collections::HashSet;

use crate::CharacterSet::CharacterSet;

#[derive(Clone, Debug, Default)]
pub struct CharacterRange {
    start: char,
    end: char,
    excludes: HashSet<char>,
    correspondingCharSet: CharacterSet,
}

impl CharacterRange {
    fn new(_start: char, _end: char, _excludes: Option<Vec<char>>) -> Self {
        Self {
            start: _start,
            end: _end,
            excludes: _excludes.unwrap_or_default().into_iter().collect(),
            correspondingCharSet: CharacterSet::new(),
        }
    }

    // C# signature: public static CharacterRange Define(char start, char end, IEnumerable<char> excludes = null)
    pub fn Define(_start: char, _end: char, _excludes: Option<Vec<char>>) -> CharacterRange {
        Self::new(_start, _end, _excludes)
    }

    // C# signature: public CharacterSet ToCharacterSet ()
    pub fn ToCharacterSet(&mut self) -> &CharacterSet {
        if self.correspondingCharSet.characters.is_empty() {
            for c in (self.start as u32)..=(self.end as u32) {
                if let Some(c) = char::from_u32(c) {
                    if !self.excludes.contains(&c) {
                        self.correspondingCharSet.characters.insert(c);
                    }
                }
            }
        }
        &self.correspondingCharSet
    }

    // C# signature: char start { get; }
    pub fn get_start(&self) -> char {
        self.start
    }

    // C# signature: char end { get; }
    pub fn get_end(&self) -> char {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use super::CharacterRange;

    #[test]
    fn materializes_character_sets_with_exclusions() {
        let mut range = CharacterRange::Define('a', 'd', Some(vec!['b']));
        let set = range.ToCharacterSet();

        assert!(set.Contains('a'));
        assert!(!set.Contains('b'));
        assert!(set.Contains('c'));
        assert!(set.Contains('d'));
        assert_eq!(range.get_start(), 'a');
        assert_eq!(range.get_end(), 'd');
    }
}
