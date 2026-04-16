// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/JsonSerialisation.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Json {
    pub _port_marker: (),
}

impl Json {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public static List<T> JArrayToRuntimeObjList<T>(List<object> jArray, bool skipLast=false)
    pub fn JArrayToRuntimeObjList(
        _jArray: Vec<crate::stub::PortStub>,
        _skipLast: bool,
    ) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: public static void WriteDictionaryRuntimeObjs(SimpleJson.Writer writer, Dictionary<string, Runtime.Object> dictionary)
    pub fn WriteDictionaryRuntimeObjs(
        _writer: crate::stub::Writer,
        _dictionary: std::collections::HashMap<String, crate::stub::PortStub>,
    ) {
    }

    // C# signature: public static void WriteListRuntimeObjs(SimpleJson.Writer writer, List<Runtime.Object> list)
    pub fn WriteListRuntimeObjs(_writer: crate::stub::Writer, _list: Vec<crate::stub::PortStub>) {}

    // C# signature: public static void WriteIntDictionary(SimpleJson.Writer writer, Dictionary<string, int> dict)
    pub fn WriteIntDictionary(
        _writer: crate::stub::Writer,
        _dict: std::collections::HashMap<String, i32>,
    ) {
    }

    // C# signature: public static void WriteRuntimeObject(SimpleJson.Writer writer, Runtime.Object obj)
    pub fn WriteRuntimeObject(_writer: crate::stub::Writer, _obj: crate::stub::PortStub) {}

    // C# signature: public static Dictionary<string, Runtime.Object> JObjectToDictionaryRuntimeObjs(Dictionary<string, object> jObject)
    pub fn JObjectToDictionaryRuntimeObjs(
        _jObject: std::collections::HashMap<String, crate::stub::PortStub>,
    ) -> std::collections::HashMap<String, crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: public static Dictionary<string, int> JObjectToIntDictionary(Dictionary<string, object> jObject)
    pub fn JObjectToIntDictionary(
        _jObject: std::collections::HashMap<String, crate::stub::PortStub>,
    ) -> std::collections::HashMap<String, i32> {
        Default::default()
    }

    // C# signature: public static Runtime.Object JTokenToRuntimeObject(object token)
    pub fn JTokenToRuntimeObject(_token: crate::stub::PortStub) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public static void WriteRuntimeContainer(SimpleJson.Writer writer, Container container, bool withoutName = false)
    pub fn WriteRuntimeContainer(
        _writer: crate::stub::Writer,
        _container: crate::stub::Container,
        _withoutName: bool,
    ) {
    }

    // C# signature: private static List<string> JArrayToTags(Dictionary<string, object> jObj, Choice choice)
    pub fn JArrayToTags(
        _jObj: std::collections::HashMap<String, crate::stub::PortStub>,
        _choice: crate::stub::Choice,
    ) -> Vec<String> {
        Default::default()
    }

    // C# signature: public static void WriteChoice(SimpleJson.Writer writer, Choice choice)
    pub fn WriteChoice(_writer: crate::stub::Writer, _choice: crate::stub::Choice) {}

    // C# signature: private static void WriteChoiceTags(SimpleJson.Writer writer, Choice choice)
    pub fn WriteChoiceTags(_writer: crate::stub::Writer, _choice: crate::stub::Choice) {}

    // C# signature: public static ListDefinitionsOrigin JTokenToListDefinitions (object obj)
    pub fn JTokenToListDefinitions(
        _obj: crate::stub::PortStub,
    ) -> crate::stub::ListDefinitionsOrigin {
        Default::default()
    }
}
