use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

fn rust_test_names() -> BTreeSet<String> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source = fs::read_to_string(manifest_dir.join("src/csharp_tests/mod.rs"))
        .expect("failed to read Rust csharp_tests module");

    let mut names = BTreeSet::new();
    for line in source.lines() {
        let line = line.trim_start();
        if let Some(rest) = line.strip_prefix("fn ") {
            if let Some((name, _)) = rest.split_once('(') {
                if name.starts_with("Test") {
                    names.insert(name.to_string());
                }
            }
        }
    }
    names
}

fn csharp_test_names() -> BTreeSet<String> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source =
        fs::read_to_string(manifest_dir.join("fixtures/csharp_tests/official_test_names.txt"))
            .expect("failed to read official C# test names");

    let mut names = BTreeSet::new();
    for line in source.lines() {
        let line = line.trim_start();
        if !line.is_empty() && !line.starts_with('#') {
            names.insert(line.to_string());
        }
    }
    names
}

fn main() {
    let rust = rust_test_names();
    let csharp = csharp_test_names();
    let missing: Vec<_> = csharp.difference(&rust).cloned().collect();

    println!("missing_count={}", missing.len());
    for name in missing {
        println!("{name}");
    }
}
