// Source: ink-c-sharp/ink-engine-runtime/StatePatch.cs

use crate::Container::Container;
use crate::Path::Path;
use crate::Value::Value;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatePatch {
    globals: HashMap<String, Value>,
    changedVariables: HashSet<String>,
    visitCountsByPath: HashMap<Path, i32>,
    turnIndicesByPath: HashMap<Path, i32>,
    visitCountsByContainer: HashMap<usize, i32>,
    turnIndicesByContainer: HashMap<usize, i32>,
}

impl StatePatch {
    fn container_key(container: &Container) -> usize {
        container as *const Container as usize
    }

    // C# signature: public StatePatch(StatePatch toCopy)
    pub fn new(toCopy: StatePatch) -> Self {
        Self {
            globals: toCopy.globals.clone(),
            changedVariables: toCopy.changedVariables.clone(),
            visitCountsByPath: toCopy.visitCountsByPath.clone(),
            turnIndicesByPath: toCopy.turnIndicesByPath.clone(),
            visitCountsByContainer: toCopy.visitCountsByContainer.clone(),
            turnIndicesByContainer: toCopy.turnIndicesByContainer.clone(),
        }
    }

    // C# signature: public bool TryGetGlobal(string name, out Runtime.Object value)
    pub fn TryGetGlobal(&self, name: String, value: &mut Option<Value>) -> bool {
        if let Some(found) = self.globals.get(&name) {
            *value = Some(found.clone());
            true
        } else {
            false
        }
    }

    // C# signature: public void SetGlobal(string name, Runtime.Object value)
    pub fn SetGlobal(&mut self, name: String, value: Value) {
        self.globals.insert(name, value);
    }

    // C# signature: public void AddChangedVariable(string name)
    pub fn AddChangedVariable(&mut self, name: String) {
        self.changedVariables.insert(name);
    }

    // C# signature: public bool TryGetVisitCount(Container container, out int count)
    pub fn TryGetVisitCount(&self, container: &Container, count: &mut i32) -> bool {
        if let Some(found) = self
            .visitCountsByContainer
            .get(&Self::container_key(container))
        {
            *count = *found;
            true
        } else if let Some(found) = self.visitCountsByPath.get(&container.get_path()) {
            *count = *found;
            true
        } else {
            false
        }
    }

    // C# signature: public void SetVisitCount(Container container, int count)
    pub fn SetVisitCount(&mut self, container: &Container, count: i32) {
        self.visitCountsByPath
            .insert(container.get_path().clone(), count);
        self.visitCountsByContainer
            .insert(Self::container_key(container), count);
    }

    // C# signature: public void SetTurnIndex(Container container, int index)
    pub fn SetTurnIndex(&mut self, container: &Container, index: i32) {
        self.turnIndicesByPath
            .insert(container.get_path().clone(), index);
        self.turnIndicesByContainer
            .insert(Self::container_key(container), index);
    }

    // C# signature: public bool TryGetTurnIndex(Container container, out int index)
    pub fn TryGetTurnIndex(&self, container: &Container, index: &mut i32) -> bool {
        if let Some(found) = self
            .turnIndicesByContainer
            .get(&Self::container_key(container))
        {
            *index = *found;
            true
        } else if let Some(found) = self.turnIndicesByPath.get(&container.get_path()) {
            *index = *found;
            true
        } else {
            false
        }
    }

    // C# signature: HashSet<string> changedVariables { get; }
    pub fn get_changedVariables(&self) -> HashSet<String> {
        self.changedVariables.clone()
    }

    pub fn get_globals(&self) -> &HashMap<String, Value> {
        &self.globals
    }

    pub fn get_visitCounts(&self) -> &HashMap<Path, i32> {
        &self.visitCountsByPath
    }

    pub fn get_turnIndices(&self) -> &HashMap<Path, i32> {
        &self.turnIndicesByPath
    }
}

#[cfg(test)]
mod tests {
    use super::StatePatch;
    use crate::Container::Container;
    use crate::Value::Value;

    #[test]
    fn copies_and_tracks_globals_and_counts() {
        let mut patch = StatePatch::new(StatePatch::default());
        patch.SetGlobal("x".to_string(), Value::new_int(7));
        patch.AddChangedVariable("x".to_string());

        let container = Container::new();
        patch.SetVisitCount(&container, 3);
        patch.SetTurnIndex(&container, 11);

        let mut found = None;
        let mut visit = 0;
        let mut turn = 0;
        assert!(patch.TryGetGlobal("x".to_string(), &mut found));
        assert!(patch.TryGetVisitCount(&container, &mut visit));
        assert!(patch.TryGetTurnIndex(&container, &mut turn));
        assert_eq!(visit, 3);
        assert_eq!(turn, 11);
        assert!(matches!(found, Some(Value::Int(_))));
        assert!(patch.get_changedVariables().contains("x"));
    }
}
