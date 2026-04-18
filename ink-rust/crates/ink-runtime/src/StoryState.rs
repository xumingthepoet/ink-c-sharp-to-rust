// Source: ink-c-sharp/ink-engine-runtime/StoryState.cs

use crate::CallStack::{CallStack, Thread};
use crate::Choice::Choice;
use crate::Container::{Container, ContentItem};
use crate::ControlCommand::{CommandType, ControlCommand};
use crate::Flow::Flow;
use crate::Glue::Glue;
use crate::InkList::{InkList, InkListItem};
use crate::JsonSerialisation::Json;
use crate::ListDefinition::ListDefinition;
use crate::ListDefinitionsOrigin::ListDefinitionsOrigin;
use crate::Path::Path;
use crate::Pointer::Pointer;
use crate::PushPop::PushPopType;
use crate::SimpleJson::{JsonObject, JsonValue, SimpleJson, Writer};
use crate::StatePatch::StatePatch;
use crate::Story::Story;
use crate::Value::{ListValue, StringValue, Value, ValueType, VariablePointerValue};
use crate::VariablesState::{VariableObserver, VariablesState};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct StoryState {
    pub onDidLoadState: Option<Arc<dyn Fn() + Send + Sync>>,
    story: Story,
    currentFlow: Flow,
    namedFlows: Option<HashMap<String, Flow>>,
    patch: Option<StatePatch>,
    visitCounts: HashMap<String, i32>,
    turnIndices: HashMap<String, i32>,
    currentErrors: Option<Vec<String>>,
    currentWarnings: Option<Vec<String>>,
    variablesState: VariablesState,
    evaluationStack: Vec<ContentItem>,
    divertedPointer: Pointer,
    currentTurnIndex: i32,
    storySeed: i32,
    previousRandom: i32,
    didSafeExit: bool,
    currentTextCache: String,
    currentTagsCache: Vec<String>,
    aliveFlowNamesCache: Vec<String>,
    outputStreamTextDirty: bool,
    outputStreamTagsDirty: bool,
    aliveFlowNamesDirty: bool,
}

impl Clone for StoryState {
    fn clone(&self) -> Self {
        Self {
            onDidLoadState: self.onDidLoadState.clone(),
            story: self.story.clone_without_state(),
            currentFlow: self.currentFlow.clone(),
            namedFlows: self.namedFlows.clone(),
            patch: self.patch.clone(),
            visitCounts: self.visitCounts.clone(),
            turnIndices: self.turnIndices.clone(),
            currentErrors: self.currentErrors.clone(),
            currentWarnings: self.currentWarnings.clone(),
            variablesState: self.variablesState.clone(),
            evaluationStack: self.evaluationStack.clone(),
            divertedPointer: self.divertedPointer.clone(),
            currentTurnIndex: self.currentTurnIndex,
            storySeed: self.storySeed,
            previousRandom: self.previousRandom,
            didSafeExit: self.didSafeExit,
            currentTextCache: self.currentTextCache.clone(),
            currentTagsCache: self.currentTagsCache.clone(),
            aliveFlowNamesCache: self.aliveFlowNamesCache.clone(),
            outputStreamTextDirty: self.outputStreamTextDirty,
            outputStreamTagsDirty: self.outputStreamTagsDirty,
            aliveFlowNamesDirty: self.aliveFlowNamesDirty,
        }
    }
}

impl Default for StoryState {
    fn default() -> Self {
        Self::new(Story::default())
    }
}

impl fmt::Debug for StoryState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StoryState")
            .field(
                "onDidLoadState",
                &self.onDidLoadState.as_ref().map(|_| "<callback>"),
            )
            .field("story", &self.story)
            .field("currentFlow", &self.currentFlow)
            .field("namedFlows", &self.namedFlows)
            .field("patch", &self.patch)
            .field("visitCounts", &self.visitCounts)
            .field("turnIndices", &self.turnIndices)
            .field("currentErrors", &self.currentErrors)
            .field("currentWarnings", &self.currentWarnings)
            .field("variablesState", &self.variablesState)
            .field("evaluationStack", &self.evaluationStack)
            .field("divertedPointer", &self.divertedPointer)
            .field("currentTurnIndex", &self.currentTurnIndex)
            .field("storySeed", &self.storySeed)
            .field("previousRandom", &self.previousRandom)
            .field("didSafeExit", &self.didSafeExit)
            .field("currentTextCache", &self.currentTextCache)
            .field("currentTagsCache", &self.currentTagsCache)
            .field("aliveFlowNamesCache", &self.aliveFlowNamesCache)
            .field("outputStreamTextDirty", &self.outputStreamTextDirty)
            .field("outputStreamTagsDirty", &self.outputStreamTagsDirty)
            .field("aliveFlowNamesDirty", &self.aliveFlowNamesDirty)
            .finish()
    }
}

impl StoryState {
    pub const kInkSaveStateVersion: i32 = 10;
    const kMinCompatibleLoadVersion: i32 = 8;
    const kDefaultFlowName: &str = "DEFAULT_FLOW";

