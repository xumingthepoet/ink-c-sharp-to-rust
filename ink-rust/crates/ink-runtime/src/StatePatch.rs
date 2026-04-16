// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/StatePatch.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct StatePatch {
    pub _port_marker: (),
}

impl StatePatch {
    // C# signature: public StatePatch(StatePatch toCopy)
    pub fn new(_toCopy: crate::stub::StatePatch) -> Self {
        Default::default()
    }

    // C# signature: public bool TryGetGlobal(string name, out Runtime.Object value)
    pub fn TryGetGlobal(&mut self, _name: String, _value: &mut crate::stub::PortStub) -> bool {
        Default::default()
    }

    // C# signature: public void SetGlobal(string name, Runtime.Object value)
    pub fn SetGlobal(&mut self, _name: String, _value: crate::stub::PortStub) {}

    // C# signature: public void AddChangedVariable(string name)
    pub fn AddChangedVariable(&mut self, _name: String) {}

    // C# signature: public bool TryGetVisitCount(Container container, out int count)
    pub fn TryGetVisitCount(
        &mut self,
        _container: crate::stub::Container,
        _count: &mut i32,
    ) -> bool {
        Default::default()
    }

    // C# signature: public void SetVisitCount(Container container, int count)
    pub fn SetVisitCount(&mut self, _container: crate::stub::Container, _count: i32) {}

    // C# signature: public void SetTurnIndex(Container container, int index)
    pub fn SetTurnIndex(&mut self, _container: crate::stub::Container, _index: i32) {}

    // C# signature: public bool TryGetTurnIndex(Container container, out int index)
    pub fn TryGetTurnIndex(
        &mut self,
        _container: crate::stub::Container,
        _index: &mut i32,
    ) -> bool {
        Default::default()
    }

    // C# signature: HashSet<string> changedVariables { get; }
    pub fn get_changedVariables(&mut self) -> std::collections::HashSet<String> {
        Default::default()
    }
}
