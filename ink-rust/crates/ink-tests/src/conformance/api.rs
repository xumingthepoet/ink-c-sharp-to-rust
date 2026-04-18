use ink_runtime::Choice::Choice;
use ink_runtime::Path::Path;
use ink_runtime::Story::Story as RuntimeStory;
use ink_runtime::Value::{Value, ValueInput};
use std::any::Any;
use std::error::Error;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryError(pub String);

impl fmt::Display for StoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for StoryError {}

fn panic_to_string(payload: Box<dyn Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "story panicked".to_string()
    }
}

fn catch_story_error<T>(f: impl FnOnce() -> T) -> Result<T, StoryError> {
    catch_unwind(AssertUnwindSafe(f)).map_err(|payload| StoryError(panic_to_string(payload)))
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueType {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
}

pub trait FromValueType: Sized {
    fn from_value(value: &ValueType) -> Option<Self>;
}

impl FromValueType for i32 {
    fn from_value(value: &ValueType) -> Option<Self> {
        value.coerce_to_int()
    }
}

impl FromValueType for bool {
    fn from_value(value: &ValueType) -> Option<Self> {
        value.coerce_to_bool()
    }
}

impl FromValueType for String {
    fn from_value(value: &ValueType) -> Option<Self> {
        match value {
            ValueType::String(text) => Some(text.clone()),
            ValueType::Int(value) => Some(value.to_string()),
            ValueType::Float(value) => Some(value.to_string()),
            ValueType::Bool(value) => Some(value.to_string()),
        }
    }
}

impl ValueType {
    pub fn new<T: Into<ValueType>>(value: T) -> Self {
        value.into()
    }

    pub fn coerce_to_int(&self) -> Option<i32> {
        match self {
            ValueType::Int(value) => Some(*value),
            ValueType::Bool(value) => Some(if *value { 1 } else { 0 }),
            ValueType::Float(value) => Some(*value as i32),
            ValueType::String(text) => text.parse::<i32>().ok(),
        }
    }

    pub fn coerce_to_bool(&self) -> Option<bool> {
        match self {
            ValueType::Bool(value) => Some(*value),
            ValueType::Int(value) => Some(*value != 0),
            ValueType::Float(value) => Some(*value != 0.0),
            ValueType::String(text) => Some(!text.is_empty()),
        }
    }

    pub fn get<T: FromValueType>(&self) -> Option<T> {
        T::from_value(self)
    }

    fn into_runtime_value(self) -> ValueInput {
        match self {
            ValueType::Bool(value) => ValueInput::Bool(value),
            ValueType::Int(value) => ValueInput::Int(value),
            ValueType::Float(value) => ValueInput::Float(value),
            ValueType::String(value) => ValueInput::String(value),
        }
    }

    fn from_runtime_value(value: Value) -> Self {
        match value {
            Value::Bool(value) => ValueType::Bool(value.value),
            Value::Int(value) => ValueType::Int(value.value),
            Value::Float(value) => ValueType::Float(value.value),
            Value::String(value) => ValueType::String(value.value),
            other => ValueType::String(other.to_string()),
        }
    }
}

impl From<&str> for ValueType {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for ValueType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for ValueType {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i32> for ValueType {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<f32> for ValueType {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

pub trait ExternalFunction: Send {
    fn call(&mut self, func_name: &str, args: Vec<ValueType>) -> Option<ValueType>;
}

pub trait VariableObserver: Send {
    fn changed(&mut self, variable_name: &str, new_value: &ValueType);
}

pub struct Story {
    inner: RuntimeStory,
}

impl Clone for Story {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Story {
    pub fn new(json: &str) -> Self {
        Self {
            inner: RuntimeStory::new_overload_2(json.to_string()),
        }
    }

    pub fn can_continue(&mut self) -> bool {
        self.inner.get_canContinue()
    }

    pub fn cont(&mut self) -> String {
        self.inner.Continue()
    }

    pub fn cont_maximally(&mut self) -> String {
        self.inner.ContinueMaximally()
    }

    pub fn continue_maximally(&mut self) -> String {
        self.cont_maximally()
    }

    pub fn choose_choice_index(&mut self, idx: usize) {
        self.inner.ChooseChoiceIndex(idx as i32);
    }

    pub fn choose_path_string(
        &mut self,
        path: &str,
        reset_callstack: bool,
        arguments: Option<Vec<ValueType>>,
    ) {
        let args = arguments
            .unwrap_or_default()
            .into_iter()
            .map(ValueType::into_runtime_value)
            .collect::<Vec<_>>();
        self.inner
            .ChoosePathString(path.to_string(), reset_callstack, args);
    }

    pub fn get_current_choices(&mut self) -> Vec<Choice> {
        self.inner.get_currentChoices()
    }

    pub fn get_current_tags(&mut self) -> Vec<String> {
        self.inner.get_currentTags()
    }

    pub fn get_current_errors(&mut self) -> Vec<String> {
        self.inner.get_currentErrors()
    }

    pub fn get_current_errors_ref(&mut self) -> Vec<String> {
        self.get_current_errors()
    }

    pub fn get_current_warnings(&mut self) -> Vec<String> {
        self.inner.get_currentWarnings()
    }

    pub fn get_current_flow_name(&mut self) -> String {
        self.inner.get_currentFlowName()
    }

    pub fn switch_flow(&mut self, flow_name: &str) {
        self.inner.SwitchFlow(flow_name.to_string());
    }

    pub fn remove_flow(&mut self, flow_name: &str) {
        self.inner.RemoveFlow(flow_name.to_string());
    }

    pub fn get_current_flow_is_default_flow(&mut self) -> bool {
        self.inner.get_currentFlowIsDefaultFlow()
    }

    pub fn get_current_text(&mut self) -> String {
        self.inner.get_currentText()
    }

    pub fn get_current_text_ref(&mut self) -> String {
        self.get_current_text()
    }

    pub fn get_global_tags(&mut self) -> Vec<String> {
        self.inner.get_globalTags()
    }

    pub fn tags_for_content_at_path(&mut self, path: &str) -> Vec<String> {
        self.inner.TagsForContentAtPath(path.to_string())
    }

    pub fn get_variable(&mut self, name: &str) -> Option<ValueType> {
        self.inner
            .get_variablesState()
            .GetVariableWithName(name.to_string())
            .map(ValueType::from_runtime_value)
    }

    pub fn set_variable(&mut self, name: &str, value: &ValueType) -> Result<(), StoryError> {
        catch_story_error(|| {
            self.inner
                .get_variablesState_mut()
                .SetIndexedValue(name.to_string(), Some(value.clone().into_runtime_value()));
        })
    }

    pub fn set_allow_external_function_fallbacks(&mut self, value: bool) {
        self.inner.set_allowExternalFunctionFallbacks(value);
    }

    pub fn evaluate_function(
        &mut self,
        function_name: &str,
        arguments: Option<Vec<ValueType>>,
        text_output: &mut String,
    ) -> Result<Option<ValueType>, StoryError> {
        let args = arguments
            .unwrap_or_default()
            .into_iter()
            .map(ValueType::into_runtime_value)
            .collect::<Vec<_>>();
        catch_story_error(|| {
            self.inner
                .EvaluateFunction_overload_2(function_name.to_string(), text_output, args)
                .map(ValueType::from_runtime_value)
        })
    }

    pub fn bind_external_function(
        &mut self,
        func_name: &str,
        func: Arc<Mutex<dyn ExternalFunction>>,
        lookahead_safe: bool,
    ) {
        let func_name = func_name.to_string();
        let func_name_for_callback = func_name.clone();
        let callback = {
            let func = func.clone();
            Arc::new(move |args: &[ValueInput]| {
                let args = args
                    .iter()
                    .cloned()
                    .filter_map(Value::Create)
                    .map(ValueType::from_runtime_value)
                    .collect::<Vec<_>>();
                let mut func = func.lock().unwrap();
                func.call(&func_name_for_callback, args)
                    .map(|value| match value {
                        ValueType::Bool(v) => Value::new_bool(v),
                        ValueType::Int(v) => Value::new_int(v),
                        ValueType::Float(v) => Value::new_float(v),
                        ValueType::String(v) => Value::new_string(v),
                    })
            })
        };

        self.inner
            .BindExternalFunctionGeneral(func_name, callback, lookahead_safe);
    }

    pub fn observe_variable(
        &mut self,
        variable_name: &str,
        observer: Arc<Mutex<dyn VariableObserver>>,
    ) {
        let variable_name = variable_name.to_string();
        let callback = {
            let observer = observer.clone();
            Arc::new(move |name: String, value: Value| {
                let converted = ValueType::from_runtime_value(value);
                observer.lock().unwrap().changed(&name, &converted);
            })
        };

        self.inner.ObserveVariable(variable_name, callback);
    }

    pub fn get_state(&mut self) -> ink_runtime::StoryState::StoryState {
        self.inner.get_state()
    }

    pub fn set_state(&mut self, state: ink_runtime::StoryState::StoryState) {
        self.inner.set_state(state);
    }

    pub fn save_state(&mut self) -> String {
        self.get_state().ToJson()
    }

    pub fn load_state(&mut self, json: &str) {
        let mut state = self.get_state();
        state.LoadJson(json.to_string());
        self.set_state(state);
    }

    pub fn get_visit_count_at_path_string(&mut self, path: &str) -> Result<i32, StoryError> {
        let path = Path::new_overload_4(path.to_string());
        let search_result = self.inner.ContentAtPath(path);
        let container = search_result
            .get_container()
            .cloned()
            .ok_or_else(|| StoryError("content path not found".to_string()))?;
        let mut state = self.get_state();
        Ok(state.VisitCountForContainer(container))
    }

    pub fn get_main_content_container(&mut self) -> ink_runtime::Container::Container {
        self.inner.get_mainContentContainer()
    }

    pub fn get_mainContentContainer(&mut self) -> ink_runtime::Container::Container {
        self.get_main_content_container()
    }

    pub fn build_string_of_hierarchy(&mut self) -> String {
        self.inner.BuildStringOfHierarchy()
    }

    pub fn get_can_continue(&mut self) -> bool {
        self.inner.get_canContinue()
    }

    pub fn get_current_choices_len(&mut self) -> usize {
        self.inner.get_currentChoices().len()
    }

    pub fn get_current_choices_ref(&mut self) -> Vec<Choice> {
        self.inner.get_currentChoices()
    }

    pub fn is_ended(&mut self) -> bool {
        !self.inner.get_canContinue() && self.inner.get_currentChoices().is_empty()
    }
}

pub mod story_error {
    pub use super::StoryError;
}

pub mod value_type {
    pub use super::{FromValueType, ValueType};
}

pub mod story {
    pub use super::{ExternalFunction, Story, VariableObserver};

    pub mod external_functions {
        pub use super::super::ExternalFunction;
    }

    pub mod variable_observer {
        pub use super::super::VariableObserver;
    }
}
