// Source: ink-c-sharp/compiler/ParsedHierarchy/ExternalDeclaration.cs

use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Story::Story;

#[derive(Clone, Debug, Default)]
pub struct ExternalDeclaration {
    pub identifier: Option<Identifier>,
    pub argumentNames: Vec<String>,
    story: Option<Story>,
}

impl ExternalDeclaration {
    // C# signature: public ExternalDeclaration (Identifier identifier, List<string> argumentNames)
    pub fn new(identifier: Identifier, argumentNames: Vec<String>) -> Self {
        Self {
            identifier: Some(identifier),
            argumentNames,
            story: None,
        }
    }

    // C# signature: public override Ink.Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> Option<()> {
        let decl = self.clone();
        let name = decl.get_name().map(|name| name.to_string());
        if let Some(story) = self.story.as_mut() {
            if name.is_some() {
                story.AddExternal(decl);
            }
        }
        None
    }

    pub fn ResolveReferences(&mut self, context: &mut Story) {
        self.story = Some(context.clone());
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
