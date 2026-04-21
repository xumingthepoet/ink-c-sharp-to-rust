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
| Compiler | 64 | 44 | 17 | 3 |
| Total | 98 | 78 | 17 | 3 |

## Continuation Plan For Future Agents

The remaining work should converge on integration tests, not on isolated compile-only stubs. Use the current annotations as a dependency map, but follow the order below when choosing work.

Non-negotiable rules for lower-capability agents:

1. Read the matching C# file before editing Rust. If behavior is unclear, inspect nearby C# callers and tests before touching `ink-rust/`.
2. Keep all edits in `ink-rust/` or this tracking file unless the user explicitly says otherwise. `ink-c-sharp/` and `blade-ink-rs/` remain read-only references.
3. Do not mark a file `[ported]` unless the entire C# file is faithfully represented and `make -C ink-rust gate` passes. Partial compile success is not enough.
4. Prefer visible unfinished markers over plausible wrong behavior. If a dependency is still missing, keep the file `[partial: ...]` and state the missing C# behavior.
5. Add or upgrade tests through `ink-rust/crates/ink-tests` or `ink-rust/crates/ink-testbed` when a behavior reaches compiler-to-runtime execution.

Recommended implementation order from the current state:

1. Finish the ParsedHierarchy flow-tree spine first: `ParsedHierarchy/Object.cs`, `ParsedHierarchy/FlowBase.cs`, `ParsedHierarchy/Story.cs`, then `ParsedHierarchy/Weave.cs`.
   The goal is one coherent ownership and resolution path for parent chains, root story lookup, subflow lookup, root weave export, named-only knots/stitches, and loose-end validation. Do not spend another pass polishing parser edge cases until this spine can generate runtime containers through the same path that C# uses.

2. Finish weave-backed content nodes next: `ParsedHierarchy/Choice.cs`, `ParsedHierarchy/ConditionalSingleBranch.cs`, `ParsedHierarchy/Conditional.cs`, and `ParsedHierarchy/Sequence.cs`.
   These should be completed against the official C# control flow rules for choice/gather mutation, branch rejoin routing, sequence branch ownership, and loose-end propagation. Use small compiler-to-runtime stories as acceptance tests, not only object-level unit tests.

3. Finish reference and expression behavior after the flow tree is stable: `ParsedHierarchy/Divert.cs`, `ParsedHierarchy/DivertTarget.cs`, `ParsedHierarchy/VariableReference.cs`, `ParsedHierarchy/VariableAssignment.cs`, `ParsedHierarchy/FunctionCall.cs`, and `ParsedHierarchy/Expression.cs`.
   The important acceptance surface is ancestry-sensitive target resolution, variable divert targets, `TURNS_SINCE`, `READ_COUNT`, function calls, temp/global variable lookup, list-backed declarations, and exact error timing.

4. Wire the parser onto the completed hierarchy: `InkParser/InkParser_Divert.cs`, `InkParser/InkParser_Choices.cs`, `InkParser/InkParser_Conditional.cs`, `InkParser/InkParser_Content.cs`, `InkParser/InkParser_Sequences.cs`, `InkParser/InkParser_Statements.cs`, `InkParser/InkParser_Knot.cs`, and `InkParser/InkParser.cs`.
   Parser work should preserve typed parsed payloads and avoid eager runtime generation. The parser should hand a faithful parsed tree to ParsedHierarchy; it should not compensate for missing ParsedHierarchy behavior with ad hoc runtime objects.

5. Complete outer integration only after the compiler path is real: `Compiler.cs` and `Plugins/PluginManager.cs`.
   Compiler completion includes parse error plumbing, debug source/range bookkeeping, include behavior, plugin hooks or an explicit compatible substitute, and command-line/immediate-mode surfaces that are used by official tests.

6. Expand integration coverage continuously.
   Use `ink-rust/crates/ink-tests` for concise regression cases and `ink-rust/crates/ink-testbed` when a case needs realistic story driving, choices, tags, or multi-step interaction. Good milestone tests are: linear text, top-level knot divert, choice selection, gather fallthrough, conditional branch, sequence branch, global/temp variable assignment, function call/return, list operations, tunnel return, include handling, and save/load runtime JSON parity.

Suggested work packages for sub-agents:

1. Assign only one ownership slice at a time. Example: one agent owns `ParsedHierarchy/FlowBase.rs` plus tests; another owns `ParsedHierarchy/Weave.rs` plus tests. Do not let two agents edit the same Rust file concurrently.
2. Ask each agent to report changed paths, C# symbols covered, tests added, remaining gaps, and whether `make -C ink-rust gate` passed.
3. For large partial files, require the agent to update the matching `[partial: ...]` annotation in this file in the same change. The annotation must say what is still missing and what dependency blocks completion.
4. Keep commits small: one large file, two or three medium files, or a tightly related test upgrade. Run `make -C ink-rust gate` before committing.

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
10. `ParsedHierarchy/ContentList.cs` (`deps=3`) [ported]
11. `ParsedHierarchy/FlowLevel.cs` (`deps=3`) [ported]
12. `ParsedHierarchy/IWeavePoint.cs` (`deps=3`) [ported]
13. `ParsedHierarchy/List.cs` (`deps=3`) [ported]
14. `ParsedHierarchy/ConstantDeclaration.cs` (`deps=4`) [ported]
15. `ParsedHierarchy/ExternalDeclaration.cs` (`deps=4`) [ported]
16. `ParsedHierarchy/StringExpression.cs` (`deps=4`) [ported]
17. `ParsedHierarchy/TunnelOnwards.cs` (`deps=4`) [ported]
18. `ParsedHierarchy/Conditional.cs` (`deps=5`) [partial: reason=conditional branches now generate runtime containers, but parser integration and weave/sequence ownership are still incomplete; missing=InkParser conditional parsing, weave-style nesting, and full branch rejoin wiring; next=port InkParser_Conditional and the remaining weave helpers]
19. `ParsedHierarchy/Gather.cs` (`deps=5`) [ported]
20. `ParsedHierarchy/ListDefinition.cs` (`deps=5`) [ported]
21. `ParsedHierarchy/Stitch.cs` (`deps=5`) [ported]
22. `ParsedHierarchy/Sequence.cs` (`deps=6`) [partial: reason=flat sequence runtime generation is ported, but nested weave-backed branch ownership and multiline sequence reconstruction still depend on the unported weave/tree model; missing=Weave integration, nested loose-end routing, and exact multiline branch conversion; next=finish ParsedHierarchy/Weave or the remaining flow-tree hierarchy]
23. `ParsedHierarchy/ConditionalSingleBranch.cs` (`deps=7`) [partial: reason=branch runtime generation is real, but the weave-based nesting model and parser ownership are still incomplete; missing=weave nesting, exact else/branch recovery, and parser integration; next=port ParsedHierarchy/Weave and InkParser_Conditional]
24. `ParsedHierarchy/Knot.cs` (`deps=7`) [ported]
25. `ParsedHierarchy/VariableAssignment.cs` (`deps=7`) [partial: variable declarations now generate runtime assignments and list-definition ownership is threaded through ResolveReferences, and list-definition backrefs are initialized when the declaration is constructed, but the exact list-definition content/parent ownership still differs from the C# object tree; missing=full content-tree parity for list-backed declarations and exact declaration-tree ownership; next=finish the remaining parsed-object ownership wiring]
26. `ParsedHierarchy/Object.cs` (`deps=8`) [partial: reason=the shared parsed-object base now tracks ancestry, identifiers, runtime storage, and typed payloads for key concrete nodes including knots/stitches; missing=reference-identity parent ownership, full concrete-node composition, and exact Story-root lookup; next=wire FlowBase/Weave choices through the typed payload model]
27. `ParsedHierarchy/Path.cs` (`deps=8`) [ported]
28. `ParsedHierarchy/FunctionCall.cs` (`deps=9`) [ported]
29. `ParsedHierarchy/VariableReference.cs` (`deps=9`) [partial: reason=constant and list-item recognition are wired, and simple read-count targets now resolve from the story tree, but exact ancestor-sensitive read-count resolution and writer-warning timing still need the full parsed-object context; missing=ancestor-aware path resolution, warning parity for function read-counts used as content, and the remaining variable lookup edge cases; next=finish the remaining object-context plumbing]
30. `ParsedHierarchy/Divert.cs` (`deps=10`) [partial: reason=runtime divert generation, stack/argument packaging, and basic story-level target path resolution are real, but ancestry-sensitive target lookup remains incomplete; missing=full ResolveTargetContent parity, variable divert targets, and argument validity checks; next=finish ParsedHierarchy/Object/FlowBase ancestry and route path resolution through it]
31. `ParsedHierarchy/DivertTarget.cs` (`deps=10`) [partial: reason=divert-target generation and equality are now real, but the ancestry-based usage checks and exact target counting still depend on the unported ParsedHierarchy.Object tree; missing=full usage-context validation and complete parent-chain counting parity; next=port ParsedHierarchy/Object and FlowBase or the remaining ancestry helpers]
32. `ParsedHierarchy/Expression.cs` (`deps=10`) [partial: reason=the wrapper now covers literal generation plus function-call, divert-target, and variable-reference expression forms, but the upstream parser still needs the remaining entry points and parse-tree wiring; missing=InkParser expression integration and the rest of the expression grammar surface; next=port InkParser/InkParser_Expressions and the remaining parser rules]
33. `ParsedHierarchy/Choice.cs` (`deps=11`) [ported]
34. `ParsedHierarchy/FlowBase.cs` (`deps=16`) [partial: reason=the flow base now separates root weave content from sub-flows, preserves typed knot/stitch subflows, lazily exports runtime containers, recurses through nested subflow lookup, and now wires direct parent-flow links for variable lookup; missing=exact nested termination/naming-collision parity and full story-root ancestry resolution parity; next=finish the remaining flow-tree wrappers and tighten parent-chain handling]
35. `ParsedHierarchy/Story.cs` (`deps=16`) [partial: reason=the parser-side story now owns top-level content, include preprocessing, typed declaration collection, external registration, global variable initialisation export, named-only top-level flow export, list-item ambiguity reporting, and source-aware error formatting; missing=FlowBase-backed root generation and full weave-processing parity; next=finish ParsedHierarchy/FlowBase and route story export through the shared flow tree]
36. `ParsedHierarchy/Weave.cs` (`deps=16`) [partial: reason=the weave helper now owns indentation hierarchy, weavepoint naming, nested weave runtime emission, ancestor naming-collision checks, and a first-pass termination path; missing=exact loose-end propagation, choice/gather container mutation, and full nested termination validation; next=finish flow-tree ownership and nested sequence/conditional weaving]

### InkParser

1. `InkParser/CommentEliminator.cs` (`deps=2`) [ported]
2. `InkParser/InkParser_AuthorWarning.cs` (`deps=2`) [ported]
3. `InkParser/InkParser_Include.cs` (`deps=3`) [ported]
4. `InkParser/InkParser_Whitespace.cs` (`deps=3`) [ported]
5. `InkParser/InkParser_CharacterRanges.cs` (`deps=4`) [ported]
6. `InkParser/InkParser_CommandLineInput.cs` (`deps=4`) [ported]
7. `InkParser/InkParser_Divert.cs` (`deps=7`) [partial: reason=the parser now resolves single diverts, thread diverts, and diverted path components, but full multi-divert/tunnel-onwards parity and tag/content list integration are still incomplete; missing=exact tunnel-onwards chain handling, tag/content-list integration, and the full arrow/divert sequence semantics; next=port ParsedHierarchy/TunnelOnwards, ContentList, and the remaining parser content tree]
8. `InkParser/InkParser_Choices.cs` (`deps=8`) [partial: choice and gather parsing are real and now preserved as typed statement payloads, but flow-level integration still depends on the unfinished parsed hierarchy tree; missing=exact weave ownership and choice/gather runtime reference parity; next=finish ParsedHierarchy/Weave and FlowBase]
9. `InkParser/InkParser_Conditional.cs` (`deps=8`) [partial: reason=conditional grammar is now real enough to parse branches, but the branch-content wrapping and source-aware error recovery still use simplified compatibility paths; missing=exact ErrorWithParsedObject parity and inline/branch ownership wiring; next=continue the remaining flow-tree/parser ownership work]
10. `InkParser/InkParser_Content.cs` (`deps=8`) [partial: reason=text/tag/glue line parsing is real, but full inline logic, choice-specific content rules, and weave integration are still incomplete; missing=multi-branch inline logic and choice-aware content handling; next=port InkParser_Choices/InkParser_Conditional and the remaining hierarchy tree]
11. `InkParser/InkParser_Sequences.cs` (`deps=8`) [partial: reason=inline sequence annotations and single-line parsing are wired, and multiline branch content now converts from parsed statements into ContentList values, but weave-backed nested sequence ownership and full branch routing still depend on the unported weave/tree model; missing=Weave integration, nested loose-end routing, and exact nested sequence ownership; next=finish ParsedHierarchy/Weave or the remaining flow-tree hierarchy]
12. `InkParser/InkParser_Logic.cs` (`deps=9`) [ported]
13. `InkParser/InkParser.cs` (`deps=9`) [partial: reason=the parser wrapper now delegates comment elimination, whitespace, include, debug metadata, and top-level Parse construction into typed statements, but full story export still depends on the ParsedHierarchy flow tree; missing=exact source/error recovery and complete flow-tree handoff; next=finish ParsedHierarchy/Story/FlowBase]
14. `InkParser/InkParser_Statements.cs` (`deps=10`) [partial: reason=the parser helper now dispatches knots, stitches, choices, gathers, declarations, externals, divert lines, text lines, and logic lines as typed objects, with divert lines consuming their terminating newline; missing=sequence/conditional branch ownership and exact story export integration; next=finish ParsedHierarchy/Weave/FlowBase and then tighten remaining statement branch parity such as top-level return diagnostics]
15. `InkParser/InkParser_Tags.cs` (`deps=10`) [ported]
16. `InkParser/InkParser_Knot.cs` (`deps=11`) [ported]
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

1. `InkTestBed/InkTestBed.cs` [ported]
