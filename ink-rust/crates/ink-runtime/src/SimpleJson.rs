// Source: ink-c-sharp/ink-engine-runtime/SimpleJson.cs

use std::collections::HashMap;
use std::fmt;
use std::io::Write;

pub type JsonObject = HashMap<String, JsonValue>;
pub type JsonArray = Vec<JsonValue>;

#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Array(JsonArray),
    Object(JsonObject),
}

impl Default for JsonValue {
    fn default() -> Self {
        Self::Null
    }
}

#[derive(Clone, Debug, Default)]
pub struct SimpleJson;

#[derive(Clone, Debug, Default)]
pub struct Reader {
    text: Vec<char>,
    offset: usize,
    rootObject: JsonValue,
}

#[derive(Default)]
pub struct Writer {
    output: String,
    stream: Option<Box<dyn Write>>,
    stateStack: Vec<StateElement>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum State {
    #[default]
    None,
    Object,
    Array,
    Property,
    PropertyName,
    String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StateElement {
    pub type_: State,
    pub childCount: i32,
}

impl SimpleJson {
    pub fn new() -> Self {
        Self
    }

    // C# signature: public static Dictionary<string, object> TextToDictionary (string text)
    pub fn TextToDictionary(text: String) -> Result<JsonObject, String> {
        Reader::new(text)?.ToDictionary()
    }

    // C# signature: public static List<object> TextToArray(string text)
    pub fn TextToArray(text: String) -> Result<JsonArray, String> {
        Reader::new(text)?.ToArray()
    }
}

impl Reader {
    pub fn new(text: String) -> Result<Self, String> {
        let mut reader = Self {
            text: text.chars().collect(),
            offset: 0,
            rootObject: JsonValue::Null,
        };

        reader.SkipWhitespace();
        reader.rootObject = reader.ReadObject()?;
        reader.SkipWhitespace();

        if reader.offset != reader.text.len() {
            return Err(format!("Unexpected token at offset {}", reader.offset));
        }

        Ok(reader)
    }

    // C# signature: public Dictionary<string, object> ToDictionary ()
    pub fn ToDictionary(&self) -> Result<JsonObject, String> {
        match &self.rootObject {
            JsonValue::Object(dict) => Ok(dict.clone()),
            _ => Err("Root object is not a dictionary".to_string()),
        }
    }

    // C# signature: public List<object> ToArray()
    pub fn ToArray(&self) -> Result<JsonArray, String> {
        match &self.rootObject {
            JsonValue::Array(list) => Ok(list.clone()),
            _ => Err("Root object is not an array".to_string()),
        }
    }

    fn IsNumberChar(c: char) -> bool {
        c.is_ascii_digit() || c == '.' || c == '-' || c == '+' || c == 'E' || c == 'e'
    }

    fn IsFirstNumberChar(c: char) -> bool {
        c.is_ascii_digit() || c == '-' || c == '+'
    }

    fn ReadObject(&mut self) -> Result<JsonValue, String> {
        let currentChar = self
            .text
            .get(self.offset)
            .copied()
            .ok_or_else(|| "Unexpected EOF while reading JSON".to_string())?;

        if currentChar == '{' {
            return self.ReadDictionary().map(JsonValue::Object);
        }

        if currentChar == '[' {
            return self.ReadArray().map(JsonValue::Array);
        }

        if currentChar == '"' {
            return self.ReadString().map(JsonValue::String);
        }

        if Self::IsFirstNumberChar(currentChar) {
            return self.ReadNumber();
        }

        if self.TryRead("true") {
            return Ok(JsonValue::Bool(true));
        }

        if self.TryRead("false") {
            return Ok(JsonValue::Bool(false));
        }

        if self.TryRead("null") {
            return Ok(JsonValue::Null);
        }

        Err(format!(
            "Unhandled object type in JSON: {}",
            self.remaining_preview(30)
        ))
    }

    fn ReadDictionary(&mut self) -> Result<JsonObject, String> {
        let mut dict = HashMap::new();

        self.Expect("{")?;
        self.SkipWhitespace();

        if self.TryRead("}") {
            return Ok(dict);
        }

        loop {
            self.SkipWhitespace();

            let key = self.ReadString()?;

            self.SkipWhitespace();
            self.Expect(":")?;
            self.SkipWhitespace();

            let val = self.ReadObject()?;
            self.ExpectCondition(!matches!(val, JsonValue::Null), Some("dictionary value"))?;

            dict.insert(key, val);

            self.SkipWhitespace();
            if !self.TryRead(",") {
                break;
            }
        }

        self.Expect("}")?;

        Ok(dict)
    }

    fn ReadArray(&mut self) -> Result<JsonArray, String> {
        let mut list = Vec::new();

        self.Expect("[")?;
        self.SkipWhitespace();

        if self.TryRead("]") {
            return Ok(list);
        }

        loop {
            self.SkipWhitespace();
            list.push(self.ReadObject()?);
            self.SkipWhitespace();

            if !self.TryRead(",") {
                break;
            }
        }

        self.Expect("]")?;

        Ok(list)
    }

    fn ReadString(&mut self) -> Result<String, String> {
        self.Expect("\"")?;

        let mut output = String::new();

        loop {
            let c = self
                .text
                .get(self.offset)
                .copied()
                .ok_or_else(|| "Unexpected EOF while reading string".to_string())?;

            if c == '\\' {
                self.offset += 1;
                let escaped = self
                    .text
                    .get(self.offset)
                    .copied()
                    .ok_or_else(|| "Unexpected EOF while reading string".to_string())?;

                match escaped {
                    '"' | '\\' | '/' => output.push(escaped),
                    'n' => output.push('\n'),
                    't' => output.push('\t'),
                    'r' | 'b' | 'f' => {}
                    'u' => {
                        if self.offset + 4 >= self.text.len() {
                            return Err("Unexpected EOF while reading string".to_string());
                        }

                        let digits: String =
                            self.text[self.offset + 1..self.offset + 5].iter().collect();
                        let value = u32::from_str_radix(&digits, 16).map_err(|_| {
                            format!(
                                "Invalid Unicode escape character at offset {}",
                                self.offset.saturating_sub(1)
                            )
                        })?;
                        let unicode = char::from_u32(value).ok_or_else(|| {
                            format!(
                                "Invalid Unicode escape character at offset {}",
                                self.offset.saturating_sub(1)
                            )
                        })?;
                        output.push(unicode);
                        self.offset += 4;
                    }
                    _ => {
                        return Err(format!(
                            "Invalid Unicode escape character at offset {}",
                            self.offset.saturating_sub(1)
                        ));
                    }
                }
            } else if c == '"' {
                break;
            } else {
                output.push(c);
            }

            self.offset += 1;
        }

        self.Expect("\"")?;

        Ok(output)
    }

    fn ReadNumber(&mut self) -> Result<JsonValue, String> {
        let startOffset = self.offset;
        let mut isFloat = false;

        while let Some(c) = self.text.get(self.offset).copied() {
            if c == '.' || c == 'e' || c == 'E' {
                isFloat = true;
            }

            if Self::IsNumberChar(c) {
                self.offset += 1;
            } else {
                break;
            }
        }

        let numStr: String = self.text[startOffset..self.offset].iter().collect();

        if isFloat {
            if let Ok(f) = numStr.parse::<f32>() {
                return Ok(JsonValue::Float(f));
            }
        } else if let Ok(i) = numStr.parse::<i32>() {
            return Ok(JsonValue::Int(i));
        }

        Err(format!("Failed to parse number value: {}", numStr))
    }

    fn TryRead(&mut self, textToRead: &str) -> bool {
        let chars: Vec<char> = textToRead.chars().collect();

        if self.offset + chars.len() > self.text.len() {
            return false;
        }

        for (i, c) in chars.iter().enumerate() {
            if *c != self.text[self.offset + i] {
                return false;
            }
        }

        self.offset += chars.len();

        true
    }

    fn Expect(&mut self, expectedStr: &str) -> Result<(), String> {
        if self.TryRead(expectedStr) {
            Ok(())
        } else {
            self.ExpectCondition(false, Some(expectedStr))
        }
    }

    fn ExpectCondition(&self, condition: bool, message: Option<&str>) -> Result<(), String> {
        if condition {
            return Ok(());
        }

        let mut output = match message {
            Some(message) => format!("Expected {}", message),
            None => "Unexpected token".to_string(),
        };
        output.push_str(&format!(" at offset {}", self.offset));

        Err(output)
    }

    fn SkipWhitespace(&mut self) {
        while let Some(c) = self.text.get(self.offset) {
            if *c == ' ' || *c == '\t' || *c == '\n' || *c == '\r' {
                self.offset += 1;
            } else {
                break;
            }
        }
    }

    fn remaining_preview(&self, max_len: usize) -> String {
        self.text
            .iter()
            .skip(self.offset)
            .take(max_len)
            .collect::<String>()
    }
}

impl Writer {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            stream: None,
            stateStack: Vec::new(),
        }
    }

