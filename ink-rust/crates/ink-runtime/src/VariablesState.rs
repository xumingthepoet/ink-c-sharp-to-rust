// Source: ink-c-sharp/ink-engine-runtime/VariablesState.cs

use crate::CallStack::CallStack;
use crate::JsonSerialisation::Json;
use crate::ListDefinitionsOrigin::ListDefinitionsOrigin;
use crate::SimpleJson::{JsonObject, Writer};
use crate::StatePatch::StatePatch;
use crate::StoryException::StoryException;
use crate::Value::{ListValue, Value, VariablePointerValue};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

const DONT_SAVE_DEFAULT_VALUES: bool = true;

pub type VariableObserver = Arc<dyn Fn(String, Value) + Send + Sync>;

#[derive(Clone)]
pub struct VariablesState {
    globalVariables: HashMap<String, Value>,
    defaultGlobalVariables: HashMap<String, Value>,
    callStack: CallStack,
    listDefsOrigin: ListDefinitionsOrigin,
    variableObservers: HashMap<String, Vec<VariableObserver>>,
    changedVariablesForBatchObs: Option<HashSet<String>>,
    batchObservingVariableChanges: bool,
    pub patch: Option<StatePatch>,
}

impl std::fmt::Debug for VariablesState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariablesState")
            .field("globalVariables", &self.globalVariables)
            .field("defaultGlobalVariables", &self.defaultGlobalVariables)
            .field("callStack", &self.callStack)
            .field("listDefsOrigin", &self.listDefsOrigin)
            .field(
                "changedVariablesForBatchObs",
                &self.changedVariablesForBatchObs,
            )
            .field(
                "batchObservingVariableChanges",
                &self.batchObservingVariableChanges,
            )
            .field("patch", &self.patch)
            .finish()
    }
}

impl VariablesState {
    pub fn new_default() -> Self {
        Self::new(
            CallStack::new(crate::Story::Story::default()),
            ListDefinitionsOrigin::default(),
        )
    }

    // C# signature: public VariablesState (CallStack callStack, ListDefinitionsOrigin listDefsOrigin)
    pub fn new(callStack: CallStack, listDefsOrigin: ListDefinitionsOrigin) -> Self {
        Self {
            callStack,
            listDefsOrigin,
            globalVariables: HashMap::new(),
            defaultGlobalVariables: HashMap::new(),
            variableObservers: HashMap::new(),
            changedVariablesForBatchObs: None,
            batchObservingVariableChanges: false,
            patch: None,
        }
    }

    // C# signature: public void StartVariableObservation()
    pub fn StartVariableObservation(&mut self) {
        self.batchObservingVariableChanges = true;
        self.changedVariablesForBatchObs = Some(HashSet::new());
    }

    // C# signature: public Dictionary<string, Object> CompleteVariableObservation()
    pub fn CompleteVariableObservation(&mut self) -> HashMap<String, Value> {
        self.batchObservingVariableChanges = false;

        let mut changedVars = HashMap::new();
        if let Some(changed) = &self.changedVariablesForBatchObs {
            for variableName in changed {
                if let Some(currentValue) = self.globalVariables.get(variableName) {
                    changedVars.insert(variableName.clone(), currentValue.clone());
                }
            }
        }

        if let Some(patch) = &self.patch {
            for variableName in patch.get_changedVariables() {
                let mut patchedVal = None;
                if patch.TryGetGlobal(variableName.clone(), &mut patchedVal) {
                    if let Some(val) = patchedVal {
                        changedVars.insert(variableName, val);
                    }
                }
            }
        }

        self.changedVariablesForBatchObs = None;
        changedVars
    }

    // C# signature: public void NotifyObservers(Dictionary<string, Object> changedVars)
    pub fn NotifyObservers(&mut self, _changedVars: HashMap<String, Value>) {
        for (variableName, value) in _changedVars {
            if let Some(observers) = self.variableObservers.get(&variableName) {
                for observer in observers {
                    observer(variableName.clone(), value.clone());
                }
            }
        }
    }

    pub fn ObserveVariable(&mut self, variableName: String, observer: VariableObserver) {
        self.variableObservers
            .entry(variableName)
            .or_insert_with(Vec::new)
            .push(observer);
    }

    pub fn ObserveVariables(&mut self, variableNames: Vec<String>, observer: VariableObserver) {
        for variableName in variableNames {
            self.ObserveVariable(variableName, observer.clone());
        }
    }

