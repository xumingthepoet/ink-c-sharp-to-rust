// Source: ink-c-sharp/ink-engine-runtime/VariableReference.cs

use crate::Path::Path;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VariableReference {
    name: Option<String>,
    pathForCount: Option<Path>,
}

impl VariableReference {
    // C# signature: public VariableReference (string name)
    pub fn new(name: String) -> Self {
        Self {
            name: Some(name),
            pathForCount: None,
        }
    }

    // C# signature: public VariableReference()
    pub fn new_overload_2() -> Self {
        Self::default()
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        if let Some(name) = &self.name {
            format!("var({})", name)
        } else {
            format!("read_count({})", self.get_pathStringForCount())
        }
    }

    // C# signature: string name { get; }
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    // C# signature: Path pathForCount { get; }
    pub fn get_pathForCount(&self) -> Option<&Path> {
        self.pathForCount.as_ref()
    }

    // C# signature: Container containerForCount { get; }
    pub fn get_containerForCount(&self) -> crate::Container::Container {
        todo!("port runtime VariableReference.containerForCount after Runtime.Object path resolution is translated");
    }

    // C# signature: string pathStringForCount { get; }
    pub fn get_pathStringForCount(&self) -> String {
        self.pathForCount
            .as_ref()
            .map(|path| path.ToString())
            .unwrap_or_default()
    }

    pub fn set_pathStringForCount(&mut self, value: Option<String>) {
        self.pathForCount = value.map(Path::new_overload_4);
    }
}

#[cfg(test)]
mod tests {
    use super::VariableReference;

    #[test]
    fn stringifies_named_and_count_references() {
        let named = VariableReference::new("score".to_string());
        let mut count = VariableReference::new_overload_2();
        count.set_pathStringForCount(Some("knot.stitch".to_string()));

        assert_eq!(named.ToString(), "var(score)");
        assert_eq!(count.ToString(), "read_count(knot.stitch)");
        assert_eq!(count.get_pathStringForCount(), "knot.stitch");
    }
}
