// Source: ink-c-sharp/ink-engine-runtime/INamedContent.cs

pub trait INamedContent {
    fn name(&self) -> Option<&str>;
}
