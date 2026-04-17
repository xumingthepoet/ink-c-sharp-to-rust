// Source: ink-c-sharp/compiler/InkParser/InkParser.cs

use crate::CharacterSet::CharacterSet;
use crate::FileHandler::{DefaultFileHandler, IFileHandler};
use crate::StringParser::StringParser::{ErrorHandler, StringParser};
use ink_runtime::DebugMetadata::DebugMetadata;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
pub struct InkParser {
    parser: StringParser,
    filename: Option<String>,
    externalErrorHandler: Option<ErrorHandler>,
    fileHandler: Arc<dyn IFileHandler + Send + Sync>,
    openFilenames: Rc<RefCell<HashSet<String>>>,
    parsingChoice: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CustomFlags {
    ParsingString = 0x1,
    TagActive = 0x2,
}

impl InkParser {
    // C# signature: public InkParser(string str, string filenameForMetadata = null, Ink.ErrorHandler externalErrorHandler = null, IFileHandler fileHandler = null)
    pub fn new(
        str: String,
        filenameForMetadata: Option<String>,
        externalErrorHandler: Option<ErrorHandler>,
        fileHandler: Option<Arc<dyn IFileHandler + Send + Sync>>,
    ) -> Self {
        let processed = crate::InkParser::CommentEliminator::CommentEliminator::new(str)
            .Process()
            .unwrap_or_default();
        let parser = StringParser::new(processed);
        let fileHandler = fileHandler.unwrap_or_else(|| Arc::new(DefaultFileHandler));
        let openFilenames = Rc::new(RefCell::new(HashSet::new()));

        if let Some(filename) = filenameForMetadata.as_ref() {
            if let Ok(full_root_ink_path) = fileHandler.ResolveInkFilename(filename) {
                openFilenames.borrow_mut().insert(full_root_ink_path);
            }
        }

        Self {
            parser,
            filename: filenameForMetadata,
            externalErrorHandler,
            fileHandler,
            openFilenames,
            parsingChoice: false,
        }
    }

    // C# signature: public Parsed.Story Parse()
    pub fn Parse(&mut self) -> crate::ParsedHierarchy::Story::Story {
        let top_level_objects = self
            .StatementsAtLevel(crate::InkParser::InkParser_Statements::StatementLevel::Top)
            .unwrap_or_default();

        crate::ParsedHierarchy::Story::Story::new(top_level_objects, false)
    }

    // C# signature: protected List<T> SeparatedList<T> (SpecificParseRule<T> mainRule, ParseRule separatorRule)
    pub fn SeparatedList<T, MainRule, SeparatorRule>(
        &mut self,
        mut mainRule: MainRule,
        mut separatorRule: SeparatorRule,
    ) -> Option<Vec<T>>
    where
        MainRule: FnMut(&mut Self) -> Option<T>,
        SeparatorRule: FnMut(&mut Self) -> Option<()>,
        T: 'static,
    {
        let firstElement = self.ParseObject(&mut mainRule)?;
        let mut allElements = vec![firstElement];

        loop {
            let nextElementRuleId = self.parser.BeginRule();

            if separatorRule(self).is_none() {
                self.parser.CancelRule(nextElementRuleId);
                break;
            }

            match self.ParseObject(&mut mainRule) {
                Some(nextElement) => {
                    self.parser.SucceedRule(nextElementRuleId, ());
                    allElements.push(nextElement);
                }
                None => {
                    self.parser.CancelRule(nextElementRuleId);
                    break;
                }
            }
        }

        Some(allElements)
    }

    // C# signature: protected override string PreProcessInputString(string str)
    pub fn PreProcessInputString(&mut self, str: String) -> String {
        crate::InkParser::CommentEliminator::CommentEliminator::new(str)
            .Process()
            .unwrap_or_default()
    }

    // C# signature: protected Runtime.DebugMetadata CreateDebugMetadata(StringParserState.Element stateAtStart, StringParserState.Element stateAtEnd)
    pub fn CreateDebugMetadata(
        &mut self,
        stateAtStart: crate::StringParser::StringParserState::Element,
        stateAtEnd: crate::StringParser::StringParserState::Element,
    ) -> DebugMetadata {
        let mut md = DebugMetadata::new();
        md.startLineNumber = stateAtStart.lineIndex + 1;
        md.endLineNumber = stateAtEnd.lineIndex + 1;
        md.startCharacterNumber = stateAtStart.characterInLineIndex + 1;
        md.endCharacterNumber = stateAtEnd.characterInLineIndex + 1;
        md.fileName = self.filename.clone();
        md
    }

