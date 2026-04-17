// Source: ink-c-sharp/ink-engine-runtime/JsonSerialisation.cs

use crate::Choice::Choice;
use crate::ChoicePoint::ChoicePoint;
use crate::Container::{Container, ContentItem};
use crate::ControlCommand::{CommandType, ControlCommand};
use crate::Divert::Divert;
use crate::Glue::Glue;
use crate::InkList::{InkList, InkListItem};
use crate::ListDefinition::ListDefinition;
use crate::ListDefinitionsOrigin::ListDefinitionsOrigin;
use crate::NativeFunctionCall::NativeFunctionCall;
use crate::Path::Path;
use crate::SimpleJson::{JsonArray, JsonObject, JsonValue, Writer};
use crate::Tag::Tag;
use crate::Value::{DivertTargetValue, ListValue, StringValue, Value, VariablePointerValue};
use crate::VariableAssignment::VariableAssignment;
use crate::VariableReference::VariableReference;
use crate::Void::Void;
use std::collections::HashMap;

fn must<T>(result: Result<T, String>) -> T {
    result.unwrap_or_else(|err| panic!("{}", err))
}

pub trait FromJsonToken: Sized {
    fn from_json_token(token: JsonValue) -> Option<Self>;
}

impl FromJsonToken for ContentItem {
    fn from_json_token(token: JsonValue) -> Option<Self> {
        Json::JTokenToRuntimeObject(token)
    }
}

