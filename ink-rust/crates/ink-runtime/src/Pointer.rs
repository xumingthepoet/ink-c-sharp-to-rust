// Source: ink-c-sharp/ink-engine-runtime/Pointer.cs

use crate::Container::{Container, ContentItem};
use crate::Path::{Component, Path};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pointer {
    pub container: Option<Container>,
    pub index: i32,
}

impl Pointer {
    // C# signature: public Pointer (Container container, int index)
    pub fn new(container: Container, index: i32) -> Self {
        Self {
            container: Some(container),
            index,
        }
    }

    // C# signature: public Runtime.Object Resolve ()
    pub fn Resolve(&self) -> Option<ContentItem> {
        let container = self.container.as_ref()?;
        if self.index < 0 {
            return Some(ContentItem::Container(Box::new(container.clone())));
        }
        if container.get_content().is_empty() {
            return Some(ContentItem::Container(Box::new(container.clone())));
        }
        container.get_content().get(self.index as usize).cloned()
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        self.to_string()
    }

    // C# signature: public static Pointer StartOf (Container container)
    pub fn StartOf(container: Container) -> Self {
        Self {
            container: Some(container),
            index: 0,
        }
    }

    // C# signature: public static Pointer Null
    pub fn Null() -> Self {
        Self {
            container: None,
            index: -1,
        }
    }

    // C# signature: bool isNull { get; }
    pub fn get_isNull(&self) -> bool {
        self.container.is_none()
    }

    // C# signature: Path path { get; }
    pub fn get_path(&self) -> Option<Path> {
        let container = self.container.as_ref()?;
        if self.index >= 0 {
            Some(
                container
                    .get_path()
                    .PathByAppendingComponent(Component::new(self.index)),
            )
        } else {
            Some(container.get_path().clone())
        }
    }
}

impl std::fmt::Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.container.as_ref() {
            None => f.write_str("Ink Pointer (null)"),
            Some(container) => write!(
                f,
                "Ink Pointer -> {} -- index {}",
                container.get_path(),
                self.index
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Pointer;
    use crate::Container::{Container, ContentItem};
    use crate::ControlCommand::ControlCommand;

    #[test]
    fn resolves_content_and_paths() {
        let mut container = Container::new();
        container.AddContent(ControlCommand::BeginString());
        container.AddContent(ControlCommand::EndString());

        let pointer = Pointer::StartOf(container.clone());
        assert!(!pointer.get_isNull());
        assert!(matches!(
            pointer.Resolve(),
            Some(ContentItem::ControlCommand(_))
        ));
        assert!(pointer.get_path().is_some());
        assert!(Pointer::Null().get_isNull());
    }
}
