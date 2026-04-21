// Source: ink-c-sharp/compiler/Compiler.cs

use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::Divert::Divert;
use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::Object::{Object, ObjectKind};
use crate::ParsedHierarchy::Story::Story as ParsedStory;
use crate::ParsedHierarchy::VariableAssignment::VariableAssignment;
use crate::Plugins::PluginManager::PluginManager;
use ink_runtime::Error::{ErrorHandler, ErrorType};
use ink_runtime::Path::Path as RuntimePath;
use ink_runtime::Story::Story as RuntimeStory;
use ink_runtime::Value::{StringValue, Value};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

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

enum ImmediateStatement {
    Divert(Divert),
    Expression(Expression),
    VariableAssignment(VariableAssignment),
}

pub struct Compiler {
    inputString: String,
    options: Options,
    parser: Option<InkParser>,
    parsedStory: Option<ParsedStory>,
    runtimeStory: Option<RuntimeStory>,
    pluginManager: Option<PluginManager>,
    _debugSourceRanges: Vec<DebugSourceRange>,
    hadParseError: bool,
}

impl Compiler {
    // C# signature: public Compiler (string inkSource, Options options = null)
    pub fn new(inkSource: String, options: Options) -> Self {
        let pluginManager = options
            .pluginDirectories
            .clone()
            .filter(|dirs| !dirs.is_empty())
            .map(PluginManager::new);
        Self {
            inputString: inkSource,
            options,
            parser: None,
            parsedStory: None,
            runtimeStory: None,
            pluginManager,
            _debugSourceRanges: Vec::new(),
            hadParseError: false,
        }
    }

    // C# signature: public Parsed.Story Parse()
    pub fn Parse(&mut self) -> ParsedStory {
        self.hadParseError = false;
        let source_filename = self.options.sourceFilename.clone();
        let had_parse_error = Arc::new(AtomicBool::new(false));
        let parse_errors: Arc<Mutex<Vec<(String, ErrorType)>>> = Arc::new(Mutex::new(Vec::new()));
        let has_error_handler = self.options.errorHandler.is_some();
        let parse_error_handler = {
            let had_parse_error = Arc::clone(&had_parse_error);
            let source_filename = source_filename.clone();
            let parse_errors = Arc::clone(&parse_errors);
            Arc::new(
                move |message: String, line: i32, _character: i32, is_warning: bool| {
                    let full_message = if let Some(filename) = &source_filename {
                        format!(
                            "{}: '{}' line {}: {}",
                            if is_warning { "WARNING" } else { "ERROR" },
                            filename,
                            line + 1,
                            message
                        )
                    } else {
                        format!(
                            "{}: line {}: {}",
                            if is_warning { "WARNING" } else { "ERROR" },
                            line + 1,
                            message
                        )
                    };

                    if !is_warning {
                        had_parse_error.store(true, Ordering::SeqCst);
                    }

                    if has_error_handler {
                        let error_type = if is_warning {
                            ErrorType::Warning
                        } else {
                            ErrorType::Error
                        };
                        parse_errors
                            .lock()
                            .unwrap_or_else(|poisoned| poisoned.into_inner())
                            .push((full_message, error_type));
                    } else {
                        panic!("{}", full_message);
                    }
                },
            )
        };

        let mut parser = InkParser::new(
            self.inputString.clone(),
            self.options.sourceFilename.clone(),
            Some(parse_error_handler),
            self.options.fileHandler.clone(),
        );
        let parsed_story = parser.Parse();
        self.hadParseError = had_parse_error.load(Ordering::SeqCst);
        self.parser = Some(parser);
        self.parsedStory = Some(parsed_story.clone());

        if let Some(handler) = &self.options.errorHandler {
            let mut handler = handler.borrow_mut();
            for (message, error_type) in parse_errors
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .drain(..)
            {
                handler(&message, error_type);
            }
        }

        parsed_story
    }

