// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/ParsedHierarchy/Story.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Story {
    pub _port_marker: (),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolType {
    PortPlaceholder,
}

impl Default for SymbolType {
    fn default() -> Self {
        Self::PortPlaceholder
    }
}

impl Story {
    // C# signature: public Story (List<Parsed.Object> toplevelObjects, bool isInclude = false)
    pub fn new(_toplevelObjects: Vec<crate::stub::PortStub>, _isInclude: bool) -> Self {
        Default::default()
    }

    // C# signature: protected override void PreProcessTopLevelObjects(List<Parsed.Object> topLevelContent)
    pub fn PreProcessTopLevelObjects(&mut self, _topLevelContent: Vec<crate::stub::PortStub>) {}

    // C# signature: public Runtime.Story ExportRuntime(ErrorHandler errorHandler = null)
    pub fn ExportRuntime(
        &mut self,
        _errorHandler: crate::stub::ErrorHandler,
    ) -> crate::stub::Story {
        Default::default()
    }

    // C# signature: public ListDefinition ResolveList (string listName)
    pub fn ResolveList(&mut self, _listName: String) -> crate::stub::ListDefinition {
        Default::default()
    }

    // C# signature: public ListElementDefinition ResolveListItem (string listName, string itemName, Parsed.Object source = null)
    pub fn ResolveListItem(
        &mut self,
        _listName: String,
        _itemName: String,
        _source: crate::stub::PortStub,
    ) -> crate::stub::ListElementDefinition {
        Default::default()
    }

    // C# signature: public override void Error(string message, Parsed.Object source, bool isWarning)
    pub fn Error(&mut self, _message: String, _source: crate::stub::PortStub, _isWarning: bool) {}

    // C# signature: public void ResetError()
    pub fn ResetError(&mut self) {}

    // C# signature: public bool IsExternal(string namedFuncTarget)
    pub fn IsExternal(&mut self, _namedFuncTarget: String) -> bool {
        Default::default()
    }

    // C# signature: public void AddExternal(ExternalDeclaration decl)
    pub fn AddExternal(&mut self, _decl: crate::stub::ExternalDeclaration) {}

    // C# signature: public void DontFlattenContainer (Runtime.Container container)
    pub fn DontFlattenContainer(&mut self, _container: crate::stub::Container) {}

    // C# signature: public static bool IsReservedKeyword (string name)
    pub fn IsReservedKeyword(_name: String) -> bool {
        Default::default()
    }

    // C# signature: public void CheckForNamingCollisions (Parsed.Object obj, Identifier identifier, SymbolType symbolType, string typeNameOverride = null)
    pub fn CheckForNamingCollisions(
        &mut self,
        _obj: crate::stub::PortStub,
        _identifier: crate::stub::Identifier,
        _symbolType: crate::stub::SymbolType,
        _typeNameOverride: String,
    ) {
    }

    // C# signature: FlowLevel flowLevel { get; }
    pub fn get_flowLevel(&mut self) -> crate::stub::FlowLevel {
        Default::default()
    }

    // C# signature: bool hadError { get; }
    pub fn get_hadError(&mut self) -> bool {
        Default::default()
    }

    // C# signature: bool hadWarning { get; }
    pub fn get_hadWarning(&mut self) -> bool {
        Default::default()
    }
}
