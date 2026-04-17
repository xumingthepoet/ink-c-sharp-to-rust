// Source: ink-c-sharp/compiler/InkParser/InkParser_AuthorWarning.cs

use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::AuthorWarning::AuthorWarning;

impl InkParser {
    pub fn AuthorWarning(&mut self) -> Option<AuthorWarning> {
        self.Whitespace();

        let identifier = self.IdentifierWithMetadata()?;
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
}
