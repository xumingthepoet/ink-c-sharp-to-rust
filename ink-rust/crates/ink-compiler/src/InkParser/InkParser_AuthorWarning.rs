// Source: ink-c-sharp/compiler/InkParser/InkParser_AuthorWarning.cs

use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::AuthorWarning::AuthorWarning;

impl InkParser {
    pub fn AuthorWarning(&mut self) -> Option<AuthorWarning> {
        self.Whitespace();

        let identifier = self.ParseIdentifierWithMetadata()?;
        if identifier.name.as_deref() != Some("TODO") {
            return None;
        }

        self.Whitespace();
        self.ParseString(":".to_string())?;
        self.Whitespace();

        let message = self
            .ParseUntilCharactersFromString("\n\r".to_string())
            .unwrap_or_default();

        Some(AuthorWarning::new(message))
    }

    fn ParseIdentifierWithMetadata(
        &mut self,
    ) -> Option<crate::ParsedHierarchy::Identifier::Identifier> {
        let parsed = self.ParseUntilCharactersFromString(":".to_string())?;
        Some(crate::ParsedHierarchy::Identifier::Identifier {
            name: Some(parsed.trim().to_string()),
            debugMetadata: None,
        })
    }
}
