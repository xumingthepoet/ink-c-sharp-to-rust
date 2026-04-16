// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/PushPop.cs

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PushPopType {
    Tunnel,
    #[default]
    Function,
    FunctionEvaluationFromGame,
}

impl PushPopType {
    pub fn new() -> Self {
        Self::default()
    }
}
