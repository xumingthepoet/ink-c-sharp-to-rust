// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/StringJoinExtension.cs

#[derive(Clone, Debug, Default)]
pub struct StringExt {
    pub _port_marker: (),
}

impl StringExt {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public static string Join<T>(string separator, List<T> objects)
    pub fn Join<T: ToString>(_separator: String, _objects: Vec<T>) -> String {
        _objects
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(&_separator)
    }
}
