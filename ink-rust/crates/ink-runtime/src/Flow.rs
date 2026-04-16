// Source: ink-c-sharp/ink-engine-runtime/Flow.cs

use crate::CallStack::{CallStack, Thread};
use crate::Choice::Choice;
use crate::JsonSerialisation::Json;
use crate::SimpleJson::{JsonObject, JsonValue, Writer};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Flow {
    pub name: String,
    pub callStack: CallStack,
    pub outputStream: Vec<crate::Container::ContentItem>,
    pub currentChoices: Vec<Choice>,
}

impl Flow {
    // C# signature: public Flow(string name, Story story)
    pub fn new(name: String, story: crate::Story::Story) -> Self {
        Self {
            name,
            callStack: CallStack::new(story),
            outputStream: Vec::new(),
            currentChoices: Vec::new(),
        }
    }

    // C# signature: public Flow(string name, Story story, Dictionary<string, object> jObject)
    pub fn new_overload_2(name: String, story: crate::Story::Story, jObject: JsonObject) -> Self {
        let mut flow = Self::new(name, story.clone());
        if let Some(JsonValue::Object(callstack)) = jObject.get("callstack") {
            flow.callStack
                .SetJsonToken(callstack.clone(), story.clone());
        }
        if let Some(JsonValue::Array(output_stream)) = jObject.get("outputStream") {
            flow.outputStream =
                Json::JArrayToRuntimeObjList_overload_2(output_stream.clone(), false);
        }
        if let Some(JsonValue::Array(current_choices)) = jObject.get("currentChoices") {
            flow.currentChoices =
                Json::JArrayToRuntimeObjList::<Choice>(current_choices.clone(), false);
        }

        if let Some(JsonValue::Object(choice_threads)) = jObject.get("choiceThreads") {
            flow.LoadFlowChoiceThreads(choice_threads.clone(), story);
        } else {
            flow.LoadFlowChoiceThreads(HashMap::new(), story);
        }

        flow
    }

    // C# signature: public void WriteJson(SimpleJson.Writer writer)
    pub fn WriteJson(&mut self, writer: &mut Writer) {
        writer
            .WriteObject(|writer| {
                writer.WritePropertyStart("callstack".to_string())?;
                self.callStack.WriteJson(writer);
                writer.WritePropertyEnd()?;

                writer.WritePropertyStart("outputStream".to_string())?;
                Json::WriteListRuntimeObjs(writer, &self.outputStream);
                writer.WritePropertyEnd()?;

                let mut has_choice_threads = false;
                for choice in &mut self.currentChoices {
                    if let Some(thread) = choice.threadAtGeneration.as_ref() {
                        choice.originalThreadIndex = thread.threadIndex;
                        if self
                            .callStack
                            .ThreadWithIndex(choice.originalThreadIndex)
                            .is_none()
                        {
                            if !has_choice_threads {
                                has_choice_threads = true;
                                writer.WritePropertyStart("choiceThreads".to_string())?;
                                writer.WriteObjectStart()?;
                            }
                            writer.WritePropertyStart(choice.originalThreadIndex.to_string())?;
                            thread.WriteJson(writer);
                            writer.WritePropertyEnd()?;
                        }
                    }
                }
                if has_choice_threads {
                    writer.WriteObjectEnd()?;
                    writer.WritePropertyEnd()?;
                }

                writer.WritePropertyStart("currentChoices".to_string())?;
                writer.WriteArrayStart()?;
                for choice in &self.currentChoices {
                    Json::WriteChoice(writer, choice);
                }
                writer.WriteArrayEnd()?;
                writer.WritePropertyEnd()?;

                Ok(())
            })
            .unwrap_or_else(|err| panic!("{}", err));
    }

    // C# signature: public void LoadFlowChoiceThreads(Dictionary<string, object> jChoiceThreads, Story story)
    pub fn LoadFlowChoiceThreads(
        &mut self,
        jChoiceThreads: JsonObject,
        story: crate::Story::Story,
    ) {
        for choice in &mut self.currentChoices {
            if let Some(foundActiveThread) =
                self.callStack.ThreadWithIndex(choice.originalThreadIndex)
            {
                choice.threadAtGeneration = Some(foundActiveThread.Copy());
            } else if let Some(JsonValue::Object(jSavedChoiceThread)) =
                jChoiceThreads.get(&choice.originalThreadIndex.to_string())
            {
                choice.threadAtGeneration = Some(Thread::new_overload_2(
                    jSavedChoiceThread.clone(),
                    story.clone(),
                ));
            }
        }
    }
}
