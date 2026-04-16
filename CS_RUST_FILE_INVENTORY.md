# CS -> Rust File Inventory

Generated from non-generated source files only. `obj/` and `bin/` are excluded.

Source files: 98
Rust implemented: 35
Rust skeleton: 63
Rust missing: 0

## Compiler

| C# source | Rust target | Rust state |
| --- | --- | --- |
| `ink-c-sharp/compiler/CharacterRange.cs` | `ink-rust/crates/ink-compiler/src/CharacterRange.rs` | skeleton |
| `ink-c-sharp/compiler/CharacterSet.cs` | `ink-rust/crates/ink-compiler/src/CharacterSet.rs` | skeleton |
| `ink-c-sharp/compiler/CommandLineInput.cs` | `ink-rust/crates/ink-compiler/src/CommandLineInput.rs` | skeleton |
| `ink-c-sharp/compiler/Compiler.cs` | `ink-rust/crates/ink-compiler/src/Compiler.rs` | skeleton |
| `ink-c-sharp/compiler/FileHandler.cs` | `ink-rust/crates/ink-compiler/src/FileHandler.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/CommentEliminator.cs` | `ink-rust/crates/ink-compiler/src/InkParser/CommentEliminator.rs` | implemented |
| `ink-c-sharp/compiler/InkParser/InkParser.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_AuthorWarning.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_AuthorWarning.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_CharacterRanges.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_CharacterRanges.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_Choices.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_Choices.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_CommandLineInput.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_CommandLineInput.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_Conditional.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_Conditional.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_Content.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_Content.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_Divert.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_Divert.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_Expressions.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_Expressions.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_Include.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_Include.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_Knot.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_Knot.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_Logic.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_Logic.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_Sequences.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_Sequences.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_Statements.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_Statements.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_Tags.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_Tags.rs` | skeleton |
| `ink-c-sharp/compiler/InkParser/InkParser_Whitespace.cs` | `ink-rust/crates/ink-compiler/src/InkParser/InkParser_Whitespace.rs` | skeleton |
| `ink-c-sharp/compiler/InkStringConversionExtensions.cs` | `ink-rust/crates/ink-compiler/src/InkStringConversionExtensions.rs` | implemented |
| `ink-c-sharp/compiler/ParsedHierarchy/AuthorWarning.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/AuthorWarning.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/Choice.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Choice.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/Conditional.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Conditional.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/ConditionalSingleBranch.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/ConditionalSingleBranch.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/ConstantDeclaration.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/ConstantDeclaration.rs` | implemented |
| `ink-c-sharp/compiler/ParsedHierarchy/ContentList.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/ContentList.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/Divert.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Divert.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/DivertTarget.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/DivertTarget.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/Expression.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Expression.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/ExternalDeclaration.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/ExternalDeclaration.rs` | implemented |
| `ink-c-sharp/compiler/ParsedHierarchy/FlowBase.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/FlowBase.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/FlowLevel.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/FlowLevel.rs` | implemented |
| `ink-c-sharp/compiler/ParsedHierarchy/FunctionCall.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/FunctionCall.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/Gather.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Gather.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/INamedContent.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/INamedContent.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/IWeavePoint.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/IWeavePoint.rs` | implemented |
| `ink-c-sharp/compiler/ParsedHierarchy/Identifier.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Identifier.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/IncludedFile.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/IncludedFile.rs` | implemented |
| `ink-c-sharp/compiler/ParsedHierarchy/Knot.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Knot.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/List.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/List.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/ListDefinition.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/ListDefinition.rs` | implemented |
| `ink-c-sharp/compiler/ParsedHierarchy/Number.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Number.rs` | implemented |
| `ink-c-sharp/compiler/ParsedHierarchy/Object.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Object.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/Path.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Path.rs` | implemented |
| `ink-c-sharp/compiler/ParsedHierarchy/Return.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Return.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/Sequence.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Sequence.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/Stitch.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Stitch.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/Story.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Story.rs` | implemented |
| `ink-c-sharp/compiler/ParsedHierarchy/StringExpression.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/StringExpression.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/Tag.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Tag.rs` | implemented |
| `ink-c-sharp/compiler/ParsedHierarchy/Text.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Text.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/TunnelOnwards.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/TunnelOnwards.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/VariableAssignment.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/VariableAssignment.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/VariableReference.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/VariableReference.rs` | implemented |
| `ink-c-sharp/compiler/ParsedHierarchy/Weave.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Weave.rs` | skeleton |
| `ink-c-sharp/compiler/ParsedHierarchy/Wrap.cs` | `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Wrap.rs` | implemented |
| `ink-c-sharp/compiler/Plugins/Plugin.cs` | `ink-rust/crates/ink-compiler/src/Plugins/Plugin.rs` | skeleton |
| `ink-c-sharp/compiler/Plugins/PluginManager.cs` | `ink-rust/crates/ink-compiler/src/Plugins/PluginManager.rs` | skeleton |
| `ink-c-sharp/compiler/Stats.cs` | `ink-rust/crates/ink-compiler/src/Stats.rs` | skeleton |
| `ink-c-sharp/compiler/StringParser/StringParser.cs` | `ink-rust/crates/ink-compiler/src/StringParser/StringParser.rs` | skeleton |
| `ink-c-sharp/compiler/StringParser/StringParserState.cs` | `ink-rust/crates/ink-compiler/src/StringParser/StringParserState.rs` | implemented |

