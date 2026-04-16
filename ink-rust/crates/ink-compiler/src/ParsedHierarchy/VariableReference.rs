// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/VariableReference.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct VariableReference {
    pub _port_marker: (),
}

impl VariableReference {
    // C# signature: public VariableReference (List<Identifier> pathIdentifiers)
    pub fn new(_pathIdentifiers: Vec<crate::stub::Identifier>) -> Self {
        Default::default()
    }

    // C# signature: public override void GenerateIntoContainer (Runtime.Container container)
    pub fn GenerateIntoContainer(&mut self, _container: crate::stub::Container) {}

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: string name { get; }
    pub fn get_name(&mut self) -> String {
        Default::default()
    }

    // C# signature: Identifier identifier { get; }
    pub fn get_identifier(&mut self) -> crate::stub::Identifier {
        Default::default()
    }

    // C# signature: List<string> path { get; }
    pub fn get_path(&mut self) -> Vec<String> {
        Default::default()
    }

    // C# signature: Runtime.VariableReference runtimeVarRef { get; }
    pub fn get_runtimeVarRef(&mut self) -> crate::stub::VariableReference {
        Default::default()
    }
}
