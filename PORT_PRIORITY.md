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
| Compiler | 64 | 36 | 25 | 3 |
| Total | 98 | 70 | 25 | 3 |

## Runtime

Runtime is fully ported. The summary table above is the only runtime status needed.

## Compiler

Source area: `ink-c-sharp/compiler/`

### ParsedHierarchy

1. `ParsedHierarchy/INamedContent.cs` (`deps=0`) [ported]
2. `ParsedHierarchy/Identifier.cs` (`deps=0`) [ported]
3. `ParsedHierarchy/AuthorWarning.cs` (`deps=1`) [ported]
4. `ParsedHierarchy/Number.cs` (`deps=1`) [ported]
5. `ParsedHierarchy/Tag.cs` (`deps=1`) [ported]
6. `ParsedHierarchy/Text.cs` (`deps=1`) [ported]
7. `ParsedHierarchy/IncludedFile.cs` (`deps=2`) [ported]
8. `ParsedHierarchy/Return.cs` (`deps=2`) [ported]
9. `ParsedHierarchy/Wrap.cs` (`deps=2`) [ported]
10. `ParsedHierarchy/ContentList.cs` (`deps=3`) [partial: content generation and whitespace trimming are ported, but dontFlatten still cannot notify Story.DontFlattenContainer because parsed-object ownership is not wired]
11. `ParsedHierarchy/FlowLevel.cs` (`deps=3`) [ported]
12. `ParsedHierarchy/IWeavePoint.cs` (`deps=3`) [ported]
13. `ParsedHierarchy/List.cs` (`deps=3`) [ported]
14. `ParsedHierarchy/ConstantDeclaration.cs` (`deps=4`) [ported]
15. `ParsedHierarchy/ExternalDeclaration.cs` (`deps=4`) [ported]
16. `ParsedHierarchy/StringExpression.cs` (`deps=4`) [ported]
17. `ParsedHierarchy/TunnelOnwards.cs` (`deps=4`) [partial: tunnel target resolution still depends on Parsed.Object ancestry and FlowBase/path resolution]
18. `ParsedHierarchy/Conditional.cs` (`deps=5`) [partial: reason=conditional branches now generate runtime containers, but parser integration and weave/sequence ownership are still incomplete; missing=InkParser conditional parsing, weave-style nesting, and full branch rejoin wiring; next=port InkParser_Conditional and the remaining weave helpers]
19. `ParsedHierarchy/Gather.cs` (`deps=5`) [partial: content tree handling is still waiting on Parsed.Object/FlowBase/ContentList porting]
20. `ParsedHierarchy/ListDefinition.cs` (`deps=5`) [ported]
21. `ParsedHierarchy/Stitch.cs` (`deps=5`) [partial: thin flow wrapper is real, but full story-owned subflow resolution and knot/stitch cross-name collision parity still depend on the remaining flow-tree structure]
22. `ParsedHierarchy/Sequence.cs` (`deps=6`) [partial: reason=flat sequence runtime generation is ported, but nested weave-backed branch ownership and multiline sequence reconstruction still depend on the unported weave/tree model; missing=Weave integration, nested loose-end routing, and exact multiline branch conversion; next=finish ParsedHierarchy/Weave or the remaining flow-tree hierarchy]
23. `ParsedHierarchy/ConditionalSingleBranch.cs` (`deps=7`) [partial: reason=branch runtime generation is real, but the weave-based nesting model and parser ownership are still incomplete; missing=weave nesting, exact else/branch recovery, and parser integration; next=port ParsedHierarchy/Weave and InkParser_Conditional]
24. `ParsedHierarchy/Knot.cs` (`deps=7`) [partial: thin flow wrapper is real, but knot/stitch ownership and cross-name collision behavior still depend on the remaining flow-tree hierarchy]
25. `ParsedHierarchy/VariableAssignment.cs` (`deps=7`) [partial: list-definition backref is not yet modeled with the C# ownership cycle, and variable resolution is still using a reduced Story-side lookup until FlowBase/Object are ported]
26. `ParsedHierarchy/Object.cs` (`deps=8`) [partial: reason=the shared parsed-object base now tracks ancestry, identifiers, and runtime storage, but it is not yet wired into the concrete parsed node types or the story-root ownership model; missing=concrete node inheritance/composition wiring and full Story-root lookup; next=port ParsedHierarchy/Story and the remaining flow-tree nodes onto this base]
27. `ParsedHierarchy/Path.cs` (`deps=8`) [ported]
28. `ParsedHierarchy/FunctionCall.cs` (`deps=9`) [partial: reason=the function-call node now generates runtime output for built-ins and native calls, but the upstream object-tree validation and full count-target handling still depend on the unported ParsedHierarchy.Object/FlowBase chain; missing=Object-style error source propagation, full TURNS_SINCE/READ_COUNT validation, and parser integration for the remaining expression entry points; next=port ParsedHierarchy/Object and finish wiring the expression parser to this node]
29. `ParsedHierarchy/VariableReference.cs` (`deps=9`) [partial: reason=constant and list-item recognition are now wired, but read-count resolution and full ancestry-based variable lookup still depend on the unported parser object tree; missing=read-count resolution, ResolveReferences parity, and runtime variable reference generation for the remaining cases; next=port ParsedHierarchy/Object and ParsedHierarchy/Path resolution]
30. `ParsedHierarchy/Divert.cs` (`deps=10`) [partial: reason=runtime divert generation and stack/argument packaging are real, but target-content resolution, reference validation, and ancestry-based path lookup still depend on the unported parser object tree; missing=ResolveReferences parity, targetContent resolution, and argument validity checks; next=port ParsedHierarchy/Object and ParsedHierarchy/FlowBase or the remaining parser ancestry helpers]
31. `ParsedHierarchy/DivertTarget.cs` (`deps=10`) [partial: reason=divert-target generation and equality are now real, but the ancestry-based usage checks and exact target counting still depend on the unported ParsedHierarchy.Object tree; missing=full usage-context validation and complete parent-chain counting parity; next=port ParsedHierarchy/Object and FlowBase or the remaining ancestry helpers]
32. `ParsedHierarchy/Expression.cs` (`deps=10`) [partial: reason=the wrapper now covers literal generation plus function-call, divert-target, and variable-reference expression forms, but the upstream parser still needs the remaining entry points and parse-tree wiring; missing=InkParser expression integration and the rest of the expression grammar surface; next=port InkParser/InkParser_Expressions and the remaining parser rules]
33. `ParsedHierarchy/Choice.cs` (`deps=11`) [partial: choice runtime generation and reference resolution are real, but the full parsed-object ownership chain, count-all-visits hookup, and flow-tree integration are still not modeled; missing=base-object content propagation and owning-story visit-count wiring; next=port ParsedHierarchy/Object and ParsedHierarchy/FlowBase]
34. `ParsedHierarchy/FlowBase.cs` (`deps=16`) [partial: reason=variable lookup and basic runtime export now work against a standalone flow base, but weave splitting, sub-flow ownership, and full C# hierarchy parity still depend on the unported story/flow object tree; missing=weave/subflow construction, flow-parent wiring, and exact story-root resolution; next=port ParsedHierarchy/Weave and the remaining flow-tree wrappers onto this base]
35. `ParsedHierarchy/Story.cs` (`deps=16`) [partial: reason=the parser-side story now owns top-level content, handles include preprocessing, and exports runtime stories, but the full variable-resolution / weave-processing pipeline is still incomplete; missing=complete const/list/variable collection, exact weave processing, and full naming-collision parity; next=finish ParsedHierarchy/FlowBase and the remaining story-tree helpers]
36. `ParsedHierarchy/Weave.cs` (`deps=16`) [partial: reason=the weave helper now owns indentation-based content ordering and basic runtime export, but the full nested weave hierarchy, loose-end routing, and termination analysis are still simplified; missing=exact recursive weave reconstruction, loose-end propagation, and full termination validation; next=finish the remaining flow-tree ownership model and nested sequence/conditional weaving]

### InkParser

1. `InkParser/CommentEliminator.cs` (`deps=2`) [ported]
2. `InkParser/InkParser_AuthorWarning.cs` (`deps=2`) [ported]
3. `InkParser/InkParser_Include.cs` (`deps=3`) [ported]
4. `InkParser/InkParser_Whitespace.cs` (`deps=3`) [ported]
5. `InkParser/InkParser_CharacterRanges.cs` (`deps=4`) [ported]
6. `InkParser/InkParser_CommandLineInput.cs` (`deps=4`) [ported]
7. `InkParser/InkParser_Divert.cs` (`deps=7`) [partial: reason=the parser now resolves single diverts, thread diverts, and diverted path components, but full multi-divert/tunnel-onwards parity and tag/content list integration are still incomplete; missing=exact tunnel-onwards chain handling, tag/content-list integration, and the full arrow/divert sequence semantics; next=port ParsedHierarchy/TunnelOnwards, ContentList, and the remaining parser content tree]
8. `InkParser/InkParser_Choices.cs` (`deps=8`) [partial: choice and gather parsing are real, but the statement dispatcher and flow-level integration still depend on the unported parsed hierarchy tree; missing=StatementsAtLevel wiring and choice/gather placement in the top-level parser; next=port InkParser_Statements and ParsedHierarchy/FlowBase]
9. `InkParser/InkParser_Conditional.cs` (`deps=8`) [partial: reason=conditional grammar is now real enough to parse branches, but the branch-content wrapping and source-aware error recovery still use simplified compatibility paths; missing=exact ErrorWithParsedObject parity and inline/branch ownership wiring; next=continue the remaining flow-tree/parser ownership work]
10. `InkParser/InkParser_Content.cs` (`deps=8`) [partial: reason=text/tag/glue line parsing is real, but full inline logic, choice-specific content rules, and weave integration are still incomplete; missing=multi-branch inline logic and choice-aware content handling; next=port InkParser_Choices/InkParser_Conditional and the remaining hierarchy tree]
11. `InkParser/InkParser_Sequences.cs` (`deps=8`) [partial: reason=inline sequence annotations and single-line parsing are wired, and multiline branch content now converts from parsed statements into ContentList values, but weave-backed nested sequence ownership and full branch routing still depend on the unported weave/tree model; missing=Weave integration, nested loose-end routing, and exact nested sequence ownership; next=finish ParsedHierarchy/Weave or the remaining flow-tree hierarchy]
12. `InkParser/InkParser_Logic.cs` (`deps=9`) [ported]
13. `InkParser/InkParser.cs` (`deps=9`) [partial: reason=the parser wrapper now delegates comment elimination, whitespace, include, debug metadata, and top-level Parse construction, but the full statement hierarchy and declaration grammar remain incomplete; missing=LogicLine, declarations, and the remaining statement branches; next=port the remaining InkParser logic and ParsedHierarchy tree]
14. `InkParser/InkParser_Statements.cs` (`deps=10`) [partial: reason=the parser helper now dispatches knots, stitches, choices, gathers, divert lines, text lines, and logic lines, but the remaining sequence/conditional statement branches and the full top-level story export still depend on the unfinished hierarchy tree; missing=the remaining statement-tree branches that belong to sequence/conditional and story export; next=port the remaining parsed hierarchy nodes that feed statement dispatch]
15. `InkParser/InkParser_Tags.cs` (`deps=10`) [ported]
16. `InkParser/InkParser_Knot.cs` (`deps=11`) [partial: knot and stitch declarations now parse and wrap runtime containers, but full recovery, statement ownership, and story-root integration still depend on the remaining parser/story tree]
17. `InkParser/InkParser_Expressions.cs` (`deps=15`) [ported]

### Outer Files

1. `StringParser/StringParserState.cs` (`deps=0`) [ported]
2. `CharacterSet.cs` (`deps=0`) [ported]
3. `CommandLineInput.cs` (`deps=0`) [ported]
4. `CharacterRange.cs` (`deps=1`) [ported]
5. `FileHandler.cs` (`deps=1`) [ported]
6. `InkStringConversionExtensions.cs` (`deps=1`) [ported]
7. `Plugins/Plugin.cs` (`deps=1`) [ported]
8. `Plugins/PluginManager.cs` (`deps=4`) [partial: reason=reflection-driven DLL loading is not yet modeled; missing=plugin discovery/loading and invoke-member dispatch; next=finish or replace plugin system integration]
9. `StringParser/StringParser.cs` (`deps=5`) [ported]
10. `Stats.cs` (`deps=7`) [ported]
11. `Compiler.cs` (`deps=11`) [partial: source compilation now reaches Parsed.Story and Runtime.Story for simple stories, but plugin processing, full command-line handling, and debug-source bookkeeping remain incomplete; missing=plugin directories, parse error plumbing, immediate-mode commands, and debug range recovery; next=finish the remaining compiler front-end helpers]

## Tool Projects

1. `InkTestBed/InkTestBed.cs` [partial: runtime play loop, JSON roundtrip, split-file utilities, and source compilation are ported, but the interactive input helpers, debug-source handling, and full C# testbed workflows remain incomplete]