## Runtime

| C# source | Rust target | Rust state |
| --- | --- | --- |
| `ink-c-sharp/ink-engine-runtime/CallStack.cs` | `ink-rust/crates/ink-runtime/src/CallStack.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/Choice.cs` | `ink-rust/crates/ink-runtime/src/Choice.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/ChoicePoint.cs` | `ink-rust/crates/ink-runtime/src/ChoicePoint.rs` | skeleton |
| `ink-c-sharp/ink-engine-runtime/Container.cs` | `ink-rust/crates/ink-runtime/src/Container.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/ControlCommand.cs` | `ink-rust/crates/ink-runtime/src/ControlCommand.rs` | skeleton |
| `ink-c-sharp/ink-engine-runtime/DebugMetadata.cs` | `ink-rust/crates/ink-runtime/src/DebugMetadata.rs` | skeleton |
| `ink-c-sharp/ink-engine-runtime/Divert.cs` | `ink-rust/crates/ink-runtime/src/Divert.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/Error.cs` | `ink-rust/crates/ink-runtime/src/Error.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/Flow.cs` | `ink-rust/crates/ink-runtime/src/Flow.rs` | skeleton |
| `ink-c-sharp/ink-engine-runtime/Glue.cs` | `ink-rust/crates/ink-runtime/src/Glue.rs` | skeleton |
| `ink-c-sharp/ink-engine-runtime/INamedContent.cs` | `ink-rust/crates/ink-runtime/src/INamedContent.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/InkList.cs` | `ink-rust/crates/ink-runtime/src/InkList.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/JsonSerialisation.cs` | `ink-rust/crates/ink-runtime/src/JsonSerialisation.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/ListDefinition.cs` | `ink-rust/crates/ink-runtime/src/ListDefinition.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/ListDefinitionsOrigin.cs` | `ink-rust/crates/ink-runtime/src/ListDefinitionsOrigin.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/NativeFunctionCall.cs` | `ink-rust/crates/ink-runtime/src/NativeFunctionCall.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/Object.cs` | `ink-rust/crates/ink-runtime/src/Object.rs` | skeleton |
| `ink-c-sharp/ink-engine-runtime/Path.cs` | `ink-rust/crates/ink-runtime/src/Path.rs` | skeleton |
| `ink-c-sharp/ink-engine-runtime/Pointer.cs` | `ink-rust/crates/ink-runtime/src/Pointer.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/Profiler.cs` | `ink-rust/crates/ink-runtime/src/Profiler.rs` | skeleton |
| `ink-c-sharp/ink-engine-runtime/PushPop.cs` | `ink-rust/crates/ink-runtime/src/PushPop.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/SearchResult.cs` | `ink-rust/crates/ink-runtime/src/SearchResult.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/SimpleJson.cs` | `ink-rust/crates/ink-runtime/src/SimpleJson.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/StatePatch.cs` | `ink-rust/crates/ink-runtime/src/StatePatch.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/Story.cs` | `ink-rust/crates/ink-runtime/src/Story.rs` | skeleton |
| `ink-c-sharp/ink-engine-runtime/StoryException.cs` | `ink-rust/crates/ink-runtime/src/StoryException.rs` | skeleton |
| `ink-c-sharp/ink-engine-runtime/StoryState.cs` | `ink-rust/crates/ink-runtime/src/StoryState.rs` | skeleton |
| `ink-c-sharp/ink-engine-runtime/StringJoinExtension.cs` | `ink-rust/crates/ink-runtime/src/StringJoinExtension.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/Tag.cs` | `ink-rust/crates/ink-runtime/src/Tag.rs` | skeleton |
| `ink-c-sharp/ink-engine-runtime/Value.cs` | `ink-rust/crates/ink-runtime/src/Value.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/VariableAssignment.cs` | `ink-rust/crates/ink-runtime/src/VariableAssignment.rs` | skeleton |
| `ink-c-sharp/ink-engine-runtime/VariableReference.cs` | `ink-rust/crates/ink-runtime/src/VariableReference.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/VariablesState.cs` | `ink-rust/crates/ink-runtime/src/VariablesState.rs` | implemented |
| `ink-c-sharp/ink-engine-runtime/Void.cs` | `ink-rust/crates/ink-runtime/src/Void.rs` | skeleton |

