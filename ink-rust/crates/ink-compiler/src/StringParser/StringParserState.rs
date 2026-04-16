// Source: ink-c-sharp/compiler/StringParser/StringParserState.cs

use std::sync::atomic::{AtomicI32, Ordering};

const EXPECTED_MAX_STACK_DEPTH: usize = 200;
static UNIQUE_ID_COUNTER: AtomicI32 = AtomicI32::new(0);

#[derive(Clone, Debug)]
pub struct StringParserState {
    stack: Vec<Element>,
    numElements: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Element {
    pub characterIndex: i32,
    pub characterInLineIndex: i32,
    pub lineIndex: i32,
    pub reportedErrorInScope: bool,
    pub uniqueId: i32,
    pub customFlags: u32,
}

impl Element {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public void CopyFrom(Element fromElement)
    pub fn CopyFrom(&mut self, fromElement: &Element) {
        self.uniqueId = UNIQUE_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
        self.characterIndex = fromElement.characterIndex;
        self.characterInLineIndex = fromElement.characterInLineIndex;
        self.lineIndex = fromElement.lineIndex;
        self.customFlags = fromElement.customFlags;
        self.reportedErrorInScope = false;
    }

    // C# signature: public void SquashFrom(Element fromElement)
    pub fn SquashFrom(&mut self, fromElement: &Element) {
        self.characterIndex = fromElement.characterIndex;
        self.characterInLineIndex = fromElement.characterInLineIndex;
        self.lineIndex = fromElement.lineIndex;
        self.reportedErrorInScope = fromElement.reportedErrorInScope;
        self.customFlags = fromElement.customFlags;
    }
}

impl Default for StringParserState {
    fn default() -> Self {
        Self::new()
    }
}

impl StringParserState {
    // C# signature: public StringParserState ()
    pub fn new() -> Self {
        Self {
            stack: vec![Element::new(); EXPECTED_MAX_STACK_DEPTH],
            numElements: 1,
        }
    }

    // C# signature: public int Push()
    pub fn Push(&mut self) -> Result<i32, String> {
        if self.numElements >= self.stack.len() {
            return Err("Stack overflow in parser state".to_string());
        }

        let prevElement = self.stack[self.numElements - 1].clone();
        let newElement = &mut self.stack[self.numElements];
        self.numElements += 1;

        newElement.CopyFrom(&prevElement);

        Ok(newElement.uniqueId)
    }

    // C# signature: public void Pop(int expectedRuleId)
    pub fn Pop(&mut self, expectedRuleId: i32) -> Result<(), String> {
        if self.numElements == 1 {
            return Err(
                "Attempting to remove final stack element is illegal! Mismatched Begin/Succceed/Fail?"
                    .to_string(),
            );
        }

        if self.currentElement().uniqueId != expectedRuleId {
            return Err(
                "Mismatched rule IDs - do you have mismatched Begin/Succeed/Fail?".to_string(),
            );
        }

        self.numElements -= 1;
        Ok(())
    }

    // C# signature: public Element Peek(int expectedRuleId)
    pub fn Peek(&self, expectedRuleId: i32) -> Result<&Element, String> {
        if self.currentElement().uniqueId != expectedRuleId {
            return Err(
                "Mismatched rule IDs - do you have mismatched Begin/Succeed/Fail?".to_string(),
            );
        }

        Ok(self.currentElement())
    }

    // C# signature: public Element PeekPenultimate()
    pub fn PeekPenultimate(&self) -> Option<&Element> {
        if self.numElements >= 2 {
            Some(&self.stack[self.numElements - 2])
        } else {
            None
        }
    }

    // C# signature: public void Squash()
    pub fn Squash(&mut self) -> Result<(), String> {
        if self.numElements < 2 {
            return Err(
                "Attempting to remove final stack element is illegal! Mismatched Begin/Succceed/Fail?"
                    .to_string(),
            );
        }

        let lastEl = self.stack[self.numElements - 1].clone();
        let penultimateEl = &mut self.stack[self.numElements - 2];
        penultimateEl.SquashFrom(&lastEl);
        self.numElements -= 1;

        Ok(())
    }

    // C# signature: public void NoteErrorReported()
    pub fn NoteErrorReported(&mut self) {
        for el in &mut self.stack {
            el.reportedErrorInScope = true;
        }
    }

    pub fn set_lineIndex(&mut self, value: i32) {
        self.currentElement_mut().lineIndex = value;
    }

    pub fn set_characterIndex(&mut self, value: i32) {
        self.currentElement_mut().characterIndex = value;
    }

    pub fn set_characterInLineIndex(&mut self, value: i32) {
        self.currentElement_mut().characterInLineIndex = value;
    }

    pub fn set_customFlags(&mut self, value: u32) {
        self.currentElement_mut().customFlags = value;
    }

    // C# signature: int lineIndex { get; }
    pub fn get_lineIndex(&self) -> i32 {
        self.currentElement().lineIndex
    }

    // C# signature: int characterIndex { get; }
    pub fn get_characterIndex(&self) -> i32 {
        self.currentElement().characterIndex
    }

    // C# signature: int characterInLineIndex { get; }
    pub fn get_characterInLineIndex(&self) -> i32 {
        self.currentElement().characterInLineIndex
    }

    // C# signature: uint customFlags { get; }
    pub fn get_customFlags(&self) -> u32 {
        self.currentElement().customFlags
    }

    // C# signature: bool errorReportedAlreadyInScope { get; }
    pub fn get_errorReportedAlreadyInScope(&self) -> bool {
        self.currentElement().reportedErrorInScope
    }

    // C# signature: int stackHeight { get; }
    pub fn get_stackHeight(&self) -> i32 {
        self.numElements as i32
    }

    // C# signature: Element currentElement { get; }
    pub fn get_currentElement(&self) -> &Element {
        self.currentElement()
    }

    fn currentElement(&self) -> &Element {
        &self.stack[self.numElements - 1]
    }

    fn currentElement_mut(&mut self) -> &mut Element {
        &mut self.stack[self.numElements - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::StringParserState;

    #[test]
    fn push_pop_and_squash_preserve_expected_state() {
        let mut state = StringParserState::new();
        state.set_lineIndex(4);
        state.set_characterIndex(12);

        let first_id = state.Push().unwrap();
        state.set_lineIndex(8);
        state.set_customFlags(7);

        assert_eq!(state.get_stackHeight(), 2);
        assert_eq!(state.Peek(first_id).unwrap().lineIndex, 8);

        let second_id = state.Push().unwrap();
        state.set_characterIndex(99);
        state.Squash().unwrap();

        assert_eq!(state.get_stackHeight(), 2);
        assert_eq!(state.get_characterIndex(), 99);
        assert!(state.Pop(second_id).is_err());
        state.Pop(first_id).unwrap();
        assert_eq!(state.get_lineIndex(), 4);
    }
}