    // C# signature: protected override void RuleDidSucceed(object result, StringParserState.Element stateAtStart, StringParserState.Element stateAtEnd)
    pub fn RuleDidSucceed(
        &mut self,
        _result: crate::stub::PortStub,
        _stateAtStart: crate::StringParser::StringParserState::Element,
        _stateAtEnd: crate::StringParser::StringParserState::Element,
    ) {
    }

    // C# signature: bool parsingStringExpression { get; }
    pub fn get_parsingStringExpression(&mut self) -> bool {
        self.parser.GetFlag(CustomFlags::ParsingString as u32)
    }

    // C# signature: bool tagActive { get; }
    pub fn get_tagActive(&mut self) -> bool {
        self.parser.GetFlag(CustomFlags::TagActive as u32)
    }

    pub fn get_parsingChoice(&self) -> bool {
        self.parsingChoice
    }

    pub fn set_parsingChoice(&mut self, value: bool) {
        self.parsingChoice = value;
    }

    pub fn get_fileHandler(&self) -> &dyn IFileHandler {
        self.fileHandler.as_ref()
    }

    pub fn clone_fileHandler(&self) -> Arc<dyn IFileHandler + Send + Sync> {
        Arc::clone(&self.fileHandler)
    }

    pub fn ParseString(&mut self, str: String) -> Option<String> {
        self.parser.ParseString(str)
    }

    pub fn ParseNewline(&mut self) -> Option<String> {
        self.parser.ParseNewline()
    }

    pub fn ParseSingleCharacter(&mut self) -> char {
        self.parser.ParseSingleCharacter()
    }

    pub fn ParseCharactersFromCharSet(
        &mut self,
        charSet: CharacterSet,
        shouldIncludeChars: bool,
        maxCount: i32,
    ) -> Option<String> {
        self.parser
            .ParseCharactersFromCharSet(charSet, shouldIncludeChars, maxCount)
    }

    pub fn ParseInt(&mut self) -> Option<i32> {
        self.parser.ParseInt()
    }

    pub fn ParseFloat(&mut self) -> Option<f32> {
        self.parser.ParseFloat()
    }

    pub fn get_remainingString(&self) -> String {
        self.parser.get_remainingString()
    }

    pub fn ParseObject<T, R>(&mut self, mut rule: R) -> Option<T>
    where
        R: FnMut(&mut Self) -> Option<T>,
        T: 'static,
    {
        let rule_id = self.parser.BeginRule();
        let stack_height_before = self.parser.get_state().get_stackHeight();

        let result = rule(self);

        if stack_height_before != self.parser.get_state().get_stackHeight() {
            panic!("Mismatched Begin/Fail/Succeed rules");
        }

        match result {
            Some(result) => Some(self.parser.SucceedRule(rule_id, result)),
            None => self.parser.FailRule(rule_id),
        }
    }

    pub fn ParseRule<T, R>(&mut self, mut rule: R) -> Option<T>
    where
        R: FnMut(&mut Self) -> Option<T>,
        T: 'static,
    {
        self.ParseObject(&mut rule)
    }

    pub fn Peek<T, R>(&mut self, mut rule: R) -> Option<T>
    where
        R: FnMut(&mut Self) -> Option<T>,
        T: 'static,
    {
        let rule_id = self.parser.BeginRule();
        let result = rule(self);
        self.parser.CancelRule(rule_id);
        result
    }

    pub fn OneOf<T>(&mut self, mut array: Vec<Box<dyn FnMut(&mut Self) -> Option<T>>>) -> Option<T>
    where
        T: 'static,
    {
        for rule in array.iter_mut() {
            if let Some(result) = self.ParseObject(|parser| rule(parser)) {
                return Some(result);
            }
        }
        None
    }

