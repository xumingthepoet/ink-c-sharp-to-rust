// Source: ink-c-sharp/compiler/InkParser/InkParser_Sequences.cs

use crate::CharacterSet::CharacterSet;
use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::ContentList::{ContentList, ContentListItem};
use crate::ParsedHierarchy::Sequence::{Sequence, SequenceType};

impl InkParser {
    // C# signature: protected Sequence InnerSequence()
    pub fn InnerSequence(&mut self) -> Option<Sequence> {
        let _ = self.Whitespace();

        let mut seqType = SequenceType::default();

        if let Some(parsedSeqType) = self.SequenceTypeAnnotation() {
            seqType = parsedSeqType;
        }

        let contentLists = self.InnerSequenceObjects()?;
        if contentLists.len() <= 1 {
            return None;
        }

        Some(Sequence::new(contentLists, seqType))
    }

    // C# signature: protected object SequenceTypeAnnotation()
    pub fn SequenceTypeAnnotation(&mut self) -> Option<SequenceType> {
        let annotation = self
            .SequenceTypeSymbolAnnotation()
            .or_else(|| self.SequenceTypeWordAnnotation());

        let Some(annotation) = annotation else {
            return None;
        };

        let bits = annotation.bits();
        let allowed = matches!(bits, 1 | 2 | 4 | 8 | 5 | 9);
        if !allowed {
            self.Error(format!(
                "Sequence type combination not supported: {}",
                annotation
            ));
            return Some(SequenceType::default());
        }

        Some(annotation)
    }

    // C# signature: protected object SequenceTypeSymbolAnnotation()
    pub fn SequenceTypeSymbolAnnotation(&mut self) -> Option<SequenceType> {
        let sequence_type_symbols = CharacterSet::new_overload_2("!&~$ ".to_string());
        let sequence_annotations =
            self.ParseCharactersFromCharSet(sequence_type_symbols, true, -1)?;

        let mut sequenceType = SequenceType::from_bits(0);
        for symbol_char in sequence_annotations.chars() {
            match symbol_char {
                '!' => sequenceType |= SequenceType::ONCE,
                '&' => sequenceType |= SequenceType::CYCLE,
                '~' => sequenceType |= SequenceType::SHUFFLE,
                '$' => sequenceType |= SequenceType::STOPPING,
                _ => {}
            }
        }

        if sequenceType.bits() == 0 {
            None
        } else {
            Some(sequenceType)
        }
    }

    // C# signature: protected object SequenceTypeWordAnnotation()
    pub fn SequenceTypeWordAnnotation(&mut self) -> Option<SequenceType> {
        let mut sequenceTypes = Vec::<SequenceType>::new();

        if let Some(first) = self.SequenceTypeSingleWord() {
            sequenceTypes.push(first);
        } else {
            return None;
        }

        loop {
            let rule_id = self.parser_mut().BeginRule();

            if self.Whitespace().is_none() {
                self.parser_mut().CancelRule(rule_id);
                break;
            }

            match self.SequenceTypeSingleWord() {
                Some(seqType) => {
                    self.parser_mut().SucceedRule(rule_id, ());
                    sequenceTypes.push(seqType);
                }
                None => {
                    self.parser_mut().CancelRule(rule_id);
                    break;
                }
            }
        }

        if self.ParseString(":".to_string()).is_none() {
            return None;
        }

        let mut combinedSequenceType = SequenceType::from_bits(0);
        for seqType in sequenceTypes {
            combinedSequenceType |= seqType;
        }

        Some(combinedSequenceType)
    }

    // C# signature: protected object SequenceTypeSingleWord()
    pub fn SequenceTypeSingleWord(&mut self) -> Option<SequenceType> {
        let word = self.IdentifierWithMetadata()?;
        match word.name.as_deref() {
            Some("once") => Some(SequenceType::ONCE),
            Some("cycle") => Some(SequenceType::CYCLE),
            Some("shuffle") => Some(SequenceType::SHUFFLE),
            Some("stopping") => Some(SequenceType::STOPPING),
            _ => None,
        }
    }

    // C# signature: protected List<ContentList> InnerSequenceObjects()
    pub fn InnerSequenceObjects(&mut self) -> Option<Vec<ContentList>> {
        if self.ParseNewline().is_some() {
            self.InnerMultilineSequenceObjects()
        } else {
            self.InnerInlineSequenceObjects()
        }
    }

    // C# signature: protected List<ContentList> InnerInlineSequenceObjects()
    pub fn InnerInlineSequenceObjects(&mut self) -> Option<Vec<ContentList>> {
        let mut result = Vec::<ContentList>::new();
        let mut justHadContent = false;

        loop {
            let mut progressed = false;

            if let Some(content) = self.MixedTextAndLogic() {
                result.push(ContentList::new(content));
                justHadContent = true;
                progressed = true;
            }

            if self.ParseString("|".to_string()).is_some() {
                if !justHadContent {
                    result.push(ContentList::new_overload_2());
                }
                justHadContent = false;
                progressed = true;
            }

            if !progressed {
                break;
            }
        }

        if !justHadContent {
            result.push(ContentList::new_overload_2());
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    // C# signature: protected List<ContentList> InnerMultilineSequenceObjects()
    pub fn InnerMultilineSequenceObjects(&mut self) -> Option<Vec<ContentList>> {
        let _ = self.MultilineWhitespace();

        todo!(
            "sequence multiline parsing needs ContentList conversion from parsed statement objects"
        )
    }

    // C# signature: protected ContentList SingleMultilineSequenceElement()
    pub fn SingleMultilineSequenceElement(&mut self) -> Option<ContentList> {
        let _ = self.Whitespace();

        if self.ParseString("->".to_string()).is_some() {
            return None;
        }

        if self.ParseString("-".to_string()).is_none() {
            return None;
        }

        let _ = self.Whitespace();

        todo!(
            "sequence multiline parsing needs ContentList conversion from parsed statement objects"
        )
    }
}
