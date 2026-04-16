// Source: ink-c-sharp/ink-engine-runtime/VariableReference.cs

use crate::Container::Container;
use crate::Path::Path;

#[derive(Clone, Debug, Default)]
pub struct VariableReference {
    pub name: Option<String>,
    pub pathForCount: Option<Path>,
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
        Default::default()
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        if let Some(name) = &self.name {
            format!("var({})", name)
        } else {
            format!(
                "read_count({})",
                self.get_pathStringForCount().unwrap_or_default()
            )
        }
    }

    pub fn set_name(&mut self, value: Option<String>) {
        self.name = value;
    }

    // C# signature: string name { get; }
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_pathForCount(&mut self, value: Option<Path>) {
        self.pathForCount = value;
    }

    // C# signature: Path pathForCount { get; }
    pub fn get_pathForCount(&self) -> Option<&Path> {
        self.pathForCount.as_ref()
    }

    // C# signature: Container containerForCount { get; }
    pub fn get_containerForCount(&self) -> Option<Container> {
        // The C# version resolves this through Runtime.Object.ResolvePath.
        // That wiring belongs with Object/Container once those are ported.
        None
    }

    pub fn set_pathStringForCount(&mut self, value: Option<String>) {
        self.pathForCount = value.map(Path::new_overload_4);
    }

    // C# signature: string pathStringForCount { get; }
    pub fn get_pathStringForCount(&self) -> Option<String> {
        self.pathForCount.as_ref().map(Path::ToString)
    }
}

#[cfg(test)]
mod tests {
    use super::VariableReference;

    #[test]
    fn formats_named_and_count_references() {
        assert_eq!(
            VariableReference::new("score".to_string()).ToString(),
            "var(score)"
        );

        let mut count_ref = VariableReference::new_overload_2();
        count_ref.set_pathStringForCount(Some("knot.stitch".to_string()));
        assert_eq!(count_ref.ToString(), "read_count(knot.stitch)");
    }
}
