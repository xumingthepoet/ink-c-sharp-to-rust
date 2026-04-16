// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/FlowBase.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct FlowBase {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct Argument {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct VariableResolveResult {
    pub _port_marker: (),
}

impl FlowBase {
    // C# signature: public FlowBase (Identifier name = null, List<Parsed.Object> topLevelObjects = null, List<Argument> arguments = null, bool isFunction = false, bool isIncludedStory = false)
    pub fn new(
        _name: crate::stub::Identifier,
        _topLevelObjects: Vec<crate::stub::PortStub>,
        _arguments: Vec<crate::stub::Argument>,
        _isFunction: bool,
        _isIncludedStory: bool,
    ) -> Self {
        Default::default()
    }

    // C# signature: protected virtual void PreProcessTopLevelObjects(List<Parsed.Object> topLevelObjects)
    pub fn PreProcessTopLevelObjects(&mut self, _topLevelObjects: Vec<crate::stub::PortStub>) {}

    // C# signature: public VariableResolveResult ResolveVariableWithName(string varName, Parsed.Object fromNode)
    pub fn ResolveVariableWithName(
        &mut self,
        _varName: String,
        _fromNode: crate::stub::PortStub,
    ) -> crate::stub::VariableResolveResult {
        Default::default()
    }

    // C# signature: public void TryAddNewVariableDeclaration(VariableAssignment varDecl)
    pub fn TryAddNewVariableDeclaration(&mut self, _varDecl: crate::stub::VariableAssignment) {}

    // C# signature: public void ResolveWeavePointNaming ()
    pub fn ResolveWeavePointNaming(&mut self) {}

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public Parsed.Object ContentWithNameAtLevel(string name, FlowLevel? level = null, bool deepSearch = false)
    pub fn ContentWithNameAtLevel(
        &mut self,
        _name: String,
        _level: crate::stub::FlowLevel,
        _deepSearch: bool,
    ) -> crate::stub::PortStub {
        Default::default()
    }

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

    // C# signature: List<Argument> arguments { get; }
    pub fn get_arguments(&mut self) -> Vec<crate::stub::Argument> {
        Default::default()
    }

    // C# signature: bool hasParameters { get; }
    pub fn get_hasParameters(&mut self) -> bool {
        Default::default()
    }

    // C# signature: FlowLevel flowLevel { get; }
    pub fn get_flowLevel(&mut self) -> crate::stub::FlowLevel {
        Default::default()
    }

    // C# signature: bool isFunction { get; }
    pub fn get_isFunction(&mut self) -> bool {
        Default::default()
    }

    // C# signature: string typeName { get; }
    pub fn get_typeName(&mut self) -> String {
        Default::default()
    }
}
