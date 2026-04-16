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
   - Function bodies may initially return empty objects, default values, `None`, empty collections, or `todo!("port pending")`.
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

## Git And Workspace Hygiene

- The root repository contains nested reference project directories. Check status in the root before committing, but do not make changes inside `ink-c-sharp/` or `blade-ink-rs/`.
- Do not revert user changes.
- Do not delete generated or fixture files from either reference project.
- Keep commits focused on one porting or compatibility topic.
