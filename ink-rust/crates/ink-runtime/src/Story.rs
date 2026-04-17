// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Story.cs

use crate::stub::*;
use crate::Choice::Choice;
use crate::ChoicePoint::ChoicePoint;
use crate::Container::{Container, ContentItem};
use crate::ControlCommand::{CommandType, ControlCommand};
use crate::Divert::Divert;
use crate::Glue::Glue;
use crate::ListDefinition::ListDefinition;
use crate::ListDefinitionsOrigin::ListDefinitionsOrigin;
use crate::NativeFunctionCall::NativeFunctionCall;
use crate::Path::Path;
use crate::Pointer::Pointer;
use crate::Profiler::Profiler;
use crate::SearchResult::SearchResult;
use crate::StoryException::StoryException;
use crate::StoryState::StoryState;
use crate::Tag::Tag;
use crate::Value::{StringValue, Value, ValueInput};
use crate::VariablesState::VariableObserver;
use crate::VariablesState::VariablesState;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

#[derive(Default)]
pub struct Story {
    main_content_container: Container,
    listDefinitions: ListDefinitionsOrigin,
    state: Option<Box<StoryState>>,
    state_snapshot_at_last_new_line: Option<Box<StoryState>>,
    temporary_evaluation_container: Option<Container>,
    _externals: HashMap<String, ExternalFunctionDef>,
    pub on_make_choice: Option<Arc<dyn Fn(Choice) + Send + Sync>>,
    pub on_evaluate_function: Option<Arc<dyn Fn(String, Vec<ValueInput>) + Send + Sync>>,
    pub on_complete_evaluate_function:
        Option<Arc<dyn Fn(String, Vec<ValueInput>, String, Option<Value>) + Send + Sync>>,
    pub on_choose_path_string: Option<Arc<dyn Fn(String, Vec<ValueInput>) + Send + Sync>>,
    _profiler: Option<Profiler>,
    recursive_continue_count: i32,
    async_continue_active: bool,
    saw_lookahead_unsafe_function_after_newline: bool,
    _has_validated_externals: bool,
    async_saving: bool,
    allowExternalFunctionFallbacks: bool,
    pub _port_marker: (),
}

impl std::fmt::Debug for Story {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Story")
            .field("main_content_container", &self.main_content_container)
            .field("listDefinitions", &self.listDefinitions)
            .field("state", &self.state.as_ref().map(|_| "<state>"))
            .field(
                "state_snapshot_at_last_new_line",
                &self
                    .state_snapshot_at_last_new_line
                    .as_ref()
                    .map(|_| "<snapshot>"),
            )
            .field(
                "temporary_evaluation_container",
                &self
                    .temporary_evaluation_container
                    .as_ref()
                    .map(|_| "<temporary>"),
            )
            .field(
                "on_make_choice",
                &self.on_make_choice.as_ref().map(|_| "<callback>"),
            )
            .field(
                "on_evaluate_function",
                &self.on_evaluate_function.as_ref().map(|_| "<callback>"),
            )
            .field(
                "on_complete_evaluate_function",
                &self
                    .on_complete_evaluate_function
                    .as_ref()
                    .map(|_| "<callback>"),
            )
            .field(
                "on_choose_path_string",
                &self.on_choose_path_string.as_ref().map(|_| "<callback>"),
            )
            .field("recursive_continue_count", &self.recursive_continue_count)
            .field("async_continue_active", &self.async_continue_active)
            .field(
                "saw_lookahead_unsafe_function_after_newline",
                &self.saw_lookahead_unsafe_function_after_newline,
            )
            .field("_has_validated_externals", &self._has_validated_externals)
            .field("async_saving", &self.async_saving)
            .field(
                "allowExternalFunctionFallbacks",
                &self.allowExternalFunctionFallbacks,
            )
            .finish()
    }
}

