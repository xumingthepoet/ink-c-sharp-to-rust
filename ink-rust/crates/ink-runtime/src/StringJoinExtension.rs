// Source: ink-c-sharp/ink-engine-runtime/StringJoinExtension.cs

pub fn Join<T: ToString>(separator: String, objects: Vec<T>) -> String {
    let mut result = String::new();
    let mut first = true;

    for o in objects {
        if !first {
            result.push_str(&separator);
        }

        result.push_str(&o.to_string());
        first = false;
    }

    result
}