impl FromJsonToken for Choice {
    fn from_json_token(token: JsonValue) -> Option<Self> {
        Json::JTokenToChoice(token)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Json;

impl Json {
    pub fn new() -> Self {
        Self
    }

    // C# signature: public static List<T> JArrayToRuntimeObjList<T>(List<object> jArray, bool skipLast=false)
    pub fn JArrayToRuntimeObjList<T: FromJsonToken>(jArray: JsonArray, skipLast: bool) -> Vec<T> {
        let count = if skipLast {
            jArray.len().saturating_sub(1)
        } else {
            jArray.len()
        };

        let mut list = Vec::with_capacity(jArray.len());
        for token in jArray.into_iter().take(count) {
            if let Some(runtime_obj) = T::from_json_token(token) {
                list.push(runtime_obj);
            }
        }

        list
    }

    pub fn JArrayToRuntimeObjList_overload_2(
        jArray: JsonArray,
        skipLast: bool,
    ) -> Vec<ContentItem> {
        Self::JArrayToRuntimeObjList::<ContentItem>(jArray, skipLast)
    }

    // C# signature: public static void WriteDictionaryRuntimeObjs(SimpleJson.Writer writer, Dictionary<string, Runtime.Object> dictionary)
    pub fn WriteDictionaryRuntimeObjs(
        writer: &mut Writer,
        dictionary: &HashMap<String, ContentItem>,
    ) {
        must(writer.WriteObjectStart());
        for (key, value) in dictionary {
            must(writer.WritePropertyStart(key.clone()));
            Self::WriteRuntimeObject(writer, value);
            must(writer.WritePropertyEnd());
        }
        must(writer.WriteObjectEnd());
    }

    // C# signature: public static void WriteListRuntimeObjs(SimpleJson.Writer writer, List<Runtime.Object> list)
    pub fn WriteListRuntimeObjs(writer: &mut Writer, list: &[ContentItem]) {
        must(writer.WriteArrayStart());
        for val in list {
            Self::WriteRuntimeObject(writer, val);
        }
        must(writer.WriteArrayEnd());
    }

    // C# signature: public static void WriteIntDictionary(SimpleJson.Writer writer, Dictionary<string, int> dict)
    pub fn WriteIntDictionary(writer: &mut Writer, dict: &HashMap<String, i32>) {
        must(writer.WriteObjectStart());
        for (key, value) in dict {
            must(writer.WriteProperty_overload_4(key.clone(), *value));
        }
        must(writer.WriteObjectEnd());
    }

    // C# signature: public static void WriteRuntimeObject(SimpleJson.Writer writer, Runtime.Object obj)
    pub fn WriteRuntimeObject(writer: &mut Writer, obj: &ContentItem) {
        match obj {
            ContentItem::Container(container) => {
                Self::WriteRuntimeContainer(writer, container.as_ref(), false);
            }
            ContentItem::Divert(divert) => {
                let div_type_key = if divert.get_isExternal() {
                    "x()"
                } else if divert.get_pushesToStack() {
                    match divert.get_stackPushType() {
                        crate::PushPop::PushPopType::Function => "f()",
                        crate::PushPop::PushPopType::Tunnel => "->t->",
                        _ => "->",
                    }
                } else {
                    "->"
                };

                let target_str = if divert.hasVariableTarget() {
                    divert.get_variableDivertName().unwrap_or("").to_string()
                } else {
                    divert.get_targetPathString()
                };

                must(writer.WriteObjectStart());
                must(writer.WriteProperty_overload_3(div_type_key.to_string(), target_str));
                if divert.hasVariableTarget() {
                    must(writer.WriteProperty_overload_5("var".to_string(), true));
                }
                if divert.get_isConditional() {
                    must(writer.WriteProperty_overload_5("c".to_string(), true));
                }
                if divert.get_externalArgs() > 0 {
                    must(
                        writer.WriteProperty_overload_4(
                            "exArgs".to_string(),
                            divert.get_externalArgs(),
                        ),
                    );
                }
                must(writer.WriteObjectEnd());
            }
            ContentItem::ChoicePoint(choicePoint) => {
                must(writer.WriteObjectStart());
                must(writer.WriteProperty_overload_3(
                    "*".to_string(),
                    choicePoint.get_pathStringOnChoice().unwrap_or_default(),
                ));
                must(writer.WriteProperty_overload_4("flg".to_string(), choicePoint.get_flags()));
                must(writer.WriteObjectEnd());
            }
            ContentItem::Value(value) => match value {
                Value::Bool(boolVal) => must(writer.Write_overload_4(boolVal.value)),
                Value::Int(intVal) => must(writer.Write(intVal.value)),
                Value::Float(floatVal) => must(writer.Write_overload_2(floatVal.value)),
                Value::String(strVal) => {
                    if strVal.isNewline {
                        must(writer.Write_overload_3("\\n".to_string(), false));
                    } else {
                        must(writer.WriteStringStart());
                        must(writer.WriteStringInner("^".to_string(), true));
                        must(writer.WriteStringInner(strVal.value.clone(), true));
                        must(writer.WriteStringEnd());
                    }
                }
                Value::DivertTarget(divTargetVal) => {
                    must(writer.WriteObjectStart());
                    must(
                        writer.WriteProperty_overload_3(
                            "^->".to_string(),
                            divTargetVal
                                .value
                                .as_ref()
                                .map(Path::ToString)
                                .unwrap_or_default(),
                        ),
                    );
                    must(writer.WriteObjectEnd());
                }
                Value::VariablePointer(varPtrVal) => {
                    must(writer.WriteObjectStart());
                    must(writer.WriteProperty_overload_3(
                        "^var".to_string(),
                        varPtrVal.value.clone().unwrap_or_default(),
                    ));
                    must(writer.WriteProperty_overload_4("ci".to_string(), varPtrVal.contextIndex));
                    must(writer.WriteObjectEnd());
                }
                Value::List(listVal) => Self::WriteInkList(writer, listVal),
            },
            ContentItem::Glue(_) => {
                must(writer.Write_overload_3("<>".to_string(), true));
            }
            ContentItem::ControlCommand(controlCmd) => {
                must(writer.Write_overload_3(
                    Self::control_command_name(controlCmd.get_commandType()).to_string(),
                    true,
                ));
            }
            ContentItem::NativeFunctionCall(nativeFunc) => {
                let mut name = nativeFunc.get_name();
                if name == "^" {
                    name = "L^".to_string();
                }
                must(writer.Write_overload_3(name, true));
            }
            ContentItem::VariableReference(varRef) => {
                must(writer.WriteObjectStart());
                if varRef.get_pathForCount().is_some() {
                    must(writer.WriteProperty_overload_3(
                        "CNT?".to_string(),
                        varRef.get_pathStringForCount(),
                    ));
                } else {
                    must(writer.WriteProperty_overload_3(
                        "VAR?".to_string(),
                        varRef.get_name().unwrap_or("").to_string(),
                    ));
                }
                must(writer.WriteObjectEnd());
            }
            ContentItem::VariableAssignment(varAss) => {
                must(writer.WriteObjectStart());
                let key = if varAss.get_isGlobal() {
                    "VAR="
                } else {
                    "temp="
                };
                must(writer.WriteProperty_overload_3(
                    key.to_string(),
                    varAss.get_variableName().unwrap_or("").to_string(),
                ));
                if !varAss.get_isNewDeclaration() {
                    must(writer.WriteProperty_overload_5("re".to_string(), true));
                }
                must(writer.WriteObjectEnd());
            }
            ContentItem::Void(_) => {
                must(writer.Write_overload_3("void".to_string(), true));
            }
            ContentItem::Tag(tag) => {
                must(writer.WriteObjectStart());
                must(writer.WriteProperty_overload_3("#".to_string(), tag.get_text().to_string()));
                must(writer.WriteObjectEnd());
            }
            ContentItem::Choice(choice) => {
                Self::WriteChoice(writer, choice);
            }
        }
    }

    // C# signature: public static Dictionary<string, Runtime.Object> JObjectToDictionaryRuntimeObjs(Dictionary<string, object> jObject)
    pub fn JObjectToDictionaryRuntimeObjs(jObject: JsonObject) -> HashMap<String, ContentItem> {
        let mut dict = HashMap::with_capacity(jObject.len());
        for (key, value) in jObject {
            if let Some(runtime_obj) = Self::JTokenToRuntimeObject(value) {
                dict.insert(key, runtime_obj);
            }
        }
        dict
    }

    // C# signature: public static Dictionary<string, int> JObjectToIntDictionary(Dictionary<string, object> jObject)
    pub fn JObjectToIntDictionary(jObject: JsonObject) -> HashMap<String, i32> {
        let mut dict = HashMap::with_capacity(jObject.len());
        for (key, value) in jObject {
            if let JsonValue::Int(int_value) = value {
                dict.insert(key, int_value);
            }
        }
        dict
    }

    // C# signature: public static Runtime.Object JTokenToRuntimeObject(object token)
    pub fn JTokenToRuntimeObject(token: JsonValue) -> Option<ContentItem> {
        match token {
            JsonValue::Int(value) => Some(ContentItem::Value(Value::new_int(value))),
            JsonValue::Float(value) => Some(ContentItem::Value(Value::new_float(value))),
            JsonValue::Bool(value) => Some(ContentItem::Value(Value::new_bool(value))),
            JsonValue::String(str) => {
                if let Some(rest) = str.strip_prefix('^') {
                    return Some(ContentItem::Value(Value::new_string(rest.to_string())));
                }

                if str == "\n" {
                    return Some(ContentItem::Value(Value::new_string("\n".to_string())));
                }

                if str == "<>" {
                    return Some(ContentItem::Glue(Glue::new()));
                }

                if let Some(cmd) = Self::command_type_for_name(&str) {
                    return Some(ContentItem::ControlCommand(ControlCommand::new(cmd)));
                }

                if str == "L^" {
                    return Some(ContentItem::NativeFunctionCall(
                        NativeFunctionCall::CallWithName("^".to_string()),
                    ));
                }

                if NativeFunctionCall::CallExistsWithName(str.clone()) {
                    return Some(ContentItem::NativeFunctionCall(
                        NativeFunctionCall::CallWithName(str),
                    ));
                }

                if str == "->->" {
                    return Some(ContentItem::ControlCommand(ControlCommand::PopTunnel()));
                }
                if str == "~ret" {
                    return Some(ContentItem::ControlCommand(ControlCommand::PopFunction()));
                }
                if str == "void" {
                    return Some(ContentItem::Void(Void::new()));
                }

                None
            }
            JsonValue::Object(obj) => {
                if let Some(JsonValue::String(path_str)) = obj.get("^->") {
                    return Some(ContentItem::Value(Value::new_divert_target(Some(
                        Path::new_overload_4(path_str.clone()),
                    ))));
                }

                if let Some(JsonValue::String(var_name)) = obj.get("^var") {
                    let mut var_ptr = VariablePointerValue::new(Some(var_name.clone()), -1);
                    if let Some(JsonValue::Int(context_index)) = obj.get("ci") {
                        var_ptr.contextIndex = *context_index;
                    }
                    return Some(ContentItem::Value(Value::VariablePointer(var_ptr)));
                }

                if let Some(target) = Self::parse_divert(&obj) {
                    return Some(ContentItem::Divert(target));
                }

                if let Some(choice) = Self::parse_choice_point(&obj) {
                    return Some(ContentItem::ChoicePoint(choice));
                }

                if let Some(var_ref) = Self::parse_variable_reference(&obj) {
                    return Some(ContentItem::VariableReference(var_ref));
                }

                if let Some(var_ass) = Self::parse_variable_assignment(&obj) {
                    return Some(ContentItem::VariableAssignment(var_ass));
                }

                if let Some(JsonValue::String(tag_text)) = obj.get("#") {
                    return Some(ContentItem::Tag(Tag::new(tag_text.clone())));
                }

                if let Some(list_val) = Self::parse_list_value(&obj) {
                    return Some(ContentItem::Value(Value::List(list_val)));
                }

                if obj.get("originalChoicePath").is_some() {
                    return Self::JTokenToChoice(JsonValue::Object(obj)).map(ContentItem::Choice);
                }

                None
            }
            JsonValue::Array(list) => Some(ContentItem::Container(Box::new(
                Self::JArrayToContainer(list),
            ))),
            JsonValue::Null => None,
        }
    }

    // C# signature: public static void WriteRuntimeContainer(SimpleJson.Writer writer, Container container, bool withoutName = false)
    pub fn WriteRuntimeContainer(writer: &mut Writer, container: &Container, withoutName: bool) {
        must(writer.WriteArrayStart());
        for content in container.get_content() {
            Self::WriteRuntimeObject(writer, content);
        }

        let named_only_content = container.get_namedOnlyContent();
        let count_flags = container.get_countFlags();
        let has_name_property = !withoutName && container.get_hasValidName();
        let has_terminator = named_only_content
            .as_ref()
            .map(|content| !content.is_empty())
            .unwrap_or(false)
            || count_flags > 0
            || has_name_property;

        if has_terminator {
            must(writer.WriteObjectStart());
        }

        if let Some(named_only_content) = named_only_content {
            for (name, named_content) in named_only_content {
                must(writer.WritePropertyStart(name.clone()));
                if let ContentItem::Container(named_container) = named_content {
                    Self::WriteRuntimeContainer(writer, named_container.as_ref(), true);
                } else {
                    Self::WriteRuntimeObject(writer, &named_content);
                }
                must(writer.WritePropertyEnd());
            }
        }

        if count_flags > 0 {
            must(writer.WriteProperty_overload_4("#f".to_string(), count_flags));
        }

        if has_name_property {
            must(
                writer.WriteProperty_overload_3("#n".to_string(), container.get_name().to_string()),
            );
        }

        if has_terminator {
            must(writer.WriteObjectEnd());
        } else {
            must(writer.WriteNull());
        }

        must(writer.WriteArrayEnd());
    }

    fn JArrayToContainer(jArray: JsonArray) -> Container {
        let mut container = if jArray.is_empty() {
            Container::new()
        } else {
            let content = Self::JArrayToRuntimeObjList_overload_2(jArray.clone(), true);
            Container::from_content(content)
        };

        if let Some(JsonValue::Object(terminating_obj)) = jArray.last().cloned() {
            let mut named_only_content = HashMap::new();

            for (key, value) in terminating_obj {
                match key.as_str() {
                    "#f" => {
                        if let JsonValue::Int(count_flags) = value {
                            container.set_countFlags(count_flags);
                        }
                    }
                    "#n" => {
                        if let JsonValue::String(name) = value {
                            container.set_name(Some(name));
                        }
                    }
                    _ => {
                        if let Some(mut named_content_item) = Self::JTokenToRuntimeObject(value) {
                            if let ContentItem::Container(named_sub_container) =
                                &mut named_content_item
                            {
                                named_sub_container.set_name(Some(key.clone()));
                            }
                            named_only_content.insert(key, named_content_item);
                        }
                    }
                }
            }

            container.set_namedOnlyContent(Some(named_only_content));
        }

        container
    }

    fn parse_divert(obj: &JsonObject) -> Option<Divert> {
        let (is_divert, pushes_to_stack, stack_push_type, external, target) =
            if let Some(JsonValue::String(prop_value)) = obj.get("->") {
                (
                    true,
                    false,
                    crate::PushPop::PushPopType::Function,
                    false,
                    prop_value.clone(),
                )
            } else if let Some(JsonValue::String(prop_value)) = obj.get("f()") {
                (
                    true,
                    true,
                    crate::PushPop::PushPopType::Function,
                    false,
                    prop_value.clone(),
                )
            } else if let Some(JsonValue::String(prop_value)) = obj.get("->t->") {
                (
                    true,
                    true,
                    crate::PushPop::PushPopType::Tunnel,
                    false,
                    prop_value.clone(),
                )
            } else if let Some(JsonValue::String(prop_value)) = obj.get("x()") {
                (
                    true,
                    false,
                    crate::PushPop::PushPopType::Function,
                    true,
                    prop_value.clone(),
                )
            } else {
                (
                    false,
                    false,
                    crate::PushPop::PushPopType::Function,
                    false,
                    String::new(),
                )
            };

        if !is_divert {
            return None;
        }

        let mut divert = if pushes_to_stack {
            Divert::new_overload_2(stack_push_type)
        } else {
            Divert::new()
        };

        divert.set_isExternal(external);
        if let Some(JsonValue::Bool(_)) = obj.get("var") {
            divert.set_variableDivertName(Some(target));
        } else {
            divert.set_targetPathString(Some(target));
        }
        divert.set_isConditional(obj.contains_key("c"));
        if external {
            if let Some(JsonValue::Int(ex_args)) = obj.get("exArgs") {
                divert.set_externalArgs(*ex_args);
            }
        }

        Some(divert)
    }

    fn parse_choice_point(obj: &JsonObject) -> Option<ChoicePoint> {
        let path = match obj.get("*") {
            Some(JsonValue::String(path)) => path.clone(),
            _ => return None,
        };

        let mut choice = ChoicePoint::new_overload_2();
        choice.set_pathStringOnChoice(path);
        if let Some(JsonValue::Int(flags)) = obj.get("flg") {
            choice.set_flags(*flags);
        }
        Some(choice)
    }

    fn parse_variable_reference(obj: &JsonObject) -> Option<VariableReference> {
        if let Some(JsonValue::String(name)) = obj.get("VAR?") {
            return Some(VariableReference::new(name.clone()));
        }
        if let Some(JsonValue::String(path)) = obj.get("CNT?") {
            let mut reference = VariableReference::new_overload_2();
            reference.set_pathStringForCount(Some(path.clone()));
            return Some(reference);
        }
        None
    }

    fn parse_variable_assignment(obj: &JsonObject) -> Option<VariableAssignment> {
        let (is_global, name) = if let Some(JsonValue::String(name)) = obj.get("VAR=") {
            (true, name.clone())
        } else if let Some(JsonValue::String(name)) = obj.get("temp=") {
            (false, name.clone())
        } else {
            return None;
        };

        let is_new_decl = !obj.contains_key("re");
        let mut var_ass = VariableAssignment::new(name, is_new_decl);
        var_ass.set_isGlobal(is_global);
        Some(var_ass)
    }

    fn parse_list_value(obj: &JsonObject) -> Option<ListValue> {
        let list_content = match obj.get("list")? {
            JsonValue::Object(content) => content,
            _ => return None,
        };

        let mut raw_list = InkList::new();
        if let Some(JsonValue::Array(names)) = obj.get("origins") {
            let origin_names = names
                .iter()
                .filter_map(|value| match value {
                    JsonValue::String(name) => Some(name.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>();
            raw_list.SetInitialOriginNames(origin_names);
        }

        for (name, value) in list_content {
            if let JsonValue::Int(val) = value {
                raw_list.insert_entry(InkListItem::new_overload_2(name.clone()), *val);
            }
        }

        let value = raw_list.get_entries().clone();
        let originNames = raw_list.get_originNames();

        Some(ListValue {
            value,
            originNames,
            origins: None,
        })
    }

    fn JArrayToTags(jObj: &JsonObject) -> Option<Vec<String>> {
        match jObj.get("tags") {
            Some(JsonValue::Array(tags)) => Some(
                tags.iter()
                    .filter_map(|value| match value {
                        JsonValue::String(tag) => Some(tag.clone()),
                        _ => None,
                    })
                    .collect(),
            ),
            _ => None,
        }
    }

    fn JTokenToChoice(token: JsonValue) -> Option<Choice> {
        let obj = match token {
            JsonValue::Object(obj) => obj,
            _ => return None,
        };

        let text = match obj.get("text") {
            Some(JsonValue::String(text)) => text.clone(),
            _ => String::new(),
        };
        let index = match obj.get("index") {
            Some(JsonValue::Int(index)) => *index,
            _ => 0,
        };
        let source_path = match obj.get("originalChoicePath") {
            Some(JsonValue::String(path)) => path.clone(),
            _ => String::new(),
        };
        let original_thread_index = match obj.get("originalThreadIndex") {
            Some(JsonValue::Int(idx)) => *idx,
            _ => 0,
        };
        let path_string_on_choice = match obj.get("targetPath") {
            Some(JsonValue::String(path)) => path.clone(),
            _ => String::new(),
        };
        let tags = Self::JArrayToTags(&obj);

        let mut choice = Choice::new();
        choice.set_text(text);
        choice.set_index(index);
        choice.sourcePath = source_path;
        choice.originalThreadIndex = original_thread_index;
        choice.set_pathStringOnChoice(Some(path_string_on_choice));
        if let Some(tags) = tags {
            choice.tags = tags;
        }
        Some(choice)
    }

    // C# signature: public static ListDefinitionsOrigin JTokenToListDefinitions (object obj)
    pub fn JTokenToListDefinitions(obj: JsonValue) -> ListDefinitionsOrigin {
        let defsObj = match obj {
            JsonValue::Object(defs) => defs,
            _ => return ListDefinitionsOrigin::default(),
        };

        let mut allDefs = Vec::new();
        for (name, listDefJson) in defsObj {
            let list_def_json = match listDefJson {
                JsonValue::Object(obj) => obj,
                _ => continue,
            };

            let mut items = HashMap::new();
            for (name_value, int_value) in list_def_json {
                if let JsonValue::Int(value) = int_value {
                    items.insert(name_value, value);
                }
            }
            allDefs.push(ListDefinition::new(name, items));
        }

        ListDefinitionsOrigin::new(allDefs)
    }

    fn command_type_for_name(name: &str) -> Option<CommandType> {
        Some(match name {
            "ev" => CommandType::EvalStart,
            "out" => CommandType::EvalOutput,
            "/ev" => CommandType::EvalEnd,
            "du" => CommandType::Duplicate,
            "pop" => CommandType::PopEvaluatedValue,
            "~ret" => CommandType::PopFunction,
            "->->" => CommandType::PopTunnel,
            "str" => CommandType::BeginString,
            "/str" => CommandType::EndString,
            "nop" => CommandType::NoOp,
            "choiceCnt" => CommandType::ChoiceCount,
            "turn" => CommandType::Turns,
            "turns" => CommandType::TurnsSince,
            "readc" => CommandType::ReadCount,
            "rnd" => CommandType::Random,
            "srnd" => CommandType::SeedRandom,
            "visit" => CommandType::VisitIndex,
            "seq" => CommandType::SequenceShuffleIndex,
            "thread" => CommandType::StartThread,
            "done" => CommandType::Done,
            "end" => CommandType::End,
            "listInt" => CommandType::ListFromInt,
            "range" => CommandType::ListRange,
            "lrnd" => CommandType::ListRandom,
            "#" => CommandType::BeginTag,
            "/#" => CommandType::EndTag,
            _ => return None,
        })
    }

    fn control_command_name(command_type: CommandType) -> &'static str {
        match command_type {
            CommandType::EvalStart => "ev",
            CommandType::EvalOutput => "out",
            CommandType::EvalEnd => "/ev",
            CommandType::Duplicate => "du",
            CommandType::PopEvaluatedValue => "pop",
            CommandType::PopFunction => "~ret",
            CommandType::PopTunnel => "->->",
            CommandType::BeginString => "str",
            CommandType::EndString => "/str",
            CommandType::NoOp => "nop",
            CommandType::ChoiceCount => "choiceCnt",
            CommandType::Turns => "turn",
            CommandType::TurnsSince => "turns",
            CommandType::ReadCount => "readc",
            CommandType::Random => "rnd",
            CommandType::SeedRandom => "srnd",
            CommandType::VisitIndex => "visit",
            CommandType::SequenceShuffleIndex => "seq",
            CommandType::StartThread => "thread",
            CommandType::Done => "done",
            CommandType::End => "end",
            CommandType::ListFromInt => "listInt",
            CommandType::ListRange => "range",
            CommandType::ListRandom => "lrnd",
            CommandType::BeginTag => "#",
            CommandType::EndTag => "/#",
            CommandType::TOTAL_VALUES => panic!("invalid command type"),
            CommandType::NotSet => panic!("invalid command type"),
        }
    }

    fn WriteInkList(writer: &mut Writer, listVal: &ListValue) {
        must(writer.WriteObjectStart());
        must(writer.WritePropertyStart("list".to_string()));
        must(writer.WriteObjectStart());
        for (item, value) in &listVal.value {
            must(writer.WritePropertyNameStart());
            must(writer.WritePropertyNameInner(
                item.originName.clone().unwrap_or_else(|| "?".to_string()),
            ));
            must(writer.WritePropertyNameInner(".".to_string()));
            must(writer.WritePropertyNameInner(item.itemName.clone().unwrap_or_default()));
            must(writer.WritePropertyNameEnd());
            must(writer.Write(*value));
            must(writer.WritePropertyEnd());
        }
        must(writer.WriteObjectEnd());
        must(writer.WritePropertyEnd());

        if listVal.value.is_empty() {
            if let Some(origin_names) = &listVal.originNames {
                if !origin_names.is_empty() {
                    must(writer.WritePropertyStart("origins".to_string()));
                    must(writer.WriteArrayStart());
                    for name in origin_names {
                        must(writer.Write_overload_3(name.clone(), true));
                    }
                    must(writer.WriteArrayEnd());
                    must(writer.WritePropertyEnd());
                }
            }
        }

        must(writer.WriteObjectEnd());
    }

    // C# signature: public static void WriteChoice(SimpleJson.Writer writer, Choice choice)
    pub fn WriteChoice(writer: &mut Writer, choice: &Choice) {
        must(writer.WriteObjectStart());
        must(writer.WriteProperty_overload_3("text".to_string(), choice.get_text().to_string()));
        must(writer.WriteProperty_overload_4("index".to_string(), choice.get_index()));
        must(
            writer.WriteProperty_overload_3(
                "originalChoicePath".to_string(),
                choice.sourcePath.clone(),
            ),
        );
        must(writer.WriteProperty_overload_4(
            "originalThreadIndex".to_string(),
            choice.originalThreadIndex,
        ));
        must(writer.WriteProperty_overload_3(
            "targetPath".to_string(),
            choice.get_pathStringOnChoice().unwrap_or_default(),
        ));
        Self::WriteChoiceTags(writer, choice);
        must(writer.WriteObjectEnd());
    }

    fn WriteChoiceTags(writer: &mut Writer, choice: &Choice) {
        if choice.tags.is_empty() {
            return;
        }

        must(writer.WritePropertyStart("tags".to_string()));
        must(writer.WriteArrayStart());
        for tag in &choice.tags {
            must(writer.Write_overload_3(tag.clone(), true));
        }
        must(writer.WriteArrayEnd());
        must(writer.WritePropertyEnd());
    }
}

#[cfg(test)]
mod tests {
    use super::Json;
    use crate::Container::ContentItem;
    use crate::SimpleJson::Writer;
    use crate::Value::Value;

    #[test]
    fn writes_and_parses_simple_strings() {
        let mut writer = Writer::new();
        writer.WriteArrayStart().unwrap();
        Json::WriteRuntimeObject(
            &mut writer,
            &ContentItem::Value(Value::new_string("hello".to_string())),
        );
        writer.WriteArrayEnd().unwrap();
        assert_eq!(writer.ToString(), "[\"^hello\"]");

        let parsed =
            Json::JTokenToRuntimeObject(crate::SimpleJson::JsonValue::String("^world".to_string()));
        match parsed {
            Some(ContentItem::Value(Value::String(str_val))) => {
                assert_eq!(str_val.value, "world");
            }
            other => panic!("unexpected parsed value: {:?}", other),
        }
    }
}
