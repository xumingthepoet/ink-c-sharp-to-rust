# AGENTS.md

This repository exists to port the official C# implementation of ink to Rust, as preparation for later custom ink development.

## Repository Layout

- `ink-c-sharp/` is the official upstream C# implementation from Inkle. Treat it as the source of truth for behavior, data formats, compiler output, runtime semantics, and edge cases.
- `blade-ink-rs/` is an unofficial Rust implementation. It is useful reference material, especially for runtime structure and Rust API ideas, but it is not authoritative when it differs from the C# implementation.
- `ink-rust/` is the target Rust port. All new implementation, tests, scaffolding, scripts, and refactors should live here.

Both reference projects are read-only in this workspace. Do not edit, format, delete, regenerate, or commit changes inside `ink-c-sharp/` or `blade-ink-rs/`. New porting work should live in `ink-rust/` unless the user explicitly changes this rule.

There is no single root build system today. Run commands from the relevant project directory or pass the correct manifest/project path.

## Porting Principles

1. Preserve official ink behavior first.
   - When C# and `blade-ink-rs` disagree, follow `ink-c-sharp`.
   - Prefer direct behavioral parity over idiomatic rewrites that subtly change semantics.
   - Keep story JSON format, save format, path resolution, visit counts, variable behavior, list behavior, divert behavior, and error timing compatible with official ink.

2. Use `blade-ink-rs` carefully.
   - Reuse design ideas only after checking the corresponding C# implementation.
   - Do not assume its conformance tests cover every official edge case.
   - If copying a structure, rename or reshape it only when Rust ownership or API clarity requires it.

3. Use a mechanical port before designing a Rust-native implementation.
   - First mirror the important C# directory and file structure under `ink-rust/`.
   - Then copy the C# type and function shape into Rust with stub bodies until the whole workspace compiles.
   - Only after the full skeleton compiles should implementation details be filled in file by file.
   - Do not start by redesigning the architecture around Rust idioms or `blade-ink-rs`.

4. Keep compatibility visible.
   - When implementing a Rust type from C#, mention the source C# file in comments or commit notes when helpful.
   - Preserve important names where they map to ink concepts, even if local Rust naming differs.
   - Avoid broad refactors while behavior is still being matched.

## Mechanical Porting Workflow

The first pass is structural, not idiomatic. The goal is to use the official C# project as a porting map so later work becomes filling known holes rather than rediscovering the whole implementation.

1. Create a one-to-one Rust skeleton.
   - `ink-c-sharp/ink-engine-runtime/Foo.cs` maps to `ink-rust/crates/ink-runtime/src/Foo.rs`.
   - `ink-c-sharp/compiler/Foo.cs` maps to `ink-rust/crates/ink-compiler/src/Foo.rs`.
   - `ink-c-sharp/compiler/InkParser/Foo.cs` maps to `ink-rust/crates/ink-compiler/src/InkParser/Foo.rs`.
   - `ink-c-sharp/compiler/ParsedHierarchy/Foo.cs` maps to `ink-rust/crates/ink-compiler/src/ParsedHierarchy/Foo.rs`.
   - `ink-c-sharp/compiler/StringParser/Foo.cs` maps to `ink-rust/crates/ink-compiler/src/StringParser/Foo.rs`.
   - `ink-c-sharp/compiler/Plugins/Foo.cs` maps to `ink-rust/crates/ink-compiler/src/Plugins/Foo.rs`.
   - Exclude `bin/`, `obj/`, and generated files.

2. Copy type and function shapes before implementing behavior.
   - C# classes become Rust structs plus impl blocks.
   - C# interfaces become Rust traits.
   - C# enums become Rust enums.
   - C# partial classes become multiple Rust files implementing the same Rust type.
   - During the initial skeleton phase only, function bodies may return empty objects, default values, `None`, empty collections, or `todo!("port pending")`.
   - Constructors should initialize enough state for `cargo check` to pass, even if behavior is incomplete.

3. Allow temporary non-idiomatic Rust.
   - It is acceptable to use C#-style names, PascalCase file names, getters/setters, broad `Rc<RefCell<_>>`, placeholder traits, and compatibility shims while the port is incomplete.
   - Add crate-level allow attributes where useful, such as `dead_code`, `unused_variables`, `non_snake_case`, and `non_camel_case_types`.
   - Do not spend time Rustifying code before the full behavior is implemented and tested.

4. Fill implementation holes file by file.
   - For each Rust file, read the corresponding C# file first.
   - Translate by symbol, not by whole file: one class/interface/enum first, then one method group, then the surrounding module.
   - Translate behavior in small sections while preserving the original control flow where practical.
   - If Rust ownership requires helper functions or wrapper types, add them locally and keep their purpose obvious.
   - If `blade-ink-rs` suggests a simpler Rust shape, use it only after confirming it preserves official C# behavior.

## File Port Completion Standard

A file is not considered ported just because it compiles. A file is only ported when the corresponding C# file's behavior has been translated completely enough that it should not need another pass except for later Rustification or refactoring.

When moving a file out of skeleton/stub state:

- Do not leave empty logic in place. No method may keep a placeholder body such as an unconditional default value, empty collection, `None`, no-op, or fake success unless the C# implementation itself has equivalent behavior.
- Do not omit part of a method, property, constructor, nested type, enum value, side effect, validation, exception path, cache behavior, ordering rule, or mutation just to make the file compile.
- Do not add new logic that is not required by the official C# behavior. Helper functions are allowed only when they faithfully express the C# logic or are necessary for Rust ownership and type safety.
- Do not replace missing dependencies with invented behavior. If a method cannot be faithfully implemented because required upstream types are still skeletons, leave this file in skeleton/stub state and document it as not ported rather than creating a partial implementation.
- Preserve meaningful `null` behavior with `Option<T>`, thrown exceptions with `Result<T, E>` or an equivalent error path, and C# collection semantics including ordering, duplicate handling, and mutability.
- Port nested types and related methods together when the C# file depends on their shared invariants. Avoid splitting a file into "mostly done" and "later" sections unless those sections are genuinely independent and no public method lies about its behavior.
- Add focused tests for the completed behavior where practical, especially for edge cases, errors, ordering, and mutation.

If a file cannot meet this standard yet, keep it as an explicit skeleton/stub. A visible stub is better than a compiling partial port that hides missing logic.

If a file is too large or difficult to complete in one pass and a partial implementation must be committed:

- Treat this as an exception, not the normal workflow.
- Do not mark the file as `[ported]`.
- Update `PORT_PRIORITY.md` in the same change with a `[partial: ...]` annotation on that file entry.
- The annotation must state why it could not be completed, which C# behavior remains unfinished, and what dependency or next step is needed to finish it.
- Keep the Rust code honest: unfinished methods must remain visibly stubbed or return an explicit error/todo path rather than silently returning plausible but wrong values.
- Prefer keeping the file as a skeleton when the incomplete sections would make public APIs lie about behavior.

After completing a file under this standard:

- Update `PORT_PRIORITY.md` in the same change and annotate that source file's list entry as ported.
- Only add the annotation after the Rust file has no placeholder behavior for the corresponding C# file and `make -C ink-rust gate` passes.
- Do not mark a file as ported for partial structure work, type-only wiring, compile-only cleanup, or tests that cover only a small subset of the C# behavior.
- If later refactoring changes the file without changing behavior, keep the ported annotation. If a later audit finds missing C# behavior, remove or correct the annotation in the same change that records the gap.

5. Rustify only after compatibility is complete.
   - Once all ported tests pass, refactor toward idiomatic Rust.
   - Each Rustification step must preserve behavior and rerun the relevant tests.
   - Renaming files to snake_case, reducing `Rc<RefCell<_>>`, improving error types, and simplifying APIs belong in this final phase.

## Symbol Translation Contract

When implementing a symbol, gather the following before writing Rust:

- The C# code for the class, interface, enum, or method group being translated.
- Referenced types and signatures needed by that symbol.
- Nullability expectations, including where C# uses `null` as a meaningful state.
- Exceptions thrown or propagated by the C# code.
- Collection types and their mutability/ordering semantics.
- The target Rust module skeleton in `ink-rust/`.

Use this fixed translation contract unless an explicit compatibility reason requires otherwise:

- C# class -> Rust struct plus impl blocks.
- C# interface -> Rust trait.
- C# sum-like type or discriminated state -> Rust enum.
- C# nullable/reference-null state -> `Option<T>`.
- C# APIs that throw for expected failure paths -> `Result<T, E>`.
- Do not use `unsafe` unless it is necessary and justified.
- Prefer `&T` and `&mut T` over cloning; clone only when ownership requires it or C# value semantics require a copy.

## Useful Source Areas

Official C# reference:

- `ink-c-sharp/ink-engine-runtime/` contains the runtime engine.
- `ink-c-sharp/compiler/` contains compiler entry points and supporting code.
- `ink-c-sharp/compiler/InkParser/` contains ink syntax parsing.
- `ink-c-sharp/compiler/ParsedHierarchy/` contains parsed AST-like structures and generation logic.
- `ink-c-sharp/tests/` contains official tests and fixtures.
- `ink-c-sharp/Documentation/ink_JSON_runtime_format.md` documents compiled story JSON.
- `ink-c-sharp/Documentation/ArchitectureAndDevOverview.md` explains the official architecture.

Rust reference:

- `blade-ink-rs/lib/src/` contains runtime library code.
- `blade-ink-rs/conformance-tests/` contains Rust-side conformance fixtures.
- `blade-ink-rs/cli-player/` contains the terminal player.

Target Rust port:

- `ink-rust/crates/ink-runtime/` should mirror `ink-c-sharp/ink-engine-runtime/`.
- `ink-rust/crates/ink-compiler/` should mirror `ink-c-sharp/compiler/`.
- `ink-rust/crates/inklecate/` should contain the Rust command-line tool.
- `ink-rust/crates/ink-tests/` should contain the ported compatibility tests.

## Verification

Use the narrowest command that proves the change. Prefer targeted tests during development and broader tests before handing off larger changes.