    // C# signature: public Writer(Stream stream)
    pub fn new_overload_2(stream: Box<dyn Write>) -> Self {
        Self {
            output: String::new(),
            stream: Some(stream),
            stateStack: Vec::new(),
        }
    }

    // C# signature: public void Clear()
    pub fn Clear(&mut self) {
        if self.stream.is_some() {
            panic!(
                "Writer.Clear() is only supported for the StringWriter variant, not for streams"
            );
        }

        self.output.clear();
    }

    // C# signature: public void WriteObject(Action<Writer> inner)
    pub fn WriteObject<F>(&mut self, inner: F) -> Result<(), String>
    where
        F: FnOnce(&mut Writer) -> Result<(), String>,
    {
        self.WriteObjectStart()?;
        inner(self)?;
        self.WriteObjectEnd()
    }

    // C# signature: public void WriteObjectStart()
    pub fn WriteObjectStart(&mut self) -> Result<(), String> {
        self.StartNewObject(true)?;
        self.stateStack.push(StateElement {
            type_: State::Object,
            childCount: 0,
        });
        self.push_char('{')?;
        Ok(())
    }

    // C# signature: public void WriteObjectEnd()
    pub fn WriteObjectEnd(&mut self) -> Result<(), String> {
        self.Assert(self.state() == State::Object)?;
        self.push_char('}')?;
        self.stateStack.pop();
        if self.state() == State::None {
            self.flush_stream()?;
        }
        Ok(())
    }

