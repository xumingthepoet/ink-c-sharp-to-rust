// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/StoryState.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct StoryState {
    pub _port_marker: (),
}

impl StoryState {
    // C# signature: public StoryState (Story story)
    pub fn new(_story: crate::stub::Story) -> Self {
        Default::default()
    }

    // C# signature: public string ToJson()
    pub fn ToJson(&mut self) -> String {
        Default::default()
    }

    // C# signature: public void ToJson(Stream stream)
    pub fn ToJson_overload_2(&mut self, _stream: crate::stub::Stream) {}

    // C# signature: public void LoadJson(string json)
    pub fn LoadJson(&mut self, _json: String) {}

    // C# signature: public int VisitCountAtPathString(string pathString)
    pub fn VisitCountAtPathString(&mut self, _pathString: String) -> i32 {
        Default::default()
    }

    // C# signature: public int VisitCountForContainer(Container container)
    pub fn VisitCountForContainer(&mut self, _container: crate::stub::Container) -> i32 {
        Default::default()
    }

    // C# signature: public void IncrementVisitCountForContainer(Container container)
    pub fn IncrementVisitCountForContainer(&mut self, _container: crate::stub::Container) {}

    // C# signature: public void RecordTurnIndexVisitToContainer(Container container)
    pub fn RecordTurnIndexVisitToContainer(&mut self, _container: crate::stub::Container) {}

    // C# signature: public int TurnsSinceForContainer(Container container)
    pub fn TurnsSinceForContainer(&mut self, _container: crate::stub::Container) -> i32 {
        Default::default()
    }

    // C# signature: public string CleanOutputWhitespace(string str)
    pub fn CleanOutputWhitespace(&mut self, _str: String) -> String {
        Default::default()
    }

    // C# signature: public void GoToStart()
    pub fn GoToStart(&mut self) {}

    // C# signature: internal void SwitchFlow_Internal(string flowName)
    pub fn SwitchFlow_Internal(&mut self, _flowName: String) {}

    // C# signature: internal void SwitchToDefaultFlow_Internal()
    pub fn SwitchToDefaultFlow_Internal(&mut self) {}

    // C# signature: internal void RemoveFlow_Internal(string flowName)
    pub fn RemoveFlow_Internal(&mut self, _flowName: String) {}

    // C# signature: public StoryState CopyAndStartPatching(bool forBackgroundSave)
    pub fn CopyAndStartPatching(&mut self, _forBackgroundSave: bool) -> crate::stub::StoryState {
        Default::default()
    }

    // C# signature: public void RestoreAfterPatch()
    pub fn RestoreAfterPatch(&mut self) {}

    // C# signature: public void ApplyAnyPatch()
    pub fn ApplyAnyPatch(&mut self) {}

    // C# signature: public void ResetErrors()
    pub fn ResetErrors(&mut self) {}

    // C# signature: public void ResetOutput(List<Runtime.Object> objs = null)
    pub fn ResetOutput(&mut self, _objs: Vec<crate::stub::PortStub>) {}

    // C# signature: public void PushToOutputStream(Runtime.Object obj)
    pub fn PushToOutputStream(&mut self, _obj: crate::stub::PortStub) {}

    // C# signature: public void PopFromOutputStream (int count)
    pub fn PopFromOutputStream(&mut self, _count: i32) {}

    // C# signature: public void PushEvaluationStack(Runtime.Object obj)
    pub fn PushEvaluationStack(&mut self, _obj: crate::stub::PortStub) {}

