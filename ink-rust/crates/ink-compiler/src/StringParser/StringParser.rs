// Source: ink-c-sharp/compiler/StringParser/StringParser.cs

use super::StringParserState::{Element, StringParserState};
use crate::CharacterSet::CharacterSet;
use std::sync::Arc;

pub type ErrorHandler = Arc<dyn Fn(String, i32, i32, bool) + Send + Sync>;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParseSuccessStruct;

pub const ParseSuccess: ParseSuccessStruct = ParseSuccessStruct;

#[derive(Clone)]
pub struct StringParser {
    chars: Vec<char>,
    state: StringParserState,
    inputString: String,
    hadError: bool,
    errorHandler: Option<ErrorHandler>,
}

impl StringParser {
    // C# signature: public StringParser (string str)
    pub fn new(str: String) -> Self {
        let str = Self::preprocess_input_string_static(str);
        let chars = str.chars().collect::<Vec<_>>();

        Self {
            chars,
            state: StringParserState::new(),
            inputString: str,
            hadError: false,
            errorHandler: None,
        }
    }

    fn preprocess_input_string_static(str: String) -> String {
        str
    }

    // C# signature: protected virtual string PreProcessInputString(string str)
    pub fn PreProcessInputString(&mut self, str: String) -> String {
        str
    }

    // C# signature: protected int BeginRule()
    pub fn BeginRule(&mut self) -> i32 {
        self.state.Push().expect("Stack overflow in parser state")
    }

    // C# signature: protected object FailRule(int expectedRuleId)
    pub fn FailRule<T>(&mut self, expectedRuleId: i32) -> Option<T> {
        self.state
            .Pop(expectedRuleId)
            .expect("Mismatched Begin/Succeed/Fail rules");
        None
    }

    // C# signature: protected void CancelRule(int expectedRuleId)
    pub fn CancelRule(&mut self, expectedRuleId: i32) {
        self.state
            .Pop(expectedRuleId)
            .expect("Mismatched Begin/Succeed/Fail rules");
    }

    // C# signature: protected object SucceedRule(int expectedRuleId, object result = null)
    pub fn SucceedRule<T: std::any::Any + 'static>(&mut self, expectedRuleId: i32, result: T) -> T {
        let start_state = self
            .state
            .PeekPenultimate()
            .expect("Missing begin rule state")
            .clone();
        let end_state = self
            .state
            .Peek(expectedRuleId)
            .expect("Mismatched Begin/Succeed/Fail rules")
            .clone();