    // C# signature: public void WriteProperty(string name, Action<Writer> inner)
    pub fn WriteProperty<F>(&mut self, name: String, inner: F) -> Result<(), String>
    where
        F: FnOnce(&mut Writer) -> Result<(), String>,
    {
        self.WritePropertyStart(name)?;
        inner(self)?;
        self.WritePropertyEnd()
    }

    // C# signature: public void WriteProperty(int id, Action<Writer> inner)
    pub fn WriteProperty_overload_2<F>(&mut self, id: i32, inner: F) -> Result<(), String>
    where
        F: FnOnce(&mut Writer) -> Result<(), String>,
    {
        self.WritePropertyStart_overload_2(id)?;
        inner(self)?;
        self.WritePropertyEnd()
    }

    // C# signature: public void WriteProperty(string name, string content)
    pub fn WriteProperty_overload_3(
        &mut self,
        name: String,
        content: String,
    ) -> Result<(), String> {
        self.WritePropertyStart(name)?;
        self.Write_overload_3(content, true)?;
        self.WritePropertyEnd()
    }

    // C# signature: public void WriteProperty(string name, int content)
    pub fn WriteProperty_overload_4(&mut self, name: String, content: i32) -> Result<(), String> {
        self.WritePropertyStart(name)?;
        self.Write(content)?;
        self.WritePropertyEnd()
    }

    // C# signature: public void WriteProperty(string name, bool content)
    pub fn WriteProperty_overload_5(&mut self, name: String, content: bool) -> Result<(), String> {
        self.WritePropertyStart(name)?;
        self.Write_overload_4(content)?;
        self.WritePropertyEnd()
    }

    // C# signature: public void WritePropertyStart(string name)
    pub fn WritePropertyStart(&mut self, name: String) -> Result<(), String> {
        self.WritePropertyStartInner(name)
    }

    // C# signature: public void WritePropertyStart(int id)
    pub fn WritePropertyStart_overload_2(&mut self, id: i32) -> Result<(), String> {
        self.WritePropertyStartInner(id)
    }

    // C# signature: public void WritePropertyEnd()
    pub fn WritePropertyEnd(&mut self) -> Result<(), String> {
        self.Assert(self.state() == State::Property)?;
        self.Assert(self.childCount() == 1)?;
        self.stateStack.pop();
        Ok(())
    }

    // C# signature: public void WritePropertyNameStart()
    pub fn WritePropertyNameStart(&mut self) -> Result<(), String> {
        self.Assert(self.state() == State::Object)?;

        if self.childCount() > 0 {
            self.push_char(',')?;
        }

        self.push_char('"')?;
        self.IncrementChildCount()?;
        self.stateStack.push(StateElement {
            type_: State::Property,
            childCount: 0,
        });
        self.stateStack.push(StateElement {
            type_: State::PropertyName,
            childCount: 0,
        });

        Ok(())
    }

    // C# signature: public void WritePropertyNameEnd()
    pub fn WritePropertyNameEnd(&mut self) -> Result<(), String> {
        self.Assert(self.state() == State::PropertyName)?;
        self.push_str("\":")?;
        self.stateStack.pop();
        Ok(())
    }

