# Port Priority

This file orders the official C# source files for implementation work in `ink-rust/`.

Ordering rule: files with fewer direct references to types defined in the same area come first. The `deps=N` value is an approximate static count of directly referenced source files in the same list. It is a guide for implementation order, not a hard architectural rule. Cycles and C# partial classes still require judgement.

Reference projects remain read-only. Implement work in `ink-rust/`.

Completion annotations: append `[ported]` to a file entry only after that file's corresponding Rust implementation fully matches the C# source file according to `AGENTS.md`'s file port completion standard and `make -C ink-rust gate` passes. Do not mark partial type wiring, compile-only cleanup, or skeleton removal as ported.

Partial annotations: if a file is too large or difficult to finish in one pass and a partial implementation must be committed, append `[partial: reason=<why>; missing=<unfinished C# behavior>; next=<dependency or next step>]`. Keep this short but concrete enough that a later agent can finish the file without rediscovering the gap. Remove the partial annotation or replace it with `[ported]` when the file is completed.

## Progress Summary

These totals are derived from the annotations below and replace the deleted symbol inventory.

| Area | Total | Ported | Partial | Remaining |
| --- | ---: | ---: | ---: | ---: |
| Runtime | 34 | 34 | 0 | 0 |
| Compiler | 64 | 25 | 9 | 30 |
| Total | 98 | 59 | 9 | 30 |

## Runtime

Source area: `ink-c-sharp/ink-engine-runtime/`

1. `DebugMetadata.cs` (`deps=0`) [ported]
2. `Error.cs` (`deps=0`) [ported]
3. `INamedContent.cs` (`deps=0`) [ported]
4. `PushPop.cs` (`deps=0`) [ported]
5. `StoryException.cs` (`deps=0`) [ported]
6. `StringJoinExtension.cs` (`deps=0`) [ported]
7. `ControlCommand.cs` (`deps=1`) [ported]
8. `Glue.cs` (`deps=1`) [ported]
9. `Path.cs` (`deps=1`) [ported]
10. `SimpleJson.cs` (`deps=1`) [ported]
11. `Tag.cs` (`deps=1`) [ported]
12. `VariableAssignment.cs` (`deps=1`) [ported]
13. `Void.cs` (`deps=1`) [ported]
14. `ListDefinition.cs` (`deps=2`) [ported]
15. `ListDefinitionsOrigin.cs` (`deps=2`) [ported]
16. `SearchResult.cs` (`deps=2`) [ported]
17. `StatePatch.cs` (`deps=2`) [ported]
18. `Choice.cs` (`deps=3`) [ported]
19. `InkList.cs` (`deps=3`) [ported]
20. `Pointer.cs` (`deps=3`) [ported]
21. `VariableReference.cs` (`deps=3`) [ported]
22. `ChoicePoint.cs` (`deps=4`) [ported]
23. `Profiler.cs` (`deps=4`) [ported]
24. `Value.cs` (`deps=4`) [ported]
25. `Container.cs` (`deps=5`) [ported]
26. `Divert.cs` (`deps=5`) [ported]
27. `Object.cs` (`deps=5`) [ported]
28. `Flow.cs` (`deps=6`) [ported]
29. `NativeFunctionCall.cs` (`deps=7`) [ported]
30. `CallStack.cs` (`deps=8`) [ported]
31. `VariablesState.cs` (`deps=9`) [ported]
32. `JsonSerialisation.cs` (`deps=19`) [ported]
33. `StoryState.cs` (`deps=20`) [ported]
34. `Story.cs` (`deps=29`) [ported]

Recommended runtime implementation phases:

- Phase R1: `DebugMetadata.cs` through `Void.cs`
- Phase R2: `ListDefinition.cs` through `VariableReference.cs`
- Phase R3: `ChoicePoint.cs` through `Object.cs`
- Phase R4: `Flow.cs` through `VariablesState.cs`
- Phase R5: `JsonSerialisation.cs`, `StoryState.cs`, `Story.cs`

## Compiler

Source area: `ink-c-sharp/compiler/`

