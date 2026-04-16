// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/StringParser/StringParserState.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct StringParserState {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct Element {
    pub _port_marker: (),
}

impl StringParserState {
    // C# signature: public StringParserState ()
    pub fn new() -> Self {
        Default::default()
    }

    // C# signature: public void CopyFrom(Element fromElement)
    pub fn CopyFrom(&mut self, _fromElement: crate::stub::Element) {}

    // C# signature: public void SquashFrom(Element fromElement)
    pub fn SquashFrom(&mut self, _fromElement: crate::stub::Element) {}

    // C# signature: public int Push()
    pub fn Push(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: public void Pop(int expectedRuleId)
    pub fn Pop(&mut self, _expectedRuleId: i32) {}

    // C# signature: public Element Peek(int expectedRuleId)
    pub fn Peek(&mut self, _expectedRuleId: i32) -> crate::stub::Element {
        Default::default()
    }

    // C# signature: public Element PeekPenultimate()
    pub fn PeekPenultimate(&mut self) -> crate::stub::Element {
        Default::default()
    }

    // C# signature: public void Squash()
    pub fn Squash(&mut self) {}

    // C# signature: public void NoteErrorReported()
    pub fn NoteErrorReported(&mut self) {}

    // C# signature: int lineIndex { get; }
    pub fn get_lineIndex(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: int characterIndex { get; }
    pub fn get_characterIndex(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: int characterInLineIndex { get; }
    pub fn get_characterInLineIndex(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: uint customFlags { get; }
    pub fn get_customFlags(&mut self) -> u32 {
        Default::default()
    }

    // C# signature: bool errorReportedAlreadyInScope { get; }
    pub fn get_errorReportedAlreadyInScope(&mut self) -> bool {
        Default::default()
    }

    // C# signature: int stackHeight { get; }
    pub fn get_stackHeight(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: Element currentElement { get; }
    pub fn get_currentElement(&mut self) -> crate::stub::Element {
        Default::default()
    }
}
