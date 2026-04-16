// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/INamedContent.cs

pub trait INamedContent {
    fn name(&self) -> Option<&str>;

    fn hasValidName(&self) -> bool {
        self.name().is_some_and(|name| !name.is_empty())
    }
}