    // C# signature: public void WritePropertyNameInner(string str)
    pub fn WritePropertyNameInner(&mut self, str: String) -> Result<(), String> {
        self.Assert(self.state() == State::PropertyName)?;
        self.push_str(&str)?;
        Ok(())
    }

    // C# signature: public void WriteArrayStart()
    pub fn WriteArrayStart(&mut self) -> Result<(), String> {
        self.StartNewObject(true)?;
        self.stateStack.push(StateElement {
            type_: State::Array,
            childCount: 0,
        });
        self.push_char('[')?;
        Ok(())
    }

    // C# signature: public void WriteArrayEnd()
    pub fn WriteArrayEnd(&mut self) -> Result<(), String> {
        self.Assert(self.state() == State::Array)?;
        self.push_char(']')?;
        self.stateStack.pop();
        Ok(())
    }

    // C# signature: public void Write(int i)
    pub fn Write(&mut self, i: i32) -> Result<(), String> {
        self.StartNewObject(false)?;
        self.push_str(&i.to_string())?;
        Ok(())
    }

    // C# signature: public void Write(float f)
    pub fn Write_overload_2(&mut self, f: f32) -> Result<(), String> {
        self.StartNewObject(false)?;

        if f.is_infinite() {
            if f.is_sign_positive() {
                self.push_str("3.4E+38")?;
            } else {
                self.push_str("-3.4E+38")?;
            }
            return Ok(());
        }

        if f.is_nan() {
            self.push_str("0.0")?;
            return Ok(());
        }

        let mut floatStr = f.to_string();
        self.push_str(&floatStr)?;

        if !floatStr.contains('.') && !floatStr.contains('E') && !floatStr.contains('e') {
            self.push_str(".0")?;
        }

        floatStr.clear();

        Ok(())
    }

    // C# signature: public void Write(string str, bool escape = true)
    pub fn Write_overload_3(&mut self, str: String, escape: bool) -> Result<(), String> {
        self.StartNewObject(false)?;
        self.push_char('"')?;
        self.WriteEscapedStringOrRaw(&str, escape)?;
        self.push_char('"')?;
        Ok(())
    }

    // C# signature: public void Write(bool b)
    pub fn Write_overload_4(&mut self, b: bool) -> Result<(), String> {
        self.StartNewObject(false)?;
        self.push_str(if b { "true" } else { "false" })?;
        Ok(())
    }

    // C# signature: public void WriteNull()
    pub fn WriteNull(&mut self) -> Result<(), String> {
        self.StartNewObject(false)?;
        self.push_str("null")?;
        Ok(())
    }

    // C# signature: public void WriteStringStart()
    pub fn WriteStringStart(&mut self) -> Result<(), String> {
        self.StartNewObject(false)?;
        self.stateStack.push(StateElement {
            type_: State::String,
            childCount: 0,
        });
        self.push_char('"')?;
        Ok(())
    }

    // C# signature: public void WriteStringEnd()
    pub fn WriteStringEnd(&mut self) -> Result<(), String> {
        self.Assert(self.state() == State::String)?;
        self.push_char('"')?;
        self.stateStack.pop();
        Ok(())
    }

    // C# signature: public void WriteStringInner(string str, bool escape = true)
    pub fn WriteStringInner(&mut self, str: String, escape: bool) -> Result<(), String> {
        self.Assert(self.state() == State::String)?;
        self.WriteEscapedStringOrRaw(&str, escape)?;
        Ok(())
    }

    // C# signature: public override string ToString()
    pub fn ToString(&self) -> String {
        self.output.clone()
    }

    fn WritePropertyStartInner<T>(&mut self, name: T) -> Result<(), String>
    where
        T: fmt::Display,
    {
        self.Assert(self.state() == State::Object)?;

        if self.childCount() > 0 {
            self.push_char(',')?;
        }

        self.push_char('"')?;
        self.push_str(&name.to_string())?;
        self.push_str("\":")?;
        self.IncrementChildCount()?;
        self.stateStack.push(StateElement {
            type_: State::Property,
            childCount: 0,
        });
        Ok(())
    }

    fn WriteEscapedStringOrRaw(&mut self, str: &str, escape: bool) -> Result<(), String> {
        if escape {
            self.WriteEscapedString(str)?;
        } else {
            self.push_str(str)?;
        }
        Ok(())
    }

    fn WriteEscapedString(&mut self, str: &str) -> Result<(), String> {
        for c in str.chars() {
            if c < ' ' {
                match c {
                    '\n' => self.push_str("\\n")?,
                    '\t' => self.push_str("\\t")?,
                    _ => {}
                }
            } else {
                match c {
                    '\\' | '"' => {
                        self.push_char('\\')?;
                        self.push_char(c)?;
                    }
                    _ => self.push_char(c)?,
                }
            }
        }
        Ok(())
    }

