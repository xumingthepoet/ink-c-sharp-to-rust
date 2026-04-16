// Source: ink-c-sharp/compiler/ParsedHierarchy/FlowLevel.cs

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlowLevel {
    #[default]
    Story,
    Knot,
    Stitch,
    WeavePoint,
}

impl FlowLevel {
    pub fn new() -> Self {
        Self::default()
    }
}
