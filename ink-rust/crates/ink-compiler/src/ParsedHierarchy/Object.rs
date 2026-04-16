// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Object.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Object {
    pub _port_marker: (),
}

impl Object {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public Parsed.Path PathRelativeTo(Parsed.Object otherObj)
    pub fn PathRelativeTo(&mut self, _otherObj: crate::stub::PortStub) -> crate::stub::Path {
        Default::default()
    }

    // C# signature: public T AddContent<T>(T subContent)
    pub fn AddContent(&mut self, _subContent: crate::stub::PortStub) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public void AddContent<T>(List<T> listContent)
    pub fn AddContent_overload_2(&mut self, _listContent: Vec<crate::stub::PortStub>) {}

    // C# signature: public T InsertContent<T>(int index, T subContent)
    pub fn InsertContent(
        &mut self,
        _index: i32,
        _subContent: crate::stub::PortStub,
    ) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public T Find<T>(FindQueryFunc<T> queryFunc = null)
    pub fn Find(&mut self, _queryFunc: crate::stub::PortStub) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public List<T> FindAll<T>(FindQueryFunc<T> queryFunc = null)
    pub fn FindAll(&mut self, _queryFunc: crate::stub::PortStub) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: public abstract Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public virtual void ResolveReferences(Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: public FlowBase ClosestFlowBase()
    pub fn ClosestFlowBase(&mut self) -> crate::stub::FlowBase {
        Default::default()
    }

    // C# signature: public virtual void Error(string message, Parsed.Object source = null, bool isWarning = false)
    pub fn Error(&mut self, _message: String, _source: crate::stub::PortStub, _isWarning: bool) {}

    // C# signature: public void Warning(string message, Parsed.Object source = null)
    pub fn Warning(&mut self, _message: String, _source: crate::stub::PortStub) {}

    // C# signature: public static implicit operator bool (Object obj)
    pub fn operator_stub(_obj: crate::stub::PortStub) -> crate::stub::implicit {
        Default::default()
    }

    // C# signature: public static bool operator ==(Object a, Object b)
    pub fn operator_stub_overload_2(_a: crate::stub::PortStub, _b: crate::stub::PortStub) -> bool {
        Default::default()
    }

    // C# signature: public override bool Equals (object obj)
    pub fn Equals(&mut self, _obj: crate::stub::PortStub) -> bool {
        Default::default()
    }

    // C# signature: public override int GetHashCode ()
    pub fn GetHashCode(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: Runtime.DebugMetadata debugMetadata { get; }
    pub fn get_debugMetadata(&mut self) -> crate::stub::DebugMetadata {
        Default::default()
    }

    // C# signature: bool hasOwnDebugMetadata { get; }
    pub fn get_hasOwnDebugMetadata(&mut self) -> bool {
        Default::default()
    }

    // C# signature: string typeName { get; }
    pub fn get_typeName(&mut self) -> String {
        Default::default()
    }

    // C# signature: Parsed.Object parent { get; }
    pub fn get_parent(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: List<Parsed.Object> content { get; }
    pub fn get_content(&mut self) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: Parsed.Story story { get; }
    pub fn get_story(&mut self) -> crate::stub::Story {
        Default::default()
    }

    // C# signature: Runtime.Object runtimeObject { get; }
    pub fn get_runtimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: Runtime.Path runtimePath { get; }
    pub fn get_runtimePath(&mut self) -> crate::stub::Path {
        Default::default()
    }

    // C# signature: Runtime.Container containerForCounting { get; }
    pub fn get_containerForCounting(&mut self) -> crate::stub::Container {
        Default::default()
    }

    // C# signature: List<Parsed.Object> ancestry { get; }
    pub fn get_ancestry(&mut self) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: string descriptionOfScope { get; }
    pub fn get_descriptionOfScope(&mut self) -> String {
        Default::default()
    }
}
