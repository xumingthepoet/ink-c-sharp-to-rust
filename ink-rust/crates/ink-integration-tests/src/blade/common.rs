pub(crate) use ink_compiler::Compiler::{Compiler, Options};

pub(crate) fn compile_string(source: &str) -> ink_runtime::Story::Story {
    let mut compiler = Compiler::new(source.to_string(), Options::default());
    compiler
        .Compile()
        .expect("source compilation should succeed")
}

pub(crate) fn continue_all(story: &mut ink_runtime::Story::Story) -> Vec<String> {
    let mut text = Vec::new();
    while story.get_canContinue() {
        let line = story.Continue();
        if !line.trim().is_empty() {
            text.push(line.trim().to_string());
        }
    }
    text
}
