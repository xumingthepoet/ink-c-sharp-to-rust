// Source: ink-c-sharp/compiler/Compiler.cs

use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::Story::Story as ParsedStory;
use ink_runtime::Error::{ErrorHandler, ErrorType};
use ink_runtime::Story::Story as RuntimeStory;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
pub struct Options {
    pub sourceFilename: Option<String>,
    pub pluginDirectories: Option<Vec<String>>,
    pub countAllVisits: bool,
    pub errorHandler: Option<Rc<RefCell<ErrorHandler>>>,
    pub fileHandler: Option<Arc<dyn crate::FileHandler::IFileHandler + Send + Sync>>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            sourceFilename: None,
            pluginDirectories: None,
            countAllVisits: false,
            errorHandler: None,
            fileHandler: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CommandLineInputResult {
    pub requestsExit: bool,
    pub choiceIdx: i32,
    pub divertedPath: Option<String>,
    pub output: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct DebugSourceRange {
    pub length: i32,
    pub debugMetadata: Option<ink_runtime::DebugMetadata::DebugMetadata>,
    pub text: String,
}

pub struct Compiler {
    inputString: String,
    options: Options,
    parser: Option<InkParser>,
    parsedStory: Option<ParsedStory>,
    runtimeStory: Option<RuntimeStory>,
    _debugSourceRanges: Vec<DebugSourceRange>,
    hadParseError: bool,
}

impl Compiler {
    // C# signature: public Compiler (string inkSource, Options options = null)
    pub fn new(inkSource: String, options: Options) -> Self {
        Self {
            inputString: inkSource,
            options,
            parser: None,
            parsedStory: None,
            runtimeStory: None,
            _debugSourceRanges: Vec::new(),
            hadParseError: false,
        }
    }

    // C# signature: public Parsed.Story Parse()
    pub fn Parse(&mut self) -> ParsedStory {
        let mut parser = InkParser::new(
            self.inputString.clone(),
            self.options.sourceFilename.clone(),
            None,
            self.options.fileHandler.clone(),
        );
        let parsed_story = parser.Parse();
        self.parser = Some(parser);
        self.parsedStory = Some(parsed_story.clone());
        parsed_story
    }

    // C# signature: public Runtime.Story Compile ()
    pub fn Compile(&mut self) -> Option<RuntimeStory> {
        let mut parsed_story = self.Parse();
        parsed_story.countAllVisits = self.options.countAllVisits;

        let runtime_story = parsed_story.ExportRuntime(self.options.errorHandler.clone());
        self.parsedStory = Some(parsed_story);
        self.runtimeStory = runtime_story.clone();
        runtime_story
    }

    // C# signature: public CommandLineInputResult HandleInput (CommandLineInput inputResult)
    pub fn HandleInput(
        &mut self,
        inputResult: crate::CommandLineInput::CommandLineInput,
    ) -> CommandLineInputResult {
        let mut result = CommandLineInputResult::default();

        if inputResult.isHelp {
            result.output = Some("help".to_string());
            return result;
        }

        if inputResult.isExit {
            result.requestsExit = true;
            return result;
        }

        if let Some(choice_idx) = inputResult.choiceInput {
            result.choiceIdx = choice_idx;
            return result;
        }

        if let Some(debug_source) = inputResult.debugSource {
            result.output = Some(format!("DebugSource: {}", debug_source));
            return result;
        }

        if let Some(debug_path) = inputResult.debugPathLookup {
            result.output = Some(format!("DebugSource: {}", debug_path));
            return result;
        }

        if inputResult.userImmediateModeStatement.is_some() {
            result.output = Some(
                "immediate mode input is not yet supported in the compiler front-end".to_string(),
            );
        }

        result
    }

    // C# signature: public void RetrieveDebugSourceForLatestContent ()
    pub fn RetrieveDebugSourceForLatestContent(&mut self) {
        self._debugSourceRanges.clear();
    }

    pub fn get_parsedStory(&mut self) -> ParsedStory {
        self.parsedStory.clone().unwrap_or_default()
    }

    pub fn get_runtimeStory(&mut self) -> Option<RuntimeStory> {
        self.runtimeStory.clone()
    }
}

fn _debug_metadata_from_offset(
    ranges: &[DebugSourceRange],
    offset: i32,
) -> Option<ink_runtime::DebugMetadata::DebugMetadata> {
    let mut curr_offset = 0;
    let mut last_valid_metadata = None;
    for range in ranges {
        if let Some(metadata) = &range.debugMetadata {
            last_valid_metadata = Some(metadata.clone());
        }

        if offset >= curr_offset && offset < curr_offset + range.length {
            return last_valid_metadata;
        }

        curr_offset += range.length;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{Compiler, Options};

    #[test]
    fn compiler_defaults_and_parses() {
        let mut compiler = Compiler::new("Hello world".to_string(), Options::default());
        let parsed = compiler.Parse();
        assert_eq!(parsed.content.len(), 1);
    }
}