    // C# signature: public Runtime.Story Compile ()
    pub fn Compile(&mut self) -> Option<RuntimeStory> {
        if let Some(plugin_manager) = self.pluginManager.as_mut() {
            self.inputString = plugin_manager.PreParse(self.inputString.clone());
        }

        let mut parsed_story = self.Parse();
        if let Some(plugin_manager) = self.pluginManager.as_mut() {
            parsed_story = plugin_manager.PostParse(parsed_story);
        }
        if self.hadParseError {
            self.parsedStory = Some(parsed_story);
            self.runtimeStory = None;
            return None;
        }
        parsed_story.countAllVisits = self.options.countAllVisits;

        let mut runtime_story = parsed_story.ExportRuntime(self.options.errorHandler.clone());
        if let (Some(plugin_manager), Some(runtime_story_value)) =
            (self.pluginManager.as_mut(), runtime_story.clone())
        {
            runtime_story =
                Some(plugin_manager.PostExport(parsed_story.clone(), runtime_story_value));
        }

        self.parsedStory = Some(parsed_story);
        self.runtimeStory = None;
        runtime_story
    }

    // C# signature: public CommandLineInputResult HandleInput (CommandLineInput inputResult)
    pub fn HandleInput(
        &mut self,
        inputResult: crate::CommandLineInput::CommandLineInput,
    ) -> Option<CommandLineInputResult> {
        let mut result = CommandLineInputResult::default();
        result.choiceIdx = -1;

        if inputResult.isHelp {
            result.output = Some("help".to_string());
            return Some(result);
        }

        if inputResult.isExit {
            result.requestsExit = true;
            return Some(result);
        }

        if let Some(choice_idx) = inputResult.choiceInput {
            result.choiceIdx = choice_idx;
            return Some(result);
        }

        if let Some(debug_source) = inputResult.debugSource {
            self.RetrieveDebugSourceForLatestContent();
            let dm = self.DebugMetadataForContentAtOffset(debug_source);
            result.output = Some(match dm {
                Some(metadata) => format!("DebugSource: {}", metadata.ToString()),
                None => "DebugSource: Unknown source".to_string(),
            });
            return Some(result);
        }

        if let Some(debug_path) = inputResult.debugPathLookup {
            let path = RuntimePath::new_overload_4(debug_path.clone());
            if let Some(runtime_story) = self.runtimeStory.as_mut() {
                let content_result = runtime_story.ContentAtPath(path);
                if let Some(container) = content_result.get_container() {
                    if let Some(metadata) = container.get_debugMetadata() {
                        result.output = Some(format!("DebugSource: {}", metadata.ToString()));
                    } else {
                        result.output = Some("DebugSource: Unknown source".to_string());
                    }
                } else {
                    result.output = Some("DebugSource: Unknown source".to_string());
                }
            } else {
                result.output = Some("DebugSource: Unknown source".to_string());
            }
            return Some(result);
        }

        if let Some(statement) = inputResult.userImmediateModeStatement.as_ref() {
            if let Some(runtime_story) = self.runtimeStory.as_mut() {
                let parsed_statement = Self::downcast_immediate_statement(statement.as_ref());
                if let Some(parsed_statement) = parsed_statement {
                    return Some(Self::execute_immediate_statement(
                        self.parsedStory
                            .as_mut()
                            .unwrap_or_else(|| panic!("immediate mode requires a parsed story")),
                        runtime_story,
                        parsed_statement,
                    ));
                }
            }
        }

        None
    }