    // C# signature: public Runtime.Object PopEvaluationStack()
    pub fn PopEvaluationStack(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public Runtime.Object PeekEvaluationStack()
    pub fn PeekEvaluationStack(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public List<Runtime.Object> PopEvaluationStack(int numberOfObjects)
    pub fn PopEvaluationStack_overload_2(
        &mut self,
        _numberOfObjects: i32,
    ) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: public void ForceEnd()
    pub fn ForceEnd(&mut self) {}

    // C# signature: public void PopCallstack (PushPopType? popType = null)
    pub fn PopCallstack(&mut self, _popType: crate::stub::PushPopType) {}

    // C# signature: public void SetChosenPath(Path path, bool incrementingTurnIndex)
    pub fn SetChosenPath(&mut self, _path: crate::stub::Path, _incrementingTurnIndex: bool) {}

    // C# signature: public void StartFunctionEvaluationFromGame (Container funcContainer, params object[] arguments)
    pub fn StartFunctionEvaluationFromGame(
        &mut self,
        _funcContainer: crate::stub::Container,
        _arguments: Vec<crate::stub::PortStub>,
    ) {
    }

    // C# signature: public void PassArgumentsToEvaluationStack (params object [] arguments)
    pub fn PassArgumentsToEvaluationStack(&mut self, _arguments: Vec<crate::stub::PortStub>) {}

    // C# signature: public bool TryExitFunctionEvaluationFromGame ()
    pub fn TryExitFunctionEvaluationFromGame(&mut self) -> bool {
        Default::default()
    }

    // C# signature: public object CompleteFunctionEvaluationFromGame ()
    pub fn CompleteFunctionEvaluationFromGame(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public void AddError(string message, bool isWarning)
    pub fn AddError(&mut self, _message: String, _isWarning: bool) {}

    // C# signature: int callstackDepth { get; }
    pub fn get_callstackDepth(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: List<Runtime.Object> outputStream { get; }
    pub fn get_outputStream(&mut self) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: List<Choice> currentChoices { get; }
    pub fn get_currentChoices(&mut self) -> Vec<crate::stub::Choice> {
        Default::default()
    }

    // C# signature: List<Choice> generatedChoices { get; }
    pub fn get_generatedChoices(&mut self) -> Vec<crate::stub::Choice> {
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

    // C# signature: VariablesState variablesState { get; }
    pub fn get_variablesState(&mut self) -> crate::stub::VariablesState {
        Default::default()
    }

    // C# signature: CallStack callStack { get; }
    pub fn get_callStack(&mut self) -> crate::stub::CallStack {
        Default::default()
    }

    // C# signature: List<Runtime.Object> evaluationStack { get; }
    pub fn get_evaluationStack(&mut self) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: Pointer divertedPointer { get; }
    pub fn get_divertedPointer(&mut self) -> crate::stub::Pointer {
        Default::default()
    }

    // C# signature: int currentTurnIndex { get; }
    pub fn get_currentTurnIndex(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: int storySeed { get; }
    pub fn get_storySeed(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: int previousRandom { get; }
    pub fn get_previousRandom(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: bool didSafeExit { get; }
    pub fn get_didSafeExit(&mut self) -> bool {
        Default::default()
    }

    // C# signature: Story story { get; }
    pub fn get_story(&mut self) -> crate::stub::Story {
        Default::default()
    }

    // C# signature: string currentPathString { get; }
    pub fn get_currentPathString(&mut self) -> String {
        Default::default()
    }

    // C# signature: string previousPathString { get; }
    pub fn get_previousPathString(&mut self) -> String {
        Default::default()
    }

    // C# signature: Runtime.Pointer currentPointer { get; }
    pub fn get_currentPointer(&mut self) -> crate::stub::Pointer {
        Default::default()
    }

    // C# signature: Pointer previousPointer { get; }
    pub fn get_previousPointer(&mut self) -> crate::stub::Pointer {
        Default::default()
    }

    // C# signature: bool canContinue { get; }
    pub fn get_canContinue(&mut self) -> bool {
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

    // C# signature: string currentText { get; }
    pub fn get_currentText(&mut self) -> String {
        Default::default()
    }

    // C# signature: List<string> currentTags { get; }
    pub fn get_currentTags(&mut self) -> Vec<String> {
        Default::default()
    }

    // C# signature: string currentFlowName { get; }
    pub fn get_currentFlowName(&mut self) -> String {
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

    // C# signature: bool inExpressionEvaluation { get; }
    pub fn get_inExpressionEvaluation(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool outputStreamEndsInNewline { get; }
    pub fn get_outputStreamEndsInNewline(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool outputStreamContainsContent { get; }
    pub fn get_outputStreamContainsContent(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool inStringEvaluation { get; }
    pub fn get_inStringEvaluation(&mut self) -> bool {
        Default::default()
    }
}
