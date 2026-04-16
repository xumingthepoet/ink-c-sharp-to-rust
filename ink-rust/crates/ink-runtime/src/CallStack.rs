// Source: ink-c-sharp/ink-engine-runtime/CallStack.cs

use crate::Pointer::Pointer;
use crate::PushPop::PushPopType;
use crate::Story::Story;
use crate::Value::{ListValue, Value};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub currentPointer: Pointer,
    pub inExpressionEvaluation: bool,
    pub temporaryVariables: HashMap<String, Value>,
    pub r#type: PushPopType,
    pub evaluationStackHeightWhenPushed: i32,
    pub functionStartInOuputStream: i32,
}

impl Element {
    pub fn new(type_: PushPopType, pointer: Pointer, inExpressionEvaluation: bool) -> Self {
        Self {
            currentPointer: pointer,
            inExpressionEvaluation,
            temporaryVariables: HashMap::new(),
            r#type: type_,
            evaluationStackHeightWhenPushed: 0,
            functionStartInOuputStream: 0,
        }
    }

    pub fn Copy(&self) -> Self {
        Self {
            currentPointer: self.currentPointer.clone(),
            inExpressionEvaluation: self.inExpressionEvaluation,
            temporaryVariables: self.temporaryVariables.clone(),
            r#type: self.r#type,
            evaluationStackHeightWhenPushed: self.evaluationStackHeightWhenPushed,
            functionStartInOuputStream: self.functionStartInOuputStream,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Thread {
    pub callstack: Vec<Element>,
    pub threadIndex: i32,
    pub previousPointer: Option<Pointer>,
}

impl Thread {
    pub fn new() -> Self {
        Self {
            callstack: Vec::new(),
            threadIndex: 0,
            previousPointer: None,
        }
    }

    pub fn new_overload_2(
        _jThreadObj: HashMap<String, crate::stub::PortStub>,
        _storyContext: Story,
    ) -> Self {
        todo!("port runtime CallStack.Thread JSON reconstruction after Story path resolution is translated");
    }

    pub fn Copy(&self) -> Self {
        let mut copy = Thread::new();
        copy.threadIndex = self.threadIndex;
        copy.callstack = self.callstack.iter().map(Element::Copy).collect();
        copy.previousPointer = self.previousPointer.clone();
        copy
    }

    pub fn WriteJson(&self, _writer: crate::stub::Writer) {
        todo!("port runtime CallStack.Thread.WriteJson after Json serialisation is translated");
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallStack {
    pub threads: Vec<Thread>,
    pub threadCounter: i32,
    pub startOfRoot: Pointer,
}

impl CallStack {
    // C# signature: public CallStack (Story storyContext)
    pub fn new(_storyContext: Story) -> Self {
        let mut callstack = Self {
            threads: Vec::new(),
            threadCounter: 0,
            startOfRoot: Pointer::Null(),
        };
        callstack.Reset();
        callstack
    }

    // C# signature: public CallStack(CallStack toCopy)
    pub fn new_overload_2(toCopy: CallStack) -> Self {
        Self {
            threads: toCopy
                .threads
                .into_iter()
                .map(|thread| thread.Copy())
                .collect(),
            threadCounter: toCopy.threadCounter,
            startOfRoot: toCopy.startOfRoot,
        }
    }

    // C# signature: public Element Copy()
    pub fn Copy(&self) -> Element {
        self.currentElement().Copy()
    }

    // C# signature: public Thread Copy()
    pub fn Copy_overload_2(&self) -> Thread {
        self.currentThread().Copy()
    }

    // C# signature: public void WriteJson(SimpleJson.Writer writer)
    pub fn WriteJson(&self, _writer: crate::stub::Writer) {
        todo!("port runtime CallStack.WriteJson after Json serialisation is translated");
    }

    // C# signature: public void Reset()
    pub fn Reset(&mut self) {
        self.threads.clear();
        let mut root_thread = Thread::new();
        root_thread.callstack.push(Element::new(
            PushPopType::Tunnel,
            self.startOfRoot.clone(),
            false,
        ));
        self.threads.push(root_thread);
    }

    // C# signature: public void SetJsonToken(Dictionary<string, object> jObject, Story storyContext)
    pub fn SetJsonToken(
        &mut self,
        _jObject: HashMap<String, crate::stub::PortStub>,
        _storyContext: Story,
    ) {
        todo!("port runtime CallStack.SetJsonToken after Json serialisation is translated");
    }

    // C# signature: public void WriteJson(SimpleJson.Writer w)
    pub fn WriteJson_overload_2(&self, _w: crate::stub::Writer) {
        todo!("port runtime CallStack.WriteJson overload after Json serialisation is translated");
    }

    // C# signature: public void PushThread()
    pub fn PushThread(&mut self) {
        let mut newThread = self.currentThread().Copy();
        self.threadCounter += 1;
        newThread.threadIndex = self.threadCounter;
        self.threads.push(newThread);
    }

    // C# signature: public Thread ForkThread()
    pub fn ForkThread(&mut self) -> Thread {
        let mut forkedThread = self.currentThread().Copy();
        self.threadCounter += 1;
        forkedThread.threadIndex = self.threadCounter;
        forkedThread
    }

    // C# signature: public void PopThread()
    pub fn PopThread(&mut self) {
        if self.canPopThread() {
            self.threads.pop();
        } else {
            panic!("Can't pop thread");
        }
    }

    // C# signature: public void Push(PushPopType type, int externalEvaluationStackHeight = 0, int outputStreamLengthWithPushed = 0)
    pub fn Push(
        &mut self,
        type_: PushPopType,
        externalEvaluationStackHeight: i32,
        outputStreamLengthWithPushed: i32,
    ) {
        let mut element = Element::new(type_, self.currentElement().currentPointer.clone(), false);
        element.evaluationStackHeightWhenPushed = externalEvaluationStackHeight;
        element.functionStartInOuputStream = outputStreamLengthWithPushed;
        self.callStack_mut().push(element);
    }

    // C# signature: public bool CanPop(PushPopType? type = null)
    pub fn CanPop(&self, type_: Option<PushPopType>) -> bool {
        if !self.canPop() {
            return false;
        }
        match type_ {
            None => true,
            Some(expected) => self.currentElement().r#type == expected,
        }
    }

    // C# signature: public void Pop(PushPopType? type = null)
    pub fn Pop(&mut self, type_: Option<PushPopType>) {
        if self.CanPop(type_) {
            self.callStack_mut().pop();
        } else {
            panic!("Mismatched push/pop in Callstack");
        }
    }

    // C# signature: public Runtime.Object GetTemporaryVariableWithName(string name, int contextIndex = -1)
    pub fn GetTemporaryVariableWithName(&self, name: String, contextIndex: i32) -> Option<Value> {
        let contextIndex = if contextIndex == -1 {
            self.currentElementIndex() + 1
        } else {
            contextIndex
        };

        let contextElement = self
            .callStack()
            .get((contextIndex - 1) as usize)
            .expect("temporary variable context index out of range");
        contextElement.temporaryVariables.get(&name).cloned()
    }

    // C# signature: public void SetTemporaryVariable(string name, Runtime.Object value, bool declareNew, int contextIndex = -1)
    pub fn SetTemporaryVariable(
        &mut self,
        name: String,
        value: Value,
        declareNew: bool,
        contextIndex: i32,
    ) {
        let contextIndex = if contextIndex == -1 {
            self.currentElementIndex() + 1
        } else {
            contextIndex
        };

        let contextElement = self
            .callStack_mut()
            .get_mut((contextIndex - 1) as usize)
            .expect("temporary variable context index out of range");

        if !declareNew && !contextElement.temporaryVariables.contains_key(&name) {
            panic!("Could not find temporary variable to set: {}", name);
        }

        if let Some(oldValue) = contextElement.temporaryVariables.get(&name) {
            let mut newValue = value.clone();
            ListValue::RetainListOriginsForAssignment(oldValue, &mut newValue);
            contextElement.temporaryVariables.insert(name, newValue);
        } else {
            contextElement.temporaryVariables.insert(name, value);
        }
    }

    // C# signature: public int ContextForVariableNamed(string name)
    pub fn ContextForVariableNamed(&self, name: String) -> i32 {
        if self.currentElement().temporaryVariables.contains_key(&name) {
            self.currentElementIndex() + 1
        } else {
            0
        }
    }

    // C# signature: public Thread ThreadWithIndex(int index)
    pub fn ThreadWithIndex(&self, index: i32) -> Option<&Thread> {
        self.threads
            .iter()
            .find(|thread| thread.threadIndex == index)
    }

    fn callStack(&self) -> &Vec<Element> {
        &self.currentThread().callstack
    }

    fn callStack_mut(&mut self) -> &mut Vec<Element> {
        let idx = self.threads.len() - 1;
        &mut self.threads[idx].callstack
    }

    pub fn currentElement(&self) -> &Element {
        self.currentThread()
            .callstack
            .last()
            .expect("call stack should always contain at least one element")
    }

    pub fn currentElementIndex(&self) -> i32 {
        self.callStack().len() as i32 - 1
    }

    pub fn currentThread(&self) -> &Thread {
        self.threads
            .last()
            .expect("call stack should always contain at least one thread")
    }

    pub fn currentThread_mut(&mut self) -> &mut Thread {
        self.threads
            .last_mut()
            .expect("call stack should always contain at least one thread")
    }

    // C# signature: bool canPop { get; }
    pub fn canPop(&self) -> bool {
        self.callStack().len() > 1
    }

    // C# signature: bool canPopThread { get; }
    pub fn canPopThread(&self) -> bool {
        self.threads.len() > 1 && !self.elementIsEvaluateFromGame()
    }

    // C# signature: bool elementIsEvaluateFromGame { get; }
    pub fn elementIsEvaluateFromGame(&self) -> bool {
        self.currentElement().r#type == PushPopType::FunctionEvaluationFromGame
    }

    // C# signature: List<Element> callStack { get; }
    pub fn get_callStack(&self) -> Vec<Element> {
        self.callStack().clone()
    }

    // C# signature: string callStackTrace { get; }
    pub fn get_callStackTrace(&self) -> String {
        let mut result = String::new();
        for (thread_index, thread) in self.threads.iter().enumerate() {
            let isCurrent = thread_index == self.threads.len() - 1;
            result.push_str(&format!(
                "=== THREAD {}/{} {}===\n",
                thread_index + 1,
                self.threads.len(),
                if isCurrent { "(current) " } else { "" }
            ));
            for element in &thread.callstack {
                if element.r#type == PushPopType::Function {
                    result.push_str("  [FUNCTION] ");
                } else {
                    result.push_str("  [TUNNEL] ");
                }

                if !element.currentPointer.get_isNull() {
                    result.push_str("<SOMEWHERE IN ");
                    if let Some(path) = element.currentPointer.get_path() {
                        result.push_str(&path.ToString());
                    }
                    result.push_str(">\n");
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::CallStack;
    use crate::Story::Story;
    use crate::Value::Value;

    #[test]
    fn tracks_temp_variables_and_stack_depth() {
        let story = Story::default();
        let mut stack = CallStack::new(story);

        assert_eq!(stack.canPop(), false);
        assert_eq!(stack.get_callStack().len(), 1);

        stack.SetTemporaryVariable("x".to_string(), Value::new_int(1), true, -1);
        assert!(stack
            .GetTemporaryVariableWithName("x".to_string(), -1)
            .is_some());
        assert_eq!(stack.ContextForVariableNamed("x".to_string()), 1);
    }
}