impl Clone for Story {
    fn clone(&self) -> Self {
        Self {
            main_content_container: self.main_content_container.clone(),
            listDefinitions: self.listDefinitions.clone(),
            state: None,
            state_snapshot_at_last_new_line: None,
            temporary_evaluation_container: None,
            _externals: self._externals.clone(),
            on_make_choice: self.on_make_choice.clone(),
            on_evaluate_function: self.on_evaluate_function.clone(),
            on_complete_evaluate_function: self.on_complete_evaluate_function.clone(),
            on_choose_path_string: self.on_choose_path_string.clone(),
            _profiler: None,
            recursive_continue_count: 0,
            async_continue_active: false,
            saw_lookahead_unsafe_function_after_newline: false,
            _has_validated_externals: false,
            async_saving: self.async_saving,
            allowExternalFunctionFallbacks: self.allowExternalFunctionFallbacks,
            _port_marker: (),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputStateChange {
    NoChange,
    NewlineRemoved,
    ExtendedBeyondNewline,
}

impl Default for OutputStateChange {
    fn default() -> Self {
        Self::NoChange
    }
}

#[derive(Clone, Default)]
pub struct ExternalFunctionDef {
    pub function: Option<crate::stub::ExternalFunction>,
    pub lookaheadSafe: bool,
}

impl Story {
    const INK_VERSION_CURRENT: i32 = 21;
    const INK_VERSION_MINIMUM_COMPATIBLE: i32 = 18;

    // C# signature: public Story (Container contentContainer, List<Runtime.ListDefinition> lists = null)
    pub fn new(_contentContainer: Container, lists: Vec<ListDefinition>) -> Self {
        let mut story = Self {
            main_content_container: _contentContainer,
            listDefinitions: ListDefinitionsOrigin::new(lists),
            state: None,
            state_snapshot_at_last_new_line: None,
            temporary_evaluation_container: None,
            _externals: HashMap::new(),
            on_make_choice: None,
            on_evaluate_function: None,
            on_complete_evaluate_function: None,
            on_choose_path_string: None,
            _profiler: None,
            recursive_continue_count: 0,
            async_continue_active: false,
            saw_lookahead_unsafe_function_after_newline: false,
            _has_validated_externals: false,
            async_saving: false,
            allowExternalFunctionFallbacks: false,
            _port_marker: (),
        };
        let state = StoryState::new(story.clone());
        story.state = Some(Box::new(state));
        story
    }

    // C# signature: public Story(string jsonString)
    pub fn new_overload_2(_jsonString: String) -> Self {
        let root_object = crate::SimpleJson::SimpleJson::TextToDictionary(_jsonString)
            .unwrap_or_else(|err| panic!("Failed to parse story JSON: {}", err));

        let version = match root_object.get("inkVersion") {
            Some(crate::SimpleJson::JsonValue::Int(value)) => *value,
            _ => panic!("ink version number not found. Are you sure it's a valid .ink.json file?"),
        };

        if version > Self::INK_VERSION_CURRENT {
            panic!("Version of ink used to build story was newer than the current version of the engine");
        } else if version < Self::INK_VERSION_MINIMUM_COMPATIBLE {
            panic!("Version of ink used to build story is too old to be loaded by this version of the engine");
        }

        let root_token = match root_object.get("root") {
            Some(token) => token.clone(),
            None => {
                panic!("Root node for ink not found. Are you sure it's a valid .ink.json file?")
            }
        };

        let list_definitions = match root_object.get("listDefs") {
            Some(token) => crate::JsonSerialisation::Json::JTokenToListDefinitions(token.clone()),
            None => ListDefinitionsOrigin::default(),
        };

        let main_content_container =
            match crate::JsonSerialisation::Json::JTokenToRuntimeObject(root_token) {
                Some(crate::Container::ContentItem::Container(container)) => *container,
                _ => panic!("Root node for ink was not a container"),
            };

        let mut story = Self {
            main_content_container,
            listDefinitions: list_definitions,
            state: None,
            state_snapshot_at_last_new_line: None,
            temporary_evaluation_container: None,
            _externals: HashMap::new(),
            on_make_choice: None,
            on_evaluate_function: None,
            on_complete_evaluate_function: None,
            on_choose_path_string: None,
            _profiler: None,
            recursive_continue_count: 0,
            async_continue_active: false,
            saw_lookahead_unsafe_function_after_newline: false,
            _has_validated_externals: false,
            async_saving: false,
            allowExternalFunctionFallbacks: false,
            _port_marker: (),
        };
        story.ResetState();
        story
    }

    // C# signature: public Profiler StartProfiling()
    pub fn StartProfiling(&mut self) -> Profiler {
        self.IfAsyncWeCant("start profiling");
        let profiler = Profiler::new();
        self._profiler = Some(profiler.clone());
        profiler
    }

    // C# signature: public void EndProfiling()
    pub fn EndProfiling(&mut self) {
        self._profiler = None;
    }

    // C# signature: public string ToJson()
    pub fn ToJson(&mut self) -> String {
        let mut writer = crate::SimpleJson::Writer::new();
        self.WriteJson(&mut writer);
        writer.ToString()
    }

    // C# signature: public void ToJson(Stream stream)
    pub fn ToJson_overload_2(&mut self, stream: Box<dyn std::io::Write>) {
        let mut writer = crate::SimpleJson::Writer::new_overload_2(stream);
        self.WriteJson(&mut writer);
    }

    fn WriteJson(&mut self, writer: &mut crate::SimpleJson::Writer) {
        writer
            .WriteObject(|writer| {
                writer.WriteProperty_overload_4(
                    "inkVersion".to_string(),
                    Self::INK_VERSION_CURRENT,
                )?;
                writer.WritePropertyStart("root".to_string())?;
                crate::JsonSerialisation::Json::WriteRuntimeContainer(
                    writer,
                    &self.main_content_container,
                    false,
                );
                writer.WritePropertyEnd()?;

                if !self.listDefinitions.get_lists().is_empty() {
                    writer.WritePropertyStart("listDefs".to_string())?;
                    writer.WriteObjectStart()?;
                    for def in self.listDefinitions.get_lists() {
                        writer.WritePropertyStart(def.get_name().to_string())?;
                        writer.WriteObjectStart()?;
                        let mut def = def.clone();
                        for (item, val) in def.get_items().clone() {
                            writer
                                .WriteProperty_overload_4(item.itemName.unwrap_or_default(), val)?;
                        }
                        writer.WriteObjectEnd()?;
                        writer.WritePropertyEnd()?;
                    }
                    writer.WriteObjectEnd()?;
                    writer.WritePropertyEnd()?;
                }

                Ok(())
            })
            .unwrap_or_else(|err| panic!("{}", err));
    }

    // C# signature: public void ResetState()
    pub fn ResetState(&mut self) {
        let story_snapshot = self.clone();
        self.state = Some(Box::new(StoryState::new(story_snapshot)));
        self.state_snapshot_at_last_new_line = None;
        self.temporary_evaluation_container = None;
        self._profiler = None;
        self.recursive_continue_count = 0;
        self.async_continue_active = false;
        self.saw_lookahead_unsafe_function_after_newline = false;
        self._has_validated_externals = false;
    }

    fn story_state_ref(&self) -> &StoryState {
        self.state.as_ref().expect("story state not initialized")
    }

    fn story_state_mut(&mut self) -> &mut StoryState {
        self.state.as_mut().expect("story state not initialized")
    }

    fn IfAsyncWeCant(&self, activity_str: &str) {
        if self.async_continue_active {
            panic!(
                "Can't {}. Story is in the middle of a ContinueAsync(). Make more ContinueAsync() calls or a single Continue() call beforehand.",
                activity_str
            );
        }
    }

    fn main_content_container_ref(&self) -> &Container {
        self.temporary_evaluation_container
            .as_ref()
            .unwrap_or(&self.main_content_container)
    }

    // C# signature: public void ResetCallstack()
    pub fn ResetCallstack(&mut self) {
        if let Some(state) = self.state.as_mut() {
            state.ForceEnd();
        }
    }

    // C# signature: public void SwitchFlow(string flowName)
    pub fn SwitchFlow(&mut self, flowName: String) {
        if let Some(state) = self.state.as_mut() {
            state.SwitchFlow_Internal(flowName);
        }
    }

    // C# signature: public void RemoveFlow(string flowName)
    pub fn RemoveFlow(&mut self, flowName: String) {
        if let Some(state) = self.state.as_mut() {
            state.RemoveFlow_Internal(flowName);
        }
    }

    // C# signature: public void SwitchToDefaultFlow()
    pub fn SwitchToDefaultFlow(&mut self) {
        if let Some(state) = self.state.as_mut() {
            state.SwitchToDefaultFlow_Internal();
        }
    }

    // C# signature: public string Continue()
    pub fn Continue(&mut self) -> String {
        self.ContinueAsync(0.0);
        self.get_currentText()
    }

    // C# signature: public void ContinueAsync (float millisecsLimitAsync)
    pub fn ContinueAsync(&mut self, millisecsLimitAsync: f32) {
        if !self._has_validated_externals {
            self.ValidateExternalBindings();
            self._has_validated_externals = true;
        }
        self.ContinueInternal(millisecsLimitAsync);
    }

    // C# signature: public string ContinueMaximally()
    pub fn ContinueMaximally(&mut self) -> String {
        self.IfAsyncWeCant("ContinueMaximally");
        let mut sb = String::new();
        while self.get_canContinue() {
            sb.push_str(&self.Continue());
        }
        sb
    }

    // C# signature: public SearchResult ContentAtPath(Path path)
    pub fn ContentAtPath(&mut self, _path: Path) -> SearchResult {
        let mut container = self.main_content_container_ref().clone();
        container.ContentAtPath(_path, 0, -1)
    }

    // C# signature: public Runtime.Container KnotContainerWithName (string name)
    pub fn KnotContainerWithName(&mut self, _name: String) -> Option<Container> {
        self.main_content_container_ref()
            .get_namedContent()
            .get(&_name)
            .and_then(|content| match content {
                crate::Container::ContentItem::Container(container) => {
                    Some(container.as_ref().clone())
                }
                _ => None,
            })
    }

    // C# signature: public Pointer PointerAtPath (Path path)
    pub fn PointerAtPath(&mut self, _path: Path) -> Pointer {
        if _path.get_length() == 0 {
            return Pointer::Null();
        }

        let mut path_length_to_use = _path.get_length();
        let result = if _path
            .get_lastComponent()
            .map(|component| component.get_isIndex())
            .unwrap_or(false)
        {
            path_length_to_use -= 1;
            let mut container = self.main_content_container_ref().clone();
            container.ContentAtPath(_path.clone(), 0, path_length_to_use)
        } else {
            let mut container = self.main_content_container_ref().clone();
            container.ContentAtPath(_path.clone(), 0, -1)
        };

        let mut pointer = if let Some(container) = result.get_container() {
            Pointer::StartOf(container.clone())
        } else {
            Pointer::Null()
        };

        if _path
            .get_lastComponent()
            .map(|component| component.get_isIndex())
            .unwrap_or(false)
        {
            pointer.index = _path
                .get_lastComponent()
                .map(|component| component.get_index())
                .unwrap_or(-1);
        } else {
            pointer.index = -1;
        }

        pointer
    }

    // C# signature: public StoryState CopyStateForBackgroundThreadSave()
    pub fn CopyStateForBackgroundThreadSave(&mut self) -> StoryState {
        let state = self
            .state
            .as_ref()
            .cloned()
            .unwrap_or_else(|| Box::new(StoryState::new(self.clone())));
        let state_to_save = (*state).clone();
        self.state = Some(Box::new(state_to_save.clone().CopyAndStartPatching(true)));
        self.async_saving = true;
        state_to_save
    }

    // C# signature: public void BackgroundSaveComplete()
    pub fn BackgroundSaveComplete(&mut self) {
        if let Some(state) = self.state.as_mut() {
            state.ApplyAnyPatch();
        }
        self.async_saving = false;
    }

    // C# signature: public void ChoosePathString (string path, bool resetCallstack = true, params object [] arguments)
    pub fn ChoosePathString(
        &mut self,
        path: String,
        resetCallstack: bool,
        arguments: Vec<ValueInput>,
    ) {
        self.IfAsyncWeCant("call ChoosePathString right now");
        if let Some(callback) = &self.on_choose_path_string {
            callback(path.clone(), arguments.clone());
        }
        if resetCallstack {
            self.ResetCallstack();
        } else if self
            .story_state_ref()
            .get_callStack()
            .currentElement()
            .r#type
            == crate::PushPop::PushPopType::Function
        {
            let mut func_detail = String::new();
            if let Some(container) = self
                .story_state_ref()
                .get_callStack()
                .currentElement()
                .currentPointer
                .container
                .clone()
            {
                func_detail = format!("({}) ", container.get_path().ToString());
            }
            panic!(
                "Story was running a function {}when you called ChoosePathString({}) - this is almost certainly not not what you want! Full stack trace: \n{}",
                func_detail,
                path,
                self.story_state_ref().get_callStack().get_callStackTrace()
            );
        }

        self.story_state_mut()
            .PassArgumentsToEvaluationStack(Self::value_inputs_to_runtime_objects(arguments));
        self.ChoosePath(Path::new_overload_4(path), true);
    }

    // C# signature: public void ChoosePath(Path p, bool incrementingTurnIndex = true)
    pub fn ChoosePath(&mut self, p: Path, incrementingTurnIndex: bool) {
        if let Some(state) = self.state.as_mut() {
            state.ChoosePath(p, incrementingTurnIndex);
        }
    }

    // C# signature: public void ChooseChoiceIndex(int choiceIdx)
    pub fn ChooseChoiceIndex(&mut self, choiceIdx: i32) {
        let choices = self.get_currentChoices();
        assert!(
            choiceIdx >= 0 && (choiceIdx as usize) < choices.len(),
            "choice out of range"
        );
        let choice_to_choose = choices[choiceIdx as usize].clone();
        if let Some(callback) = &self.on_make_choice {
            callback(choice_to_choose);
        }
        if let Some(state) = self.state.as_mut() {
            state.ChooseChoiceIndex(choiceIdx);
        }
    }

    // C# signature: public bool HasFunction (string functionName)
    pub fn HasFunction(&mut self, _functionName: String) -> bool {
        self.KnotContainerWithName(_functionName).is_some()
    }

    // C# signature: public object EvaluateFunction (string functionName, params object [] arguments)
    pub fn EvaluateFunction(
        &mut self,
        functionName: String,
        arguments: Vec<ValueInput>,
    ) -> Option<Value> {
        if let Some(callback) = &self.on_evaluate_function {
            callback(functionName.clone(), arguments.clone());
        }
        let mut text_output = String::new();
        self.EvaluateFunction_overload_2(functionName, &mut text_output, arguments)
    }

    // C# signature: public object EvaluateFunction (string functionName, out string textOutput, params object [] arguments)
    pub fn EvaluateFunction_overload_2(
        &mut self,
        functionName: String,
        textOutput: &mut String,
        arguments: Vec<ValueInput>,
    ) -> Option<Value> {
        self.IfAsyncWeCant("evaluate a function");

        if functionName.is_empty() || functionName.trim().is_empty() {
            panic!("Function is empty or white space.");
        }

        let funcContainer = self
            .KnotContainerWithName(functionName.clone())
            .unwrap_or_else(|| panic!("Function doesn't exist: '{}'", functionName));

        let outputStreamBefore = self.story_state_ref().get_outputStream();
        self.story_state_mut().ResetOutput(Vec::new());

        self.story_state_mut().StartFunctionEvaluationFromGame(
            funcContainer,
            Self::value_inputs_to_runtime_objects(arguments.clone()),
        );

        let mut stringOutput = String::new();
        while self.get_canContinue() {
            stringOutput.push_str(&self.Continue());
        }
        *textOutput = stringOutput;

        self.story_state_mut().ResetOutput(outputStreamBefore);
        let result = self.story_state_mut().CompleteFunctionEvaluationFromGame();
        if let Some(callback) = &self.on_complete_evaluate_function {
            callback(functionName, arguments, textOutput.clone(), result.clone());
        }
        result
    }

    // C# signature: public Runtime.Object EvaluateExpression(Runtime.Container exprContainer)
    pub fn EvaluateExpression(&mut self, exprContainer: Container) -> Option<Value> {
        let startCallStackHeight = self.story_state_ref().get_callStack().currentElementIndex();
        self.story_state_mut()
            .PushCallstack(crate::PushPop::PushPopType::Tunnel, 0, 0);

        self.temporary_evaluation_container = Some(exprContainer);
        self.story_state_mut().GoToStart();

        let evalStackHeight = self.story_state_ref().get_evaluationStack().len();
        self.Continue();
        self.temporary_evaluation_container = None;

        if self.story_state_ref().get_callStack().currentElementIndex() > startCallStackHeight {
            self.story_state_mut().PopCallstack(None);
        }

        let endStackHeight = self.story_state_ref().get_evaluationStack().len();
        if endStackHeight > evalStackHeight {
            match self.story_state_mut().PopEvaluationStack() {
                ContentItem::Value(value) => Some(value),
                _ => None,
            }
        } else {
            None
        }
    }

    // C# signature: public bool TryGetExternalFunction(string functionName, out ExternalFunction externalFunction)
    pub fn TryGetExternalFunction(
        &mut self,
        functionName: String,
        externalFunction: &mut crate::stub::ExternalFunction,
    ) -> bool {
        if let Some(externalFunctionDef) = self._externals.get(&functionName) {
            if let Some(function) = &externalFunctionDef.function {
                *externalFunction = function.clone();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    // C# signature: public void CallExternalFunction(string funcName, int numberOfArguments)
    pub fn CallExternalFunction(&mut self, funcName: String, numberOfArguments: i32) {
        let Some(funcDef) = self._externals.get(&funcName) else {
            panic!(
                "Trying to call EXTERNAL function '{}' which has not been bound.",
                funcName
            );
        };

        if let Some(state) = self.state.as_mut() {
            if !funcDef.lookaheadSafe && state.get_inStringEvaluation() {
                self.Error(
                    format!(
                        "External function {} could not be called because it wasn't marked as lookaheadSafe when BindExternalFunction was called and the story is in the middle of string generation.",
                        funcName
                    ),
                    false,
                );
                return;
            }

            let mut arguments = Vec::new();
            for _ in 0..numberOfArguments {
                let popped = state.PopEvaluationStack();
                if let crate::Container::ContentItem::Value(value) = popped {
                    arguments.push(value);
                }
            }
            arguments.reverse();

            if let Some(function) = &funcDef.function {
                let return_value = (function)(&arguments);
                match return_value {
                    Some(value) => {
                        state.PushEvaluationStack(crate::Container::ContentItem::Value(value))
                    }
                    None => state.PushEvaluationStack(crate::Container::ContentItem::Void(
                        crate::Void::Void::new(),
                    )),
                }
            } else {
                panic!(
                    "Trying to call EXTERNAL function '{}' which has not been bound.",
                    funcName
                );
            }
        }
    }

    // C# signature: public void BindExternalFunctionGeneral(string funcName, ExternalFunction func, bool lookaheadSafe = true)
    pub fn BindExternalFunctionGeneral(
        &mut self,
        funcName: String,
        func: crate::stub::ExternalFunction,
        lookaheadSafe: bool,
    ) {
        self.IfAsyncWeCant("bind an external function");
        assert!(
            !self._externals.contains_key(&funcName),
            "Function '{}' has already been bound.",
            funcName
        );
        self._externals.insert(
            funcName,
            ExternalFunctionDef {
                function: Some(func),
                lookaheadSafe,
            },
        );
    }

    // C# signature: public void BindExternalFunction(string funcName, Func<object> func, bool lookaheadSafe=false)
    pub fn BindExternalFunction(
        &mut self,
        funcName: String,
        func: crate::stub::ExternalFunction,
        lookaheadSafe: bool,
    ) {
        self.BindExternalFunctionGeneral(funcName, func, lookaheadSafe);
    }

    // C# signature: public void BindExternalFunction(string funcName, Action act, bool lookaheadSafe=false)
    pub fn BindExternalFunction_overload_2(
        &mut self,
        funcName: String,
        act: crate::stub::ExternalFunction,
        lookaheadSafe: bool,
    ) {
        self.BindExternalFunctionGeneral(funcName, act, lookaheadSafe);
    }

    // C# signature: public void UnbindExternalFunction(string funcName)
    pub fn UnbindExternalFunction(&mut self, funcName: String) {
        self._externals.remove(&funcName);
    }

    // C# signature: public void ValidateExternalBindings()
    pub fn ValidateExternalBindings(&mut self) {
        let mut missing_externals = std::collections::HashSet::new();
        self.ValidateExternalBindings_container(
            &self.main_content_container,
            &mut missing_externals,
        );

        if !missing_externals.is_empty() {
            let mut missing = missing_externals.into_iter().collect::<Vec<_>>();
            missing.sort();
            self.Error(
                format!(
                    "ERROR: Missing function binding for external{}: '{}'",
                    if missing.len() > 1 { "s" } else { "" },
                    missing.join("', '")
                ),
                false,
            );
        }
    }

    // C# signature: public void ObserveVariable(string variableName, VariableObserver observer)
    pub fn ObserveVariable(&mut self, variableName: String, observer: VariableObserver) {
        if let Some(state) = self.state.as_mut() {
            state.ObserveVariable(variableName, observer);
        }
    }

    // C# signature: public void ObserveVariables(IList<string> variableNames, VariableObserver observer)
    pub fn ObserveVariables(&mut self, variableNames: Vec<String>, observer: VariableObserver) {
        if let Some(state) = self.state.as_mut() {
            state.ObserveVariables(variableNames, observer);
        }
    }

    // C# signature: public void RemoveVariableObserver(VariableObserver observer = null, string specificVariableName = null)
    pub fn RemoveVariableObserver(
        &mut self,
        observer: VariableObserver,
        specificVariableName: String,
    ) {
        if let Some(state) = self.state.as_mut() {
            let observer_ref = Some(&observer);
            let specific_ref = if specificVariableName.is_empty() {
                None
            } else {
                Some(specificVariableName.as_str())
            };
            state.RemoveVariableObserver(observer_ref, specific_ref);
        }
    }

    fn ValidateExternalBindings_container(
        &self,
        container: &Container,
        missingExternals: &mut std::collections::HashSet<String>,
    ) {
        for innerContent in container.get_content() {
            if let crate::Container::ContentItem::Container(child) = innerContent {
                if !child.get_hasValidName() {
                    self.ValidateExternalBindings_content(innerContent, missingExternals);
                }
            } else {
                self.ValidateExternalBindings_content(innerContent, missingExternals);
            }
        }

        for innerKeyValue in container.get_namedContent().values() {
            self.ValidateExternalBindings_content(innerKeyValue, missingExternals);
        }
    }

    fn ValidateExternalBindings_content(
        &self,
        content: &crate::Container::ContentItem,
        missingExternals: &mut std::collections::HashSet<String>,
    ) {
        if let crate::Container::ContentItem::Container(container) = content {
            self.ValidateExternalBindings_container(container, missingExternals);
            return;
        }

        if let crate::Container::ContentItem::Divert(divert) = content {
            if divert.get_isExternal() {
                let name = divert.get_targetPathString();
                if !self._externals.contains_key(&name) {
                    if self.allowExternalFunctionFallbacks {
                        if !self
                            .main_content_container
                            .get_namedContent()
                            .contains_key(&name)
                        {
                            missingExternals.insert(name);
                        }
                    } else {
                        missingExternals.insert(name);
                    }
                }
            }
        }
    }

    // C# signature: public List<string> TagsForContentAtPath (string path)
    pub fn TagsForContentAtPath(&mut self, path: String) -> Vec<String> {
        self.TagsAtStartOfFlowContainerWithPathString(path)
    }

    // C# signature: public virtual string BuildStringOfHierarchy()
    pub fn BuildStringOfHierarchy(&mut self) -> String {
        let mut container = self.main_content_container_ref().clone();
        container.BuildStringOfHierarchy_overload_2()
    }

    // C# signature: private void NextContent()
    pub fn NextContent(&mut self) {
        self.next_content().unwrap();
    }

    fn reset_errors(&mut self) {
        self.story_state_mut().ResetErrors();
    }

    fn state_snapshot(&mut self) {
        let mut tmp_state = self.story_state_mut().CopyAndStartPatching(false);
        let current_state = self.state.as_mut().expect("story state not initialized");
        std::mem::swap(&mut tmp_state, current_state);
        self.state_snapshot_at_last_new_line = Some(Box::new(tmp_state));
    }

    fn restore_state_snapshot(&mut self) {
        if let Some(snapshot) = self.state_snapshot_at_last_new_line.as_mut() {
            snapshot.RestoreAfterPatch();
        }
        self.state = Some(self.state_snapshot_at_last_new_line.take().unwrap());
        if !self.async_saving {
            self.story_state_mut().ApplyAnyPatch();
        }
    }

    fn discard_snapshot(&mut self) {
        if !self.async_saving {
            self.story_state_mut().ApplyAnyPatch();
        }
        self.state_snapshot_at_last_new_line = None;
    }

    fn calculate_newline_output_state_change(
        prev_text: &str,
        curr_text: &str,
        prev_tag_count: i32,
        curr_tag_count: i32,
    ) -> OutputStateChange {
        let newline_still_exists = curr_text.len() >= prev_text.len()
            && !prev_text.is_empty()
            && curr_text.as_bytes()[prev_text.len() - 1] == b'\n';
        if prev_tag_count == curr_tag_count
            && prev_text.len() == curr_text.len()
            && newline_still_exists
        {
            return OutputStateChange::NoChange;
        }
        if !newline_still_exists {
            return OutputStateChange::NewlineRemoved;
        }
        if curr_tag_count > prev_tag_count {
            return OutputStateChange::ExtendedBeyondNewline;
        }
        for c in curr_text.as_bytes().iter().skip(prev_text.len()) {
            if *c != b' ' && *c != b'\t' {
                return OutputStateChange::ExtendedBeyondNewline;
            }
        }
        OutputStateChange::NoChange
    }

    fn visit_container(&mut self, container: &Container, at_start: bool) {
        if !container.get_countingAtStartOnly() || at_start {
            if container.get_visitsShouldBeCounted() {
                self.story_state_mut()
                    .IncrementVisitCountForContainer(container.clone());
            }
            if container.get_turnIndexShouldBeCounted() {
                self.story_state_mut()
                    .RecordTurnIndexVisitToContainer(container.clone());
            }
        }
    }

    fn get_container_ancestry_paths(container: &Container) -> Vec<String> {
        let mut paths = Vec::new();
        let mut current = Some(container.clone());
        while let Some(curr) = current {
            paths.push(curr.get_path().ToString());
            current = curr.get_parent().cloned();
        }
        paths
    }

    fn visit_changed_containers_due_to_divert(&mut self) {
        let previous_pointer = self.story_state_ref().get_previousPointer();
        let pointer = self.story_state_ref().get_currentPointer();

        if pointer.get_isNull() || pointer.index == -1 {
            return;
        }

        let mut prev_ancestor_paths = HashSet::new();
        if !previous_pointer.get_isNull() {
            let mut prev_ancestor = previous_pointer.Resolve().and_then(|item| match item {
                ContentItem::Container(container) => Some(*container),
                _ => None,
            });
            if prev_ancestor.is_none() {
                prev_ancestor = previous_pointer.container.clone();
            }
            while let Some(prev) = prev_ancestor {
                prev_ancestor_paths.insert(prev.get_path().ToString());
                prev_ancestor = prev.get_parent().cloned();
            }
        }

        let Some(current_child_of_container) = pointer.Resolve() else {
            return;
        };
        let current_child_of_container = match current_child_of_container {
            ContentItem::Container(container) => *container,
            _ => return,
        };

        let mut current_container_ancestor = current_child_of_container.get_parent().cloned();

        while let Some(current_container) = current_container_ancestor {
            let current_path = current_container.get_path().ToString();
            if !prev_ancestor_paths.contains(&current_path)
                || current_container.get_countingAtStartOnly()
            {
                let entering_at_start = current_container
                    .get_content()
                    .first()
                    .and_then(|first| match first {
                        ContentItem::Container(container) => Some(container.get_path().ToString()),
                        _ => None,
                    })
                    .map(|first_path| {
                        first_path == current_child_of_container.get_path().ToString()
                    })
                    .unwrap_or(false);
                self.visit_container(&current_container, entering_at_start);
            }
            current_container_ancestor = current_container.get_parent().cloned();
        }
    }

    fn is_truthy(&self, obj: ContentItem) -> Result<bool, StoryException> {
        match obj {
            ContentItem::Value(value) => Ok(value.is_truthy()),
            ContentItem::Void(_) => Ok(false),
            other => Err(StoryException::new_overload_2(format!(
                "Unexpected object in truthy evaluation: {:?}",
                other
            ))),
        }
    }

    fn value_inputs_to_runtime_objects(arguments: Vec<ValueInput>) -> Vec<ContentItem> {
        arguments
            .into_iter()
            .filter_map(Value::Create)
            .map(ContentItem::Value)
            .collect()
    }

    fn process_choice(
        &mut self,
        choice_point: &mut ChoicePoint,
    ) -> Result<Option<Choice>, StoryException> {
        let mut show_choice = true;

        if choice_point.get_hasCondition() {
            let condition_value = self.story_state_mut().PopEvaluationStack();
            if !self.is_truthy(condition_value)? {
                show_choice = false;
            }
        }

        let mut start_text = String::new();
        let mut choice_only_text = String::new();
        let mut tags: Vec<String> = Vec::new();

        if choice_point.get_hasChoiceOnlyContent() {
            choice_only_text = self.pop_choice_string_and_tags(&mut tags);
        }

        if choice_point.get_hasStartContent() {
            start_text = self.pop_choice_string_and_tags(&mut tags);
        }

        if choice_point.get_onceOnly() {
            if let Some(choice_target) = choice_point.get_choiceTarget() {
                if self
                    .story_state_mut()
                    .VisitCountForContainer(choice_target.clone())
                    > 0
                {
                    show_choice = false;
                }
            }
        }

        if !show_choice {
            return Ok(None);
        }

        start_text.push_str(&choice_only_text);
        let choice = Choice {
            text: start_text.trim().to_string(),
            sourcePath: String::new(),
            index: 0,
            targetPath: choice_point.get_pathOnChoice(),
            threadAtGeneration: Some(self.story_state_mut().ForkThread()),
            originalThreadIndex: 0,
            isInvisibleDefault: choice_point.get_isInvisibleDefault(),
            tags,
        };

        Ok(Some(choice))
    }

    fn try_follow_default_invisible_choice(&mut self) -> Result<(), StoryException> {
        let all_choices = self.story_state_ref().get_currentChoices();
        let mut invisible_choices: Vec<Choice> = Vec::new();
        for choice in all_choices.iter() {
            if choice.isInvisibleDefault {
                invisible_choices.push(choice.clone());
            }
        }

        if invisible_choices.is_empty() || all_choices.len() > invisible_choices.len() {
            return Ok(());
        }

        let choice = invisible_choices[0].clone();
        if let Some(thread) = choice.get_threadAtGeneration() {
            self.story_state_mut().SetCurrentThread(thread.clone());
        }

        if self.state_snapshot_at_last_new_line.is_some() {
            let fork_thread = self.story_state_mut().ForkThread();
            self.story_state_mut().SetCurrentThread(fork_thread);
        }

        if let Some(target_path) = choice.targetPath.clone() {
            self.story_state_mut().ChoosePath(target_path, false);
        }

        Ok(())
    }

    fn pop_choice_string_and_tags(&mut self, tags: &mut Vec<String>) -> String {
        let obj = self.story_state_mut().PopEvaluationStack();
        let choice_only_str_val = match obj {
            ContentItem::Value(Value::String(value)) => value,
            other => panic!("Expected string for choice text, got {:?}", other),
        };

        while !self.story_state_ref().get_evaluationStack().is_empty()
            && matches!(
                self.story_state_mut().PeekEvaluationStack(),
                ContentItem::Tag(_)
            )
        {
            let tag = match self.story_state_mut().PopEvaluationStack() {
                ContentItem::Tag(tag) => tag,
                _ => unreachable!(),
            };
            tags.insert(0, tag.get_text().to_string());
        }

        choice_only_str_val.value
    }

    fn perform_logic_and_flow_control(
        &mut self,
        content_obj: &Option<ContentItem>,
    ) -> Result<bool, StoryException> {
        let Some(content_obj) = content_obj.clone() else {
            return Ok(false);
        };

        if let ContentItem::Divert(current_divert) = content_obj.clone() {
            if current_divert.get_isConditional() {
                let value = self.story_state_mut().PopEvaluationStack();
                if !self.is_truthy(value)? {
                    return Ok(true);
                }
            }

            if current_divert.hasVariableTarget() {
                let var_name = current_divert
                    .get_variableDivertName()
                    .unwrap_or_default()
                    .to_string();
                let variable_value = self
                    .story_state_ref()
                    .get_variablesState()
                    .GetVariableWithName(var_name.clone());
                if let Some(var_contents) = variable_value {
                    if let Value::DivertTarget(target_value) = var_contents {
                        if let Some(target_path) = target_value.value {
                            let p = self.PointerAtPath(target_path);
                            self.story_state_mut().set_divertedPointer(p);
                        } else {
                            return Err(StoryException::new_overload_2(format!(
                                "Tried to divert to a target from a variable, but the variable ({}) was empty/null.",
                                var_name
                            )));
                        }
                    } else {
                        return Err(StoryException::new_overload_2(format!(
                            "Tried to divert to a target from a variable, but the variable ({}) didn't contain a divert target.",
                            var_name
                        )));
                    }
                } else {
                    return Err(StoryException::new_overload_2(format!(
                        "Tried to divert using a target from a variable that could not be found ({})",
                        var_name
                    )));
                }
            } else if current_divert.get_isExternal() {
                self.CallExternalFunction(
                    current_divert.get_targetPathString(),
                    current_divert.get_externalArgs(),
                );
                return Ok(true);
            } else {
                self.story_state_mut()
                    .set_divertedPointer(current_divert.get_targetPointer());
            }

            if current_divert.get_pushesToStack() {
                let output_len = self.story_state_ref().get_outputStream().len() as i32;
                self.story_state_mut().PushCallstack(
                    current_divert.get_stackPushType(),
                    0,
                    output_len,
                );
            }

            return Ok(true);
        }

        if let ContentItem::ControlCommand(eval_command) = content_obj.clone() {
            match eval_command.get_commandType() {
                CommandType::EvalStart => {
                    if self.story_state_ref().get_inExpressionEvaluation() {
                        return Err(StoryException::new_overload_2(
                            "Already in expression evaluation?".to_string(),
                        ));
                    }
                    self.story_state_mut().set_inExpressionEvaluation(true);
                }
                CommandType::EvalOutput => {
                    if !self.story_state_ref().get_evaluationStack().is_empty() {
                        let output = self.story_state_mut().PopEvaluationStack();
                        if !matches!(output, ContentItem::Void(_)) {
                            let text = Value::new_string(format!("{:?}", output));
                            self.story_state_mut()
                                .PushToOutputStream(ContentItem::Value(text));
                        }
                    }
                }
                CommandType::EvalEnd => {
                    if !self.story_state_ref().get_inExpressionEvaluation() {
                        return Err(StoryException::new_overload_2(
                            "Not in expression evaluation mode".to_string(),
                        ));
                    }
                    self.story_state_mut().set_inExpressionEvaluation(false);
                }
                CommandType::Duplicate => {
                    let obj = self.story_state_mut().PeekEvaluationStack();
                    self.story_state_mut().PushEvaluationStack(obj);
                }
                CommandType::PopEvaluatedValue => {
                    self.story_state_mut().PopEvaluationStack();
                }
                CommandType::PopFunction | CommandType::PopTunnel => {
                    let pop_type = if eval_command.get_commandType() == CommandType::PopFunction {
                        crate::PushPop::PushPopType::Function
                    } else {
                        crate::PushPop::PushPopType::Tunnel
                    };

                    let mut override_tunnel_return_target = None;
                    if pop_type == crate::PushPop::PushPopType::Tunnel {
                        let popped = self.story_state_mut().PopEvaluationStack();
                        if let ContentItem::Value(Value::DivertTarget(divert_target)) = popped {
                            override_tunnel_return_target = divert_target.value;
                        } else if !matches!(popped, ContentItem::Void(_)) {
                            return Err(StoryException::new_overload_2(
                                "Expected void if ->-> doesn't override target".to_string(),
                            ));
                        }
                    }

                    if self.story_state_mut().TryExitFunctionEvaluationFromGame() {
                        return Ok(true);
                    }

                    self.story_state_mut().PopCallstack(Some(pop_type));

                    if let Some(override_target) = override_tunnel_return_target {
                        let p = self.PointerAtPath(override_target);
                        self.story_state_mut().set_divertedPointer(p);
                    }
                }
                CommandType::BeginString => {
                    self.story_state_mut()
                        .PushToOutputStream(ContentItem::ControlCommand(eval_command.clone()));
                    if !self.story_state_ref().get_inExpressionEvaluation() {
                        return Err(StoryException::new_overload_2(
                            "Expected to be in an expression when evaluating a string".to_string(),
                        ));
                    }
                    self.story_state_mut().set_inExpressionEvaluation(false);
                }
                CommandType::EndString => {
                    let mut content_stack_for_string: VecDeque<ContentItem> = VecDeque::new();
                    let mut content_to_retain: VecDeque<ContentItem> = VecDeque::new();
                    let mut output_count_consumed = 0;
                    for obj in self.story_state_ref().get_outputStream().iter().rev() {
                        output_count_consumed += 1;
                        if matches!(obj, ContentItem::ControlCommand(command) if command.get_commandType() == CommandType::BeginString)
                        {
                            break;
                        }
                        if matches!(obj, ContentItem::Tag(_)) {
                            content_to_retain.push_back(obj.clone());
                        } else {
                            content_stack_for_string.push_front(obj.clone());
                        }
                    }
                    while let Some(obj) = content_stack_for_string.pop_front() {
                        self.story_state_mut().PushToOutputStream(obj);
                    }
                    while let Some(obj) = content_to_retain.pop_front() {
                        self.story_state_mut().PushToOutputStream(obj);
                    }
                    self.story_state_mut()
                        .PopFromOutputStream(output_count_consumed as i32);
                    self.story_state_mut()
                        .PushToOutputStream(ContentItem::ControlCommand(eval_command.clone()));
                }
                CommandType::NoOp => {}
                CommandType::ChoiceCount => {
                    let choice_count = self.story_state_ref().get_currentChoices().len() as i32;
                    self.story_state_mut()
                        .PushEvaluationStack(ContentItem::Value(Value::new_int(choice_count)));
                }
                CommandType::Turns => {
                    let current_turn_index = self.story_state_ref().get_currentTurnIndex();
                    self.story_state_mut()
                        .PushEvaluationStack(ContentItem::Value(Value::new_int(
                            current_turn_index,
                        )));
                }
                CommandType::TurnsSince => {}
                CommandType::ReadCount => {}
                CommandType::Random => {}
                CommandType::SeedRandom => {}
                CommandType::VisitIndex => {}
                CommandType::SequenceShuffleIndex => {}
                CommandType::StartThread => {
                    let output_len = self.story_state_ref().get_outputStream().len() as i32;
                    self.story_state_mut().PushCallstack(
                        crate::PushPop::PushPopType::Tunnel,
                        0,
                        output_len,
                    );
                    self.story_state_mut().PushThread();
                }
                CommandType::Done | CommandType::End => {
                    self.story_state_mut().ForceEnd();
                }
                CommandType::ListFromInt | CommandType::ListRange | CommandType::ListRandom => {}
                CommandType::BeginTag | CommandType::EndTag => {
                    self.story_state_mut()
                        .PushToOutputStream(ContentItem::ControlCommand(eval_command.clone()));
                }
                CommandType::TOTAL_VALUES => {}
                CommandType::NotSet => {}
            }
            return Ok(true);
        }

        if let ContentItem::ChoicePoint(mut choice_point) = content_obj {
            let choice = self.process_choice(&mut choice_point)?;
            if let Some(choice) = choice {
                self.story_state_mut().AddGeneratedChoice(choice);
            }
            return Ok(false);
        }

        Ok(false)
    }

    fn continue_single_step(&mut self) -> Result<bool, StoryException> {
        self.step()?;

        if !self.get_canContinue() {
            self.try_follow_default_invisible_choice()?;
        }

        if !self.story_state_ref().get_inStringEvaluation() {
            if let Some(snapshot) = self.state_snapshot_at_last_new_line.as_ref() {
                let mut snapshot = snapshot.as_ref().clone();
                let prev_text = snapshot.get_currentText();
                let curr_text = self.story_state_mut().get_currentText();
                let prev_tags = snapshot.get_currentTags().len() as i32;
                let curr_tags = self.story_state_mut().get_currentTags().len() as i32;
                let change = Self::calculate_newline_output_state_change(
                    &prev_text, &curr_text, prev_tags, curr_tags,
                );
                if change == OutputStateChange::ExtendedBeyondNewline
                    || self.saw_lookahead_unsafe_function_after_newline
                {
                    self.restore_state_snapshot();
                    return Ok(true);
                } else if change == OutputStateChange::NewlineRemoved {
                    self.state_snapshot_at_last_new_line = None;
                    self.discard_snapshot();
                }
            }

            if self.story_state_ref().get_outputStreamEndsInNewline() {
                if self.get_canContinue() {
                    if self.state_snapshot_at_last_new_line.is_none() {
                        self.state_snapshot();
                    }
                } else {
                    self.discard_snapshot();
                }
            }
        }

        Ok(false)
    }

    fn step(&mut self) -> Result<(), StoryException> {
        let mut should_add_to_stream = true;
        let mut pointer = self.story_state_ref().get_currentPointer();

        if pointer.get_isNull() {
            return Ok(());
        }

        let mut current_content_obj = pointer.Resolve();
        while let Some(ContentItem::Container(container)) = current_content_obj.clone() {
            self.visit_container(&container, true);
            if container.get_content().is_empty() {
                break;
            }
            pointer = Pointer::StartOf((*container).clone());
            current_content_obj = pointer.Resolve();
        }

        self.story_state_mut().set_currentPointer(pointer.clone());
        let is_logic_or_flow_control = self.perform_logic_and_flow_control(&current_content_obj)?;

        if self.story_state_ref().get_currentPointer().get_isNull() {
            return Ok(());
        }

        if is_logic_or_flow_control {
            should_add_to_stream = false;
        }

        if let Some(content_obj) = current_content_obj.clone() {
            if matches!(content_obj, ContentItem::Container(_)) {
                should_add_to_stream = false;
            }
            if let ContentItem::ChoicePoint(mut choice_point) = content_obj {
                let choice = self.process_choice(&mut choice_point)?;
                if let Some(choice) = choice {
                    self.story_state_mut().AddGeneratedChoice(choice);
                }
                should_add_to_stream = false;
            }
        }

        if should_add_to_stream {
            if let Some(content_obj) = current_content_obj.clone() {
                if let ContentItem::Value(Value::VariablePointer(var_pointer)) = &content_obj {
                    if var_pointer.contextIndex == -1 {
                        let context_idx = self
                            .story_state_ref()
                            .get_callStack()
                            .ContextForVariableNamed(var_pointer.value.clone().unwrap_or_default());
                        current_content_obj =
                            Some(ContentItem::Value(Value::new_variable_pointer(
                                var_pointer.value.clone(),
                                context_idx as i32,
                            )));
                    }
                }

                if self.story_state_ref().get_inExpressionEvaluation() {
                    self.story_state_mut()
                        .PushEvaluationStack(content_obj.clone());
                } else {
                    self.story_state_mut()
                        .PushToOutputStream(content_obj.clone());
                }
            }
        }

        self.next_content()?;

        if let Some(content_obj) = current_content_obj {
            if let ContentItem::ControlCommand(control_cmd) = content_obj {
                if control_cmd.get_commandType() == CommandType::StartThread {
                    self.story_state_mut().PushThread();
                }
            }
        }

        Ok(())
    }

    fn next_content(&mut self) -> Result<(), StoryException> {
        let cp = self.story_state_ref().get_currentPointer();
        self.story_state_mut().set_previousPointer(cp);

        if !self.story_state_ref().get_divertedPointer().get_isNull() {
            let dp = self.story_state_ref().get_divertedPointer();
            self.story_state_mut().set_currentPointer(dp);
            self.story_state_mut().set_divertedPointer(Pointer::Null());
            self.visit_changed_containers_due_to_divert();
            if !self.story_state_ref().get_currentPointer().get_isNull() {
                return Ok(());
            }
        }

        let successful_pointer_increment = self.increment_content_pointer();
        if !successful_pointer_increment {
            let mut did_pop = false;
            if self
                .story_state_ref()
                .get_callStack()
                .CanPop(Some(crate::PushPop::PushPopType::Function))
            {
                self.story_state_mut()
                    .PopCallstack(Some(crate::PushPop::PushPopType::Function));
                if self.story_state_ref().get_inExpressionEvaluation() {
                    self.story_state_mut()
                        .PushEvaluationStack(ContentItem::Void(crate::Void::Void::new()));
                }
                did_pop = true;
            } else if self.story_state_ref().get_callStack().canPopThread() {
                self.story_state_mut().PopThread();
                did_pop = true;
            } else {
                self.story_state_mut().TryExitFunctionEvaluationFromGame();
            }

            if did_pop && !self.story_state_ref().get_currentPointer().get_isNull() {
                self.next_content()?;
            }
        }

        Ok(())
    }

    fn increment_content_pointer(&mut self) -> bool {
        let mut successful_increment = true;
        let mut pointer = self.story_state_ref().get_currentPointer();
        pointer.index += 1;
        let mut container = match pointer.container.clone() {
            Some(container) => container,
            None => return false,
        };

        while pointer.index >= container.get_content().len() as i32 {
            successful_increment = false;
            let next_ancestor = container.get_parent().cloned();
            let Some(next_ancestor) = next_ancestor else {
                break;
            };

            let index_in_ancestor =
                next_ancestor
                    .get_content()
                    .iter()
                    .position(|content| match content {
                        ContentItem::Container(child) => child.get_path() == container.get_path(),
                        _ => false,
                    });

            let Some(index_in_ancestor) = index_in_ancestor else {
                break;
            };

            pointer = Pointer::new(next_ancestor, index_in_ancestor as i32);
            container = pointer.container.clone().unwrap();
            pointer.index += 1;
            successful_increment = true;
        }

        if !successful_increment {
            pointer = Pointer::Null();
        }

        self.story_state_mut().set_currentPointer(pointer);

        successful_increment
    }

    fn ContinueInternal(&mut self, millisecsLimitAsync: f32) {
        let is_async_time_limited = millisecsLimitAsync > 0.0;
        self.recursive_continue_count += 1;

        if !self.async_continue_active {
            self.async_continue_active = is_async_time_limited;
            if !self.get_canContinue() {
                panic!("Can't continue - should check canContinue before calling Continue");
            }
            self.story_state_mut().set_didSafeExit(false);
            self.story_state_mut().ResetOutput(Vec::new());
            if self.recursive_continue_count == 1 {
                self.story_state_mut().StartVariableObservation();
            }
        } else if self.async_continue_active && !is_async_time_limited {
            self.async_continue_active = false;
        }

        let start = Instant::now();
        let mut output_stream_ends_in_newline = false;
        self.saw_lookahead_unsafe_function_after_newline = false;

        loop {
            match self.continue_single_step() {
                Ok(r) => output_stream_ends_in_newline = r,
                Err(e) => {
                    self.story_state_mut().AddError(e.to_string(), false);
                    break;
                }
            }

            if output_stream_ends_in_newline {
                break;
            }
            if self.async_continue_active
                && start.elapsed().as_millis() as f32 > millisecsLimitAsync
            {
                break;
            }
            if !self.get_canContinue() {
                break;
            }
        }

        let mut changed_variables_to_observe = None;
        if output_stream_ends_in_newline || !self.get_canContinue() {
            if self.state_snapshot_at_last_new_line.is_some() {
                self.restore_state_snapshot();
            }

            if !self.get_canContinue() {
                if self.story_state_ref().get_currentChoices().is_empty()
                    && !self.story_state_ref().get_didSafeExit()
                    && self.temporary_evaluation_container.is_none()
                {
                    self.story_state_mut()
                        .AddError("unexpectedly reached end of content".to_string(), false);
                }
            }

            self.story_state_mut().set_didSafeExit(false);
            self.saw_lookahead_unsafe_function_after_newline = false;

            if self.recursive_continue_count == 1 {
                changed_variables_to_observe =
                    Some(self.story_state_mut().CompleteVariableObservation());
            }

            self.async_continue_active = false;
        }

        self.recursive_continue_count -= 1;

        if self.story_state_ref().get_hasError() || self.story_state_ref().get_hasWarning() {
            if self.story_state_ref().get_hasError() {
                let mut msg = String::from("Ink had ");
                msg.push_str(&self.story_state_ref().get_currentErrors().len().to_string());
                msg.push_str(" error");
                if self.story_state_ref().get_currentErrors().len() != 1 {
                    msg.push('s');
                }
                panic!("{}", msg);
            }
        }

        if let Some(changed) = changed_variables_to_observe {
            self.story_state_mut().NotifyObservers(changed);
        }
    }

    // C# signature: public void Error(string message, bool useEndLineNumber = false)
    pub fn Error(&mut self, message: String, useEndLineNumber: bool) {
        let mut err = StoryException::new_overload_2(message);
        err.useEndLineNumber = useEndLineNumber;
        panic!("{}", err);
    }

    // C# signature: public void Warning (string message)
    pub fn Warning(&mut self, message: String) {
        if let Some(state) = self.state.as_mut() {
            state.AddError(message, true);
        }
    }

    // C# signature: List<Choice> currentChoices { get; }
    pub fn get_currentChoices(&mut self) -> Vec<Choice> {
        self.state
            .as_ref()
            .map(|state| {
                let mut choices = Vec::new();
                for choice in state.get_currentChoices() {
                    if !choice.isInvisibleDefault {
                        let mut choice = choice;
                        choice.index = choices.len() as i32;
                        choices.push(choice);
                    }
                }
                choices
            })
            .unwrap_or_default()
    }

    // C# signature: string currentText { get; }
    pub fn get_currentText(&mut self) -> String {
        self.state
            .as_mut()
            .map(|state| state.get_currentText())
            .unwrap_or_default()
    }

    // C# signature: List<string> currentTags { get; }
    pub fn get_currentTags(&mut self) -> Vec<String> {
        self.state
            .as_mut()
            .map(|state| state.get_currentTags())
            .unwrap_or_default()
    }

    // C# signature: List<string> currentErrors { get; }
    pub fn get_currentErrors(&mut self) -> Vec<String> {
        self.state
            .as_ref()
            .map(|state| state.get_currentErrors())
            .unwrap_or_default()
    }

    // C# signature: List<string> currentWarnings { get; }
    pub fn get_currentWarnings(&mut self) -> Vec<String> {
        self.state
            .as_ref()
            .map(|state| state.get_currentWarnings())
            .unwrap_or_default()
    }

    // C# signature: bool currentFlowIsDefaultFlow { get; }
    pub fn get_currentFlowIsDefaultFlow(&mut self) -> bool {
        self.state
            .as_ref()
            .map(|state| state.get_currentFlowIsDefaultFlow())
            .unwrap_or(true)
    }

    // C# signature: List<string> aliveFlowNames { get; }
    pub fn get_aliveFlowNames(&mut self) -> Vec<String> {
        self.state
            .as_mut()
            .map(|state| state.get_aliveFlowNames())
            .unwrap_or_default()
    }

    // C# signature: bool hasError { get; }
    pub fn get_hasError(&mut self) -> bool {
        self.state
            .as_ref()
            .map(|state| state.get_hasError())
            .unwrap_or(false)
    }

    // C# signature: bool hasWarning { get; }
    pub fn get_hasWarning(&mut self) -> bool {
        self.state
            .as_ref()
            .map(|state| state.get_hasWarning())
            .unwrap_or(false)
    }

    // C# signature: VariablesState variablesState { get; }
    pub fn get_variablesState(&mut self) -> VariablesState {
        self.state
            .as_ref()
            .map(|state| state.get_variablesState())
            .unwrap_or_default()
    }

    // C# signature: ListDefinitionsOrigin listDefinitions { get; }
    pub fn get_listDefinitions(&mut self) -> ListDefinitionsOrigin {
        self.listDefinitions.clone()
    }

    // C# signature: StoryState state { get; }
    pub fn get_state(&mut self) -> StoryState {
        self.state
            .as_ref()
            .map(|state| *state.clone())
            .unwrap_or_default()
    }

    // C# signature: bool canContinue { get; }
    pub fn get_canContinue(&mut self) -> bool {
        self.state
            .as_ref()
            .map(|state| state.get_canContinue())
            .unwrap_or(false)
    }

    // C# signature: bool asyncContinueComplete { get; }
    pub fn get_asyncContinueComplete(&mut self) -> bool {
        true
    }

    // C# signature: bool allowExternalFunctionFallbacks { get; }
    pub fn get_allowExternalFunctionFallbacks(&mut self) -> bool {
        self.allowExternalFunctionFallbacks
    }

    // C# signature: bool allowExternalFunctionFallbacks { get; set; }
    pub fn set_allowExternalFunctionFallbacks(&mut self, value: bool) {
        self.allowExternalFunctionFallbacks = value;
    }

    // C# signature: List<string> globalTags { get; }
    pub fn get_globalTags(&mut self) -> Vec<String> {
        self.TagsAtStartOfFlowContainerWithPathString("".to_string())
    }

    // C# signature: Container mainContentContainer { get; }
    pub fn get_mainContentContainer(&mut self) -> Container {
        self.main_content_container.clone()
    }

    fn TagsAtStartOfFlowContainerWithPathString(&mut self, pathString: String) -> Vec<String> {
        let path = Path::new_overload_4(pathString);
        let Some(mut flow_container) = self.ContentAtPath(path).get_container().cloned() else {
            return Vec::new();
        };

        loop {
            let Some(first_content) = flow_container.get_content().first() else {
                break;
            };
            if let crate::Container::ContentItem::Container(container) = first_content {
                flow_container = container.as_ref().clone();
            } else {
                break;
            }
        }

        let mut in_tag = false;
        let mut tags = Vec::new();
        for content in flow_container.get_content() {
            match content {
                crate::Container::ContentItem::ControlCommand(command) => {
                    if command.get_commandType() == crate::ControlCommand::CommandType::BeginTag {
                        in_tag = true;
                    } else if command.get_commandType()
                        == crate::ControlCommand::CommandType::EndTag
                    {
                        in_tag = false;
                    }
                }
                crate::Container::ContentItem::Value(Value::String(str_value)) if in_tag => {
                    tags.push(str_value.value.clone());
                }
                crate::Container::ContentItem::Value(_) if in_tag => {
                    // Match C# behavior: only plain text is allowed inside tags.
                    // The runtime error path is not wired yet, so keep the behavior visible.
                    return tags;
                }
                _ if in_tag => {
                    break;
                }
                _ => {}
            }
        }

        tags
    }
}

#[cfg(test)]
mod tests {
    use super::Story;
    use crate::Container::Container;
    use crate::ControlCommand::ControlCommand;
    use crate::Path::{Component, Path};
    use crate::Value::Value;

    #[test]
    fn resolves_root_content_and_named_functions() {
        let mut root = Container::new();
        let mut knot = Container::new();
        knot.set_name(Some("knot".to_string()));
        knot.AddContent(ControlCommand::BeginString());
        root.AddContent(knot);

        let mut story = Story::new(root, Vec::new());

        assert!(story.HasFunction("knot".to_string()));
        assert!(story
            .ContentAtPath(Path::new_overload_3(vec![Component::new(0)], false))
            .get_container()
            .is_some());
        assert!(story
            .PointerAtPath(Path::new_overload_3(vec![Component::new(0)], false))
            .get_path()
            .is_some());
        assert!(story.get_mainContentContainer().get_name().is_empty());
    }

    #[test]
    fn extracts_start_tags_for_named_content_paths() {
        let mut root = Container::new();
        let mut knot = Container::new();
        knot.set_name(Some("knot".to_string()));
        knot.AddContent(ControlCommand::BeginTag());
        knot.AddContent(Value::new_string("tag-one".to_string()));
        knot.AddContent(ControlCommand::EndTag());
        root.AddContent(knot);

        let mut story = Story::new(root, Vec::new());

        assert_eq!(
            story.TagsForContentAtPath("0".to_string()),
            vec!["tag-one".to_string()]
        );
        assert_eq!(
            story.BuildStringOfHierarchy(),
            "[\n    [ (knot)\n        BeginTag,\n        \"tag-one\",\n        EndTag\n    ]\n]"
        );
    }
}
