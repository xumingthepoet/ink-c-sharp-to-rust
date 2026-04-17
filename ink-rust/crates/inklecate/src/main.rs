#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

use ink_compiler::Compiler::Compiler;
use ink_compiler::Compiler::Options;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let mut args = env::args().skip(1);
    let Some(first) = args.next() else {
        eprintln!("Usage: inklecate <file.ink>");
        std::process::exit(1);
    };

    let path = Path::new(&first);
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let mut compiler = Compiler::new(source, Options::default());
    let Some(mut runtime_story) = compiler.Compile() else {
        eprintln!("Compilation failed");
        std::process::exit(1);
    };

    println!("{}", runtime_story.ToJson());
}