    pub fn Optional<T, R>(mut rule: R) -> Box<dyn FnMut(&mut Self) -> Option<()> + 'static>
    where
        R: FnMut(&mut Self) -> Option<T> + 'static,
        T: 'static,
    {
        Box::new(move |parser| {
            let _ = parser.ParseObject(&mut rule);
            Some(())
        })
    }

    pub fn Exclude<T, R>(mut rule: R) -> Box<dyn FnMut(&mut Self) -> Option<()> + 'static>
    where
        R: FnMut(&mut Self) -> Option<T> + 'static,
        T: 'static,
    {
        Box::new(move |parser| {
            if parser.ParseObject(&mut rule).is_some() {
                Some(())
            } else {
                None
            }
        })
    }

    pub fn OptionalExclude<T, R>(mut rule: R) -> Box<dyn FnMut(&mut Self) -> Option<()> + 'static>
    where
        R: FnMut(&mut Self) -> Option<T> + 'static,
        T: 'static,
    {
        Box::new(move |parser| {
            let _ = parser.ParseObject(&mut rule);
            Some(())
        })
    }

    pub fn String(&mut self, str: String) -> Box<dyn FnMut(&mut Self) -> Option<String> + 'static> {
        Box::new(move |parser| parser.ParseString(str.clone()))
    }

    pub fn Interleave<T, RA, RB>(
        &mut self,
        mut ruleA: RA,
        mut ruleB: RB,
        mut untilTerminator: Option<Box<dyn FnMut(&mut Self) -> Option<T>>>,
        flatten: bool,
    ) -> Option<Vec<T>>
    where
        RA: FnMut(&mut Self) -> Option<T>,
        RB: FnMut(&mut Self) -> Option<T>,
        T: 'static,
    {
        let rule_id = self.parser.BeginRule();
        let mut results = Vec::new();

        let first_a = self.ParseObject(&mut ruleA)?;
        self.TryAddResultToList(Some(first_a), &mut results, flatten);

        loop {
            if let Some(ref mut until_rule) = untilTerminator {
                if self.Peek(|parser| until_rule(parser)).is_some() {
                    break;
                }
            }

            let Some(last_main_value) = self.ParseObject(&mut ruleB) else {
                break;
            };
            self.TryAddResultToList(Some(last_main_value), &mut results, flatten);

            let Some(outer_value) = self.ParseObject(&mut ruleA) else {
                break;
            };
            self.TryAddResultToList(Some(outer_value), &mut results, flatten);
        }

        if results.is_empty() {
            self.parser.FailRule(rule_id)
        } else {
            Some(self.parser.SucceedRule(rule_id, results))
        }
    }

    fn TryAddResultToList<T>(&mut self, result: Option<T>, list: &mut Vec<T>, _flatten: bool) {
        if let Some(result) = result {
            list.push(result);
        }
    }

    pub fn ParseCharactersFromString(&mut self, str: String, maxCount: i32) -> Option<String> {
        self.parser.ParseCharactersFromString(str, maxCount)
    }

    pub fn ParseUntilCharactersFromString(&mut self, str: String) -> Option<String> {
        self.parser.ParseUntilCharactersFromString(str, -1)
    }

    pub fn ParseUntil<T, R>(
        &mut self,
        stopRule: R,
        pauseCharacters: Option<CharacterSet>,
        endCharacters: Option<CharacterSet>,
    ) -> Option<String>
    where
        R: FnMut(&mut Self) -> Option<T>,
        T: 'static,
    {
        let mut stopRule = stopRule;
        let mut pause_and_end = CharacterSet::new();
        if let Some(pause) = pauseCharacters.as_ref() {
            pause_and_end.AddCharacters(pause.characters.iter().copied());
        }
        if let Some(end) = endCharacters.as_ref() {
            pause_and_end.AddCharacters(end.characters.iter().copied());
        }

        let mut parsed_string = String::new();

        loop {
            if let Some(partial) = self
                .parser
                .ParseUntilCharactersFromCharSet(pause_and_end.clone(), -1)
            {
                parsed_string.push_str(&partial);
            }

            if self.Peek(|parser| stopRule(parser)).is_some() {
                break;
            }

            if self.parser.get_endOfInput() {
                break;
            }

            let pause_character = self.parser.get_currentCharacter();
            if pauseCharacters
                .as_ref()
                .map_or(false, |set| set.Contains(pause_character))
            {
                parsed_string.push(self.ParseSingleCharacter());
                continue;
            } else {
                break;
            }
        }

        if parsed_string.is_empty() {
            None
        } else {
            Some(parsed_string)
        }
    }