    // C# signature: public StoryState (Story story)
    pub fn new(mut story: Story) -> Self {
        let listDefinitions = story.get_listDefinitions();
        let currentFlow = Flow::new(Self::kDefaultFlowName.to_string(), story.clone());
        let variablesState = VariablesState::new(currentFlow.callStack.clone(), listDefinitions);

        let mut state = Self {
            onDidLoadState: None,
            story,
            currentFlow,
            namedFlows: None,
            patch: None,
            visitCounts: HashMap::new(),
            turnIndices: HashMap::new(),
            currentErrors: None,
            currentWarnings: None,
            variablesState,
            evaluationStack: Vec::new(),
            divertedPointer: Pointer::Null(),
            currentTurnIndex: -1,
            storySeed: 0,
            previousRandom: 0,
            didSafeExit: false,
            currentTextCache: String::new(),
            currentTagsCache: Vec::new(),
            aliveFlowNamesCache: Vec::new(),
            outputStreamTextDirty: true,
            outputStreamTagsDirty: true,
            aliveFlowNamesDirty: true,
        };

        let time_seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_millis() as i32)
            .unwrap_or(0);
        let mut random = crate::Story::SimpleRandom::new(time_seed);
        state.storySeed = random.next() % 100;
        state.GoToStart();
        state
    }

    pub fn sync_variables_callstack(&mut self) {
        let old_callstack = self.variablesState.get_callStack();
        let mut new_callstack = self.currentFlow.callStack.clone();

        for (new_thread, old_thread) in new_callstack.threads.iter_mut().zip(old_callstack.threads)
        {
            for (new_el, old_el) in new_thread.callstack.iter_mut().zip(old_thread.callstack) {
                new_el.temporaryVariables = old_el.temporaryVariables;
            }
        }

        self.currentFlow.callStack = new_callstack.clone();
        self.variablesState.set_callStack(new_callstack);
    }

    // C# signature: public string ToJson()
    pub fn ToJson(&mut self) -> String {
        let mut writer = Writer::new();
        self.WriteJson(&mut writer);
        writer.ToString()
    }

    // C# signature: public void ToJson(Stream stream)
    pub fn ToJson_overload_2(&mut self, stream: Box<dyn std::io::Write>) {
        let mut writer = Writer::new_overload_2(stream);
        self.WriteJson(&mut writer);
    }

    // C# signature: public void LoadJson(string json)
    pub fn LoadJson(&mut self, json: String) {
        let jObject = SimpleJson::TextToDictionary(json)
            .unwrap_or_else(|err| panic!("Failed to parse story state JSON: {}", err));
        self.LoadJsonObj(jObject);
        if let Some(callback) = &self.onDidLoadState {
            callback();
        }
    }

    // C# signature: public int VisitCountAtPathString(string pathString)
    pub fn VisitCountAtPathString(&mut self, pathString: String) -> i32 {
        if let Some(patch) = &self.patch {
            let result = self
                .story
                .ContentAtPath(Path::new_overload_4(pathString.clone()));
            if let Some(container) = result.get_container().cloned() {
                let mut visitCountOut = 0;
                if patch.TryGetVisitCount(&container, &mut visitCountOut) {
                    return visitCountOut;
                }
            }
        }

        self.visitCounts.get(&pathString).copied().unwrap_or(0)
    }

    // C# signature: public int VisitCountForContainer(Container container)
    pub fn VisitCountForContainer(&mut self, container: Container) -> i32 {
        if !container.get_visitsShouldBeCounted() {
            let debug_metadata = container
                .get_debugMetadata()
                .map(|metadata| metadata.to_string())
                .unwrap_or_default();
            self.story.Error(
                format!(
                    "Read count for target ({} - on {}) unknown.",
                    container.get_name(),
                    debug_metadata
                ),
                false,
            );
            return 0;
        }

        if let Some(patch) = &self.patch {
            let mut count = 0;
            if patch.TryGetVisitCount(&container, &mut count) {
                return count;
            }
        }

        self.visitCounts
            .get(&container.get_path().ToString())
            .copied()
            .unwrap_or(0)
    }

    // C# signature: public void IncrementVisitCountForContainer(Container container)
    pub fn IncrementVisitCountForContainer(&mut self, container: Container) {
        let currCount = self.VisitCountForContainer(container.clone()) + 1;
        if let Some(patch) = &mut self.patch {
            patch.SetVisitCount(&container, currCount);
            return;
        }

        let key = container.get_path().ToString();
        let count = self.visitCounts.get(&key).copied().unwrap_or(0) + 1;
        self.visitCounts.insert(key, count);
    }

    // C# signature: public void RecordTurnIndexVisitToContainer(Container container)
    pub fn RecordTurnIndexVisitToContainer(&mut self, container: Container) {
        if let Some(patch) = &mut self.patch {
            patch.SetTurnIndex(&container, self.currentTurnIndex);
            return;
        }

        self.turnIndices
            .insert(container.get_path().ToString(), self.currentTurnIndex);
    }

    // C# signature: public int TurnsSinceForContainer(Container container)
    pub fn TurnsSinceForContainer(&mut self, container: Container) -> i32 {
        if !container.get_turnIndexShouldBeCounted() {
            let debug_metadata = container
                .get_debugMetadata()
                .map(|metadata| metadata.to_string())
                .unwrap_or_default();
            self.story.Error(
                format!(
                    "TURNS_SINCE() for target ({} - on {}) unknown.",
                    container.get_name(),
                    debug_metadata
                ),
                false,
            );
        }

        if let Some(patch) = &self.patch {
            let mut index = 0;
            if patch.TryGetTurnIndex(&container, &mut index) {
                return self.currentTurnIndex - index;
            }
        }

        let key = container.get_path().ToString();
        self.turnIndices
            .get(&key)
            .map(|index| self.currentTurnIndex - *index)
            .unwrap_or(-1)
    }

    // C# signature: public string CleanOutputWhitespace(string str)
    pub fn CleanOutputWhitespace(&self, str: String) -> String {
        let mut sb = String::with_capacity(str.len());
        let mut currentWhitespaceStart = -1;
        let mut startOfLine = 0;

        for (i, c) in str.chars().enumerate() {
            let isInlineWhitespace = c == ' ' || c == '\t';
            if isInlineWhitespace && currentWhitespaceStart == -1 {
                currentWhitespaceStart = i as i32;
            }

            if !isInlineWhitespace {
                if c != '\n' && currentWhitespaceStart > 0 && currentWhitespaceStart != startOfLine
                {
                    sb.push(' ');
                }
                currentWhitespaceStart = -1;
            }

            if c == '\n' {
                startOfLine = i as i32 + 1;
            }

            if !isInlineWhitespace {
                sb.push(c);
            }
        }

        sb
    }

    // C# signature: public void GoToStart()
    pub fn GoToStart(&mut self) {
        self.currentFlow.callStack.currentThread_mut().callstack[0].currentPointer =
            Pointer::StartOf(Rc::new(self.story.get_mainContentContainer()));
    }

    // C# signature: internal void SwitchFlow_Internal(string flowName)
    pub fn SwitchFlow_Internal(&mut self, flowName: String) {
        if flowName.is_empty() {
            panic!("Must pass a non-null string to Story.SwitchFlow");
        }

        if std::env::var_os("INK_DEBUG_RUNTIME").is_some() {
            eprintln!(
                "switch_flow start from={} to={} current_choices={} current_ptr={:?}",
                self.currentFlow.name,
                flowName,
                self.currentFlow.currentChoices.len(),
                self.currentFlow
                    .callStack
                    .currentElement()
                    .currentPointer
                    .get_path()
                    .map(|p| p.ToString())
            );
        }
        if std::env::var_os("INK_DEBUG_CHOICE").is_some() {
            eprintln!(
                "switch_flow choice state from={} choices={} ptr={} canContinue={}",
                self.currentFlow.name,
                self.currentFlow.currentChoices.len(),
                self.currentFlow
                    .callStack
                    .currentElement()
                    .currentPointer
                    .get_path()
                    .map(|p| p.ToString())
                    .unwrap_or_else(|| "<null>".to_string()),
                self.get_canContinue()
            );
        }

        if self.namedFlows.is_none() {
            let mut named_flows = HashMap::new();
            named_flows.insert(Self::kDefaultFlowName.to_string(), self.currentFlow.clone());
            self.namedFlows = Some(named_flows);
        }

        if flowName == self.currentFlow.name {
            return;
        }

        if let Some(named) = self.namedFlows.as_mut() {
            named.insert(self.currentFlow.name.clone(), self.currentFlow.clone());
        }

        let flow = if let Some(named) = self.namedFlows.as_mut() {
            named.entry(flowName.clone()).or_insert_with(|| {
                self.aliveFlowNamesDirty = true;
                Flow::new(flowName.clone(), self.story.clone())
            });
            let mut flow = named.get(&flowName).cloned().unwrap();
            if std::env::var_os("INK_DEBUG_RUNTIME").is_some() {
                eprintln!(
                    "switch_flow retrieved name={} ptr={} prev={}",
                    flowName,
                    flow.callStack
                        .currentElement()
                        .currentPointer
                        .get_path()
                        .map(|p| p.ToString())
                        .unwrap_or_else(|| "<null>".to_string()),
                    flow.callStack
                        .currentThread()
                        .previousPointer
                        .as_ref()
                        .and_then(|p| p.get_path())
                        .map(|p| p.ToString())
                        .unwrap_or_else(|| "<null>".to_string())
                );
            }
            if flowName == Self::kDefaultFlowName && flow.currentChoices.is_empty() {
                if flow.callStack.currentElement().currentPointer.get_isNull() {
                    if let Some(prev) = flow.callStack.currentThread().previousPointer.clone() {
                        flow.callStack.currentThread_mut().callstack[0].currentPointer = prev;
                    }
                }
            }
            flow
        } else {
            Flow::new(flowName.clone(), self.story.clone())
        };

        self.currentFlow = flow;
        self.variablesState
            .set_callStack(self.currentFlow.callStack.clone());
        self.OutputStreamDirty();

        if std::env::var_os("INK_DEBUG_RUNTIME").is_some() {
            eprintln!(
                "switch_flow end current={} choices={} ptr={:?}",
                self.currentFlow.name,
                self.currentFlow.currentChoices.len(),
                self.currentFlow
                    .callStack
                    .currentElement()
                    .currentPointer
                    .get_path()
                    .map(|p| p.ToString())
            );
        }
        if std::env::var_os("INK_DEBUG_CHOICE").is_some() {
            eprintln!(
                "switch_flow done current={} choices={} ptr={} canContinue={}",
                self.currentFlow.name,
                self.currentFlow.currentChoices.len(),
                self.currentFlow
                    .callStack
                    .currentElement()
                    .currentPointer
                    .get_path()
                    .map(|p| p.ToString())
                    .unwrap_or_else(|| "<null>".to_string()),
                self.get_canContinue()
            );
        }
    }

    // C# signature: internal void SwitchToDefaultFlow_Internal()
    pub fn SwitchToDefaultFlow_Internal(&mut self) {
        if self.namedFlows.is_none() {
            return;
        }
        self.SwitchFlow_Internal(Self::kDefaultFlowName.to_string());
    }

    // C# signature: internal void RemoveFlow_Internal(string flowName)
    pub fn RemoveFlow_Internal(&mut self, flowName: String) {
        if flowName.is_empty() {
            panic!("Must pass a non-null string to Story.DestroyFlow");
        }
        if flowName == Self::kDefaultFlowName {
            panic!("Cannot destroy default flow");
        }

        if self.currentFlow.name == flowName {
            self.SwitchToDefaultFlow_Internal();
        }

        if let Some(named) = self.namedFlows.as_mut() {
            named.remove(&flowName);
            self.aliveFlowNamesDirty = true;
        }
    }

    // C# signature: public StoryState CopyAndStartPatching(bool forBackgroundSave)
    pub fn CopyAndStartPatching(&mut self, _forBackgroundSave: bool) -> StoryState {
        let mut copy = StoryState::new(self.story.clone());
        copy.patch = Some(StatePatch::new(self.patch.clone().unwrap_or_default()));
        copy.currentFlow = self.currentFlow.clone();
        copy.currentFlow.callStack = CallStack::new_overload_2(self.currentFlow.callStack.clone());
        copy.currentFlow.outputStream = self.currentFlow.outputStream.clone();
        copy.currentFlow.currentChoices = self
            .currentFlow
            .currentChoices
            .iter()
            .map(|choice| choice.Clone())
            .collect();
        copy.variablesState = self.variablesState.clone();
        let old_callstack = copy.variablesState.get_callStack();
        let mut new_callstack = copy.currentFlow.callStack.clone();
        for (new_thread, old_thread) in new_callstack.threads.iter_mut().zip(old_callstack.threads)
        {
            for (new_el, old_el) in new_thread.callstack.iter_mut().zip(old_thread.callstack) {
                new_el.temporaryVariables = old_el.temporaryVariables;
            }
        }
        copy.variablesState.set_callStack(new_callstack);
        copy.variablesState.patch = copy.patch.clone();
        copy.evaluationStack = self.evaluationStack.clone();
        copy.divertedPointer = self.divertedPointer.clone();
        copy.currentTurnIndex = self.currentTurnIndex;
        copy.storySeed = self.storySeed;
        copy.previousRandom = self.previousRandom;
        copy.didSafeExit = self.didSafeExit;
        copy.visitCounts = self.visitCounts.clone();
        copy.turnIndices = self.turnIndices.clone();
        if let Some(named) = &self.namedFlows {
            let mut named_flows = named.clone();
            named_flows.insert(self.currentFlow.name.clone(), copy.currentFlow.clone());
            copy.namedFlows = Some(named_flows);
        }
        copy.currentErrors = self.currentErrors.clone();
        copy.currentWarnings = self.currentWarnings.clone();
        copy.OutputStreamDirty();
        copy
    }

    // C# signature: public void RestoreAfterPatch()
    pub fn RestoreAfterPatch(&mut self) {
        let old_callstack = self.variablesState.get_callStack();
        let mut new_callstack = self.get_callStack();
        for (new_thread, old_thread) in new_callstack.threads.iter_mut().zip(old_callstack.threads)
        {
            for (new_el, old_el) in new_thread.callstack.iter_mut().zip(old_thread.callstack) {
                new_el.temporaryVariables = old_el.temporaryVariables;
            }
        }
        self.variablesState.set_callStack(new_callstack);
        self.variablesState.patch = self.patch.clone();
    }

    pub(crate) fn merge_live_flow_state_from(&mut self, live_state: &StoryState) {
        self.currentFlow.currentChoices = live_state.currentFlow.currentChoices.clone();
        Self::copy_temporary_variables(
            &mut self.currentFlow.callStack,
            &live_state.variablesState.get_callStack(),
        );

        if let Some(named_flows) = &mut self.namedFlows {
            if let Some(flow) = named_flows.get_mut(&self.currentFlow.name) {
                flow.currentChoices = self.currentFlow.currentChoices.clone();
                Self::copy_temporary_variables(
                    &mut flow.callStack,
                    &live_state.variablesState.get_callStack(),
                );
            }
        }
    }

    fn copy_temporary_variables(dst: &mut CallStack, src: &CallStack) {
        for (dst_thread, src_thread) in dst.threads.iter_mut().zip(src.threads.iter()) {
            for (dst_el, src_el) in dst_thread
                .callstack
                .iter_mut()
                .zip(src_thread.callstack.iter())
            {
                dst_el.temporaryVariables = src_el.temporaryVariables.clone();
            }
        }
    }

    // C# signature: public void ApplyAnyPatch()
    pub fn ApplyAnyPatch(&mut self) {
        if self.patch.is_none() {
            return;
        }

        self.variablesState.ApplyPatch();

        if let Some(patch) = &self.patch {
            for (pathToCount, newCount) in patch.get_visitCounts() {
                self.visitCounts.insert(pathToCount.ToString(), *newCount);
            }
            for (pathToIndex, newCount) in patch.get_turnIndices() {
                self.turnIndices.insert(pathToIndex.ToString(), *newCount);
            }
        }

        self.patch = None;
    }

    // C# signature: public void ResetErrors()
    pub fn ResetErrors(&mut self) {
        self.currentErrors = None;
        self.currentWarnings = None;
    }

    // C# signature: internal void StartVariableObservation()
    pub fn StartVariableObservation(&mut self) {
        self.variablesState.StartVariableObservation();
    }

    // C# signature: internal HashMap<string, Runtime.Object> CompleteVariableObservation()
    pub fn CompleteVariableObservation(&mut self) -> HashMap<String, Value> {
        self.variablesState.CompleteVariableObservation()
    }

    // C# signature: internal void NotifyObservers(Dictionary<string, Runtime.Object> changedVars)
    pub fn NotifyObservers(&mut self, changedVars: HashMap<String, Value>) {
        self.variablesState.NotifyObservers(changedVars);
    }

    // C# signature: public void ResetOutput(List<Runtime.Object> objs = null)
    pub fn ResetOutput(&mut self, objs: Vec<ContentItem>) {
        self.currentFlow.outputStream.clear();
        self.currentFlow.outputStream.extend(objs);
        self.OutputStreamDirty();
    }

    // C# signature: public void PushToOutputStream(Runtime.Object obj)
    pub fn PushToOutputStream(&mut self, obj: ContentItem) {
        if let ContentItem::Value(Value::String(text)) = &obj {
            if let Some(listText) = self.TrySplittingHeadTailWhitespace(text) {
                for textObj in listText {
                    self.PushToOutputStreamIndividual(ContentItem::Value(Value::String(
                        StringValue::new(textObj),
                    )));
                }
                self.OutputStreamDirty();
                return;
            }
        }

        self.PushToOutputStreamIndividual(obj);
        self.OutputStreamDirty();
    }

    // C# signature: public void PopFromOutputStream (int count)
    pub fn PopFromOutputStream(&mut self, count: i32) {
        let len = self.currentFlow.outputStream.len();
        self.currentFlow
            .outputStream
            .truncate(len.saturating_sub(count as usize));
        self.OutputStreamDirty();
    }

    fn TrySplittingHeadTailWhitespace(&self, single: &StringValue) -> Option<Vec<String>> {
        let str = &single.value;
        let mut headFirstNewlineIdx = -1;
        let mut headLastNewlineIdx = -1;
        for (i, c) in str.chars().enumerate() {
            if c == '\n' {
                if headFirstNewlineIdx == -1 {
                    headFirstNewlineIdx = i as i32;
                }
                headLastNewlineIdx = i as i32;
            } else if c == ' ' || c == '\t' {
                continue;
            } else {
                break;
            }
        }

        let mut tailLastNewlineIdx = -1;
        let mut tailFirstNewlineIdx = -1;
        for (offset, c) in str.chars().rev().enumerate() {
            let i = str.len().saturating_sub(offset + 1) as i32;
            if c == '\n' {
                if tailLastNewlineIdx == -1 {
                    tailLastNewlineIdx = i;
                }
                tailFirstNewlineIdx = i;
            } else if c == ' ' || c == '\t' {
                continue;
            } else {
                break;
            }
        }

        if headFirstNewlineIdx == -1 && tailLastNewlineIdx == -1 {
            return None;
        }

        let mut listTexts = Vec::new();
        let mut innerStrStart = 0usize;
        let mut innerStrEnd = str.len();

        if headFirstNewlineIdx != -1 {
            if headFirstNewlineIdx > 0 {
                listTexts.push(str[..headFirstNewlineIdx as usize].to_string());
            }
            listTexts.push("\n".to_string());
            innerStrStart = (headLastNewlineIdx + 1) as usize;
        }

        if tailLastNewlineIdx != -1 {
            innerStrEnd = tailFirstNewlineIdx as usize;
        }

        if innerStrEnd > innerStrStart {
            listTexts.push(str[innerStrStart..innerStrEnd].to_string());
        }

        if tailLastNewlineIdx != -1 && tailFirstNewlineIdx > headLastNewlineIdx {
            listTexts.push("\n".to_string());
            if tailLastNewlineIdx < str.len() as i32 - 1 {
                listTexts.push(str[(tailLastNewlineIdx as usize + 1)..].to_string());
            }
        }

        Some(listTexts)
    }

    fn PushToOutputStreamIndividual(&mut self, obj: ContentItem) {
        let glue = matches!(obj, ContentItem::Glue(_));
        let text = match &obj {
            ContentItem::Value(Value::String(text)) => Some(text.clone()),
            _ => None,
        };

        let mut includeInOutput = true;

        if glue {
            self.TrimNewlinesFromOutputStream();
            includeInOutput = true;
        } else if let Some(text) = text.as_ref() {
            let mut functionTrimIndex = -1;
            let currEl = self.currentFlow.callStack.currentElement().clone();
            if currEl.r#type == PushPopType::Function {
                functionTrimIndex = currEl.functionStartInOuputStream;
            }

            let mut glueTrimIndex = -1;
            for i in (0..self.currentFlow.outputStream.len()).rev() {
                match &self.currentFlow.outputStream[i] {
                    ContentItem::Glue(_) => {
                        glueTrimIndex = i as i32;
                        break;
                    }
                    ContentItem::ControlCommand(cmd)
                        if cmd.get_commandType() == CommandType::BeginString =>
                    {
                        if i as i32 >= functionTrimIndex {
                            functionTrimIndex = -1;
                        }
                        break;
                    }
                    _ => {}
                }
            }

            let trimIndex = if glueTrimIndex != -1 && functionTrimIndex != -1 {
                glueTrimIndex.min(functionTrimIndex)
            } else if glueTrimIndex != -1 {
                glueTrimIndex
            } else {
                functionTrimIndex
            };

            if trimIndex != -1 {
                let is_newline = text.value == "\n";
                if is_newline {
                    includeInOutput = false;
                } else if !text.value.trim().is_empty() {
                    if glueTrimIndex > -1 {
                        self.RemoveExistingGlue();
                    }

                    if functionTrimIndex > -1 {
                        let callstack_elements =
                            &mut self.currentFlow.callStack.currentThread_mut().callstack;
                        for el in callstack_elements.iter_mut().rev() {
                            if el.r#type == PushPopType::Function {
                                el.functionStartInOuputStream = -1;
                            } else {
                                break;
                            }
                        }
                    }
                }
            } else if text.value == "\n" {
                if self.outputStreamEndsInNewline() || !self.outputStreamContainsContent() {
                    includeInOutput = false;
                }
            }
        }

        if includeInOutput {
            self.currentFlow.outputStream.push(obj);
            self.OutputStreamDirty();
        }
    }

    fn TrimNewlinesFromOutputStream(&mut self) {
        let mut removeWhitespaceFrom = None;
        let mut i = self.currentFlow.outputStream.len();
        while i > 0 {
            i -= 1;
            match &self.currentFlow.outputStream[i] {
                ContentItem::ControlCommand(_) => break,
                ContentItem::Value(Value::String(txt))
                    if !txt.isNewline && !txt.isInlineWhitespace =>
                {
                    break
                }
                ContentItem::Value(Value::String(txt)) if txt.isNewline => {
                    removeWhitespaceFrom = Some(i);
                }
                _ => {}
            }
        }

        if let Some(removeWhitespaceFrom) = removeWhitespaceFrom {
            let mut i = removeWhitespaceFrom;
            while i < self.currentFlow.outputStream.len() {
                if matches!(
                    self.currentFlow.outputStream[i],
                    ContentItem::Value(Value::String(_))
                ) {
                    self.currentFlow.outputStream.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        self.OutputStreamDirty();
    }

    fn RemoveExistingGlue(&mut self) {
        for i in (0..self.currentFlow.outputStream.len()).rev() {
            match self.currentFlow.outputStream.get(i) {
                Some(ContentItem::Glue(_)) => {
                    self.currentFlow.outputStream.remove(i);
                }
                Some(ContentItem::ControlCommand(_)) => break,
                _ => {}
            }
        }

        self.OutputStreamDirty();
    }

    // C# signature: public void PushEvaluationStack(Runtime.Object obj)
    pub fn PushEvaluationStack(&mut self, obj: ContentItem) {
        let mut obj = obj;
        if std::env::var_os("INK_DEBUG_RUNTIME").is_some() {
            eprintln!("push eval {:?}", obj);
        }
        if let ContentItem::Value(Value::List(listValue)) = &mut obj {
            if let Some(origin_names) = listValue.originNames.clone() {
                let list_definitions = self.story.get_listDefinitions();
                let mut origins = Vec::new();
                for n in origin_names {
                    if let Some(def) = list_definitions.TryListGetDefinition(n.clone()) {
                        if !origins.contains(def) {
                            origins.push(def.clone());
                        }
                    }
                }
                listValue.origins = Some(origins);
            }
        }

        self.evaluationStack.push(obj);
    }

    // C# signature: public Runtime.Object PopEvaluationStack()
    pub fn PopEvaluationStack(&mut self) -> ContentItem {
        if self.evaluationStack.is_empty() && std::env::var_os("INK_DEBUG_RUNTIME").is_some() {
            eprintln!(
                "pop eval empty currentElementIndex={} inExpr={} out={} temps={:?}",
                self.currentFlow.callStack.currentElementIndex(),
                self.currentFlow
                    .callStack
                    .currentElement()
                    .inExpressionEvaluation,
                self.currentFlow.outputStream.len(),
                self.currentFlow
                    .callStack
                    .currentElement()
                    .temporaryVariables
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
            );
            eprintln!("{:?}", std::backtrace::Backtrace::force_capture());
        }
        self.evaluationStack
            .pop()
            .expect("evaluation stack underflow")
    }

    // C# signature: public Runtime.Object PeekEvaluationStack()
    pub fn PeekEvaluationStack(&mut self) -> ContentItem {
        self.evaluationStack
            .last()
            .cloned()
            .expect("evaluation stack underflow")
    }

    // C# signature: public List<Runtime.Object> PopEvaluationStack(int numberOfObjects)
    pub fn PopEvaluationStack_overload_2(&mut self, numberOfObjects: i32) -> Vec<ContentItem> {
        if numberOfObjects as usize > self.evaluationStack.len() {
            panic!("trying to pop too many objects");
        }

        let split = self.evaluationStack.len() - numberOfObjects as usize;
        self.evaluationStack.split_off(split)
    }

    // C# signature: public void ForceEnd()
    pub fn ForceEnd(&mut self) {
        self.currentFlow.callStack.Reset();
        self.sync_variables_callstack();
        self.currentFlow.currentChoices.clear();
        self.currentPointer_set(Pointer::Null());
        self.previousPointer_set(Pointer::Null());
        self.didSafeExit = true;
    }

    fn TrimWhitespaceFromFunctionEnd(&mut self) {
        if self.currentFlow.callStack.currentElement().r#type != PushPopType::Function {
            return;
        }
        let mut functionStartPoint = self
            .currentFlow
            .callStack
            .currentElement()
            .functionStartInOuputStream;
        if functionStartPoint == -1 {
            functionStartPoint = 0;
        }

        let mut i = self.currentFlow.outputStream.len();
        while i > functionStartPoint as usize {
            i -= 1;
            match &self.currentFlow.outputStream[i] {
                ContentItem::Value(Value::String(txt))
                    if txt.isNewline || txt.isInlineWhitespace =>
                {
                    self.currentFlow.outputStream.remove(i);
                    self.OutputStreamDirty();
                }
                ContentItem::ControlCommand(_) => break,
                ContentItem::Value(Value::String(_)) => break,
                _ => {}
            }
        }
    }

    // C# signature: public void PopCallstack (PushPopType? popType = null)
    pub fn PopCallstack(&mut self, popType: Option<PushPopType>) {
        if self.currentFlow.callStack.currentElement().r#type == PushPopType::Function {
            self.TrimWhitespaceFromFunctionEnd();
        }
        self.currentFlow.callStack.Pop(popType);
        self.sync_variables_callstack();
    }

    // C# signature: public void SetChosenPath(Path path, bool incrementingTurnIndex)
    pub fn SetChosenPath(&mut self, path: Path, incrementingTurnIndex: bool) {
        self.currentFlow.currentChoices.clear();
        let mut newPointer = self.story.PointerAtPath(path);
        if !newPointer.get_isNull() && newPointer.index == -1 {
            newPointer.index = 0;
        }
        self.currentPointer_set(newPointer);
        if std::env::var_os("INK_DEBUG_CHOICE").is_some() {
            eprintln!(
                "set_chosen_path current_ptr={}",
                self.currentPointer()
                    .get_path()
                    .map(|p| p.ToString())
                    .unwrap_or_else(|| "<null>".to_string())
            );
        }
        self.sync_variables_callstack();
        if incrementingTurnIndex {
            self.currentTurnIndex += 1;
        }
    }

    // C# signature: public void ChoosePath(Path path, bool incrementingTurnIndex = true)
    pub fn ChoosePath(&mut self, path: Path, incrementingTurnIndex: bool) {
        self.SetChosenPath(path, incrementingTurnIndex);
    }

    // C# signature: public void ChooseChoiceIndex(int choiceIdx)
    pub fn ChooseChoiceIndex(&mut self, choiceIdx: i32) {
        let visible_choices: Vec<Choice> = self
            .currentFlow
            .currentChoices
            .iter()
            .filter(|choice| !choice.isInvisibleDefault)
            .cloned()
            .collect();

        if choiceIdx < 0 || (choiceIdx as usize) >= visible_choices.len() {
            panic!("choice out of range");
        }

        let choice_to_choose = visible_choices[choiceIdx as usize].clone();
        if let Some(thread_at_generation) = choice_to_choose.threadAtGeneration.clone() {
            self.currentFlow
                .callStack
                .set_currentThread(thread_at_generation);
            if std::env::var_os("INK_DEBUG_RUNTIME").is_some() {
                eprintln!(
                    "choose_choice_index set thread temps={:?}",
                    self.currentFlow
                        .callStack
                        .currentThread()
                        .callstack
                        .last()
                        .map(|el| el.temporaryVariables.keys().cloned().collect::<Vec<_>>())
                );
            }
            if let Some(current_element) = self
                .currentFlow
                .callStack
                .currentThread_mut()
                .callstack
                .last_mut()
            {
                current_element.inExpressionEvaluation = false;
            }
            self.variablesState
                .set_callStack(self.currentFlow.callStack.clone());
            if std::env::var_os("INK_DEBUG_RUNTIME").is_some() {
                eprintln!(
                    "choose_choice_index synced vars temps={:?}",
                    self.variablesState
                        .get_callStack()
                        .currentElement()
                        .temporaryVariables
                        .keys()
                        .cloned()
                        .collect::<Vec<_>>()
                );
            }
        }

        let target_path = choice_to_choose.targetPath.unwrap_or_else(|| Path::new());
        self.SetChosenPath(target_path, true);
    }

    // C# signature: public void StartFunctionEvaluationFromGame (Container funcContainer, params object[] arguments)
    pub fn StartFunctionEvaluationFromGame(
        &mut self,
        funcContainer: Container,
        arguments: Vec<ContentItem>,
    ) {
        self.currentFlow.callStack.Push(
            PushPopType::FunctionEvaluationFromGame,
            self.evaluationStack.len() as i32,
            0,
        );
        self.currentFlow
            .callStack
            .currentThread_mut()
            .callstack
            .last_mut()
            .unwrap()
            .currentPointer = Pointer::StartOf(Rc::new(funcContainer));
        self.PassArgumentsToEvaluationStack(arguments);
        self.sync_variables_callstack();
    }

    pub fn PassArgumentsToEvaluationStack(&mut self, arguments: Vec<ContentItem>) {
        for arg in arguments {
            self.PushEvaluationStack(arg);
        }
    }

    pub fn TryExitFunctionEvaluationFromGame(&mut self) -> bool {
        if self.currentFlow.callStack.currentElement().r#type
            == PushPopType::FunctionEvaluationFromGame
        {
            self.currentPointer_set(Pointer::Null());
            self.didSafeExit = true;
            true
        } else {
            false
        }
    }

    pub fn CompleteFunctionEvaluationFromGame(&mut self) -> Option<Value> {
        if self.currentFlow.callStack.currentElement().r#type
            != PushPopType::FunctionEvaluationFromGame
        {
            panic!(
                "Expected external function evaluation to be complete. Stack trace: {}",
                self.currentFlow.callStack.get_callStackTrace()
            );
        }

        let originalEvaluationStackHeight = self
            .currentFlow
            .callStack
            .currentElement()
            .evaluationStackHeightWhenPushed as usize;

        let mut returnedObj: Option<ContentItem> = None;
        while self.evaluationStack.len() > originalEvaluationStackHeight {
            let poppedObj = self.PopEvaluationStack();
            if returnedObj.is_none() {
                returnedObj = Some(poppedObj);
            }
        }

        self.PopCallstack(Some(PushPopType::FunctionEvaluationFromGame));

        let returnedObj = returnedObj?;
        match returnedObj {
            ContentItem::Void(_) => None,
            ContentItem::Value(Value::DivertTarget(divert)) => {
                divert.value.map(|path| Value::new_string(path.ToString()))
            }
            ContentItem::Value(value) => Some(value),
            _ => None,
        }
    }

    pub fn AddError(&mut self, message: String, isWarning: bool) {
        if isWarning {
            self.currentWarnings
                .get_or_insert_with(Vec::new)
                .push(message);
        } else {
            self.currentErrors
                .get_or_insert_with(Vec::new)
                .push(message);
        }
    }

    pub fn ObserveVariable(&mut self, variableName: String, observer: VariableObserver) {
        self.variablesState.ObserveVariable(variableName, observer);
    }

    pub fn ObserveVariables(&mut self, variableNames: Vec<String>, observer: VariableObserver) {
        self.variablesState
            .ObserveVariables(variableNames, observer);
    }

    pub fn RemoveVariableObserver(
        &mut self,
        observer: Option<&VariableObserver>,
        specificVariableName: Option<&str>,
    ) {
        self.variablesState
            .RemoveVariableObserver(observer, specificVariableName);
    }

    pub fn get_callstackDepth(&self) -> i32 {
        self.currentFlow.callStack.currentElementIndex() + 1
    }

    pub fn get_outputStream(&self) -> Vec<ContentItem> {
        self.currentFlow.outputStream.clone()
    }

    pub fn get_currentChoices(&self) -> Vec<Choice> {
        if self.get_canContinue() {
            Vec::new()
        } else {
            self.currentFlow.currentChoices.clone()
        }
    }

    pub fn get_generatedChoices(&self) -> Vec<Choice> {
        self.currentFlow.currentChoices.clone()
    }

    // C# signature: internal void AddGeneratedChoice(Choice choice)
    pub fn AddGeneratedChoice(&mut self, choice: Choice) {
        self.currentFlow.currentChoices.push(choice);
    }

    // C# signature: internal void ClearGeneratedChoices()
    pub fn ClearGeneratedChoices(&mut self) {
        self.currentFlow.currentChoices.clear();
    }

    // C# signature: internal void PushCallstack(PushPopType type, int externalEvaluationStackHeight = 0, int outputStreamLengthWithPushed = 0)
    pub fn PushCallstack(
        &mut self,
        type_: crate::PushPop::PushPopType,
        externalEvaluationStackHeight: i32,
        outputStreamLengthWithPushed: i32,
    ) {
        self.currentFlow.callStack.Push(
            type_,
            externalEvaluationStackHeight,
            outputStreamLengthWithPushed,
        );
        self.sync_variables_callstack();
    }

    // C# signature: internal void SetCurrentThread(CallStack.Thread thread)
    pub fn SetCurrentThread(&mut self, thread: crate::CallStack::Thread) {
        self.currentFlow.callStack.set_currentThread(thread);
        if let Some(current_element) = self
            .currentFlow
            .callStack
            .currentThread_mut()
            .callstack
            .last_mut()
        {
            current_element.inExpressionEvaluation = false;
        }
        self.variablesState
            .set_callStack(self.currentFlow.callStack.clone());
    }

    // C# signature: internal CallStack.Thread ForkThread()
    pub fn ForkThread(&mut self) -> crate::CallStack::Thread {
        self.currentFlow.callStack.ForkThread()
    }

    // C# signature: internal void PushThread()
    pub fn PushThread(&mut self) {
        self.currentFlow.callStack.PushThread();
        self.sync_variables_callstack();
    }

    // C# signature: internal void PopThread()
    pub fn PopThread(&mut self) {
        self.currentFlow.callStack.PopThread();
        self.sync_variables_callstack();
    }

    pub fn get_currentErrors(&self) -> Vec<String> {
        self.currentErrors.clone().unwrap_or_default()
    }

    pub fn get_currentWarnings(&self) -> Vec<String> {
        self.currentWarnings.clone().unwrap_or_default()
    }

    pub fn get_variablesState(&self) -> VariablesState {
        self.variablesState.clone()
    }

    pub fn get_variablesState_mut(&mut self) -> &mut VariablesState {
        &mut self.variablesState
    }

    pub fn set_variablesState(&mut self, variablesState: VariablesState) {
        self.variablesState = variablesState;
    }

    pub fn get_callStack(&self) -> CallStack {
        self.currentFlow.callStack.clone()
    }

    pub fn get_evaluationStack(&self) -> Vec<ContentItem> {
        self.evaluationStack.clone()
    }

    pub fn get_divertedPointer(&self) -> Pointer {
        self.divertedPointer.clone()
    }

    // C# signature: internal void set_divertedPointer(Pointer value)
    pub fn set_divertedPointer(&mut self, value: Pointer) {
        self.divertedPointer = value;
    }

    pub fn get_currentTurnIndex(&self) -> i32 {
        self.currentTurnIndex
    }

    pub fn get_storySeed(&self) -> i32 {
        self.storySeed
    }

    pub fn set_storySeed(&mut self, value: i32) {
        self.storySeed = value;
    }

    pub fn get_previousRandom(&self) -> i32 {
        self.previousRandom
    }

    pub fn set_previousRandom(&mut self, value: i32) {
        self.previousRandom = value;
    }

    pub fn get_didSafeExit(&self) -> bool {
        self.didSafeExit
    }

    // C# signature: internal void set_didSafeExit(bool value)
    pub fn set_didSafeExit(&mut self, value: bool) {
        self.didSafeExit = value;
    }

    pub fn get_story(&self) -> Story {
        self.story.clone()
    }

    pub fn get_currentPathString(&self) -> Option<String> {
        let pointer = self.currentPointer();
        if pointer.get_isNull() {
            None
        } else {
            pointer.get_path().map(|path| path.ToString())
        }
    }

    pub fn get_previousPathString(&self) -> Option<String> {
        let pointer = self.previousPointer();
        if pointer.get_isNull() {
            None
        } else {
            pointer.get_path().map(|path| path.ToString())
        }
    }

    pub fn get_currentPointer(&self) -> Pointer {
        self.currentPointer()
    }

    // C# signature: internal void set_currentPointer(Pointer value)
    pub fn set_currentPointer(&mut self, value: Pointer) {
        self.currentPointer_set(value);
    }

    pub fn get_previousPointer(&self) -> Pointer {
        self.previousPointer()
    }

    // C# signature: internal void set_previousPointer(Pointer value)
    pub fn set_previousPointer(&mut self, value: Pointer) {
        self.previousPointer_set(value);
    }

    pub fn get_canContinue(&self) -> bool {
        !self.currentPointer().get_isNull() && !self.get_hasError()
    }

    pub fn get_hasError(&self) -> bool {
        self.currentErrors
            .as_ref()
            .map(|errs| !errs.is_empty())
            .unwrap_or(false)
    }

    pub fn get_hasWarning(&self) -> bool {
        self.currentWarnings
            .as_ref()
            .map(|warns| !warns.is_empty())
            .unwrap_or(false)
    }

    pub fn get_currentText(&mut self) -> String {
        if self.outputStreamTextDirty {
            let mut sb = String::new();
            let mut inTag = false;
            for outputObj in &self.currentFlow.outputStream {
                match outputObj {
                    ContentItem::Value(Value::String(textContent)) if !inTag => {
                        sb.push_str(&textContent.value);
                    }
                    ContentItem::ControlCommand(controlCommand) => {
                        if controlCommand.commandType == CommandType::BeginTag {
                            inTag = true;
                        } else if controlCommand.commandType == CommandType::EndTag {
                            inTag = false;
                        }
                    }
                    _ => {}
                }
            }
            self.currentTextCache = self.CleanOutputWhitespace(sb);
            self.outputStreamTextDirty = false;
        }
        self.currentTextCache.clone()
    }

    pub fn get_currentTags(&mut self) -> Vec<String> {
        if self.outputStreamTagsDirty {
            self.currentTagsCache = Vec::new();
            let mut inTag = false;
            let mut sb = String::new();
            for outputObj in &self.currentFlow.outputStream {
                match outputObj {
                    ContentItem::ControlCommand(controlCommand) => {
                        if controlCommand.commandType == CommandType::BeginTag {
                            if inTag && !sb.is_empty() {
                                let cleaned = self.CleanOutputWhitespace(sb.clone());
                                self.currentTagsCache.push(cleaned);
                                sb.clear();
                            }
                            inTag = true;
                        } else if controlCommand.commandType == CommandType::EndTag {
                            if !sb.is_empty() {
                                let cleaned = self.CleanOutputWhitespace(sb.clone());
                                self.currentTagsCache.push(cleaned);
                                sb.clear();
                            }
                            inTag = false;
                        }
                    }
                    ContentItem::Value(Value::String(strVal)) if inTag => {
                        sb.push_str(&strVal.value);
                    }
                    ContentItem::Tag(tag) => {
                        if !tag.get_text().is_empty() {
                            self.currentTagsCache.push(tag.get_text().to_string());
                        }
                    }
                    _ => {}
                }
            }
            if !sb.is_empty() {
                let cleaned = self.CleanOutputWhitespace(sb.clone());
                self.currentTagsCache.push(cleaned);
            }
            self.outputStreamTagsDirty = false;
        }
        self.currentTagsCache.clone()
    }

    pub fn get_currentFlowName(&self) -> String {
        self.currentFlow.name.clone()
    }

    pub fn get_currentFlowIsDefaultFlow(&self) -> bool {
        self.currentFlow.name == Self::kDefaultFlowName
    }

    pub fn get_aliveFlowNames(&mut self) -> Vec<String> {
        if self.aliveFlowNamesDirty {
            self.aliveFlowNamesCache = Vec::new();
            if let Some(named) = &self.namedFlows {
                for flowName in named.keys() {
                    if flowName != Self::kDefaultFlowName {
                        self.aliveFlowNamesCache.push(flowName.clone());
                    }
                }
            }
            self.aliveFlowNamesDirty = false;
        }
        self.aliveFlowNamesCache.clone()
    }

    pub fn get_inExpressionEvaluation(&self) -> bool {
        self.currentFlow
            .callStack
            .currentElement()
            .inExpressionEvaluation
    }

    pub fn set_inExpressionEvaluation(&mut self, value: bool) {
        self.currentFlow
            .callStack
            .currentThread_mut()
            .callstack
            .last_mut()
            .unwrap()
            .inExpressionEvaluation = value;
    }

    pub fn get_outputStreamEndsInNewline(&self) -> bool {
        for obj in self.currentFlow.outputStream.iter().rev() {
            match obj {
                ContentItem::ControlCommand(_) => break,
                ContentItem::Value(Value::String(text)) => {
                    if text.isNewline {
                        return true;
                    } else if text.get_isNonWhitespace() {
                        break;
                    }
                }
                _ => {}
            }
        }
        false
    }

    pub fn get_outputStreamContainsContent(&self) -> bool {
        self.currentFlow
            .outputStream
            .iter()
            .any(|content| matches!(content, ContentItem::Value(Value::String(_))))
    }

    pub fn get_inStringEvaluation(&self) -> bool {
        self.currentFlow.outputStream.iter().rev().any(|obj| {
            matches!(obj, ContentItem::ControlCommand(cmd) if cmd.commandType == CommandType::BeginString)
        })
    }

    fn WriteJson(&mut self, writer: &mut Writer) {
        writer
            .WriteObject(|writer| {
                writer.WritePropertyStart("flows".to_string())?;
                writer.WriteObjectStart()?;
                if let Some(namedFlows) = &self.namedFlows {
                    let mut flows_to_write = namedFlows.clone();
                    flows_to_write.insert(self.currentFlow.name.clone(), self.currentFlow.clone());
                    for (name, flow) in &flows_to_write {
                        writer.WriteProperty(name.clone(), |writer| {
                            let mut flow = flow.clone();
                            flow.WriteJson(writer);
                            Ok(())
                        })?;
                    }
                } else {
                    let mut flow = self.currentFlow.clone();
                    writer.WriteProperty(self.currentFlow.name.clone(), |writer| {
                        flow.WriteJson(writer);
                        Ok(())
                    })?;
                }
                writer.WriteObjectEnd()?;
                writer.WritePropertyEnd()?;

                writer.WriteProperty_overload_3(
                    "currentFlowName".to_string(),
                    self.currentFlow.name.clone(),
                )?;

                writer.WritePropertyStart("variablesState".to_string())?;
                self.variablesState.WriteJson(writer);
                writer.WritePropertyEnd()?;

                writer.WritePropertyStart("evalStack".to_string())?;
                Json::WriteListRuntimeObjs(writer, &self.evaluationStack);
                writer.WritePropertyEnd()?;

                if !self.divertedPointer.get_isNull() {
                    if let Some(path) = self.divertedPointer.get_path() {
                        writer.WriteProperty_overload_3(
                            "currentDivertTarget".to_string(),
                            path.get_componentsString(),
                        )?;
                    }
                }

                writer.WritePropertyStart("visitCounts".to_string())?;
                writer.WriteObjectStart()?;
                for (k, v) in &self.visitCounts {
                    writer.WriteProperty_overload_4(k.clone(), *v)?;
                }
                writer.WriteObjectEnd()?;
                writer.WritePropertyEnd()?;

                writer.WritePropertyStart("turnIndices".to_string())?;
                writer.WriteObjectStart()?;
                for (k, v) in &self.turnIndices {
                    writer.WriteProperty_overload_4(k.clone(), *v)?;
                }
                writer.WriteObjectEnd()?;
                writer.WritePropertyEnd()?;

                writer.WriteProperty_overload_4("turnIdx".to_string(), self.currentTurnIndex)?;
                writer.WriteProperty_overload_4("storySeed".to_string(), self.storySeed)?;
                writer
                    .WriteProperty_overload_4("previousRandom".to_string(), self.previousRandom)?;
                writer.WriteProperty_overload_4(
                    "inkSaveVersion".to_string(),
                    Self::kInkSaveStateVersion,
                )?;

                Ok(())
            })
            .unwrap_or_else(|err| panic!("{}", err));
    }

    fn LoadJsonObj(&mut self, jObject: JsonObject) {
        let jSaveVersion = match jObject.get("inkSaveVersion") {
            Some(JsonValue::Int(value)) => *value,
            _ => panic!("ink save format incorrect, can't load."),
        };
        if jSaveVersion < Self::kMinCompatibleLoadVersion {
            panic!(
                "Ink save format isn't compatible with the current version (saw '{}', but minimum is {}), so can't load.",
                jSaveVersion,
                Self::kMinCompatibleLoadVersion
            );
        }

        if let Some(JsonValue::Object(flowsObj)) = jObject.get("flows") {
            if flowsObj.len() == 1 {
                self.namedFlows = None;
            } else if self.namedFlows.is_none() {
                self.namedFlows = Some(HashMap::new());
            } else if let Some(named) = self.namedFlows.as_mut() {
                named.clear();
            }

            for (name, flowObj) in flowsObj {
                if let JsonValue::Object(flowObj) = flowObj {
                    let flow =
                        Flow::new_overload_2(name.clone(), self.story.clone(), flowObj.clone());
                    if flowsObj.len() == 1 {
                        self.currentFlow = flow.clone();
                    } else {
                        self.namedFlows.as_mut().unwrap().insert(name.clone(), flow);
                    }
                }
            }

            if let Some(named) = &self.namedFlows {
                if named.len() > 1 {
                    if let Some(JsonValue::String(currFlowName)) = jObject.get("currentFlowName") {
                        self.currentFlow = named
                            .get(currFlowName)
                            .cloned()
                            .unwrap_or_else(|| self.currentFlow.clone());
                    }
                }
            }
        } else {
            self.namedFlows = None;
            self.currentFlow.name = Self::kDefaultFlowName.to_string();
            if let Some(JsonValue::Object(callstackThreads)) = jObject.get("callstackThreads") {
                self.currentFlow
                    .callStack
                    .SetJsonToken(callstackThreads.clone(), self.story.clone());
            }
            if let Some(JsonValue::Array(outputStream)) = jObject.get("outputStream") {
                self.currentFlow.outputStream =
                    Json::JArrayToRuntimeObjList_overload_2(outputStream.clone(), false);
            }
            if let Some(JsonValue::Array(currentChoices)) = jObject.get("currentChoices") {
                self.currentFlow.currentChoices =
                    Json::JArrayToRuntimeObjList::<Choice>(currentChoices.clone(), false);
            }
            let mut jChoiceThreadsObj = None;
            if let Some(obj) = jObject.get("choiceThreads") {
                jChoiceThreadsObj = Some(obj.clone());
            }
            if let Some(JsonValue::Object(choiceThreads)) = jChoiceThreadsObj {
                self.currentFlow
                    .LoadFlowChoiceThreads(choiceThreads, self.story.clone());
            }
        }

        self.OutputStreamDirty();
        self.aliveFlowNamesDirty = true;

        if let Some(JsonValue::Object(vars)) = jObject.get("variablesState") {
            self.variablesState.SetJsonToken(vars.clone());
        }
        self.variablesState
            .set_callStack(self.currentFlow.callStack.clone());

        if let Some(JsonValue::Array(evalStack)) = jObject.get("evalStack") {
            self.evaluationStack =
                Json::JArrayToRuntimeObjList_overload_2(evalStack.clone(), false);
        }

        if let Some(currentDivertTargetPath) = jObject.get("currentDivertTarget") {
            self.divertedPointer = Pointer::Null();
            if let JsonValue::String(path) = currentDivertTargetPath {
                self.divertedPointer = self.story.PointerAtPath(Path::new_overload_4(path.clone()));
            }
        }

        if let Some(JsonValue::Object(visitCounts)) = jObject.get("visitCounts") {
            self.visitCounts = Json::JObjectToIntDictionary(visitCounts.clone());
        }
        if let Some(JsonValue::Object(turnIndices)) = jObject.get("turnIndices") {
            self.turnIndices = Json::JObjectToIntDictionary(turnIndices.clone());
        }

        self.currentTurnIndex = match jObject.get("turnIdx") {
            Some(JsonValue::Int(value)) => *value,
            _ => 0,
        };
        self.storySeed = match jObject.get("storySeed") {
            Some(JsonValue::Int(value)) => *value,
            _ => self.storySeed,
        };
        self.previousRandom = match jObject.get("previousRandom") {
            Some(JsonValue::Int(value)) => *value,
            _ => 0,
        };
    }

    fn currentPointer(&self) -> Pointer {
        self.currentFlow
            .callStack
            .currentElement()
            .currentPointer
            .clone()
    }

    fn currentPointer_set(&mut self, pointer: Pointer) {
        self.currentFlow
            .callStack
            .currentThread_mut()
            .callstack
            .last_mut()
            .unwrap()
            .currentPointer = pointer;
    }

    fn previousPointer(&self) -> Pointer {
        self.currentFlow
            .callStack
            .currentThread()
            .previousPointer
            .clone()
            .unwrap_or_else(Pointer::Null)
    }

    fn previousPointer_set(&mut self, pointer: Pointer) {
        self.currentFlow
            .callStack
            .currentThread_mut()
            .previousPointer = Some(pointer);
    }

    fn OutputStreamDirty(&mut self) {
        self.outputStreamTextDirty = true;
        self.outputStreamTagsDirty = true;
    }

    fn outputStreamEndsInNewline(&self) -> bool {
        self.get_outputStreamEndsInNewline()
    }

    fn outputStreamContainsContent(&self) -> bool {
        self.get_outputStreamContainsContent()
    }
}

#[cfg(test)]
mod tests {
    use super::StoryState;
    use crate::Story::Story;
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

    #[test]
    fn invokes_load_callback_after_json_roundtrip() {
        let mut state = StoryState::new(Story::default());
        let json = state.ToJson();
        let called = Arc::new(AtomicBool::new(false));
        let callback_called = called.clone();

        state.onDidLoadState = Some(Arc::new(move || {
            callback_called.store(true, Ordering::SeqCst);
        }));

        state.LoadJson(json);
        assert!(called.load(Ordering::SeqCst));
    }
}
