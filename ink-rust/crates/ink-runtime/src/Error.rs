// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Error.cs

pub type ErrorHandler = Box<dyn FnMut(&str, ErrorType)>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ErrorType {
    Author,
    Warning,
    #[default]
    Error,
}

impl ErrorType {
    pub fn new() -> Self {
        Self::default()
    }
}
