// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/SimpleJson.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct SimpleJson {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct Reader {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct Writer {
    pub _port_marker: (),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum State {
    PortPlaceholder,
}

impl Default for State {
    fn default() -> Self {
        Self::PortPlaceholder
    }
}

#[derive(Clone, Debug, Default)]
pub struct StateElement {
    pub _port_marker: (),
}

impl SimpleJson {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public static Dictionary<string, object> TextToDictionary (string text)
    pub fn TextToDictionary(
        _text: String,
    ) -> std::collections::HashMap<String, crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: public static List<object> TextToArray(string text)
    pub fn TextToArray(_text: String) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: public Dictionary<string, object> ToDictionary ()
    pub fn ToDictionary(&mut self) -> std::collections::HashMap<String, crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: public List<object> ToArray()
    pub fn ToArray(&mut self) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: public void Clear()
    pub fn Clear(&mut self) {}

    // C# signature: public void WriteObject(Action<Writer> inner)
    pub fn WriteObject(&mut self, _inner: crate::stub::PortStub) {}

    // C# signature: public void WriteObjectStart()
    pub fn WriteObjectStart(&mut self) {}

    // C# signature: public void WriteObjectEnd()
    pub fn WriteObjectEnd(&mut self) {}

    // C# signature: public void WriteProperty(string name, Action<Writer> inner)
    pub fn WriteProperty(&mut self, _name: String, _inner: crate::stub::PortStub) {}

    // C# signature: public void WriteProperty(int id, Action<Writer> inner)
    pub fn WriteProperty_overload_2(&mut self, _id: i32, _inner: crate::stub::PortStub) {}

    // C# signature: public void WriteProperty(string name, string content)
    pub fn WriteProperty_overload_3(&mut self, _name: String, _content: String) {}

    // C# signature: public void WriteProperty(string name, int content)
    pub fn WriteProperty_overload_4(&mut self, _name: String, _content: i32) {}

    // C# signature: public void WriteProperty(string name, bool content)
    pub fn WriteProperty_overload_5(&mut self, _name: String, _content: bool) {}

    // C# signature: public void WritePropertyStart(string name)
    pub fn WritePropertyStart(&mut self, _name: String) {}

    // C# signature: public void WritePropertyStart(int id)
    pub fn WritePropertyStart_overload_2(&mut self, _id: i32) {}

    // C# signature: public void WritePropertyEnd()
    pub fn WritePropertyEnd(&mut self) {}

    // C# signature: public void WritePropertyNameStart()
    pub fn WritePropertyNameStart(&mut self) {}

    // C# signature: public void WritePropertyNameEnd()
    pub fn WritePropertyNameEnd(&mut self) {}

    // C# signature: public void WritePropertyNameInner(string str)
    pub fn WritePropertyNameInner(&mut self, _str: String) {}

    // C# signature: public void WriteArrayStart()
    pub fn WriteArrayStart(&mut self) {}

    // C# signature: public void WriteArrayEnd()
    pub fn WriteArrayEnd(&mut self) {}

    // C# signature: public void Write(int i)
    pub fn Write(&mut self, _i: i32) {}

    // C# signature: public void Write(float f)
    pub fn Write_overload_2(&mut self, _f: f32) {}

    // C# signature: public void Write(string str, bool escape = true)
    pub fn Write_overload_3(&mut self, _str: String, _escape: bool) {}

    // C# signature: public void Write(bool b)
    pub fn Write_overload_4(&mut self, _b: bool) {}

    // C# signature: public void WriteNull()
    pub fn WriteNull(&mut self) {}

    // C# signature: public void WriteStringStart()
    pub fn WriteStringStart(&mut self) {}

    // C# signature: public void WriteStringEnd()
    pub fn WriteStringEnd(&mut self) {}

    // C# signature: public void WriteStringInner(string str, bool escape = true)
    pub fn WriteStringInner(&mut self, _str: String, _escape: bool) {}

    // C# signature: public override string ToString()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }
}
