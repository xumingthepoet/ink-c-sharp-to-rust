// Source: ink-c-sharp/ink-engine-runtime/Choice.cs

use crate::CallStack::Thread;
use crate::Path::Path;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Choice {
    pub text: String,
    pub sourcePath: String,
    pub index: i32,
    pub targetPath: Option<Path>,
    pub threadAtGeneration: Option<Thread>,
    pub originalThreadIndex: i32,
    pub isInvisibleDefault: bool,
    pub tags: Vec<String>,
}

impl Choice {
    // C# signature: public Choice()
    pub fn new() -> Self {
        Default::default()
    }

    // C# signature: public Choice Clone()
    pub fn Clone(&self) -> Choice {
        self.clone()
    }

    pub fn set_text(&mut self, value: String) {
        self.text = value;
    }

    // C# signature: string text { get; }
    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_pathStringOnChoice(&mut self, value: Option<String>) {
        self.targetPath = value.map(Path::new_overload_4);
    }

    // C# signature: string pathStringOnChoice { get; }
    pub fn get_pathStringOnChoice(&self) -> Option<String> {
        self.targetPath.as_ref().map(Path::ToString)
    }

    pub fn set_index(&mut self, value: i32) {
        self.index = value;
    }

    // C# signature: int index { get; }
    pub fn get_index(&self) -> i32 {
        self.index
    }

    pub fn set_threadAtGeneration(&mut self, value: Option<Thread>) {
        self.threadAtGeneration = value;
    }

    // C# signature: CallStack.Thread threadAtGeneration { get; }
    pub fn get_threadAtGeneration(&self) -> Option<&Thread> {
        self.threadAtGeneration.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::Choice;
    use crate::CallStack::Thread;
    use crate::Pointer::Pointer;

    #[test]
    fn path_string_updates_target_path() {
        let mut choice = Choice::new();
        choice.set_text("Go".to_string());
        choice.set_index(2);
        choice.set_pathStringOnChoice(Some("knot.stitch".to_string()));

        let cloned = choice.Clone();

        assert_eq!(cloned.get_text(), "Go");
        assert_eq!(cloned.get_index(), 2);
        assert_eq!(
            cloned.get_pathStringOnChoice(),
            Some("knot.stitch".to_string())
        );
    }

    #[test]
    fn clone_copies_generation_thread() {
        let mut choice = Choice::new();
        choice.set_threadAtGeneration(Some(Thread {
            callstack: Vec::new(),
            threadIndex: 7,
            previousPointer: Some(Pointer::Null()),
        }));

        let cloned = choice.Clone();

        let thread = cloned.get_threadAtGeneration().expect("thread copied");
        assert_eq!(thread.threadIndex, 7);
        assert!(thread.previousPointer.as_ref().unwrap().get_isNull());
    }
}
