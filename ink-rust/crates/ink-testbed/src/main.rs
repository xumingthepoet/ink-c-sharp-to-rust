#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

use ink_testbed::InkTestBed::InkTestBed;
use std::env;
use std::path::Path;

fn main() {
    let mut args = env::args().skip(1);
    let Some(first) = args.next() else {
        eprintln!(
            "InkTestBed currently supports JSON story playback and roundtrip. Source compilation is still pending."
        );
        eprintln!("Usage:");
        eprintln!("  ink-testbed <story.json>");
        eprintln!("  ink-testbed --roundtrip <story.json>");
        eprintln!("  ink-testbed --split <file.ink>");
        std::process::exit(1);
    };

    let result = match first.as_str() {
        "--roundtrip" => {
            match args.next() {
                Some(path) => run_roundtrip(Path::new(&path)),
                None => Err("missing JSON path for --roundtrip".to_string()),
            }
        }
        "--split" => {
            match args.next() {
                Some(path) => run_split(Path::new(&path)),
                None => Err("missing file path for --split".to_string()),
            }
        }
        path if path.ends_with(".json") => run_play_json(Path::new(path)),
        path if path.ends_with(".ink") => Err(
            "source compilation is still pending in the compiler front-end; pass a compiled .json story instead"
                .to_string(),
        ),
        other => Err(format!(
            "unrecognized argument '{}'; pass a .json story path, --roundtrip, or --split",
            other
        )),
    };

    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run_play_json(path: &Path) -> Result<(), String> {
    let mut bed = InkTestBed::new();
    bed.load_story_json_file(path)?;
    bed.play()
}

fn run_roundtrip(path: &Path) -> Result<(), String> {
    let mut bed = InkTestBed::new();
    bed.load_story_json_file(path)?;
    bed.json_roundtrip()
}

fn run_split(path: &Path) -> Result<(), String> {
    let (first, second) = InkTestBed::split_file(path)?;
    println!("--- FIRST INK VERSION ---");
    println!("{}", first);
    println!("--- SECOND INK VERSION ---");
    println!("{}", second);
    Ok(())
}
