// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/VariablesState.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct VariablesState {
    pub _port_marker: (),
}

impl VariablesState {
    // C# signature: public VariablesState (CallStack callStack, ListDefinitionsOrigin listDefsOrigin)
    pub fn new(
        _callStack: crate::stub::CallStack,
        _listDefsOrigin: crate::stub::ListDefinitionsOrigin,
    ) -> Self {
        Default::default()
    }

    // C# signature: public void StartVariableObservation()
    pub fn StartVariableObservation(&mut self) {}

    // C# signature: public Dictionary<string, Object> CompleteVariableObservation()
    pub fn CompleteVariableObservation(
        &mut self,
    ) -> std::collections::HashMap<String, crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: public void NotifyObservers(Dictionary<string, Object> changedVars)
    pub fn NotifyObservers(
        &mut self,
        _changedVars: std::collections::HashMap<String, crate::stub::PortStub>,
    ) {
    }

    // C# signature: public IEnumerator<string> GetEnumerator()
    pub fn GetEnumerator(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public void ApplyPatch()
    pub fn ApplyPatch(&mut self) {}

    // C# signature: public void SetJsonToken(Dictionary<string, object> jToken)
    pub fn SetJsonToken(
        &mut self,
        _jToken: std::collections::HashMap<String, crate::stub::PortStub>,
    ) {
    }

    // C# signature: public void WriteJson(SimpleJson.Writer writer)
    pub fn WriteJson(&mut self, _writer: crate::stub::Writer) {}

    // C# signature: public bool RuntimeObjectsEqual(Runtime.Object obj1, Runtime.Object obj2)
    pub fn RuntimeObjectsEqual(
        &mut self,
        _obj1: crate::stub::PortStub,
        _obj2: crate::stub::PortStub,
    ) -> bool {
        Default::default()
    }

    // C# signature: public Runtime.Object GetVariableWithName(string name)
    pub fn GetVariableWithName(&mut self, _name: String) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public Runtime.Object TryGetDefaultVariableValue (string name)
    pub fn TryGetDefaultVariableValue(&mut self, _name: String) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public bool GlobalVariableExistsWithName(string name)
    pub fn GlobalVariableExistsWithName(&mut self, _name: String) -> bool {
        Default::default()
    }

    // C# signature: public Runtime.Object ValueAtVariablePointer(VariablePointerValue pointer)
    pub fn ValueAtVariablePointer(
        &mut self,
        _pointer: crate::stub::VariablePointerValue,
    ) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public void Assign(VariableAssignment varAss, Runtime.Object value)
    pub fn Assign(
        &mut self,
        _varAss: crate::stub::VariableAssignment,
        _value: crate::stub::PortStub,
    ) {
    }

    // C# signature: public void SnapshotDefaultGlobals ()
    pub fn SnapshotDefaultGlobals(&mut self) {}

    // C# signature: public void SetGlobal(string variableName, Runtime.Object value)
    pub fn SetGlobal(&mut self, _variableName: String, _value: crate::stub::PortStub) {}

    // C# signature: CallStack callStack { get; }
    pub fn get_callStack(&mut self) -> crate::stub::CallStack {
        Default::default()
    }
}
