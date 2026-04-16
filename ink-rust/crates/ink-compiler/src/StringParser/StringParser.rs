// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/StringParser/StringParser.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct StringParser {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct ParseSuccessStruct {
    pub _port_marker: (),
}

impl StringParser {
    // C# signature: public StringParser (string str)
    pub fn new(_str: String) -> Self {
        Default::default()
    }

    // C# signature: protected virtual string PreProcessInputString(string str)
    pub fn PreProcessInputString(&mut self, _str: String) -> String {
        Default::default()
    }

    // C# signature: protected int BeginRule()
    pub fn BeginRule(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: protected object FailRule(int expectedRuleId)
    pub fn FailRule(&mut self, _expectedRuleId: i32) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected void CancelRule(int expectedRuleId)
    pub fn CancelRule(&mut self, _expectedRuleId: i32) {}

    // C# signature: protected object SucceedRule(int expectedRuleId, object result = null)
    pub fn SucceedRule(
        &mut self,
        _expectedRuleId: i32,
        _result: crate::stub::PortStub,
    ) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected virtual void RuleDidSucceed(object result, StringParserState.Element startState, StringParserState.Element endState)
    pub fn RuleDidSucceed(
        &mut self,
        _result: crate::stub::PortStub,
        _startState: crate::stub::Element,
        _endState: crate::stub::Element,
    ) {
    }

    // C# signature: protected object Expect(ParseRule rule, string message = null, ParseRule recoveryRule = null)
    pub fn Expect(
        &mut self,
        _rule: crate::stub::ParseRule,
        _message: String,
        _recoveryRule: crate::stub::ParseRule,
    ) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected void Error(string message, bool isWarning = false)
    pub fn Error(&mut self, _message: String, _isWarning: bool) {}

    // C# signature: protected void ErrorWithParsedObject(string message, Parsed.Object result, bool isWarning = false)
    pub fn ErrorWithParsedObject(
        &mut self,
        _message: String,
        _result: crate::stub::PortStub,
        _isWarning: bool,
    ) {
    }

    // C# signature: protected void ErrorOnLine(string message, int lineNumber, bool isWarning)
    pub fn ErrorOnLine(&mut self, _message: String, _lineNumber: i32, _isWarning: bool) {}

    // C# signature: protected void Warning(string message)
    pub fn Warning(&mut self, _message: String) {}

    // C# signature: public string LineRemainder()
    pub fn LineRemainder(&mut self) -> String {
        Default::default()
    }

    // C# signature: public void SetFlag(uint flag, bool trueOrFalse)
    pub fn SetFlag(&mut self, _flag: u32, _trueOrFalse: bool) {}

    // C# signature: public bool GetFlag(uint flag)
    pub fn GetFlag(&mut self, _flag: u32) -> bool {
        Default::default()
    }

    // C# signature: public object ParseObject(ParseRule rule)
    pub fn ParseObject(&mut self, _rule: crate::stub::ParseRule) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public T Parse<T>(SpecificParseRule<T> rule)
    pub fn Parse(&mut self, _rule: crate::stub::PortStub) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public object OneOf(params ParseRule[] array)
    pub fn OneOf(&mut self, _array: Vec<crate::stub::ParseRule>) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public List<object> OneOrMore(ParseRule rule)
    pub fn OneOrMore(&mut self, _rule: crate::stub::ParseRule) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: public ParseRule Optional(ParseRule rule)
    pub fn Optional(&mut self, _rule: crate::stub::ParseRule) -> crate::stub::ParseRule {
        Default::default()
    }

    // C# signature: public ParseRule Exclude(ParseRule rule)
    pub fn Exclude(&mut self, _rule: crate::stub::ParseRule) -> crate::stub::ParseRule {
        Default::default()
    }

    // C# signature: public ParseRule OptionalExclude(ParseRule rule)
    pub fn OptionalExclude(&mut self, _rule: crate::stub::ParseRule) -> crate::stub::ParseRule {
        Default::default()
    }

    // C# signature: protected ParseRule String(string str)
    pub fn String(&mut self, _str: String) -> crate::stub::ParseRule {
        Default::default()
    }

    // C# signature: private void TryAddResultToList<T>(object result, List<T> list, bool flatten = true)
    pub fn TryAddResultToList(
        &mut self,
        _result: crate::stub::PortStub,
        _list: Vec<crate::stub::PortStub>,
        _flatten: bool,
    ) {
    }

    // C# signature: public List<T> Interleave<T>(ParseRule ruleA, ParseRule ruleB, ParseRule untilTerminator = null, bool flatten = true)
    pub fn Interleave(
        &mut self,
        _ruleA: crate::stub::ParseRule,
        _ruleB: crate::stub::ParseRule,
        _untilTerminator: crate::stub::ParseRule,
        _flatten: bool,
    ) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: public string ParseString(string str)
    pub fn ParseString(&mut self, _str: String) -> String {
        Default::default()
    }

    // C# signature: public char ParseSingleCharacter()
    pub fn ParseSingleCharacter(&mut self) -> char {
        Default::default()
    }

    // C# signature: public string ParseUntilCharactersFromString(string str, int maxCount = -1)
    pub fn ParseUntilCharactersFromString(&mut self, _str: String, _maxCount: i32) -> String {
        Default::default()
    }

    // C# signature: public string ParseUntilCharactersFromCharSet(CharacterSet charSet, int maxCount = -1)
    pub fn ParseUntilCharactersFromCharSet(
        &mut self,
        _charSet: crate::stub::CharacterSet,
        _maxCount: i32,
    ) -> String {
        Default::default()
    }

    // C# signature: public string ParseCharactersFromString(string str, int maxCount = -1)
    pub fn ParseCharactersFromString(&mut self, _str: String, _maxCount: i32) -> String {
        Default::default()
    }

    // C# signature: public string ParseCharactersFromString(string str, bool shouldIncludeStrChars, int maxCount = -1)
    pub fn ParseCharactersFromString_overload_2(
        &mut self,
        _str: String,
        _shouldIncludeStrChars: bool,
        _maxCount: i32,
    ) -> String {
        Default::default()
    }

    // C# signature: public string ParseCharactersFromCharSet(CharacterSet charSet, bool shouldIncludeChars = true, int maxCount = -1)
    pub fn ParseCharactersFromCharSet(
        &mut self,
        _charSet: crate::stub::CharacterSet,
        _shouldIncludeChars: bool,
        _maxCount: i32,
    ) -> String {
        Default::default()
    }

    // C# signature: public object Peek(ParseRule rule)
    pub fn Peek(&mut self, _rule: crate::stub::ParseRule) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public string ParseUntil(ParseRule stopRule, CharacterSet pauseCharacters = null, CharacterSet endCharacters = null)
    pub fn ParseUntil(
        &mut self,
        _stopRule: crate::stub::ParseRule,
        _pauseCharacters: crate::stub::CharacterSet,
        _endCharacters: crate::stub::CharacterSet,
    ) -> String {
        Default::default()
    }

    // C# signature: public int? ParseInt()
    pub fn ParseInt(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: public float? ParseFloat()
    pub fn ParseFloat(&mut self) -> f32 {
        Default::default()
    }

    // C# signature: protected string ParseNewline()
    pub fn ParseNewline(&mut self) -> String {
        Default::default()
    }

    // C# signature: ErrorHandler errorHandler { get; }
    pub fn get_errorHandler(&mut self) -> crate::stub::ErrorHandler {
        Default::default()
    }

    // C# signature: char currentCharacter { get; }
    pub fn get_currentCharacter(&mut self) -> char {
        Default::default()
    }

    // C# signature: StringParserState state { get; }
    pub fn get_state(&mut self) -> crate::stub::StringParserState {
        Default::default()
    }

    // C# signature: bool hadError { get; }
    pub fn get_hadError(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool endOfInput { get; }
    pub fn get_endOfInput(&mut self) -> bool {
        Default::default()
    }

    // C# signature: string remainingString { get; }
    pub fn get_remainingString(&mut self) -> String {
        Default::default()
    }

    // C# signature: int remainingLength { get; }
    pub fn get_remainingLength(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: string inputString { get; }
    pub fn get_inputString(&mut self) -> String {
        Default::default()
    }

    // C# signature: int lineIndex { get; }
    pub fn get_lineIndex(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: int characterInLineIndex { get; }
    pub fn get_characterInLineIndex(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: int index { get; }
    pub fn get_index(&mut self) -> i32 {
        Default::default()
    }
}
