// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Choice.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Choice {
    pub _port_marker: (),
}

impl Choice {
    // C# signature: public Choice (ContentList startContent, ContentList choiceOnlyContent, ContentList innerContent)
    pub fn new(
        _startContent: crate::stub::ContentList,
        _choiceOnlyContent: crate::stub::ContentList,
        _innerContent: crate::stub::ContentList,
    ) -> Self {
        Default::default()
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public override void ResolveReferences(Story context)
    pub fn ResolveReferences(&mut self, _context: crate::stub::Story) {}

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: ContentList startContent { get; }
    pub fn get_startContent(&mut self) -> crate::stub::ContentList {
        Default::default()
    }

    // C# signature: ContentList choiceOnlyContent { get; }
    pub fn get_choiceOnlyContent(&mut self) -> crate::stub::ContentList {
        Default::default()
    }

    // C# signature: ContentList innerContent { get; }
    pub fn get_innerContent(&mut self) -> crate::stub::ContentList {
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

    // C# signature: Expression condition { get; }
    pub fn get_condition(&mut self) -> crate::stub::Expression {
        Default::default()
    }

    // C# signature: bool onceOnly { get; }
    pub fn get_onceOnly(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool isInvisibleDefault { get; }
    pub fn get_isInvisibleDefault(&mut self) -> bool {
        Default::default()
    }

    // C# signature: int indentationDepth { get; }
    pub fn get_indentationDepth(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: bool hasWeaveStyleInlineBrackets { get; }
    pub fn get_hasWeaveStyleInlineBrackets(&mut self) -> bool {
        Default::default()
    }

    // C# signature: Runtime.Container runtimeContainer { get; }
    pub fn get_runtimeContainer(&mut self) -> crate::stub::Container {
        Default::default()
    }

    // C# signature: Runtime.Container innerContentContainer { get; }
    pub fn get_innerContentContainer(&mut self) -> crate::stub::Container {
        Default::default()
    }

    // C# signature: Runtime.Container containerForCounting { get; }
    pub fn get_containerForCounting(&mut self) -> crate::stub::Container {
        Default::default()
    }

    // C# signature: Runtime.Path runtimePath { get; }
    pub fn get_runtimePath(&mut self) -> crate::stub::Path {
        Default::default()
    }
}