    pub fn RemoveVariableObserver(
        &mut self,
        observer: Option<&VariableObserver>,
        specificVariableName: Option<&str>,
    ) {
        if let Some(specificVariableName) = specificVariableName {
            if let Some(observers) = self.variableObservers.get_mut(specificVariableName) {
                if let Some(observer) = observer {
                    observers.retain(|candidate| !Arc::ptr_eq(candidate, observer));
                } else {
                    self.variableObservers.remove(specificVariableName);
                    return;
                }
                if observers.is_empty() {
                    self.variableObservers.remove(specificVariableName);
                }
            }
        } else if let Some(observer) = observer {
            let keys = self.variableObservers.keys().cloned().collect::<Vec<_>>();
            for key in keys {
                let mut should_remove = false;
                if let Some(observers) = self.variableObservers.get_mut(&key) {
                    observers.retain(|candidate| !Arc::ptr_eq(candidate, observer));
                    should_remove = observers.is_empty();
                }
                if should_remove {
                    self.variableObservers.remove(&key);
                }
            }
        }
    }

    // C# signature: public IEnumerator<string> GetEnumerator()
    pub fn GetEnumerator(&self) -> std::collections::hash_map::Keys<'_, String, Value> {
        self.globalVariables.keys()
    }

    // C# signature: public void ApplyPatch()
    pub fn ApplyPatch(&mut self) {
        if let Some(patch) = &self.patch {
            for (name, value) in patch.get_globals() {
                self.globalVariables.insert(name.clone(), value.clone());
            }

            if let Some(changed) = &mut self.changedVariablesForBatchObs {
                for name in patch.get_changedVariables() {
                    changed.insert(name);
                }
            }
        }

        self.patch = None;
    }

    // C# signature: public void SetJsonToken(Dictionary<string, object> jToken)
    pub fn SetJsonToken(&mut self, jToken: JsonObject) {
        self.globalVariables.clear();

        for (name, default_value) in &self.defaultGlobalVariables {
            if let Some(loaded_token) = jToken.get(name) {
                if let Some(runtime_obj) = Json::JTokenToRuntimeObject(loaded_token.clone()) {
                    match runtime_obj {
                        crate::Container::ContentItem::Value(value) => {
                            self.globalVariables.insert(name.clone(), value);
                        }
                        _ => panic!(
                            "Unexpected non-value runtime object when loading global variable {}",
                            name
                        ),
                    }
                } else {
                    self.globalVariables
                        .insert(name.clone(), default_value.clone());
                }
            } else {
                self.globalVariables
                    .insert(name.clone(), default_value.clone());
            }
        }
    }

    // C# signature: public void WriteJson(SimpleJson.Writer writer)
    pub fn WriteJson(&mut self, writer: &mut Writer) {
        writer
            .WriteObject(|writer| {
                for (name, value) in &self.globalVariables {
                    if DONT_SAVE_DEFAULT_VALUES {
                        if let Some(default_value) = self.defaultGlobalVariables.get(name) {
                            if self.RuntimeObjectsEqual(value, default_value) {
                                continue;
                            }
                        }
                    }

                    writer.WritePropertyStart(name.clone())?;
                    Json::WriteRuntimeObject(
                        writer,
                        &crate::Container::ContentItem::Value(value.clone()),
                    );
                    writer.WritePropertyEnd()?;
                }
                Ok(())
            })
            .unwrap_or_else(|err| panic!("{}", err));
    }

    // C# signature: public bool RuntimeObjectsEqual(Runtime.Object obj1, Runtime.Object obj2)
    pub fn RuntimeObjectsEqual(&self, obj1: &Value, obj2: &Value) -> bool {
        match (obj1, obj2) {
            (Value::Bool(a), Value::Bool(b)) => a.value == b.value,
            (Value::Int(a), Value::Int(b)) => a.value == b.value,
            (Value::Float(a), Value::Float(b)) => a.value == b.value,
            (Value::String(a), Value::String(b)) => a.value == b.value,
            (Value::DivertTarget(a), Value::DivertTarget(b)) => a.value == b.value,
            (Value::VariablePointer(a), Value::VariablePointer(b)) => {
                a.value == b.value && a.contextIndex == b.contextIndex
            }
            (Value::List(a), Value::List(b)) => {
                a.value == b.value && a.originNames == b.originNames
            }
            _ => false,
        }
    }

    // C# signature: public Runtime.Object GetVariableWithName(string name)
    pub fn GetVariableWithName(&mut self, name: String) -> Option<Value> {
        self.GetVariableWithName_overload_2(name, -1)
    }

    fn GetVariableWithName_overload_2(&mut self, name: String, contextIndex: i32) -> Option<Value> {
        let varValue = self.GetRawVariableWithName(name.clone(), contextIndex)?;
        if let Value::VariablePointer(varPointer) = &varValue {
            self.ValueAtVariablePointer(varPointer.clone())
        } else {
            Some(varValue)
        }
    }

    // C# signature: public Runtime.Object TryGetDefaultVariableValue (string name)
    pub fn TryGetDefaultVariableValue(&mut self, name: String) -> Option<Value> {
        self.defaultGlobalVariables.get(&name).cloned()
    }

    // C# signature: public bool GlobalVariableExistsWithName(string name)
    pub fn GlobalVariableExistsWithName(&mut self, name: String) -> bool {
        self.globalVariables.contains_key(&name)
            || self.defaultGlobalVariables.contains_key(&name)
            || self
                .listDefsOrigin
                .FindSingleItemListWithName(name)
                .is_some()
    }

    fn GetVariableWithName_overload_3(&mut self, name: String, contextIndex: i32) -> Option<Value> {
        self.GetRawVariableWithName(name, contextIndex)
            .and_then(|varValue| match varValue {
                Value::VariablePointer(varPointer) => self.ValueAtVariablePointer(varPointer),
                other => Some(other),
            })
    }

    fn GetRawVariableWithName(&mut self, name: String, contextIndex: i32) -> Option<Value> {
        if contextIndex == 0 || contextIndex == -1 {
            if let Some(patch) = &self.patch {
                let mut patched = None;
                if patch.TryGetGlobal(name.clone(), &mut patched) {
                    return patched;
                }
            }

            if let Some(varValue) = self.globalVariables.get(&name) {
                return Some(varValue.clone());
            }

            if let Some(varValue) = self.defaultGlobalVariables.get(&name) {
                return Some(varValue.clone());
            }

            if let Some(listItemValue) =
                self.listDefsOrigin.FindSingleItemListWithName(name.clone())
            {
                return Some(Value::List(listItemValue.clone()));
            }
        }

        self.callStack
            .GetTemporaryVariableWithName(name, contextIndex)
    }

    // C# signature: public Runtime.Object ValueAtVariablePointer(VariablePointerValue pointer)
    pub fn ValueAtVariablePointer(&mut self, pointer: VariablePointerValue) -> Option<Value> {
        pointer
            .value
            .clone()
            .and_then(|name| self.GetVariableWithName_overload_3(name, pointer.contextIndex))
    }

    // C# signature: public void Assign(VariableAssignment varAss, Runtime.Object value)
    pub fn Assign(&mut self, varAss: crate::VariableAssignment::VariableAssignment, value: Value) {
        let mut name = varAss.get_variableName().unwrap_or("").to_string();
        let mut contextIndex = -1;

        let mut setGlobal = if varAss.get_isNewDeclaration() {
            varAss.get_isGlobal()
        } else {
            self.GlobalVariableExistsWithName(name.clone())
        };

        let mut value = value;

        if varAss.get_isNewDeclaration() {
            if let Value::VariablePointer(varPointer) = value.clone() {
                if let Some(fullyResolvedVariablePointer) = self.ResolveVariablePointer(varPointer)
                {
                    value = Value::VariablePointer(fullyResolvedVariablePointer);
                }
            }
        } else {
            loop {
                let existingPointer = match self.GetRawVariableWithName(name.clone(), contextIndex)
                {
                    Some(Value::VariablePointer(pointer)) => Some(pointer),
                    _ => None,
                };

                if let Some(existingPointer) = existingPointer {
                    name = existingPointer.value.unwrap_or_default();
                    contextIndex = existingPointer.contextIndex;
                    setGlobal = contextIndex == 0;
                } else {
                    break;
                }
            }
        }

        if setGlobal {
            self.SetGlobal(name, value);
        } else {
            self.callStack.SetTemporaryVariable(
                name,
                value,
                varAss.get_isNewDeclaration(),
                contextIndex,
            );
        }
    }

    // C# signature: public void SnapshotDefaultGlobals ()
    pub fn SnapshotDefaultGlobals(&mut self) {
        self.defaultGlobalVariables = self.globalVariables.clone();
    }

    fn RetainListOriginsForAssignment(&self, oldValue: &Value, newValue: &mut Value) {
        if let (Value::List(oldList), Value::List(newList)) = (oldValue, newValue) {
            if newList.value.is_empty() {
                newList.originNames = oldList.originNames.clone();
            }
        }
    }

    // C# signature: public void SetGlobal(string variableName, Runtime.Object value)
    pub fn SetGlobal(&mut self, variableName: String, value: Value) {
        let mut oldValue = None;
        if let Some(patch) = &mut self.patch {
            let mut patched = None;
            if patch.TryGetGlobal(variableName.clone(), &mut patched) {
                oldValue = patched;
            }
        }

        if oldValue.is_none() {
            oldValue = self.globalVariables.get(&variableName).cloned();
        }

        let mut value = value;
        if let Some(oldValue) = &oldValue {
            self.RetainListOriginsForAssignment(oldValue, &mut value);
        }

        if let Some(patch) = &mut self.patch {
            patch.SetGlobal(variableName.clone(), value.clone());
            patch.AddChangedVariable(variableName.clone());
        } else {
            self.globalVariables
                .insert(variableName.clone(), value.clone());
        }

        if self.batchObservingVariableChanges {
            if let Some(changed) = &mut self.changedVariablesForBatchObs {
                changed.insert(variableName);
            }
        } else if oldValue.as_ref() != Some(&value) {
            if let Some(observers) = self.variableObservers.get(&variableName) {
                for observer in observers {
                    observer(variableName.clone(), value.clone());
                }
            }
        }
    }

    fn ResolveVariablePointer(
        &mut self,
        varPointer: VariablePointerValue,
    ) -> Option<VariablePointerValue> {
        let mut contextIndex = varPointer.contextIndex;
        let variableName = varPointer.value.clone()?;

        if contextIndex == -1 {
            contextIndex = self.GetContextIndexOfVariableNamed(variableName.clone());
        }

        let valueOfVariablePointedTo =
            self.GetRawVariableWithName(variableName.clone(), contextIndex)?;

        if let Value::VariablePointer(doubleRedirectionPointer) = valueOfVariablePointedTo {
            Some(doubleRedirectionPointer)
        } else {
            Some(VariablePointerValue::new(Some(variableName), contextIndex))
        }
    }

    fn GetContextIndexOfVariableNamed(&mut self, varName: String) -> i32 {
        if self.GlobalVariableExistsWithName(varName) {
            0
        } else {
            self.callStack.currentElementIndex()
        }
    }

    // C# signature: CallStack callStack { get; }
    pub fn get_callStack(&self) -> CallStack {
        self.callStack.clone()
    }

    pub fn set_callStack(&mut self, callStack: CallStack) {
        self.callStack = callStack;
    }
}

