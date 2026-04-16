// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/InkParser/InkParser.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct InkParser {
    pub _port_marker: (),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CustomFlags {
    PortPlaceholder,
}

impl Default for CustomFlags {
    fn default() -> Self {
        Self::PortPlaceholder
    }
}

impl InkParser {
    // C# signature: public InkParser(string str, string filenameForMetadata = null, Ink.ErrorHandler externalErrorHandler = null, IFileHandler fileHandler = null)
    pub fn new(
        _str: String,
        _filenameForMetadata: String,
        _externalErrorHandler: crate::stub::ErrorHandler,
        _fileHandler: crate::stub::IFileHandler,
    ) -> Self {
        Default::default()
    }

    // C# signature: public Parsed.Story Parse()
    pub fn Parse(&mut self) -> crate::stub::Story {
        Default::default()
    }

    // C# signature: protected List<T> SeparatedList<T> (SpecificParseRule<T> mainRule, ParseRule separatorRule)
    pub fn SeparatedList(
        &mut self,
        _mainRule: crate::stub::PortStub,
        _separatorRule: crate::stub::ParseRule,
    ) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: protected override string PreProcessInputString(string str)
    pub fn PreProcessInputString(&mut self, _str: String) -> String {
        Default::default()
    }

    // C# signature: protected Runtime.DebugMetadata CreateDebugMetadata(StringParserState.Element stateAtStart, StringParserState.Element stateAtEnd)
    pub fn CreateDebugMetadata(
        &mut self,
        _stateAtStart: crate::stub::Element,
        _stateAtEnd: crate::stub::Element,
    ) -> crate::stub::DebugMetadata {
        Default::default()
    }

    // C# signature: protected override void RuleDidSucceed(object result, StringParserState.Element stateAtStart, StringParserState.Element stateAtEnd)
    pub fn RuleDidSucceed(
        &mut self,
        _result: crate::stub::PortStub,
        _stateAtStart: crate::stub::Element,
        _stateAtEnd: crate::stub::Element,
    ) {
    }

    // C# signature: bool parsingStringExpression { get; }
    pub fn get_parsingStringExpression(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool tagActive { get; }
    pub fn get_tagActive(&mut self) -> bool {
        Default::default()
    }
}
