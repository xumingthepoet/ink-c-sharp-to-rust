// Source: ink-c-sharp/compiler/ParsedHierarchy/Story.cs

use crate::ParsedHierarchy::ConstantDeclaration::ConstantDeclaration;
use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::ExternalDeclaration::ExternalDeclaration;
use crate::ParsedHierarchy::FlowLevel::FlowLevel;
use crate::ParsedHierarchy::FunctionCall::FunctionCall;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::ListDefinition::{ListDefinition, ListElementDefinition};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Story {
    pub constants: HashMap<String, Expression>,
    pub externals: HashMap<String, ExternalDeclaration>,
    listDefs: HashMap<String, ListDefinition>,
    hadError: bool,
    hadWarning: bool,
    isInclude: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolType {
    Knot,
    List,
    ListItem,
    Var,
    SubFlowAndWeave,
    Arg,
    Temp,
}

impl Story {
    // C# signature: public Story (List<Parsed.Object> toplevelObjects, bool isInclude = false)
    pub fn new(_toplevelObjects: Vec<crate::stub::PortStub>, isInclude: bool) -> Self {
        Self {
            isInclude,
            ..Default::default()
        }
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
    pub fn ResolveList(&self, listName: String) -> Option<ListDefinition> {
        self.listDefs.get(&listName).cloned()
    }

    // C# signature: public ListElementDefinition ResolveListItem (string listName, string itemName, Parsed.Object source = null)
    pub fn ResolveListItem(
        &mut self,
        listName: String,
        itemName: String,
        _source: crate::stub::PortStub,
    ) -> Option<ListElementDefinition> {
        if let Some(list) = self.listDefs.get(&listName) {
            let mut list = list.clone();
            return list.ItemNamed(itemName).cloned();
        }

        let mut found_item: Option<ListElementDefinition> = None;
        let mut found_list_name: Option<String> = None;
        let mut ambiguity: Option<(String, String)> = None;

        for (_named_list, list) in &self.listDefs {
            let mut list = list.clone();
            let list_name = list.get_name().unwrap_or("").to_string();
            if let Some(item) = list.ItemNamed(itemName.clone()) {
                if found_item.is_some() {
                    ambiguity = Some((found_list_name.clone().unwrap_or_default(), list_name));
                    break;
                }

                found_list_name = Some(list_name);
                found_item = Some(item.clone());
            }
        }

        if let Some((first, second)) = ambiguity {
            self.Error(
                format!(
                    "Ambiguous item name '{}' found in multiple sets, including {} and {}",
                    itemName, first, second
                ),
                Default::default(),
                false,
            );
            return None;
        }

        found_item
    }

    // C# signature: public override void Error(string message, Parsed.Object source, bool isWarning)
    pub fn Error(&mut self, _message: String, _source: crate::stub::PortStub, isWarning: bool) {
        self.hadWarning = isWarning;
        self.hadError = !isWarning;
    }

    // C# signature: public void ResetError()
    pub fn ResetError(&mut self) {
        self.hadError = false;
        self.hadWarning = false;
    }

    // C# signature: public bool IsExternal(string namedFuncTarget)
    pub fn IsExternal(&self, namedFuncTarget: String) -> bool {
        self.externals.contains_key(&namedFuncTarget)
    }

    // C# signature: public void AddExternal(ExternalDeclaration decl)
    pub fn AddExternal(&mut self, decl: ExternalDeclaration) {
        if let Some(name) = decl.get_name().map(|name| name.to_string()) {
            if self.externals.contains_key(&name) {
                self.Error(
                    format!("Duplicate EXTERNAL definition of '{}'", name),
                    Default::default(),
                    false,
                );
            } else {
                self.externals.insert(name, decl);
            }
        }
    }

    // C# signature: public void DontFlattenContainer (Runtime.Container container)
    pub fn DontFlattenContainer(&mut self, _container: crate::stub::Container) {}

    // C# signature: public static bool IsReservedKeyword (string name)
    pub fn IsReservedKeyword(name: String) -> bool {
        matches!(
            name.as_str(),
            "true"
                | "false"
                | "not"
                | "return"
                | "else"
                | "VAR"
                | "CONST"
                | "temp"
                | "LIST"
                | "function"
        )
    }

    // C# signature: public void CheckForNamingCollisions (Parsed.Object obj, Identifier identifier, SymbolType symbolType, string typeNameOverride = null)
    pub fn CheckForNamingCollisions(
        &mut self,
        _obj: crate::stub::PortStub,
        identifier: Identifier,
        symbolType: SymbolType,
        _typeNameOverride: String,
    ) {
        let Some(name) = identifier.name else {
            return;
        };

        if Self::IsReservedKeyword(name.clone()) {
            self.Error(
                format!("'{}' is a reserved keyword", name),
                Default::default(),
                false,
            );
            return;
        }

        match symbolType {
            SymbolType::List => {
                if self.listDefs.contains_key(&name)
                    || self.constants.contains_key(&name)
                    || self.externals.contains_key(&name)
                    || FunctionCall::IsBuiltIn(name.clone())
                {
                    self.Error(
                        format!("name '{}' has already been used", name),
                        Default::default(),
                        false,
                    );
                }

                for list_def in self.listDefs.values() {
                    for item in &list_def.itemDefinitions {
                        if item.get_name() == Some(name.as_str()) {
                            self.Error(
                                format!("name '{}' has already been used", name),
                                Default::default(),
                                false,
                            );
                            return;
                        }
                    }
                }
            }
            SymbolType::Var
            | SymbolType::Temp
            | SymbolType::Arg
            | SymbolType::Knot
            | SymbolType::SubFlowAndWeave
            | SymbolType::ListItem => {
                if self.constants.contains_key(&name)
                    || self.listDefs.contains_key(&name)
                    || self.externals.contains_key(&name)
                    || FunctionCall::IsBuiltIn(name.clone())
                {
                    self.Error(
                        format!("name '{}' has already been used", name),
                        Default::default(),
                        false,
                    );
                }

                for list_def in self.listDefs.values() {
                    for item in &list_def.itemDefinitions {
                        if item.get_name() == Some(name.as_str()) {
                            self.Error(
                                format!("name '{}' has already been used", name),
                                Default::default(),
                                false,
                            );
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn register_constant(&mut self, name: String, expr: Expression) {
        self.constants.insert(name, expr);
    }

    pub fn register_list_definition(&mut self, list_def: ListDefinition) {
        if let Some(name) = list_def.get_name().map(|name| name.to_string()) {
            self.listDefs.insert(name, list_def);
        }
    }

    // FlowBase/Story compatibility surface
    pub fn get_flowLevel(&self) -> FlowLevel {
        FlowLevel::Story
    }

    // C# signature: bool hadError { get; }
    pub fn get_hadError(&self) -> bool {
        self.hadError
    }

    // C# signature: bool hadWarning { get; }
    pub fn get_hadWarning(&self) -> bool {
        self.hadWarning
    }

    pub fn get_isInclude(&self) -> bool {
        self.isInclude
    }
}
