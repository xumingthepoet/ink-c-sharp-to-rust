// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Weave.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Weave {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct GatherPointToResolve {
    pub _port_marker: (),
}

impl Weave {
    // C# signature: public Weave(List<Parsed.Object> cont, int indentIndex=-1)
    pub fn new(_cont: Vec<crate::stub::PortStub>, _indentIndex: i32) -> Self {
        Default::default()
    }

    // C# signature: public void ResolveWeavePointNaming ()
    pub fn ResolveWeavePointNaming(&mut self) {}

    // C# signature: public int DetermineBaseIndentationFromContent(List<Parsed.Object> contentList)
    pub fn DetermineBaseIndentationFromContent(
        &mut self,
        _contentList: Vec<crate::stub::PortStub>,
    ) -> i32 {
        Default::default()
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public void AddRuntimeForNestedWeave(Weave nestedResult)
    pub fn AddRuntimeForNestedWeave(&mut self, _nestedResult: crate::stub::Weave) {}

    // C# signature: public override void ResolveReferences(Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: public IWeavePoint WeavePointNamed(string name)
    pub fn WeavePointNamed(&mut self, _name: String) -> crate::stub::IWeavePoint {
        Default::default()
    }

    // C# signature: public void ValidateTermination (BadTerminationHandler badTerminationHandler)
    pub fn ValidateTermination(
        &mut self,
        _badTerminationHandler: crate::stub::BadTerminationHandler,
    ) {
    }

    // C# signature: Runtime.Container rootContainer { get; }
    pub fn get_rootContainer(&mut self) -> crate::stub::Container {
        Default::default()
    }

    // C# signature: int baseIndentIndex { get; }
    pub fn get_baseIndentIndex(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: Parsed.Object lastParsedSignificantObject { get; }
    pub fn get_lastParsedSignificantObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }
}