All test commands must run with a timeout. The default timeout is 5 seconds. Language implementation bugs often produce infinite loops, so do not leave a stuck test process running. If GNU `timeout` is unavailable, use `gtimeout` or the runner/tool timeout facility, and kill the process promptly if it exceeds the limit.

Official C# checks:

```sh
timeout 5s dotnet test ink-c-sharp/tests/tests.csproj
timeout 5s dotnet build ink-c-sharp/ink.sln
```

Rust checks:

```sh
make -C ink-rust gate
timeout 5s cargo test --manifest-path ink-rust/Cargo.toml
timeout 5s cargo test --manifest-path ink-rust/Cargo.toml -p ink-runtime
timeout 5s cargo test --manifest-path ink-rust/Cargo.toml -p ink-compiler
timeout 5s cargo test --manifest-path ink-rust/Cargo.toml -p ink-tests
```

If a command cannot be run because dependencies are missing, record that explicitly in the final handoff.

The final compatibility target is to port or connect every test case from both reference projects into `ink-rust/crates/ink-tests/`. Reference tests and fixtures may be read, but the reference projects themselves must remain unchanged. If a `blade-ink-rs` test disagrees with official C# behavior, keep the test concept but change the expected result to match official C#.

After any phase-level change, run `make -C ink-rust gate`. If it passes, the phase is commit-ready. Small exploratory edits do not need a commit, but completed skeleton, fill-in, test-port, and refactor phases should be committed after the gate passes.

## Coding Guidelines

- Read the relevant C# implementation before changing Rust behavior.
- During the skeleton phase, preserve C# file and symbol names more than Rust style.
- Prefer structured parsers and typed data over ad hoc string manipulation.
- Keep public APIs stable unless the task explicitly requires redesign.
- Do not change generated fixtures or compiled JSON unless the behavior change requires it.
- Keep comments short and reserved for non-obvious compatibility details.
- Use `rg` for source searches.
- Use `cargo fmt` for Rust formatting when Rust files are changed.
- Do not rewrite unrelated files or clean up unrelated style issues.

## Debug Notes

- When a debugging session reveals a useful invariant, a recurring failure mode, or a better tool path, record it here in one or two sentences.
- Prefer concrete observations over general advice. Write down the exact symptom, the file or symbol involved, and the shortest reliable way to reproduce or inspect it.
- Keep the note actionable for future agents. The goal is to avoid re-discovering the same dead ends and to reduce token spent on repeated search.
- Update this section when a diagnosis changes the preferred workflow for a subsystem, especially for parser, runtime, or testbed issues.
- `InkParser_Statements.wrap_divert_line()` should close active tags before wrapping divert pieces into a `ContentList`; otherwise divert lines can leak tag state into the next statement.
- `VariableReference` read-count warnings only need the surrounding parsed container context. Preserve a small parent-context flag from the direct parsed wrapper instead of trying to reconstruct full ancestry from a synthetic root.
- `Compiler::Parse()` error callbacks must stay `Send + Sync`; do not capture `Rc<RefCell<...>>` runtime handlers inside the parser closure. Format the parser message locally and keep forwarding outside the callback boundary.
- For `ParsedHierarchy/Weave`, the C# termination check is not just "content exists"; it must scan backward for non-returning divert-like flow exits and then inspect following flow content. `BadNestedTerminationHandler` also needs to walk ancestor chains and special-case the one-choice conditional tutorial message, so any future weave cleanup should preserve that ancestor-sensitive path before refactoring the flow tree.
- For `ParsedHierarchy/ConditionalSingleBranch`, the "Saw the text 'else:'" warning should inspect parsed `ContentList` text nodes, not runtime container strings. The warning is a parse-tree check on `_innerWeave.content`, so container-based checks can silently miss the warning until after code generation has already reshaped the branch.
- `ParsedHierarchy/VariableAssignment` ownership is easiest to verify through `Object::from_variable_assignment(...).content` payloads. Runtime output alone can miss tree-wiring regressions for list-backed declarations.

## Git And Workspace Hygiene

- The root repository contains nested reference project directories. Check status in the root before committing, but do not make changes inside `ink-c-sharp/` or `blade-ink-rs/`.
- Do not revert user changes.
- Do not delete generated or fixture files from either reference project.
- Keep commits focused on one porting or compatibility topic.
- Before every commit, run `make -C ink-rust gate` and only commit if it passes. If the gate cannot run, do not commit unless the user explicitly accepts that state.
- Commit completed porting work in small batches: 3-5 small files, 2-3 medium files, or 1 large file. Documentation-only changes may be committed separately.
- When a file is marked `[ported]` or `[partial: ...]` in `PORT_PRIORITY.md`, include that annotation update in the same commit as the code change that made it true, and update the progress summary in `PORT_PRIORITY.md` in the same commit if the totals changed.
- Do not commit a file as ported if it only compiles, only removes a stub, or still hides missing C# behavior behind empty/default logic.
- If an in-progress change uncovers that a previous commit was only a partial port, make a corrective commit that updates `PORT_PRIORITY.md` and restores visible unfinished markers before continuing.