impl Default for VariablesState {
    fn default() -> Self {
        Self::new_default()
    }
}

#[cfg(test)]
mod tests {
    use super::VariablesState;
    use crate::CallStack::CallStack;
    use crate::ListDefinitionsOrigin::ListDefinitionsOrigin;
    use crate::PushPop::PushPopType;
    use crate::StatePatch::StatePatch;
    use crate::Value::{Value, VariablePointerValue};
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

    #[test]
    fn gets_sets_and_batches_globals() {
        let story = crate::Story::Story::default();
        let callstack = CallStack::new(story);
        let mut vars = VariablesState::new(callstack, ListDefinitionsOrigin::default());

        vars.SetGlobal("x".to_string(), Value::new_int(1));
        assert!(matches!(
            vars.GetVariableWithName("x".to_string()),
            Some(Value::Int(_))
        ));
        vars.StartVariableObservation();
        vars.SetGlobal("x".to_string(), Value::new_int(2));
        let changed = vars.CompleteVariableObservation();
        assert!(matches!(changed.get("x"), Some(Value::Int(_))));
    }

    #[test]
    fn applies_patch_and_resolves_variable_pointers() {
        let story = crate::Story::Story::default();
        let callstack = CallStack::new(story);
        let mut vars = VariablesState::new(callstack, ListDefinitionsOrigin::default());
        vars.SetGlobal("score".to_string(), Value::new_int(3));

        let mut patch = StatePatch::default();
        patch.SetGlobal("score".to_string(), Value::new_int(7));
        patch.AddChangedVariable("score".to_string());
        vars.patch = Some(patch);
        vars.ApplyPatch();

        assert!(matches!(
            vars.GetVariableWithName("score".to_string()),
            Some(Value::Int(_))
        ));

        let pointer = VariablePointerValue::new(Some("score".to_string()), 0);
        assert!(matches!(
            vars.ValueAtVariablePointer(pointer),
            Some(Value::Int(_))
        ));
    }

    #[test]
    fn notifies_variable_observers_on_change() {
        let story = crate::Story::Story::default();
        let callstack = CallStack::new(story);
        let mut vars = VariablesState::new(callstack, ListDefinitionsOrigin::default());
        vars.SetGlobal("score".to_string(), Value::new_int(1));

        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();
        vars.ObserveVariable(
            "score".to_string(),
            Arc::new(move |name, value| {
                assert_eq!(name, "score");
                assert!(matches!(value, Value::Int(_)));
                called_clone.store(true, Ordering::SeqCst);
            }),
        );

        vars.SetGlobal("score".to_string(), Value::new_int(2));
        assert!(called.load(Ordering::SeqCst));
    }
}
