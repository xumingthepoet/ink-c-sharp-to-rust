// Source: ink-c-sharp/compiler/InkParser/InkParser_Include.cs

use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::IncludedFile::IncludedFile;
use crate::ParsedHierarchy::Story::Story as ParsedStory;

impl InkParser {
    pub fn IncludeStatement(&mut self) -> Option<IncludedFile> {
        self.Whitespace();

        self.ParseString("INCLUDE".to_string())?;
        self.Whitespace();

        let filename = self
            .ParseUntilCharactersFromString("\n\r".to_string())
            .unwrap_or_default()
            .trim_end_matches([' ', '\t'])
            .to_string();

        let fullFilename = self.get_fileHandler().ResolveInkFilename(&filename).ok()?;

        if self.get_openFilenames().contains(&fullFilename) {
            self.Error(format!(
                "Recursive INCLUDE detected: '{}' is already open.",
                fullFilename
            ));
            let _ = self.ParseUntilCharactersFromString("\r\n".to_string());
            return Some(IncludedFile::new(ParsedStory::default()));
        }

        self.AddOpenFilenameShared(fullFilename.clone());

        let includedString = match self.get_fileHandler().LoadInkFileContents(&fullFilename) {
            Ok(contents) => Some(contents),
            Err(_) => {
                self.Error(format!("Failed to load: '{}'", filename));
                None
            }
        };

        let includedStory = includedString.map(|includedString| {
            let mut parser = InkParser::new(
                includedString,
                Some(filename.clone()),
                self.get_externalErrorHandler(),
                Some(self.clone_fileHandler()),
            );
            parser.set_openFilenames_shared(self.clone_openFilenames());
            parser.AddOpenFilenameShared(fullFilename.clone());
            parser.Parse()
        });

        self.RemoveOpenFilenameShared(fullFilename);

        Some(IncludedFile::new(includedStory.unwrap_or_default()))
    }
}
