// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/InkStringConversionExtensions.cs

#[derive(Clone, Debug, Default)]
pub struct InkStringConversionExtensions {
    pub _port_marker: (),
}

impl InkStringConversionExtensions {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public static string[] ToStringsArray<T>(this List<T> list)
    pub fn ToStringsArray<T: ToString>(_list: &[T]) -> Vec<String> {
        _list.iter().map(ToString::to_string).collect()
    }
}
