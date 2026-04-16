// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Story.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Story {
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
    pub fn new(
        _contentContainer: crate::stub::Container,
        _lists: Vec<crate::stub::ListDefinition>,
    ) -> Self {
        Default::default()
    }

    // C# signature: public Story(string jsonString)
    pub fn new_overload_2(_jsonString: String) -> Self {
        Default::default()
    }

    // C# signature: public Profiler StartProfiling()
    pub fn StartProfiling(&mut self) -> crate::stub::Profiler {
        Default::default()
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
    pub fn ContentAtPath(&mut self, _path: crate::stub::Path) -> crate::stub::SearchResult {
        Default::default()
    }

    // C# signature: public Runtime.Container KnotContainerWithName (string name)
    pub fn KnotContainerWithName(&mut self, _name: String) -> crate::stub::Container {
        Default::default()
    }

    // C# signature: public Pointer PointerAtPath (Path path)
    pub fn PointerAtPath(&mut self, _path: crate::stub::Path) -> crate::stub::Pointer {
        Default::default()
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
        Default::default()
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
    pub fn TagsForContentAtPath(&mut self, _path: String) -> Vec<String> {
        Default::default()
    }

    // C# signature: public virtual string BuildStringOfHierarchy()
    pub fn BuildStringOfHierarchy(&mut self) -> String {
        Default::default()
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
    pub fn get_listDefinitions(&mut self) -> crate::stub::ListDefinitionsOrigin {
        Default::default()
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
    pub fn get_mainContentContainer(&mut self) -> crate::stub::Container {
        Default::default()
    }
}
