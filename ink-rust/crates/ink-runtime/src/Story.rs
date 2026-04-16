// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Story.cs

use crate::stub::*;
use crate::Container::Container;
use crate::ListDefinition::ListDefinition;
use crate::ListDefinitionsOrigin::ListDefinitionsOrigin;
use crate::Path::Path;
use crate::Pointer::Pointer;
use crate::Profiler::Profiler;
use crate::SearchResult::SearchResult;
use crate::Value::Value;

#[derive(Clone, Debug, Default)]
pub struct Story {
    main_content_container: Container,
    listDefinitions: ListDefinitionsOrigin,
    pub _port_marker: (),
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

#[derive(Clone, Debug, Default)]
pub struct ExternalFunctionDef {
    pub _port_marker: (),
}

impl Story {
    // C# signature: public Story (Container contentContainer, List<Runtime.ListDefinition> lists = null)
    pub fn new(_contentContainer: Container, lists: Vec<ListDefinition>) -> Self {
        Self {
            main_content_container: _contentContainer,
            listDefinitions: ListDefinitionsOrigin::new(lists),
            _port_marker: (),
        }
    }

    // C# signature: public Story(string jsonString)
    pub fn new_overload_2(_jsonString: String) -> Self {
        Default::default()
    }

    // C# signature: public Profiler StartProfiling()
    pub fn StartProfiling(&mut self) -> Profiler {
        Profiler::new()
    }

    // C# signature: public void EndProfiling()
    pub fn EndProfiling(&mut self) {}

    // C# signature: public string ToJson()
    pub fn ToJson(&mut self) -> String {
        Default::default()
    }

    // C# signature: public void ToJson(Stream stream)
    pub fn ToJson_overload_2(&mut self, _stream: crate::stub::Stream) {}

    // C# signature: public void ResetState()
    pub fn ResetState(&mut self) {}

    // C# signature: public void ResetCallstack()
    pub fn ResetCallstack(&mut self) {}

    // C# signature: public void SwitchFlow(string flowName)
    pub fn SwitchFlow(&mut self, _flowName: String) {}

    // C# signature: public void RemoveFlow(string flowName)
    pub fn RemoveFlow(&mut self, _flowName: String) {}

    // C# signature: public void SwitchToDefaultFlow()
    pub fn SwitchToDefaultFlow(&mut self) {}

    // C# signature: public string Continue()
    pub fn Continue(&mut self) -> String {
        Default::default()
    }

    // C# signature: public void ContinueAsync (float millisecsLimitAsync)
    pub fn ContinueAsync(&mut self, _millisecsLimitAsync: f32) {}

    // C# signature: public string ContinueMaximally()
    pub fn ContinueMaximally(&mut self) -> String {
        Default::default()
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
    pub fn CopyStateForBackgroundThreadSave(&mut self) -> crate::stub::StoryState {
        Default::default()
    }

    // C# signature: public void BackgroundSaveComplete()
    pub fn BackgroundSaveComplete(&mut self) {}

    // C# signature: public void ChoosePathString (string path, bool resetCallstack = true, params object [] arguments)
    pub fn ChoosePathString(
        &mut self,
        _path: String,
        _resetCallstack: bool,
        _arguments: Vec<crate::stub::PortStub>,
    ) {
    }

    // C# signature: public void ChoosePath(Path p, bool incrementingTurnIndex = true)
    pub fn ChoosePath(&mut self, _p: crate::stub::Path, _incrementingTurnIndex: bool) {}

    // C# signature: public void ChooseChoiceIndex(int choiceIdx)
    pub fn ChooseChoiceIndex(&mut self, _choiceIdx: i32) {}

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
        Default::default()
    }

    // C# signature: public object EvaluateFunction (string functionName, out string textOutput, params object [] arguments)
    pub fn EvaluateFunction_overload_2(
        &mut self,
        _functionName: String,
        _textOutput: &mut String,
        _arguments: Vec<crate::stub::PortStub>,
    ) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public Runtime.Object EvaluateExpression(Runtime.Container exprContainer)
    pub fn EvaluateExpression(
        &mut self,
        _exprContainer: crate::stub::Container,
    ) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public bool TryGetExternalFunction(string functionName, out ExternalFunction externalFunction)
    pub fn TryGetExternalFunction(
        &mut self,
        _functionName: String,
        _externalFunction: &mut crate::stub::ExternalFunction,
    ) -> bool {
        Default::default()
    }

    // C# signature: public void CallExternalFunction(string funcName, int numberOfArguments)
    pub fn CallExternalFunction(&mut self, _funcName: String, _numberOfArguments: i32) {}

    // C# signature: public void BindExternalFunctionGeneral(string funcName, ExternalFunction func, bool lookaheadSafe = true)
    pub fn BindExternalFunctionGeneral(
        &mut self,
        _funcName: String,
        _func: crate::stub::ExternalFunction,
        _lookaheadSafe: bool,
    ) {
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
    pub fn UnbindExternalFunction(&mut self, _funcName: String) {}

    // C# signature: public void ValidateExternalBindings()
    pub fn ValidateExternalBindings(&mut self) {}

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
    pub fn Error(&mut self, _message: String, _useEndLineNumber: bool) {}

    // C# signature: public void Warning (string message)
    pub fn Warning(&mut self, _message: String) {}

    // C# signature: List<Choice> currentChoices { get; }
    pub fn get_currentChoices(&mut self) -> Vec<crate::stub::Choice> {
        Default::default()
    }

    // C# signature: string currentText { get; }
    pub fn get_currentText(&mut self) -> String {
        Default::default()
    }

    // C# signature: List<string> currentTags { get; }
    pub fn get_currentTags(&mut self) -> Vec<String> {
        Default::default()
    }

    // C# signature: List<string> currentErrors { get; }
    pub fn get_currentErrors(&mut self) -> Vec<String> {
        Default::default()
    }

    // C# signature: List<string> currentWarnings { get; }
    pub fn get_currentWarnings(&mut self) -> Vec<String> {
        Default::default()
    }

    // C# signature: bool currentFlowIsDefaultFlow { get; }
    pub fn get_currentFlowIsDefaultFlow(&mut self) -> bool {
        Default::default()
    }

    // C# signature: List<string> aliveFlowNames { get; }
    pub fn get_aliveFlowNames(&mut self) -> Vec<String> {
        Default::default()
    }

    // C# signature: bool hasError { get; }
    pub fn get_hasError(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool hasWarning { get; }
    pub fn get_hasWarning(&mut self) -> bool {
        Default::default()
    }

    // C# signature: VariablesState variablesState { get; }
    pub fn get_variablesState(&mut self) -> crate::stub::VariablesState {
        Default::default()
    }

    // C# signature: ListDefinitionsOrigin listDefinitions { get; }
    pub fn get_listDefinitions(&mut self) -> ListDefinitionsOrigin {
        self.listDefinitions.clone()
    }

    // C# signature: StoryState state { get; }
    pub fn get_state(&mut self) -> crate::stub::StoryState {
        Default::default()
    }

    // C# signature: bool canContinue { get; }
    pub fn get_canContinue(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool asyncContinueComplete { get; }
    pub fn get_asyncContinueComplete(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool allowExternalFunctionFallbacks { get; }
    pub fn get_allowExternalFunctionFallbacks(&mut self) -> bool {
        Default::default()
    }

    // C# signature: List<string> globalTags { get; }
    pub fn get_globalTags(&mut self) -> Vec<String> {
        Default::default()
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