        self.RuleDidSucceed(&result, start_state, end_state);
        self.state
            .Squash()
            .expect("Mismatched Begin/Succeed/Fail rules");
        result
    }

    // C# signature: protected virtual void RuleDidSucceed(object result, StringParserState.Element startState, StringParserState.Element endState)
    pub fn RuleDidSucceed(
        &mut self,
        _result: &dyn std::any::Any,
        _startState: Element,
        _endState: Element,
    ) {
    }

    // C# signature: protected object Expect(ParseRule rule, string message = null, ParseRule recoveryRule = null)
    pub fn Expect<T, R>(
        &mut self,
        mut rule: R,
        message: Option<String>,
        mut recoveryRule: Option<Box<dyn FnMut(&mut Self) -> Option<T>>>,
    ) -> Option<T>
    where
        R: FnMut(&mut Self) -> Option<T>,
        T: 'static,
    {
        let result = self.ParseObject(&mut rule);
        if result.is_none() {
            let message = message.unwrap_or_else(|| "parse rule".to_string());
            let line_remainder = self.LineRemainder();
            let but_saw = if line_remainder.is_empty() {
                "end of line".to_string()
            } else {
                format!("'{}'", line_remainder)
            };

            self.Error(format!("Expected {} but saw {}", message, but_saw), false);

            if let Some(ref mut recovery_rule) = recoveryRule {
                return recovery_rule(self);
            }
        }

        result
    }

    // C# signature: protected void Error(string message, bool isWarning = false)
    pub fn Error(&mut self, message: String, isWarning: bool) {
        self.ErrorOnLine(message, self.lineIndex() + 1, isWarning);
    }

    // C# signature: protected void ErrorWithParsedObject(string message, Parsed.Object result, bool isWarning = false)
    pub fn ErrorWithParsedObject(&mut self, message: String, _result: (), isWarning: bool) {
        self.ErrorOnLine(message, self.lineIndex() + 1, isWarning);
    }

    // C# signature: protected void ErrorOnLine(string message, int lineNumber, bool isWarning)
    pub fn ErrorOnLine(&mut self, message: String, lineNumber: i32, isWarning: bool) {
        if !self.state.get_errorReportedAlreadyInScope() {
            if let Some(handler) = &self.errorHandler {
                handler(message, self.index(), lineNumber - 1, isWarning);
            } else {
                let error_type = if isWarning { "Warning" } else { "Error" };
                panic!("{} on line {}: {}", error_type, lineNumber, message);
            }
            self.state.NoteErrorReported();
        }

        if !isWarning {
            self.hadError = true;
        }
    }

    // C# signature: protected void Warning(string message)
    pub fn Warning(&mut self, message: String) {
        self.Error(message, true);
    }

    // C# signature: public string LineRemainder()
    pub fn LineRemainder(&mut self) -> String {
        self.Peek(|parser| parser.ParseUntilCharactersFromString("\n\r".to_string(), -1))
            .unwrap_or_default()
    }

    // C# signature: public void SetFlag(uint flag, bool trueOrFalse)
    pub fn SetFlag(&mut self, flag: u32, trueOrFalse: bool) {
        self.state.set_customFlags(if trueOrFalse {
            self.state.get_customFlags() | flag
        } else {
            self.state.get_customFlags() & !flag
        });
    }

    // C# signature: public bool GetFlag(uint flag)
    pub fn GetFlag(&mut self, flag: u32) -> bool {
        (self.state.get_customFlags() & flag) != 0
    }

    // C# signature: public object ParseObject(ParseRule rule)
    pub fn ParseObject<T, R>(&mut self, mut rule: R) -> Option<T>
    where
        R: FnMut(&mut Self) -> Option<T>,
        T: std::any::Any + 'static,
    {
        let rule_id = self.BeginRule();
        let stack_height_before = self.state.get_stackHeight();

        let result = rule(self);

        if stack_height_before != self.state.get_stackHeight() {
            panic!("Mismatched Begin/Fail/Succeed rules");
        }

        match result {
            Some(result) => Some(self.SucceedRule(rule_id, result)),
            None => self.FailRule(rule_id),
        }
    }

    // C# signature: public T Parse<T>(SpecificParseRule<T> rule)
    pub fn Parse<T, R>(&mut self, mut rule: R) -> Option<T>
    where
        R: FnMut(&mut Self) -> Option<T>,
        T: std::any::Any + 'static,
    {
        self.ParseObject(&mut rule)
    }

    // C# signature: public object OneOf(params ParseRule[] array)
    pub fn OneOf<T>(&mut self, mut array: Vec<Box<dyn FnMut(&mut Self) -> Option<T>>>) -> Option<T>
    where
        T: std::any::Any + 'static,
    {
        for rule in array.iter_mut() {
            if let Some(result) = self.ParseObject(|parser| rule(parser)) {
                return Some(result);
            }
        }
        None
    }

    // C# signature: public List<object> OneOrMore(ParseRule rule)
    pub fn OneOrMore<T, R>(&mut self, mut rule: R) -> Option<Vec<T>>
    where
        R: FnMut(&mut Self) -> Option<T>,
        T: std::any::Any + 'static,
    {
        let mut results = Vec::new();

        loop {
            let result = self.ParseObject(&mut rule);
            match result {
                Some(result) => results.push(result),
                None => break,
            }
        }

        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }

    // C# signature: public ParseRule Optional(ParseRule rule)
    pub fn Optional<T, R>(mut rule: R) -> impl FnMut(&mut Self) -> Option<()>
    where
        R: FnMut(&mut Self) -> Option<T> + 'static,
        T: std::any::Any + 'static,
    {
        move |parser| {
            let _ = parser.ParseObject(&mut rule);
            Some(())
        }
    }

    // C# signature: public ParseRule Exclude(ParseRule rule)
    pub fn Exclude<T, R>(mut rule: R) -> impl FnMut(&mut Self) -> Option<()>
    where
        R: FnMut(&mut Self) -> Option<T> + 'static,
        T: std::any::Any + 'static,
    {
        move |parser| {
            if parser.ParseObject(&mut rule).is_some() {
                Some(())
            } else {
                None
            }
        }
    }

    // C# signature: public ParseRule OptionalExclude(ParseRule rule)
    pub fn OptionalExclude<T, R>(mut rule: R) -> impl FnMut(&mut Self) -> Option<()>
    where
        R: FnMut(&mut Self) -> Option<T> + 'static,
        T: std::any::Any + 'static,
    {
        move |parser| {
            let _ = parser.ParseObject(&mut rule);
            Some(())
        }
    }

    // C# signature: protected ParseRule String(string str)
    pub fn String(&mut self, str: String) -> impl FnMut(&mut Self) -> Option<String> {
        move |parser| parser.ParseString(str.clone())
    }

    // C# signature: private void TryAddResultToList<T>(object result, List<T> list, bool flatten = true)
    fn TryAddResultToList<T>(&mut self, result: Option<T>, list: &mut Vec<T>, _flatten: bool) {
        if let Some(result) = result {
            list.push(result);
        }
    }

    // C# signature: public List<T> Interleave<T>(ParseRule ruleA, ParseRule ruleB, ParseRule untilTerminator = null, bool flatten = true)
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
        T: std::any::Any + 'static,
    {
        let rule_id = self.BeginRule();
        let mut results = Vec::new();

        let first_a = self.ParseObject(&mut ruleA);
        let Some(first_a) = first_a else {
            return self.FailRule(rule_id);
        };
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
            self.FailRule(rule_id)
        } else {
            Some(self.SucceedRule(rule_id, results))
        }
    }

    // C# signature: public string ParseString(string str)
    pub fn ParseString(&mut self, str: String) -> Option<String> {
        if str.chars().count() > self.get_remainingLength() as usize {
            return None;
        }

        let rule_id = self.BeginRule();

        let mut i = self.index() as usize;
        let mut cli = self.characterInLineIndex();
        let mut li = self.lineIndex();

        let mut success = true;
        for c in str.chars() {
            if self.chars.get(i).copied() != Some(c) {
                success = false;
                break;
            }
            if c == '\n' {
                li += 1;
                cli = -1;
            }
            i += 1;
            cli += 1;
        }

        self.state.set_characterIndex(i as i32);
        self.state.set_characterInLineIndex(cli);
        self.state.set_lineIndex(li);

        if success {
            Some(self.SucceedRule(rule_id, str))
        } else {
            self.FailRule(rule_id)
        }
    }

    // C# signature: public char ParseSingleCharacter()
    pub fn ParseSingleCharacter(&mut self) -> char {
        if self.get_remainingLength() > 0 {
            let c = self.chars[self.index() as usize];
            if c == '\n' {
                self.state.set_lineIndex(self.lineIndex() + 1);
                self.state.set_characterInLineIndex(-1);
            }
            self.state.set_characterIndex(self.index() + 1);
            self.state
                .set_characterInLineIndex(self.characterInLineIndex() + 1);
            c
        } else {
            '\0'
        }
    }

    // C# signature: public string ParseUntilCharactersFromString(string str, int maxCount = -1)
    pub fn ParseUntilCharactersFromString(&mut self, str: String, maxCount: i32) -> Option<String> {
        self.ParseCharactersFromString_overload_2(str, false, maxCount)
    }

    // C# signature: public string ParseUntilCharactersFromCharSet(CharacterSet charSet, int maxCount = -1)
    pub fn ParseUntilCharactersFromCharSet(
        &mut self,
        charSet: CharacterSet,
        maxCount: i32,
    ) -> Option<String> {
        self.ParseCharactersFromCharSet(charSet, false, maxCount)
    }

    // C# signature: public string ParseCharactersFromString(string str, int maxCount = -1)
    pub fn ParseCharactersFromString(&mut self, str: String, maxCount: i32) -> Option<String> {
        self.ParseCharactersFromString_overload_2(str, true, maxCount)
    }

    // C# signature: public string ParseCharactersFromString(string str, bool shouldIncludeStrChars, int maxCount = -1)
    pub fn ParseCharactersFromString_overload_2(
        &mut self,
        str: String,
        shouldIncludeStrChars: bool,
        maxCount: i32,
    ) -> Option<String> {
        self.ParseCharactersFromCharSet(
            CharacterSet::new_overload_2(str),
            shouldIncludeStrChars,
            maxCount,
        )
    }

    // C# signature: public string ParseCharactersFromCharSet(CharacterSet charSet, bool shouldIncludeChars = true, int maxCount = -1)
    pub fn ParseCharactersFromCharSet(
        &mut self,
        charSet: CharacterSet,
        shouldIncludeChars: bool,
        maxCount: i32,
    ) -> Option<String> {
        let maxCount = if maxCount == -1 { i32::MAX } else { maxCount };
        let start_index = self.index();
        let mut i = self.index() as usize;
        let mut cli = self.characterInLineIndex();
        let mut li = self.lineIndex();
        let mut count = 0;

        while i < self.chars.len()
            && charSet.Contains(self.chars[i]) == shouldIncludeChars
            && count < maxCount
        {
            if self.chars[i] == '\n' {
                li += 1;
                cli = -1;
            }
            i += 1;
            cli += 1;
            count += 1;
        }

        self.state.set_characterIndex(i as i32);
        self.state.set_characterInLineIndex(cli);
        self.state.set_lineIndex(li);

        if i > start_index as usize {
            Some(self.chars[start_index as usize..i].iter().collect())
        } else {
            None
        }
    }

    // C# signature: public object Peek(ParseRule rule)
    pub fn Peek<T, R>(&mut self, mut rule: R) -> Option<T>
    where
        R: FnMut(&mut Self) -> Option<T>,
        T: std::any::Any + 'static,
    {
        let rule_id = self.BeginRule();
        let result = rule(self);
        self.CancelRule(rule_id);
        result
    }

    // C# signature: public string ParseUntil(ParseRule stopRule, CharacterSet pauseCharacters = null, CharacterSet endCharacters = null)
    pub fn ParseUntil<T, R>(
        &mut self,
        mut stopRule: R,
        pauseCharacters: Option<CharacterSet>,
        endCharacters: Option<CharacterSet>,
    ) -> Option<String>
    where
        R: FnMut(&mut Self) -> Option<T>,
        T: std::any::Any + 'static,
    {
        let rule_id = self.BeginRule();

        let mut pause_and_end = CharacterSet::new();
        if let Some(pause) = pauseCharacters.as_ref() {
            pause_and_end.AddCharacters(pause.characters.iter().copied());
        }
        if let Some(end) = endCharacters.as_ref() {
            pause_and_end.AddCharacters(end.characters.iter().copied());
        }

        let mut parsed_string = String::new();

        loop {
            if let Some(partial) = self.ParseUntilCharactersFromCharSet(pause_and_end.clone(), -1) {
                parsed_string.push_str(&partial);
            }

            if self.Peek(|parser| stopRule(parser)).is_some() {
                break;
            }

            if self.get_endOfInput() {
                break;
            }

            let pause_character = self.get_currentCharacter();
            if pauseCharacters
                .as_ref()
                .map_or(false, |set| set.Contains(pause_character))
            {
                parsed_string.push(pause_character);
                if pause_character == '\n' {
                    self.state.set_lineIndex(self.lineIndex() + 1);
                    self.state.set_characterInLineIndex(-1);
                }
                self.state.set_characterIndex(self.index() + 1);
                self.state
                    .set_characterInLineIndex(self.characterInLineIndex() + 1);
                continue;
            } else {
                break;
            }
        }

        if parsed_string.is_empty() {
            self.FailRule(rule_id)
        } else {
            Some(self.SucceedRule(rule_id, parsed_string))
        }
    }

    // C# signature: public int? ParseInt()
    pub fn ParseInt(&mut self) -> Option<i32> {
        let rule_id = self.BeginRule();

        let negative = self.ParseString("-".to_string()).is_some();
        let _ = self.ParseCharactersFromString(" \t".to_string(), -1);
        let parsed_string = self.ParseCharactersFromCharSet(
            CharacterSet::new_overload_2("0123456789".to_string()),
            true,
            -1,
        );

        let Some(parsed_string) = parsed_string else {
            return self.FailRule(rule_id);
        };

        match parsed_string.parse::<i32>() {
            Ok(parsed_int) => {
                Some(self.SucceedRule(rule_id, if negative { -parsed_int } else { parsed_int }))
            }
            Err(_) => {
                self.Error(
                    format!(
                        "Failed to read integer value: {}. Perhaps it's out of the range of acceptable numbers ink supports? ({} to {})",
                        parsed_string,
                        i32::MIN,
                        i32::MAX
                    ),
                    false,
                );
                self.FailRule(rule_id)
            }
        }
    }

    // C# signature: public float? ParseFloat()
    pub fn ParseFloat(&mut self) -> Option<f32> {
        let rule_id = self.BeginRule();

        let old_index = self.index();
        let old_cli = self.characterInLineIndex();

        let leading_int = self.ParseInt();
        if let Some(leading_int) = leading_int {
            if self.ParseString(".".to_string()).is_some() {
                let after_decimal = self
                    .ParseCharactersFromCharSet(
                        CharacterSet::new_overload_2("0123456789".to_string()),
                        true,
                        -1,
                    )
                    .unwrap_or_default();
                let parsed = format!("{}.{}", leading_int, after_decimal)
                    .parse::<f32>()
                    .unwrap();
                return Some(self.SucceedRule(rule_id, parsed));
            }
        }

        self.state.set_characterIndex(old_index);
        self.state.set_characterInLineIndex(old_cli);
        self.FailRule(rule_id)
    }

    // C# signature: protected string ParseNewline()
    pub fn ParseNewline(&mut self) -> Option<String> {
        let rule_id = self.BeginRule();
        let _ = self.ParseString("\r".to_string());

        if self.ParseString("\n".to_string()).is_none() {
            self.FailRule(rule_id)
        } else {
            Some(self.SucceedRule(rule_id, "\n".to_string()))
        }
    }

    // C# signature: ErrorHandler errorHandler { get; }
    pub fn get_errorHandler(&self) -> Option<ErrorHandler> {
        self.errorHandler.clone()
    }

    // C# signature: char currentCharacter { get; }
    pub fn get_currentCharacter(&self) -> char {
        if self.index() >= 0 && self.get_remainingLength() > 0 {
            self.chars[self.index() as usize]
        } else {
            '\0'
        }
    }

    // C# signature: StringParserState state { get; }
    pub fn get_state(&self) -> &StringParserState {
        &self.state
    }

    // C# signature: bool hadError { get; }
    pub fn get_hadError(&self) -> bool {
        self.hadError
    }

    // C# signature: bool endOfInput { get; }
    pub fn get_endOfInput(&self) -> bool {
        self.index() as usize >= self.chars.len()
    }

    // C# signature: string remainingString { get; }
    pub fn get_remainingString(&self) -> String {
        if self.index() as usize >= self.chars.len() {
            String::new()
        } else {
            self.chars[self.index() as usize..].iter().collect()
        }
    }

    // C# signature: int remainingLength { get; }
    pub fn get_remainingLength(&self) -> i32 {
        self.chars.len() as i32 - self.index()
    }

    // C# signature: string inputString { get; }
    pub fn get_inputString(&self) -> String {
        self.inputString.clone()
    }

    // C# signature: int lineIndex { get; }
    pub fn lineIndex(&self) -> i32 {
        self.state.get_lineIndex()
    }

    // C# signature: int characterInLineIndex { get; }
    pub fn characterInLineIndex(&self) -> i32 {
        self.state.get_characterInLineIndex()
    }

    // C# signature: int index { get; }
    pub fn index(&self) -> i32 {
        self.state.get_characterIndex()
    }
}

