// Source: ink-c-sharp/compiler/ParsedHierarchy/Gather.cs

use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Story::Story;
use ink_runtime::Container::Container;
use ink_runtime::Container::ContentItem;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Gather {
    identifier: Option<Identifier>,
    indentationDepth: i32,
    countAllVisits: bool,
}

impl Gather {
    // C# signature: public Gather (Identifier identifier, int indentationDepth)
    pub fn new(identifier: Identifier, indentationDepth: i32) -> Self {
        Self {
            identifier: Some(identifier),
            indentationDepth,
            countAllVisits: false,
        }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> ContentItem {
        let mut container = Container::new();
        container.set_name(self.get_name().map(|name| name.to_string()));

        if self.countAllVisits {
            container.set_countFlags(1 | 4);
        } else {
            container.set_countFlags(4);
        }

        ContentItem::Container(Box::new(container))
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        self.countAllVisits = context.get_countAllVisits();

        if let Some(identifier) = &self.identifier {
            if identifier
                .name
                .as_ref()
                .map(|name| !name.is_empty())
                .unwrap_or(false)
            {
                context.CheckForNamingCollisions(
                    Default::default(),
                    identifier.clone(),
                    crate::ParsedHierarchy::Story::SymbolType::SubFlowAndWeave,
                    String::new(),
                );
            }
        }
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

    // C# signature: int indentationDepth { get; }
    pub fn get_indentationDepth(&self) -> i32 {
        self.indentationDepth
    }

    // C# signature: Runtime.Container runtimeContainer { get; }
    pub fn get_runtimeContainer(&mut self) -> Option<Container> {
        match self.GenerateRuntimeObject() {
            ContentItem::Container(container) => Some(*container),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Gather;
    use crate::ParsedHierarchy::Identifier::Identifier;

    #[test]
    fn stores_identifier_and_depth() {
        let gather = Gather::new(
            Identifier {
                name: Some("label".to_string()),
                debugMetadata: None,
            },
            3,
        );

        assert_eq!(gather.get_name(), Some("label"));
        assert_eq!(gather.get_indentationDepth(), 3);
    }
}
