// Source: ink-c-sharp/compiler/ParsedHierarchy/ExternalDeclaration.cs

use crate::ParsedHierarchy::Identifier::Identifier;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
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
    pub fn GenerateRuntimeObject(&self) -> Option<ink_runtime::Object::Object> {
        // Externals are metadata registered on Parsed.Story and do not emit runtime objects.
        None
    }

    // C# signature: string name { get; }
    pub fn get_name(&self) -> Option<&str> {
        self.identifier
            .as_ref()
            .and_then(|identifier| identifier.name.as_ref().map(std::string::String::as_str))
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

    #[test]
    fn exposes_external_metadata_without_runtime_object() {
        let decl = ExternalDeclaration::new(
            Identifier {
                name: Some("look_up".to_string()),
                debugMetadata: None,
            },
            vec!["query".to_string()],
        );

        assert_eq!(decl.get_name(), Some("look_up"));
        assert_eq!(decl.get_argumentNames(), &["query".to_string()]);
        assert!(decl.GenerateRuntimeObject().is_none());
    }
}