#[cfg(test)]
mod tests {
    use super::StringParser;
    use crate::CharacterSet::CharacterSet;

    #[test]
    fn parses_strings_and_tracks_line_state() {
        let mut parser = StringParser::new("hello\nworld".to_string());
        assert_eq!(
            parser.ParseString("hello".to_string()),
            Some("hello".to_string())
        );
        assert_eq!(parser.get_currentCharacter(), '\n');
        assert_eq!(parser.ParseNewline(), Some("\n".to_string()));
        assert_eq!(
            parser.ParseString("world".to_string()),
            Some("world".to_string())
        );
        assert!(parser.get_endOfInput());
    }

    #[test]
    fn parses_numbers_and_rolls_back_on_failure() {
        let mut parser = StringParser::new("12.5 rest".to_string());
        assert_eq!(parser.ParseFloat(), Some(12.5));
        assert_eq!(
            parser.ParseCharactersFromString(" ".to_string(), -1),
            Some(" ".to_string())
        );
        assert_eq!(
            parser.ParseCharactersFromCharSet(
                CharacterSet::new_overload_2("rest".to_string()),
                true,
                -1
            ),
            Some("rest".to_string())
        );
    }

    #[test]
    fn parse_until_stops_at_pause_character() {
        let mut parser = StringParser::new("foo,bar".to_string());
        let parsed = parser
            .ParseUntil(
                |p| p.ParseString(",".to_string()),
                Some(CharacterSet::new_overload_2(",".to_string())),
                None,
            )
            .unwrap();

        assert_eq!(parsed, "foo");
        assert_eq!(parser.get_currentCharacter(), ',');
    }
}
