// Source: ink-c-sharp/ink-engine-runtime/VariableReference.cs

use crate::Path::Path;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VariableReference {
    parent: Option<Box<crate::Container::Container>>,
    name: Option<String>,
    pathForCount: Option<Path>,
}

impl VariableReference {
    // C# signature: public VariableReference (string name)
    pub fn new(name: String) -> Self {
        Self {
            parent: None,
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

    pub fn set_name(&mut self, value: Option<String>) {
        self.name = value;
    }

    // C# signature: Path pathForCount { get; }
    pub fn get_pathForCount(&self) -> Option<&Path> {
        self.pathForCount.as_ref()
    }

    pub fn set_pathForCount(&mut self, value: Option<Path>) {
        self.pathForCount = value;
    }

    // C# signature: Container containerForCount { get; }
    pub fn get_containerForCount(&self) -> crate::Container::Container {
        let path = self
            .pathForCount
            .clone()
            .expect("variable reference count path must be set");
        let parent = self
            .parent
            .as_ref()
            .expect("variable reference must be parented");
        let root = parent
            .get_rootContentContainer()
            .unwrap_or_else(|| parent.as_ref().clone());
        let mut search_root = if path.get_isRelative() {
            parent.as_ref().clone()
        } else {
            root
        };

        search_root
            .ContentAtPath(path, 0, -1)
            .get_container()
            .cloned()
            .expect("variable reference container not found")
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

    pub fn set_parent(&mut self, parent: Option<Box<crate::Container::Container>>) {
        self.parent = parent;
    }

    pub fn get_parent(&self) -> Option<&crate::Container::Container> {
        self.parent.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::VariableReference;
    use crate::Container::Container;

    #[test]
    fn stringifies_named_and_count_references() {
        let named = VariableReference::new("score".to_string());
        let mut count = VariableReference::new_overload_2();
        count.set_pathStringForCount(Some("knot.stitch".to_string()));
        count.set_name(None);

        assert_eq!(named.ToString(), "var(score)");
        assert_eq!(count.ToString(), "read_count(knot.stitch)");
        assert_eq!(count.get_pathStringForCount(), "knot.stitch");
    }

    #[test]
    fn resolves_count_container_from_parent_chain() {
        let mut target = Container::new();
        target.set_name(Some("target".to_string()));

        let mut root = Container::new();
        root.AddToNamedContentOnly(target.clone());

        let mut reference = VariableReference::new_overload_2();
        reference.set_parent(Some(Box::new(root.clone())));
        reference.set_pathStringForCount(Some("target".to_string()));

        let resolved = reference.get_containerForCount();
        assert_eq!(resolved.get_name(), "target");
    }

    #[test]
    fn supports_setting_named_reference_properties() {
        let mut reference = VariableReference::new_overload_2();
        reference.set_name(Some("score".to_string()));
        reference.set_pathForCount(None);
        assert_eq!(reference.get_name(), Some("score"));
        assert!(reference.get_pathForCount().is_none());
    }
}
