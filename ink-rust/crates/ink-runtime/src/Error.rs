// Source: ink-c-sharp/ink-engine-runtime/Error.cs

pub type ErrorHandler = Box<dyn FnMut(&str, ErrorType)>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorType {
    Author,
    Warning,
    Error,
}
