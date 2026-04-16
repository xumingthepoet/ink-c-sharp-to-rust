// Source: ink-c-sharp/compiler/InkStringConversionExtensions.cs

pub fn ToStringsArray<T: ToString>(list: &[T]) -> Vec<String> {
    let mut strings = Vec::with_capacity(list.len());
    for item in list {
        strings.push(item.to_string());
    }
    strings
}
