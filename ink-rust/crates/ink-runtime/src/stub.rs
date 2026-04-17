#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PortStub {
    pub _port_marker: (),
}

pub type Argument = PortStub;
pub type AuthorWarning = PortStub;
pub type BadTerminationHandler = PortStub;
pub type BinaryExpression = PortStub;
pub type BoolValue = PortStub;
pub type CallStack = PortStub;
pub type CharacterRange = PortStub;
pub type CharacterSet = PortStub;
pub type Choice = PortStub;
pub type ChoicePoint = PortStub;
pub type CommandLineInput = PortStub;
pub type CommandLineInputResult = PortStub;
pub type CommandType = PortStub;
pub type CommentEliminator = PortStub;
pub type Compiler = PortStub;
pub type Component = PortStub;
pub type Conditional = PortStub;
pub type ConditionalSingleBranch = PortStub;
pub type ConstantDeclaration = PortStub;
pub type Container = PortStub;
pub type ContentList = PortStub;
pub type ControlCommand = PortStub;
pub type CountFlags = PortStub;
pub type CustomFlags = PortStub;
pub type DebugMetadata = PortStub;
pub type DebugSourceRange = PortStub;
pub type DefaultFileHandler = PortStub;
pub type Divert = PortStub;
pub type DivertTarget = PortStub;
pub type DivertTargetValue = PortStub;
pub type Element = PortStub;
pub type ErrorHandler = PortStub;
pub type ErrorType = PortStub;
pub type Expression = PortStub;
pub type ExternalDeclaration = PortStub;
pub type ExternalFunction =
    Arc<dyn Fn(&[crate::Value::Value]) -> Option<crate::Value::Value> + Send + Sync>;
pub type ExternalFunctionDef = PortStub;
pub type FloatValue = PortStub;
pub type Flow = PortStub;
pub type FlowBase = PortStub;
pub type FlowDecl = PortStub;
pub type FlowLevel = PortStub;
pub type FunctionCall = PortStub;
pub type Gather = PortStub;
pub type GatherPointToResolve = PortStub;
pub type Glue = PortStub;
pub type IFileHandler = PortStub;
pub type INamedContent = PortStub;
pub type IPlugin = PortStub;
pub type IWeavePoint = PortStub;
pub type Identifier = PortStub;
pub type IncDecExpression = PortStub;
pub type IncludedFile = PortStub;
pub type InfixOperator = PortStub;
pub type InkList = PortStub;
pub type InkListItem = PortStub;
pub type InkParser = PortStub;
pub type InkStringConversionExtensions = PortStub;
pub type IntValue = PortStub;
pub type Json = PortStub;
pub type Knot = PortStub;
pub type LegacyTag = PortStub;
pub type List = PortStub;
pub type ListDefinition = PortStub;
pub type ListDefinitionsOrigin = PortStub;
pub type ListElementDefinition = PortStub;
pub type ListValue = PortStub;
pub type MultipleConditionExpression = PortStub;
pub type NameWithMetadata = PortStub;
pub type NativeFunctionCall = PortStub;
pub type Number = PortStub;
pub type Options = PortStub;
pub type OutputStateChange = PortStub;
pub type ParseRule = PortStub;
pub type ParseSuccessStruct = PortStub;
pub type Path = PortStub;
pub type PluginManager = PortStub;
pub type Pointer = PortStub;
pub type ProfileNode = PortStub;
pub type Profiler = PortStub;
pub type PushPopType = PortStub;
pub type Reader = PortStub;
pub type Return = PortStub;
pub type SearchResult = PortStub;
pub type Sequence = PortStub;
pub type SequenceDivertToResolve = PortStub;
pub type SequenceType = PortStub;
pub type SimpleJson = PortStub;
pub type State = PortStub;
pub type StateElement = PortStub;
pub type StatePatch = PortStub;
pub type StatementLevel = PortStub;
pub type Stats = PortStub;
pub type StepDetails = PortStub;
pub type Stitch = PortStub;
pub type Story = PortStub;
pub type StoryException = PortStub;
pub type StoryState = PortStub;
pub type Stream = PortStub;
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StringBuilder {
    buf: Rc<RefCell<String>>,
}

impl StringBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn Append(&mut self, value: impl AsRef<str>) {
        self.buf.borrow_mut().push_str(value.as_ref());
    }

    pub fn AppendChar(&mut self, value: char) {
        self.buf.borrow_mut().push(value);
    }

    pub fn AppendLine(&mut self, value: impl AsRef<str>) {
        self.Append(value);
        self.AppendChar('\n');
    }

    pub fn ToString(&self) -> String {
        self.buf.borrow().clone()
    }
}
pub type StringExpression = PortStub;
pub type StringExt = PortStub;
pub type StringParser = PortStub;
pub type StringParserState = PortStub;
pub type StringValue = PortStub;
pub type SymbolType = PortStub;
pub type Tag = PortStub;
pub type Text = PortStub;
pub type Thread = PortStub;
pub type TunnelOnwards = PortStub;
pub type UnaryExpression = PortStub;
pub type Value = PortStub;
pub type ValueType = PortStub;
pub type VariableAssignment = PortStub;
pub type VariableObserver = PortStub;
pub type VariablePointerValue = PortStub;
pub type VariableReference = PortStub;
pub type VariableResolveResult = PortStub;
pub type VariablesState = PortStub;
pub type Void = PortStub;
pub type Weave = PortStub;
pub type Wrap = PortStub;
pub type Writer = PortStub;
pub type implicit = PortStub;
