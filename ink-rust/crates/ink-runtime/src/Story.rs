// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Story.cs

use crate::stub::*;
use crate::Choice::Choice;
use crate::Container::Container;
use crate::ListDefinition::ListDefinition;
use crate::ListDefinitionsOrigin::ListDefinitionsOrigin;
use crate::Path::Path;
use crate::Pointer::Pointer;
use crate::Profiler::Profiler;
use crate::SearchResult::SearchResult;
use crate::StoryException::StoryException;
use crate::StoryState::StoryState;
use crate::Value::Value;
use crate::VariablesState::VariablesState;
use std::collections::HashMap;

#[derive(Default)]
pub struct Story {
    main_content_container: Container,
    listDefinitions: ListDefinitionsOrigin,
    state: Option<Box<StoryState>>,
    _externals: HashMap<String, ExternalFunctionDef>,
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
            _externals: self._externals.clone(),
            async_saving: self.async_saving,
            allowExternalFunctionFallbacks: self.allowExternalFunctionFallbacks,
            _port_marker: (),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputStateChange {
    PortPlaceholder,
}

impl Default for OutputStateChange {
    fn default() -> Self {
        Self::PortPlaceholder
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
            _externals: HashMap::new(),
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
            _externals: HashMap::new(),
            async_saving: false,
            allowExternalFunctionFallbacks: false,
            _port_marker: (),
        };
        story.ResetState();
        story
    }

    // C# signature: public Profiler StartProfiling()
    pub fn StartProfiling(&mut self) -> Profiler {
        Profiler::new()
    }

    // C# signature: public void EndProfiling()
    pub fn EndProfiling(&mut self) {}

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
        self.get_currentText()
    }

    // C# signature: public void ContinueAsync (float millisecsLimitAsync)
    pub fn ContinueAsync(&mut self, millisecsLimitAsync: f32) {
        let _ = millisecsLimitAsync;
    }

    // C# signature: public string ContinueMaximally()
    pub fn ContinueMaximally(&mut self) -> String {
        self.get_currentText()
    }

    // C# signature: public SearchResult ContentAtPath(Path path)
    pub fn ContentAtPath(&mut self, _path: Path) -> SearchResult {
        self.main_content_container.ContentAtPath(_path, 0, -1)
    }

    // C# signature: public Runtime.Container KnotContainerWithName (string name)
    pub fn KnotContainerWithName(&mut self, _name: String) -> Option<Container> {
        self.main_content_container
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
            self.main_content_container
                .ContentAtPath(_path.clone(), 0, path_length_to_use)
        } else {
            self.main_content_container
                .ContentAtPath(_path.clone(), 0, -1)
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
        arguments: Vec<crate::stub::PortStub>,
    ) {
        let _ = arguments;
        if let Some(state) = self.state.as_mut() {
            if resetCallstack {
                state.ForceEnd();
            }
            state.ChoosePath(Path::new_overload_4(path), true);
        }
    }

    // C# signature: public void ChoosePath(Path p, bool incrementingTurnIndex = true)
    pub fn ChoosePath(&mut self, p: Path, incrementingTurnIndex: bool) {
        if let Some(state) = self.state.as_mut() {
            state.ChoosePath(p, incrementingTurnIndex);
        }
    }

    // C# signature: public void ChooseChoiceIndex(int choiceIdx)
    pub fn ChooseChoiceIndex(&mut self, choiceIdx: i32) {
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
        _functionName: String,
        _arguments: Vec<crate::stub::PortStub>,
    ) -> crate::stub::PortStub {
        let _ = (_functionName, _arguments);
        Default::default()
    }

    // C# signature: public object EvaluateFunction (string functionName, out string textOutput, params object [] arguments)
    pub fn EvaluateFunction_overload_2(
        &mut self,
        _functionName: String,
        _textOutput: &mut String,
        _arguments: Vec<crate::stub::PortStub>,
    ) -> crate::stub::PortStub {
        let _ = (_functionName, _textOutput, _arguments);
        Default::default()
    }

    // C# signature: public Runtime.Object EvaluateExpression(Runtime.Container exprContainer)
    pub fn EvaluateExpression(&mut self, _exprContainer: Container) -> crate::stub::PortStub {
        Default::default()
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
        _funcName: String,
        _func: crate::stub::PortStub,
        _lookaheadSafe: bool,
    ) {
    }

    // C# signature: public void BindExternalFunction(string funcName, Action act, bool lookaheadSafe=false)
    pub fn BindExternalFunction_overload_2(
        &mut self,
        _funcName: String,
        _act: crate::stub::PortStub,
        _lookaheadSafe: bool,
    ) {
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
    pub fn ObserveVariable(
        &mut self,
        _variableName: String,
        _observer: crate::stub::VariableObserver,
    ) {
    }

    // C# signature: public void ObserveVariables(IList<string> variableNames, VariableObserver observer)
    pub fn ObserveVariables(
        &mut self,
        _variableNames: Vec<String>,
        _observer: crate::stub::VariableObserver,
    ) {
    }

    // C# signature: public void RemoveVariableObserver(VariableObserver observer = null, string specificVariableName = null)
    pub fn RemoveVariableObserver(
        &mut self,
        _observer: crate::stub::VariableObserver,
        _specificVariableName: String,
    ) {
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
        self.main_content_container
            .BuildStringOfHierarchy_overload_2()
    }

    // C# signature: private void NextContent()
    pub fn NextContent(&mut self) {}

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
            "[\n    [ (knot)\n        BeginTag\n        tag-one\n        EndTag\n    ]\n]"
        );
    }
}
