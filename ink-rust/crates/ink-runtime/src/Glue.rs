// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Glue.cs

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Glue;

impl Glue {
    // C# signature: public Glue()
    pub fn new() -> Self {
        Self
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        self.to_string()
    }
}

impl std::fmt::Display for Glue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Glue")
    }
}
