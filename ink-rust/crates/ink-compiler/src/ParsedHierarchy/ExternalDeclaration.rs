// Source: ink-c-sharp/compiler/ParsedHierarchy/ExternalDeclaration.cs

use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Story::Story;

#[derive(Clone, Debug, Default)]
pub struct ExternalDeclaration {
    pub identifier: Option<Identifier>,
    pub argumentNames: Vec<String>,
}

impl ExternalDeclaration {
    // C# signature: public ExternalDeclaration (Identifier identifier, List<string> argumentNames)
    pub fn new(identifier: Identifier, argumentNames: Vec<String>) -> Self {
        Self {
            identifier: Some(identifier),
            argumentNames,
        }
    }

    // C# signature: public override Ink.Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&self) -> Option<()> {
        None
    }

    pub fn ResolveReferences(&mut self, context: &mut Story) {
        context.AddExternal(self.clone());
    }

    // C# signature: string name { get; }
    pub fn get_name(&self) -> Option<&str> {
        self.identifier
            .as_ref()
            .and_then(|identifier| identifier.name.as_deref())
    }

    // C# signature: Identifier identifier { get; }
    pub fn get_identifier(&self) -> Option<&Identifier> {
        self.identifier.as_ref()
    }

    // C# signature: List<string> argumentNames { get; }
    pub fn get_argumentNames(&self) -> &[String] {
        &self.argumentNames
    }
}

#[cfg(test)]
mod tests {
    use super::ExternalDeclaration;
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::Story::Story;

    #[test]
    fn registers_with_story_during_resolution() {
        let mut story = Story::default();
        let mut decl = ExternalDeclaration::new(
            Identifier {
                name: Some("my_external".to_string()),
                debugMetadata: None,
            },
            vec!["arg".to_string()],
        );

        decl.ResolveReferences(&mut story);

        assert!(story.IsExternal("my_external".to_string()));
        assert_eq!(decl.get_name(), Some("my_external"));
        assert_eq!(decl.get_argumentNames(), &["arg".to_string()]);
        assert!(decl.GenerateRuntimeObject().is_none());
    }
}
