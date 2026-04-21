#!/usr/bin/env python3

from __future__ import annotations

import re
from pathlib import Path


def repo_root() -> Path:
    return Path(__file__).resolve().parents[3]


def crate_root() -> Path:
    return Path(__file__).resolve().parents[1]


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def transform_api(source: str) -> str:
    source = source.replace(
        "use ink_runtime::Choice::Choice;\n",
        "use ink_compiler::Compiler::Compiler;\nuse ink_compiler::Compiler::Options;\nuse ink_runtime::Choice::Choice;\n",
    )
    source = re.sub(
        r"pub fn new\(json: &str\) -> Self \{\n\s*Self \{\n\s*inner: RuntimeStory::new_overload_2\(json\.to_string\(\)\),\n\s*\}\n\s*\}",
        (
            "pub fn new(source: &str) -> Self {\n"
            "        let mut compiler = Compiler::new(source.to_string(), Options::default());\n"
            "        let inner = compiler.Compile().expect(\"source compilation should succeed\");\n"
            "        Self { inner }\n"
            "    }"
        ),
        source,
        count=1,
    )
    return source


def transform_common(source: str) -> str:
    return """#![allow(dead_code)]

use crate::compiler_conformance::api::Story;
use std::{fs, path::Path};

use rand::Rng;

pub fn next_all(story: &mut Story, text: &mut Vec<String>) {
    while story.can_continue() {
        let line = story.cont();
        print!("{line}");

        if !line.trim().is_empty() {
            text.push(line.trim().to_string());
        }
    }

    if !story.get_current_errors().is_empty() {
        panic!("{}", join_text(&story.get_current_errors()));
    }
}

pub fn join_text(text: &[String]) -> String {
    let mut sb = String::new();
    for s in text {
        sb.push_str(s);
    }
    sb
}

pub fn run_story(
    filename: &str,
    choice_list: Option<Vec<usize>>,
    errors: &mut Vec<String>,
) -> Vec<String> {
    let mut story = compile_story(filename);
    let mut text = Vec::new();
    let mut choice_list_index = 0;
    let mut rng = rand::rng();

    while story.can_continue() || !story.get_current_choices().is_empty() {
        println!("{}", story.build_string_of_hierarchy());

        while story.can_continue() {
            let line = story.cont();
            print!("{}", line);
            text.push(line);
        }

        if !story.get_current_errors().is_empty() {
            for error_msg in story.get_current_errors() {
                println!("{}", error_msg);
                errors.push(error_msg.to_string());
            }
        }

        let current_choices = story.get_current_choices();
        if !current_choices.is_empty() {
            let len = current_choices.len();

            for choice in current_choices {
                println!("{}", choice.text);
                text.push(format!("{}\\n", choice.text));
            }

            if let Some(choice_list) = &choice_list {
                if choice_list_index < choice_list.len() {
                    story.choose_choice_index(choice_list[choice_list_index]);
                    choice_list_index += 1;
                } else {
                    let random_choice_index = rng.random_range(0..len);
                    story.choose_choice_index(random_choice_index);
                }
            } else {
                let random_choice_index = rng.random_range(0..len);
                story.choose_choice_index(random_choice_index);
            }
        }
    }

    text
}

pub fn compile_story(filename: &str) -> Story {
    let source = get_ink_string(filename);
    let mut story = Story::new(&source);
    assert_compiled_json_matches_fixture(filename, &mut story);
    story
}

pub fn get_ink_string(filename: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/conformance")
        .join(filename);
    fs::read_to_string(path).expect("fixture ink must exist")
}

fn get_fixture_json_string(filename: &str) -> String {
    let filename = if let Some(prefix) = filename.strip_suffix(".ink") {
        format!("{prefix}.ink.json")
    } else {
        format!("{filename}.ink.json")
    };
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/conformance")
        .join(filename);
    fs::read_to_string(path).expect("fixture json must exist")
}

pub fn assert_compiled_json_matches_fixture(filename: &str, story: &mut Story) {
    let generated_json = story.to_json();
    let fixture_json = get_fixture_json_string(filename);
    if generated_json != fixture_json {
        eprintln!("compiled json mismatch for {filename}");
        eprintln!("--- generated ---\\n{generated_json}");
        eprintln!("--- fixture ---\\n{fixture_json}");
        panic!("compiled json differs from fixture");
    }
}
"""


def transform_module(source: str) -> str:
    source = source.replace("crate::conformance::", "crate::compiler_conformance::")
    source = source.replace("common::get_json_string(", "common::get_ink_string(")
    source = source.replace(".ink.json", ".ink")
    return source


def main() -> None:
    root = crate_root()
    src_dir = root / "src" / "conformance"
    dest_dir = root / "src" / "compiler_conformance"
    dest_dir.mkdir(parents=True, exist_ok=True)

    api_source = (src_dir / "api.rs").read_text(encoding="utf-8")
    common_source = (src_dir / "common.rs").read_text(encoding="utf-8")
    mod_source = (src_dir / "mod.rs").read_text(encoding="utf-8")

    write_text(dest_dir / "api.rs", transform_api(api_source))
    write_text(dest_dir / "common.rs", transform_common(common_source))
    write_text(dest_dir / "mod.rs", mod_source)

    generated = []
    for path in sorted(src_dir.glob("*.rs")):
        if path.name in {"api.rs", "common.rs", "mod.rs"}:
            continue
        text = path.read_text(encoding="utf-8")
        generated_text = transform_module(text)
        write_text(dest_dir / path.name, generated_text)
        generated.append(path.name)

    print(f"generated {len(generated) + 3} compiler_conformance files")
    for name in generated:
        print(name)


if __name__ == "__main__":
    main()
