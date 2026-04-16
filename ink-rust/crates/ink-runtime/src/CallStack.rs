// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/CallStack.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct CallStack {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct Element {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct Thread {
    pub _port_marker: (),
}

impl CallStack {
    // C# signature: public CallStack (Story storyContext)
    pub fn new(_storyContext: crate::stub::Story) -> Self {
        Default::default()
    }

    // C# signature: public CallStack(CallStack toCopy)
    pub fn new_overload_2(_toCopy: crate::stub::CallStack) -> Self {
        Default::default()
    }

    // C# signature: public Element Copy()
    pub fn Copy(&mut self) -> crate::stub::Element {
        Default::default()
    }

    // C# signature: public Thread Copy()
    pub fn Copy_overload_2(&mut self) -> crate::stub::Thread {
        Default::default()
    }

    // C# signature: public void WriteJson(SimpleJson.Writer writer)
    pub fn WriteJson(&mut self, _writer: crate::stub::Writer) {}

    // C# signature: public void Reset()
    pub fn Reset(&mut self) {}

    // C# signature: public void SetJsonToken(Dictionary<string, object> jObject, Story storyContext)
    pub fn SetJsonToken(
        &mut self,
        _jObject: std::collections::HashMap<String, crate::stub::PortStub>,
        _storyContext: crate::stub::Story,
    ) {
    }

    // C# signature: public void WriteJson(SimpleJson.Writer w)
    pub fn WriteJson_overload_2(&mut self, _w: crate::stub::Writer) {}

    // C# signature: public void PushThread()
    pub fn PushThread(&mut self) {}

    // C# signature: public Thread ForkThread()
    pub fn ForkThread(&mut self) -> crate::stub::Thread {
        Default::default()
    }

    // C# signature: public void PopThread()
    pub fn PopThread(&mut self) {}

    // C# signature: public void Push(PushPopType type, int externalEvaluationStackHeight = 0, int outputStreamLengthWithPushed = 0)
    pub fn Push(
        &mut self,
        _type: crate::stub::PushPopType,
        _externalEvaluationStackHeight: i32,
        _outputStreamLengthWithPushed: i32,
    ) {
    }

    // C# signature: public bool CanPop(PushPopType? type = null)
    pub fn CanPop(&mut self, _type: crate::stub::PushPopType) -> bool {
        Default::default()
    }

    // C# signature: public void Pop(PushPopType? type = null)
    pub fn Pop(&mut self, _type: crate::stub::PushPopType) {}

    // C# signature: public Runtime.Object GetTemporaryVariableWithName(string name, int contextIndex = -1)
    pub fn GetTemporaryVariableWithName(
        &mut self,
        _name: String,
        _contextIndex: i32,
    ) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public void SetTemporaryVariable(string name, Runtime.Object value, bool declareNew, int contextIndex = -1)
    pub fn SetTemporaryVariable(
        &mut self,
        _name: String,
        _value: crate::stub::PortStub,
        _declareNew: bool,
        _contextIndex: i32,
    ) {
    }

    // C# signature: public int ContextForVariableNamed(string name)
    pub fn ContextForVariableNamed(&mut self, _name: String) -> i32 {
        Default::default()
    }

    // C# signature: public Thread ThreadWithIndex(int index)
    pub fn ThreadWithIndex(&mut self, _index: i32) -> crate::stub::Thread {
        Default::default()
    }

    // C# signature: List<Element> elements { get; }
    pub fn get_elements(&mut self) -> Vec<crate::stub::Element> {
        Default::default()
    }

    // C# signature: int depth { get; }
    pub fn get_depth(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: Element currentElement { get; }
    pub fn get_currentElement(&mut self) -> crate::stub::Element {
        Default::default()
    }

    // C# signature: int currentElementIndex { get; }
    pub fn get_currentElementIndex(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: Thread currentThread { get; }
    pub fn get_currentThread(&mut self) -> crate::stub::Thread {
        Default::default()
    }

    // C# signature: bool canPop { get; }
    pub fn get_canPop(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool canPopThread { get; }
    pub fn get_canPopThread(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool elementIsEvaluateFromGame { get; }
    pub fn get_elementIsEvaluateFromGame(&mut self) -> bool {
        Default::default()
    }

    // C# signature: List<Element> callStack { get; }
    pub fn get_callStack(&mut self) -> Vec<crate::stub::Element> {
        Default::default()
    }

    // C# signature: string callStackTrace { get; }
    pub fn get_callStackTrace(&mut self) -> String {
        Default::default()
    }
}
