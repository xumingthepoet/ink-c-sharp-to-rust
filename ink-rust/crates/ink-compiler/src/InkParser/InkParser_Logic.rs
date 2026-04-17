// Source: ink-c-sharp/compiler/InkParser/InkParser_Logic.cs

use crate::CharacterSet::CharacterSet;
use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::Identifier::Identifier;

impl InkParser {
    // C# signature: protected Parsed.Object LogicLine()
    pub fn LogicLine(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected Parsed.Object VariableDeclaration()
    pub fn VariableDeclaration(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected Parsed.VariableAssignment ListDeclaration ()
    pub fn ListDeclaration(&mut self) -> crate::stub::VariableAssignment {
        Default::default()
    }

    // C# signature: protected Parsed.ListDefinition ListDefinition ()
    pub fn ListDefinition(&mut self) -> crate::stub::ListDefinition {
        Default::default()
    }

    // C# signature: protected string ListElementDefinitionSeparator ()
    pub fn ListElementDefinitionSeparator(&mut self) -> String {
        Default::default()
    }

    // C# signature: protected Parsed.ListElementDefinition ListElementDefinition ()
    pub fn ListElementDefinition(&mut self) -> crate::stub::ListElementDefinition {
        Default::default()
    }

    // C# signature: protected Parsed.Object ConstDeclaration()
    pub fn ConstDeclaration(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    fn identifier_char_set(&mut self) -> CharacterSet {
        let mut identifierCharSet = CharacterSet::new();
        identifierCharSet
            .AddRange('A', 'Z')
            .AddRange('a', 'z')
            .AddRange('0', '9')
            .AddRange('_', '_');
        self.ExtendIdentifierCharacterRanges(&mut identifierCharSet);
        identifierCharSet
    }

    // C# signature: protected Identifier IdentifierWithMetadata()
    pub fn IdentifierWithMetadata(&mut self) -> Option<Identifier> {
        let name = self.Identifier()?;
        Some(Identifier {
            name: Some(name),
            debugMetadata: None,
        })
    }

    // C# signature: protected string Identifier()
    pub fn Identifier(&mut self) -> Option<String> {
        let identifierCharSet = self.identifier_char_set();
        let name = self.ParseCharactersFromCharSet(identifierCharSet, true, -1)?;

        if name.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }

        Some(name)
    }
}

#[cfg(test)]
mod tests {
    use crate::InkParser::InkParser::InkParser;

    #[test]
    fn parses_identifiers_and_rejects_digit_only_names() {
        let mut parser = InkParser::new("alpha123".to_string(), None, None, None);
        assert_eq!(parser.Identifier().as_deref(), Some("alpha123"));

        let mut digit_parser = InkParser::new("12345".to_string(), None, None, None);
        assert!(digit_parser.Identifier().is_none());

        let mut metadata_parser = InkParser::new("name".to_string(), None, None, None);
        let identifier = metadata_parser.IdentifierWithMetadata().unwrap();
        assert_eq!(identifier.name.as_deref(), Some("name"));
        assert!(identifier.debugMetadata.is_none());
    }
}