    // C# signature: public void RetrieveDebugSourceForLatestContent ()
    pub fn RetrieveDebugSourceForLatestContent(&mut self) {
        self._debugSourceRanges.clear();
        if let Some(runtime_story) = self.runtimeStory.as_mut() {
            for output_obj in runtime_story.get_state().get_outputStream() {
                if let ink_runtime::Container::ContentItem::Value(Value::String(textContent)) =
                    output_obj
                {
                    self._debugSourceRanges.push(DebugSourceRange {
                        length: textContent.value.len() as i32,
                        debugMetadata: textContent.debugMetadata.clone(),
                        text: textContent.value.clone(),
                    });
                }
            }
        }
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

impl Compiler {
    fn DebugMetadataForContentAtOffset(
        &self,
        offset: i32,
    ) -> Option<ink_runtime::DebugMetadata::DebugMetadata> {
        _debug_metadata_from_offset(&self._debugSourceRanges, offset)
    }

    fn downcast_immediate_statement(statement: &dyn std::any::Any) -> Option<ImmediateStatement> {
        if let Some(divert) = statement.downcast_ref::<Divert>() {
            return Some(ImmediateStatement::Divert(divert.clone()));
        }

        if let Some(expression) = statement.downcast_ref::<Expression>() {
            return Some(ImmediateStatement::Expression(expression.clone()));
        }

        if let Some(assignment) = statement.downcast_ref::<VariableAssignment>() {
            return Some(ImmediateStatement::VariableAssignment(assignment.clone()));
        }

        None
    }

    fn execute_immediate_statement(
        parsed_story: &mut ParsedStory,
        runtime_story: &mut RuntimeStory,
        parsed_statement: ImmediateStatement,
    ) -> CommandLineInputResult {
        let mut result = CommandLineInputResult::default();
        result.choiceIdx = -1;

        match parsed_statement {
            ImmediateStatement::Divert(mut divert) => {
                let _ = divert.GenerateRuntimeObject();
                divert.ResolveReferences(parsed_story);
                result.divertedPath = divert
                    .get_runtimeDivert()
                    .and_then(|runtime_divert| runtime_divert.get_targetPath())
                    .map(|target| target.ToString())
                    .or_else(|| Some(String::new()));
            }
            ImmediateStatement::Expression(mut expression) => {
                let runtime_obj = expression.GenerateRuntimeObject();
                expression.ResolveReferences(parsed_story);
                if let Some(value) = runtime_story.EvaluateExpression(runtime_obj) {
                    result.output = Some(value.ToString());
                }
            }
            ImmediateStatement::VariableAssignment(mut assignment) => {
                if assignment.get_isNewTemporaryDeclaration() {
                    parsed_story.TryAddNewVariableDeclaration(assignment.clone());
                }

                let mut parsed_story_parent = Object::with_kind(ObjectKind::Story);
                parsed_story_parent.content = parsed_story.content.clone();
                assignment
                    .get_base_mut()
                    .set_parent(Some(Box::new(parsed_story_parent)));
                let runtime_object = assignment.GenerateRuntimeObject();
                assignment.ResolveReferences(parsed_story);

                if parsed_story.get_hadError() {
                    parsed_story.ResetError();
                    return result;
                }

                if let Some(runtime_object) = runtime_object {
                    if let ink_runtime::Container::ContentItem::Container(container) =
                        runtime_object
                    {
                        if let Some(value) =
                            runtime_story.EvaluateExpression(container.as_ref().clone())
                        {
                            result.output = Some(value.ToString());
                        }
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::{Compiler, Options};
    use ink_runtime::Error::ErrorType;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn compiler_defaults_and_parses() {
        let mut compiler = Compiler::new("Hello world".to_string(), Options::default());
        let parsed = compiler.Parse();
        assert_eq!(parsed.content.len(), 1);
    }

    #[test]
    fn compiler_parse_errors_are_forwarded_to_handler() {
        let captured: Rc<RefCell<Vec<(String, ErrorType)>>> = Rc::new(RefCell::new(Vec::new()));
        let handler_capture = Rc::clone(&captured);
        let error_handler = Rc::new(RefCell::new(Box::new(
            move |message: &str, error_type: ErrorType| {
                handler_capture
                    .borrow_mut()
                    .push((message.to_string(), error_type));
            },
        ) as ink_runtime::Error::ErrorHandler));

        let mut options = Options::default();
        options.errorHandler = Some(error_handler);

        let mut compiler = Compiler::new("~ return 5\n".to_string(), options);
        let compiled = compiler.Compile();

        assert!(compiled.is_none());
        let captured = captured.borrow();
        assert!(!captured.is_empty());
        assert!(captured
            .iter()
            .any(|(message, error_type)| *error_type == ErrorType::Error
                && message.contains("should not have return statement outside of a knot")));
    }

    #[test]
    fn compiler_handle_input_returns_none_for_unhandled_input() {
        let mut compiler = Compiler::new(String::new(), Options::default());
        assert!(compiler
            .HandleInput(crate::CommandLineInput::CommandLineInput::default())
            .is_none());
    }
}
