// Source: ink-c-sharp/ink-engine-runtime/StoryException.cs

#[derive(Clone, Debug, Default)]
pub struct StoryException {
    pub useEndLineNumber: bool,
    pub message: Option<String>,
}

impl StoryException {
    // C# signature: public StoryException ()
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public StoryException(string message)
    pub fn new_overload_2(_message: String) -> Self {
        Self {
            useEndLineNumber: false,
            message: Some(_message),
        }
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl std::fmt::Display for StoryException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.message {
            Some(message) => write!(f, "{message}"),
            None => write!(f, "StoryException"),
        }
    }
}

impl std::error::Error for StoryException {}