    pub fn ParseUntilCharactersFromCharSet(
        &mut self,
        charSet: CharacterSet,
        maxCount: i32,
    ) -> Option<String> {
        self.parser
            .ParseUntilCharactersFromCharSet(charSet, maxCount)
    }

    pub fn LineRemainder(&mut self) -> String {
        self.parser.LineRemainder()
    }

    pub fn ExtendIdentifierCharacterRanges(&mut self, identifierCharSet: &mut CharacterSet) {
        for mut char_range in
            crate::InkParser::InkParser_CharacterRanges::InkParser::ListAllCharacterRanges()
        {
            let character_set = char_range.ToCharacterSet();
            identifierCharSet.AddCharacters(character_set.characters.iter().copied());
        }
    }

    pub fn get_endOfInput(&self) -> bool {
        self.parser.get_endOfInput()
    }

    pub fn get_currentCharacter(&self) -> char {
        self.parser.get_currentCharacter()
    }

    pub fn get_lineIndex(&self) -> i32 {
        self.parser.lineIndex()
    }

    pub fn get_characterInLineIndex(&self) -> i32 {
        self.parser.characterInLineIndex()
    }

    pub fn get_externalErrorHandler(&self) -> Option<ErrorHandler> {
        self.externalErrorHandler.clone()
    }

    pub fn Error(&mut self, message: String) {
        if let Some(handler) = &self.externalErrorHandler {
            handler(message, 0, 0, false);
        }
    }

    pub fn Warning(&mut self, message: String) {
        if let Some(handler) = &self.externalErrorHandler {
            handler(message, 0, 0, true);
        }
    }

    pub fn get_openFilenames(&self) -> HashSet<String> {
        self.openFilenames.borrow().clone()
    }

    pub fn clone_openFilenames(&self) -> Rc<RefCell<HashSet<String>>> {
        Rc::clone(&self.openFilenames)
    }

    pub fn set_openFilenames_shared(&mut self, openFilenames: Rc<RefCell<HashSet<String>>>) {
        self.openFilenames = openFilenames;
    }

    pub fn set_flag(&mut self, flag: CustomFlags, value: bool) {
        self.parser.SetFlag(flag as u32, value);
    }

    pub fn get_flag(&mut self, flag: CustomFlags) -> bool {
        self.parser.GetFlag(flag as u32)
    }

    pub fn parser_mut(&mut self) -> &mut StringParser {
        &mut self.parser
    }

    pub fn AddOpenFilenameShared(&mut self, fullFilename: String) {
        self.openFilenames.borrow_mut().insert(fullFilename);
    }

    pub fn RemoveOpenFilenameShared(&mut self, fullFilename: String) {
        self.openFilenames.borrow_mut().remove(&fullFilename);
    }
}

#[cfg(test)]
mod tests {
    use super::InkParser;
    use crate::StringParser::StringParserState::Element;

    #[test]
    fn preprocesses_comments_and_builds_debug_metadata() {
        let mut parser = InkParser::new(
            "one // comment\n two".to_string(),
            Some("story.ink".to_string()),
            None,
            None,
        );

        assert_eq!(
            parser.PreProcessInputString("a/*x\n*/b".to_string()),
            "a\nb"
        );

        let md = parser.CreateDebugMetadata(
            Element {
                lineIndex: 1,
                characterInLineIndex: 2,
                ..Default::default()
            },
            Element {
                lineIndex: 3,
                characterInLineIndex: 4,
                ..Default::default()
            },
        );

        assert_eq!(md.startLineNumber, 2);
        assert_eq!(md.endLineNumber, 4);
        assert_eq!(md.startCharacterNumber, 3);
        assert_eq!(md.endCharacterNumber, 5);
        assert_eq!(md.fileName.as_deref(), Some("story.ink"));

        // Keep the constructed parser live to ensure the constructor path stays valid.
        assert!(parser.get_openFilenames().len() <= 1);
    }
}
