// Source: ink-c-sharp/compiler/InkParser/InkParser_CharacterRanges.cs

use crate::CharacterRange::CharacterRange;
use crate::CharacterSet::CharacterSet;

#[derive(Clone, Debug, Default)]
pub struct InkParser {
    pub _port_marker: (),
}

impl InkParser {
    pub fn new() -> Self {
        Self::default()
    }

    fn chars_inclusive(start: char, end: char) -> Vec<char> {
        (start as u32..=end as u32)
            .filter_map(char::from_u32)
            .collect()
    }

    fn latin_basic() -> CharacterRange {
        CharacterRange::Define(
            '\u{0041}',
            '\u{007A}',
            Some(Self::chars_inclusive('\u{005B}', '\u{0060}')),
        )
    }

    fn latin_extended_a() -> CharacterRange {
        CharacterRange::Define('\u{0100}', '\u{017F}', None)
    }

    fn latin_extended_b() -> CharacterRange {
        CharacterRange::Define('\u{0180}', '\u{024F}', None)
    }

    fn greek() -> CharacterRange {
        let mut excludes = Self::chars_inclusive('\u{0378}', '\u{0385}');
        excludes.extend("\u{0374}\u{0375}\u{0378}\u{0387}\u{038B}\u{038D}\u{03A2}".chars());
        CharacterRange::Define('\u{0370}', '\u{03FF}', Some(excludes))
    }

    fn cyrillic() -> CharacterRange {
        CharacterRange::Define(
            '\u{0400}',
            '\u{04FF}',
            Some(Self::chars_inclusive('\u{0482}', '\u{0489}')),
        )
    }

    fn armenian() -> CharacterRange {
        let mut excludes = vec!['\u{0530}'];
        excludes.extend(Self::chars_inclusive('\u{0557}', '\u{0560}'));
        excludes.extend(Self::chars_inclusive('\u{0588}', '\u{058E}'));
        CharacterRange::Define('\u{0530}', '\u{058F}', Some(excludes))
    }

    fn hebrew() -> CharacterRange {
        CharacterRange::Define('\u{0590}', '\u{05FF}', Some(Vec::new()))
    }

    fn arabic() -> CharacterRange {
        CharacterRange::Define('\u{0600}', '\u{06FF}', Some(Vec::new()))
    }

    fn korean() -> CharacterRange {
        CharacterRange::Define('\u{AC00}', '\u{D7AF}', Some(Vec::new()))
    }

    fn latin1_supplement() -> CharacterRange {
        CharacterRange::Define('\u{0080}', '\u{00FF}', Some(Vec::new()))
    }

    fn cjk_unified_ideographs() -> CharacterRange {
        CharacterRange::Define('\u{4E00}', '\u{9FFF}', Some(Vec::new()))
    }

    fn hiragana() -> CharacterRange {
        CharacterRange::Define('\u{3041}', '\u{3096}', None)
    }

    fn katakana() -> CharacterRange {
        CharacterRange::Define('\u{30A0}', '\u{30FC}', None)
    }

    // C# signature: private void ExtendIdentifierCharacterRanges(CharacterSet identifierCharSet)
    pub fn ExtendIdentifierCharacterRanges(&mut self, identifierCharSet: &mut CharacterSet) {
        for mut char_range in Self::ListAllCharacterRanges() {
            let character_set = char_range.ToCharacterSet();
            identifierCharSet.AddCharacters(character_set.characters.iter().copied());
        }
    }

    // C# signature: public static CharacterRange[] ListAllCharacterRanges()
    pub fn ListAllCharacterRanges() -> Vec<CharacterRange> {
        vec![
            Self::latin_basic(),
            Self::latin_extended_a(),
            Self::latin_extended_b(),
            Self::arabic(),
            Self::armenian(),
            Self::cyrillic(),
            Self::greek(),
            Self::hebrew(),
            Self::korean(),
            Self::latin1_supplement(),
            Self::cjk_unified_ideographs(),
            Self::hiragana(),
            Self::katakana(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::InkParser;
    use crate::CharacterSet::CharacterSet;

    #[test]
    fn lists_supported_character_ranges_in_expected_order() {
        let ranges = InkParser::ListAllCharacterRanges();
        assert_eq!(ranges.len(), 13);
        assert_eq!(ranges[0].get_start(), '\u{0041}');
        assert_eq!(ranges[12].get_end(), '\u{30FC}');

        let mut identifier_chars = CharacterSet::new();
        let mut parser = InkParser::new();
        parser.ExtendIdentifierCharacterRanges(&mut identifier_chars);
        assert!(identifier_chars.Contains('A'));
        assert!(identifier_chars.Contains('Ω'));
        assert!(identifier_chars.Contains('あ'));
    }
}
