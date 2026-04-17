# Port Priority

This file orders the official C# source files for implementation work in `ink-rust/`.

Ordering rule: files with fewer direct references to types defined in the same area come first. The `deps=N` value is an approximate static count of directly referenced source files in the same list. It is a guide for implementation order, not a hard architectural rule. Cycles and C# partial classes still require judgement.

Reference projects remain read-only. Implement work in `ink-rust/`.

Completion annotations: append `[ported]` to a file entry only after that file's corresponding Rust implementation fully matches the C# source file according to `AGENTS.md`'s file port completion standard and `make -C ink-rust gate` passes. Do not mark partial type wiring, compile-only cleanup, or skeleton removal as ported.

Partial annotations: if a file is too large or difficult to finish in one pass and a partial implementation must be committed, append `[partial: reason=<why>; missing=<unfinished C# behavior>; next=<dependency or next step>]`. Keep this short but concrete enough that a later agent can finish the file without rediscovering the gap. Remove the partial annotation or replace it with `[ported]` when the file is completed.

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
17. `StatePatch.cs` (`deps=2`) [partial: reason=global-variable patching and visit/turn bookkeeping are real, but container identity is tracked through path keys rather than the exact upstream object-reference map; missing=object-identity-based patch lookups and the remaining save-state integration; next=port the Story/Container save graph and decide whether to keep path-keyed patches or reintroduce identity tracking]
18. `Choice.cs` (`deps=3`) [ported]
19. `InkList.cs` (`deps=3`) [ported]
20. `Pointer.cs` (`deps=3`) [ported]
21. `VariableReference.cs` (`deps=3`) [ported]
22. `ChoicePoint.cs` (`deps=4`) [ported]
23. `Profiler.cs` (`deps=4`) [ported]
24. `Value.cs` (`deps=4`) [ported]
25. `Container.cs` (`deps=5`) [partial: reason=the container now stores real runtime content items, supports add/insert operations, indexed path lookup, named-only content, and hierarchy stringification, but parent/backlink bookkeeping still differs from upstream; missing=exact parent-chain ownership and mutation propagation; next=introduce a shared parent-link model that preserves upstream hierarchy semantics]
26. `Divert.cs` (`deps=5`) [ported]
27. `Object.cs` (`deps=5`) [partial: reason=runtime path helpers are real, but parent backlinks and Copy/debug-line parity are still not wired into the current container model; missing=full parent-chain integration and exact runtime object cloning semantics; next=thread parent links through the runtime content graph or rework the path cache to match C#]
28. `Flow.cs` (`deps=6`) [ported]
29. `NativeFunctionCall.cs` (`deps=7`) [ported]
30. `CallStack.cs` (`deps=8`) [ported]
31. `VariablesState.cs` (`deps=9`) [ported]
32. `JsonSerialisation.cs` (`deps=19`) [partial: reason=runtime object and choice JSON encoding/decoding are now real, but flow/callstack/story save-load still depend on the unported StoryState/Story graph and several runtime object ancestry paths; missing=full Flow.WriteJson/SetJsonToken, CallStack JSON reconstruction, and story-state roundtrip wiring; next=port the story/save-state graph and remaining runtime ancestry helpers]
33. `StoryState.cs` (`deps=20`) [partial: reason=save/load, flow switching, output stream, and function-evaluation state are now real, but the layer still needs a final upstream parity audit around callback timing and story lifecycle edge cases; missing=final verification of restore/push/pop/thread transitions and remaining JSON edge cases; next=finish the Story callback/lifecycle audit and compare against C# save-load behavior]
34. `Story.cs` (`deps=29`) [partial: reason=continue/evaluate/choose/path selection and external binding are now wired, but the public event hook surface and a final upstream parity audit still need a last pass; missing=final callback subscription validation and any remaining lifecycle edge cases; next=compare the Story callback timing and function-evaluation flow against the C# runtime]

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
9. `ParsedHierarchy/AuthorWarning.cs` (`deps=1`) [partial: reason=Parsed.Object warning plumbing is not ported; missing=GenerateRuntimeObject must call Warning(warningMessage); next=port ParsedHierarchy/Object error-warning context]
10. `ParsedHierarchy/Number.cs` (`deps=1`) [ported]
11. `ParsedHierarchy/Tag.cs` (`deps=1`) [ported]
12. `ParsedHierarchy/Text.cs` (`deps=1`) [ported]
13. `Plugins/Plugin.cs` (`deps=1`) [ported]
14. `InkParser/CommentEliminator.cs` (`deps=2`) [ported]
15. `InkParser/InkParser_AuthorWarning.cs` (`deps=2`)
16. `ParsedHierarchy/IncludedFile.cs` (`deps=2`) [ported]
17. `ParsedHierarchy/Return.cs` (`deps=2`) [ported]
18. `ParsedHierarchy/Wrap.cs` (`deps=2`) [ported]
19. `InkParser/InkParser_Include.cs` (`deps=3`)
20. `InkParser/InkParser_Whitespace.cs` (`deps=3`)
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
36. `StringParser/StringParser.cs` (`deps=5`)
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
50. `InkParser/InkParser.cs` (`deps=9`)
51. `ParsedHierarchy/FunctionCall.cs` (`deps=9`) [partial: reason=builtin name detection is now real, but proxy divert generation, native function generation, and special-case argument handling are still stubbed; missing=GenerateIntoContainer, ResolveReferences, and the runtime/linkage-backed properties; next=port the full function-call generation path]
52. `ParsedHierarchy/VariableReference.cs` (`deps=9`) [partial: reason=constant and list-item recognition are now wired, but read-count resolution and full ancestry-based variable lookup still depend on the unported parser object tree; missing=read-count resolution, ResolveReferences parity, and runtime variable reference generation for the remaining cases; next=port ParsedHierarchy/Object and ParsedHierarchy/Path resolution]
53. `InkParser/InkParser_Statements.cs` (`deps=10`)
54. `ParsedHierarchy/Divert.cs` (`deps=10`)
55. `ParsedHierarchy/DivertTarget.cs` (`deps=10`)
56. `ParsedHierarchy/Expression.cs` (`deps=10`) [partial: reason=the wrapper now covers literal generation and recursive expression evaluation for the core AST forms, but the full upstream expression hierarchy still needs the remaining node variants and parser integrations; missing=FunctionCall/DivertTarget wiring and final parity for the expression node set; next=port the remaining expression-derived parser nodes and expand the wrapper variants]
57. `Compiler.cs` (`deps=11`)
58. `InkParser/InkParser_Knot.cs` (`deps=11`)
59. `ParsedHierarchy/Choice.cs` (`deps=11`)
60. `InkParser/InkParser_Expressions.cs` (`deps=15`)
61. `ParsedHierarchy/FlowBase.cs` (`deps=16`)
62. `ParsedHierarchy/Story.cs` (`deps=16`) [partial: reason=the parser-side story now handles symbol registration and list/external lookup, but top-level object processing, runtime export, and the full variable-resolution / weave-processing pipeline are still skeletons; missing=top-level AST traversal, runtime export, flattening, and variable resolution; next=port ParsedHierarchy/FlowBase, ParsedHierarchy/Object, and the remaining export pipeline]
63. `ParsedHierarchy/Weave.cs` (`deps=16`)
64. `InkParser/InkParser_Logic.cs` (`deps=19`)

Recommended compiler implementation phases:

- Phase C1: `CharacterSet.cs` through `Plugins/Plugin.cs`
- Phase C2: `InkParser/CommentEliminator.cs` through `Plugins/PluginManager.cs`
- Phase C3: `ParsedHierarchy/Conditional.cs` through `Stats.cs`
- Phase C4: `InkParser/InkParser_Choices.cs` through `ParsedHierarchy/VariableReference.cs`
- Phase C5: `InkParser/InkParser_Statements.cs` through `InkParser/InkParser_Logic.cs`