1. `CharacterSet.cs` (`deps=0`) [ported]
2. `CommandLineInput.cs` (`deps=0`) [ported]
3. `ParsedHierarchy/INamedContent.cs` (`deps=0`) [ported]
4. `ParsedHierarchy/Identifier.cs` (`deps=0`) [ported]
5. `StringParser/StringParserState.cs` (`deps=0`) [ported]
6. `CharacterRange.cs` (`deps=1`) [ported]
7. `FileHandler.cs` (`deps=1`) [ported]
8. `InkStringConversionExtensions.cs` (`deps=1`) [ported]
9. `ParsedHierarchy/AuthorWarning.cs` (`deps=1`) [ported]
10. `ParsedHierarchy/Number.cs` (`deps=1`) [ported]
11. `ParsedHierarchy/Tag.cs` (`deps=1`) [ported]
12. `ParsedHierarchy/Text.cs` (`deps=1`) [ported]
13. `Plugins/Plugin.cs` (`deps=1`) [ported]
14. `InkParser/CommentEliminator.cs` (`deps=2`) [ported]
15. `InkParser/InkParser_AuthorWarning.cs` (`deps=2`) [partial: reason=the parser wrapper is real, but identifier-with-metadata parsing is still simplified; missing=IdentifierWithMetadata parity and exact TODO token handling; next=port identifier parsing into InkParser and reuse it here]
16. `ParsedHierarchy/IncludedFile.cs` (`deps=2`) [ported]
17. `ParsedHierarchy/Return.cs` (`deps=2`) [ported]
18. `ParsedHierarchy/Wrap.cs` (`deps=2`) [ported]
19. `InkParser/InkParser_Include.cs` (`deps=3`) [partial: reason=include loading is wired, but root-parser recursion tracking and exact error recovery are simplified; missing=root-parser chained include semantics and full open-file bookkeeping parity; next=port the remaining parser hierarchy so include can match C# recursion behavior]
20. `InkParser/InkParser_Whitespace.cs` (`deps=3`) [ported]
21. `ParsedHierarchy/ContentList.cs` (`deps=3`)
22. `ParsedHierarchy/FlowLevel.cs` (`deps=3`) [ported]
23. `ParsedHierarchy/IWeavePoint.cs` (`deps=3`) [ported]
24. `ParsedHierarchy/List.cs` (`deps=3`)
25. `InkParser/InkParser_CharacterRanges.cs` (`deps=4`) [ported]
26. `InkParser/InkParser_CommandLineInput.cs` (`deps=4`)
27. `ParsedHierarchy/ConstantDeclaration.cs` (`deps=4`) [ported]
28. `ParsedHierarchy/ExternalDeclaration.cs` (`deps=4`) [ported]
29. `ParsedHierarchy/StringExpression.cs` (`deps=4`) [ported]
30. `ParsedHierarchy/TunnelOnwards.cs` (`deps=4`)
31. `Plugins/PluginManager.cs` (`deps=4`)
32. `ParsedHierarchy/Conditional.cs` (`deps=5`)
33. `ParsedHierarchy/Gather.cs` (`deps=5`)
34. `ParsedHierarchy/ListDefinition.cs` (`deps=5`) [ported]
35. `ParsedHierarchy/Stitch.cs` (`deps=5`)
36. `StringParser/StringParser.cs` (`deps=5`) [partial: reason=the core scanner, rule stack, and basic character parsing are real, but the full delegate/reflection parser ecosystem and InkParser integration are still unfinished; missing=full ParseRule plumbing, parser combinator parity, and the higher-level InkParser entry points; next=port the remaining StringParser combinator layer and wire InkParser onto it]
37. `InkParser/InkParser_Tags.cs` (`deps=6`)
38. `ParsedHierarchy/Sequence.cs` (`deps=6`)
39. `InkParser/InkParser_Divert.cs` (`deps=7`)
40. `ParsedHierarchy/ConditionalSingleBranch.cs` (`deps=7`)
41. `ParsedHierarchy/Knot.cs` (`deps=7`)
42. `ParsedHierarchy/VariableAssignment.cs` (`deps=7`)
43. `Stats.cs` (`deps=7`)
44. `InkParser/InkParser_Choices.cs` (`deps=8`)
45. `InkParser/InkParser_Conditional.cs` (`deps=8`)
46. `InkParser/InkParser_Content.cs` (`deps=8`)
47. `InkParser/InkParser_Sequences.cs` (`deps=8`)
48. `ParsedHierarchy/Object.cs` (`deps=8`)
49. `ParsedHierarchy/Path.cs` (`deps=8`) [partial: reason=path component storage and string formatting are real, but ancestry-based resolution still depends on the unported ParsedHierarchy.Object/FlowBase tree; missing=ResolveFromContext and child lookup through the parser hierarchy; next=port ParsedHierarchy/Object and ParsedHierarchy/FlowBase]
50. `InkParser/InkParser.cs` (`deps=9`) [partial: reason=the parser wrapper now delegates comment elimination, whitespace, include, and debug metadata, but the main Parse entry point and statement hierarchy are still unported; missing=full statement parsing and parsed-object tree construction; next=port InkParser statement/content rules and the remaining ParsedHierarchy tree]
51. `InkParser/InkParser_Logic.cs` (`deps=9`) [partial: reason=identifier parsing is real, but the line/variable/list/inline logic parser family is still skeletal; missing=LogicLine, VariableDeclaration, List/Const declarations, inline logic, and expression helper parity; next=port the remaining InkParser logic and expression rules]
52. `ParsedHierarchy/FunctionCall.cs` (`deps=9`) [partial: reason=builtin name detection is now real, but proxy divert generation, native function generation, and special-case argument handling are still stubbed; missing=GenerateIntoContainer, ResolveReferences, and the runtime/linkage-backed properties; next=port the full function-call generation path]
53. `ParsedHierarchy/VariableReference.cs` (`deps=9`) [partial: reason=constant and list-item recognition are now wired, but read-count resolution and full ancestry-based variable lookup still depend on the unported parser object tree; missing=read-count resolution, ResolveReferences parity, and runtime variable reference generation for the remaining cases; next=port ParsedHierarchy/Object and ParsedHierarchy/Path resolution]
54. `InkParser/InkParser_Statements.cs` (`deps=10`)
55. `ParsedHierarchy/Divert.cs` (`deps=10`) [partial: reason=runtime divert generation and stack/argument packaging are real, but target-content resolution, reference validation, and ancestry-based path lookup still depend on the unported parser object tree; missing=ResolveReferences parity, targetContent resolution, and argument validity checks; next=port ParsedHierarchy/Object and ParsedHierarchy/FlowBase or the remaining parser ancestry helpers]
56. `ParsedHierarchy/DivertTarget.cs` (`deps=10`)
57. `ParsedHierarchy/Expression.cs` (`deps=10`) [partial: reason=the wrapper now covers literal generation and recursive expression evaluation for the core AST forms, but the full upstream expression hierarchy still needs the remaining node variants and parser integrations; missing=FunctionCall/DivertTarget wiring and final parity for the expression node set; next=port the remaining expression-derived parser nodes and expand the wrapper variants]
58. `Compiler.cs` (`deps=11`)
59. `InkParser/InkParser_Knot.cs` (`deps=11`)
60. `ParsedHierarchy/Choice.cs` (`deps=11`)
61. `InkParser/InkParser_Expressions.cs` (`deps=15`)
62. `ParsedHierarchy/FlowBase.cs` (`deps=16`)
63. `ParsedHierarchy/Story.cs` (`deps=16`) [partial: reason=the parser-side story now handles symbol registration and list/external lookup, but top-level object processing, runtime export, and the full variable-resolution / weave-processing pipeline are still skeletons; missing=top-level AST traversal, runtime export, flattening, and variable resolution; next=port ParsedHierarchy/FlowBase, ParsedHierarchy/Object, and the remaining export pipeline]
64. `ParsedHierarchy/Weave.cs` (`deps=16`)
Recommended compiler implementation phases:

- Phase C1: `CharacterSet.cs` through `Plugins/Plugin.cs`
- Phase C2: `InkParser/CommentEliminator.cs` through `Plugins/PluginManager.cs`
- Phase C3: `ParsedHierarchy/Conditional.cs` through `Stats.cs`
- Phase C4: `InkParser/InkParser_Choices.cs` through `ParsedHierarchy/VariableReference.cs`
- Phase C5: `InkParser/InkParser_Statements.cs` through `InkParser/InkParser_Logic.cs`