    fn StartNewObject(&mut self, container: bool) -> Result<(), String> {
        let state = self.state();

        if container {
            self.Assert(state == State::None || state == State::Property || state == State::Array)?;
        } else {
            self.Assert(state == State::Property || state == State::Array)?;
        }

        if state == State::Array && self.childCount() > 0 {
            self.push_char(',')?;
        }

        if state == State::Property {
            self.Assert(self.childCount() == 0)?;
        }

        if state == State::Array || state == State::Property {
            self.IncrementChildCount()?;
        }

        Ok(())
    }

    fn state(&self) -> State {
        self.stateStack
            .last()
            .map(|state| state.type_)
            .unwrap_or(State::None)
    }

    fn childCount(&self) -> i32 {
        self.stateStack
            .last()
            .map(|state| state.childCount)
            .unwrap_or_default()
    }

    fn IncrementChildCount(&mut self) -> Result<(), String> {
        let state = self
            .stateStack
            .last_mut()
            .ok_or_else(|| "Assert failed while writing JSON".to_string())?;
        state.childCount += 1;
        Ok(())
    }

    fn Assert(&self, condition: bool) -> Result<(), String> {
        if condition {
            Ok(())
        } else {
            Err("Assert failed while writing JSON".to_string())
        }
    }

    fn push_str(&mut self, text: &str) -> Result<(), String> {
        self.output.push_str(text);
        if let Some(stream) = self.stream.as_mut() {
            stream
                .write_all(text.as_bytes())
                .map_err(|err| err.to_string())?;
        }
        Ok(())
    }

    fn push_char(&mut self, c: char) -> Result<(), String> {
        let mut buffer = [0; 4];
        let text = c.encode_utf8(&mut buffer);
        self.push_str(text)
    }

    fn flush_stream(&mut self) -> Result<(), String> {
        if let Some(stream) = self.stream.as_mut() {
            stream.flush().map_err(|err| err.to_string())?;
        }
        Ok(())
    }
}

impl fmt::Display for Writer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.output)
    }
}

#[cfg(test)]
mod tests {
    use super::{JsonValue, SimpleJson, Writer};
    use std::cell::RefCell;
    use std::io::{self, Write};
    use std::rc::Rc;

    struct SharedSink(Rc<RefCell<Vec<u8>>>);

    impl Write for SharedSink {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.borrow_mut().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn reads_dictionary_with_nested_values() {
        let dict = SimpleJson::TextToDictionary(
            "{\"int\":1,\"float\":2.5,\"bool\":true,\"array\":[false,null,\"x\\n\"]}".to_string(),
        )
        .unwrap();

        assert_eq!(dict.get("int"), Some(&JsonValue::Int(1)));
        assert_eq!(dict.get("float"), Some(&JsonValue::Float(2.5)));
        assert_eq!(dict.get("bool"), Some(&JsonValue::Bool(true)));
        assert_eq!(
            dict.get("array"),
            Some(&JsonValue::Array(vec![
                JsonValue::Bool(false),
                JsonValue::Null,
                JsonValue::String("x\n".to_string())
            ]))
        );
    }

    #[test]
    fn writes_json_object() {
        let mut writer = Writer::new();

        writer
            .WriteObject(|w| {
                w.WriteProperty_overload_4("count".to_string(), 3)?;
                w.WriteProperty_overload_5("ok".to_string(), true)?;
                w.WriteProperty("nested".to_string(), |w| {
                    w.WriteArrayStart()?;
                    w.Write(1)?;
                    w.Write_overload_3("x\"y".to_string(), true)?;
                    w.WriteArrayEnd()
                })
            })
            .unwrap();

        assert_eq!(
            writer.ToString(),
            "{\"count\":3,\"ok\":true,\"nested\":[1,\"x\\\"y\"]}"
        );
    }

    #[test]
    fn writes_json_object_to_stream() {
        let sink = Rc::new(RefCell::new(Vec::new()));
        let mut writer = Writer::new_overload_2(Box::new(SharedSink(sink.clone())));

        writer
            .WriteObject(|w| {
                w.WriteProperty_overload_4("count".to_string(), 3)?;
                w.WriteProperty_overload_5("ok".to_string(), true)
            })
            .unwrap();

        let output = String::from_utf8(sink.borrow().clone()).unwrap();
        assert_eq!(output, "{\"count\":3,\"ok\":true}");
        assert_eq!(writer.ToString(), "{\"count\":3,\"ok\":true}");
    }
}
