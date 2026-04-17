// Source: ink-c-sharp/compiler/ParsedHierarchy/Story.cs

use crate::ParsedHierarchy::ConstantDeclaration::ConstantDeclaration;
use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::ExternalDeclaration::ExternalDeclaration;
use crate::ParsedHierarchy::FlowBase::VariableResolveResult;
use crate::ParsedHierarchy::FlowLevel::FlowLevel;
use crate::ParsedHierarchy::FunctionCall::FunctionCall;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::ListDefinition::{ListDefinition, ListElementDefinition};
use crate::ParsedHierarchy::Object::{Object, ObjectKind};
use ink_runtime::Container::{Container, ContentItem};
use ink_runtime::ControlCommand::ControlCommand;
use ink_runtime::Error::{ErrorHandler, ErrorType};
use ink_runtime::Story::Story as RuntimeStory;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct Story {
    pub content: Vec<Object>,
    pub constants: HashMap<String, Expression>,
    pub externals: HashMap<String, ExternalDeclaration>,
    pub variableDeclarations:
        HashMap<String, crate::ParsedHierarchy::VariableAssignment::VariableAssignment>,
    listDefs: HashMap<String, ListDefinition>,
    pub countAllVisits: bool,
    dontFlattenContainers: Vec<Container>,
    hadError: bool,
    hadWarning: bool,
    isInclude: bool,
    errorHandler: Option<Rc<RefCell<ErrorHandler>>>,
}

impl std::fmt::Debug for Story {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Story")
            .field("content", &self.content)
            .field("constants", &self.constants)
            .field("externals", &self.externals)
            .field("variableDeclarations", &self.variableDeclarations)
            .field("listDefs", &self.listDefs)
            .field("countAllVisits", &self.countAllVisits)
            .field("dontFlattenContainers", &self.dontFlattenContainers.len())
            .field("hadError", &self.hadError)
            .field("hadWarning", &self.hadWarning)
            .field("isInclude", &self.isInclude)
            .field(
                "errorHandler",
                &self.errorHandler.as_ref().map(|_| "<callback>"),
            )
            .finish()
    }
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
    pub fn new(toplevelObjects: Vec<Object>, isInclude: bool) -> Self {
        Self {
            content: toplevelObjects,
            isInclude,
            countAllVisits: false,
            dontFlattenContainers: Vec::new(),
            variableDeclarations: HashMap::new(),
            ..Default::default()
        }
    }

    // C# signature: protected override void PreProcessTopLevelObjects(List<Parsed.Object> topLevelContent)
    pub fn PreProcessTopLevelObjects(&mut self, topLevelContent: Vec<Object>) {
        let mut flowsFromOtherFiles = Vec::<Object>::new();
        let mut processed = Vec::<Object>::new();

        for obj in topLevelContent {
            if obj.kind == ObjectKind::Plain
                && obj.get_runtimeObject().is_none()
                && !obj.content.is_empty()
            {
                let mut included_content = Vec::<Object>::new();
                for sub_obj in obj.content.into_iter() {
                    if matches!(sub_obj.kind, ObjectKind::Knot | ObjectKind::Stitch) {
                        flowsFromOtherFiles.push(sub_obj);
                    } else {
                        included_content.push(sub_obj);
                    }
                }
                if !included_content.is_empty() {
                    processed.extend(included_content);
                    processed.push(Self::newline_object());
                }
                continue;
            }

            processed.push(obj);
        }

        processed.extend(flowsFromOtherFiles);
        self.content = processed;
    }

    // C# signature: public Runtime.Story ExportRuntime(ErrorHandler errorHandler = null)
    pub fn ExportRuntime(
        &mut self,
        errorHandler: Option<Rc<RefCell<ErrorHandler>>>,
    ) -> Option<RuntimeStory> {
        self.errorHandler = errorHandler;
        self.ResetError();

        let content = std::mem::take(&mut self.content);
        self.PreProcessTopLevelObjects(content);
        self.attach_parents();

        self.constants.clear();
        self.externals.clear();
        self.listDefs.clear();
        self.dontFlattenContainers.clear();

        let mut content = std::mem::take(&mut self.content);
        for obj in &mut content {
            obj.ResolveReferences(self);
        }
        self.content = content;

        if self.hadError {
            return None;
        }

        let mut rootContainer = Container::new();
        for obj in &self.content {
            if let Some(runtime_object) = obj.get_runtimeObject().cloned() {
                if runtime_object.get_hasValidName() {
                    rootContainer.AddContent(runtime_object);
                } else {
                    rootContainer.AddContentsOfContainer(runtime_object);
                }
            }
        }
        rootContainer.AddContent(ControlCommand::Done());

        let runtimeLists = self
            .listDefs
            .values_mut()
            .map(|list_def| list_def.get_runtimeListDefinition())
            .collect::<Vec<_>>();

        self.flatten_containers_in(&mut rootContainer);

        let mut runtimeStory = RuntimeStory::new(rootContainer, runtimeLists);
        runtimeStory.ResetState();
        Some(runtimeStory)
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
    pub fn Error(&mut self, message: String, _source: crate::stub::PortStub, isWarning: bool) {
        self.hadWarning = isWarning;
        self.hadError = !isWarning;
        if let Some(handler) = &self.errorHandler {
            let mut handler = handler.borrow_mut();
            handler(
                &message,
                if isWarning {
                    ErrorType::Warning
                } else {
                    ErrorType::Error
                },
            );
        }
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
    pub fn DontFlattenContainer(&mut self, container: Container) {
        if !self
            .dontFlattenContainers
            .iter()
            .any(|known| known == &container)
        {
            self.dontFlattenContainers.push(container);
        }
    }

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
                    let is_owning_variable_assignment = self.listDefs.values().any(|list_def| {
                        list_def
                            .variableAssignment
                            .as_ref()
                            .map(|assignment| assignment.get_variableName() == name)
                            .unwrap_or(false)
                    });

                    if !is_owning_variable_assignment {
                        self.Error(
                            format!("name '{}' has already been used", name),
                            Default::default(),
                            false,
                        );
                    }
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
                let is_owning_variable_assignment = self.listDefs.values().any(|list_def| {
                    list_def
                        .variableAssignment
                        .as_ref()
                        .map(|assignment| assignment.get_variableName() == name)
                        .unwrap_or(false)
                });

                if (self.constants.contains_key(&name)
                    || self.listDefs.contains_key(&name)
                    || self.externals.contains_key(&name)
                    || FunctionCall::IsBuiltIn(name.clone()))
                    && !is_owning_variable_assignment
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

    pub fn ContentWithNameAtLevel(
        &self,
        name: String,
        level: Option<FlowLevel>,
        deepSearch: bool,
    ) -> Option<Object> {
        for obj in &self.content {
            let matches_level = match level {
                Some(FlowLevel::Story) => obj.kind == ObjectKind::Story,
                Some(FlowLevel::Knot) => obj.kind == ObjectKind::Knot,
                Some(FlowLevel::Stitch) => obj.kind == ObjectKind::Stitch,
                Some(FlowLevel::WeavePoint) => obj.kind == ObjectKind::WeavePoint,
                None => true,
            };
            if matches_level
                && obj
                    .identifier
                    .as_ref()
                    .and_then(|identifier| identifier.name.as_deref())
                    == Some(name.as_str())
            {
                return Some(obj.clone());
            }

            if deepSearch {
                if let Some(found) = obj.FindAll(None).into_iter().find(|candidate| {
                    candidate
                        .identifier
                        .as_ref()
                        .and_then(|identifier| identifier.name.as_deref())
                        == Some(name.as_str())
                }) {
                    return Some(found);
                }
            }
        }

        None
    }

    pub fn register_constant(&mut self, name: String, expr: Expression) {
        self.constants.insert(name, expr);
    }

    pub fn register_list_definition(&mut self, list_def: ListDefinition) {
        if let Some(name) = list_def.get_name().map(|name| name.to_string()) {
            self.listDefs.insert(name, list_def);
        }
    }

    pub fn ResolveReferences(&mut self) {
        let mut content = std::mem::take(&mut self.content);
        for obj in &mut content {
            obj.ResolveReferences(self);
        }
        self.content = content;
    }

    pub fn TryAddNewVariableDeclaration(
        &mut self,
        varDecl: crate::ParsedHierarchy::VariableAssignment::VariableAssignment,
    ) {
        let varName = varDecl.get_variableName();
        if varName.is_empty() {
            return;
        }

        if self.variableDeclarations.contains_key(&varName) {
            self.Error(
                format!(
                    "found declaration variable '{}' that was already declared",
                    varName
                ),
                Default::default(),
                false,
            );
            return;
        }

        self.variableDeclarations.insert(varName, varDecl);
    }

    pub fn ResolveVariableWithName(
        &self,
        varName: String,
        _fromNode: crate::stub::PortStub,
    ) -> VariableResolveResult {
        if self.constants.contains_key(&varName) {
            return VariableResolveResult {
                found: true,
                isGlobal: true,
                isArgument: false,
                isTemporary: false,
                ownerFlow: Some("Story".to_string()),
            };
        }

        if self.listDefs.contains_key(&varName) {
            return VariableResolveResult {
                found: true,
                isGlobal: true,
                isArgument: false,
                isTemporary: false,
                ownerFlow: Some("Story".to_string()),
            };
        }

        if self.variableDeclarations.contains_key(&varName) {
            return VariableResolveResult {
                found: true,
                isGlobal: true,
                isArgument: false,
                isTemporary: false,
                ownerFlow: Some("Story".to_string()),
            };
        }

        VariableResolveResult::default()
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

    pub fn get_countAllVisits(&self) -> bool {
        self.countAllVisits
    }

    pub fn set_countAllVisits(&mut self, value: bool) {
        self.countAllVisits = value;
    }

    fn attach_parents(&mut self) {
        fn attach_object_parent(parent: &Object, child: &mut Object) {
            child.set_parent(Some(Box::new(parent.clone())));
            let parent_clone = child.clone();
            for nested in &mut child.content {
                attach_object_parent(&parent_clone, nested);
            }
        }

        let roots = self.content.clone();
        for child in &mut self.content {
            let parentless_root = Object::with_kind(ObjectKind::Story);
            let root_parent = if child.identifier.is_some() {
                let mut root = parentless_root.clone();
                root.content = roots.clone();
                root
            } else {
                parentless_root
            };
            attach_object_parent(&root_parent, child);
        }
    }

    fn flatten_containers_in(&self, container: &mut Container) {
        *container = self.flatten_container(container.clone());
    }

    fn flatten_container(&self, container: Container) -> Container {
        let mut flattened = Container::new();
        flattened.set_name(Some(container.get_name().to_string()).filter(|name| !name.is_empty()));
        flattened.set_countFlags(container.get_countFlags());
        flattened.set_debugMetadata(container.get_debugMetadata().cloned());

        for content in container.get_content().iter().cloned() {
            match content {
                ContentItem::Container(child) => {
                    let child_container = self.flatten_container(*child);
                    let should_flatten = !child_container.get_hasValidName()
                        && child_container.get_namedContent().is_empty()
                        && !self
                            .dontFlattenContainers
                            .iter()
                            .any(|known| known == &child_container);
                    if should_flatten {
                        flattened.AddContentsOfContainer(child_container);
                    } else {
                        flattened.AddContent(ContentItem::Container(Box::new(child_container)));
                    }
                }
                other => flattened.AddContent(other),
            }
        }

        if let Some(named_only_content) = container.get_namedOnlyContent() {
            let named_only_content = named_only_content
                .into_iter()
                .map(|(name, content)| (name, self.flatten_content_item(content)))
                .collect::<HashMap<_, _>>();
            flattened.set_namedOnlyContent(Some(named_only_content));
        }

        flattened
    }

    fn flatten_content_item(&self, content: ContentItem) -> ContentItem {
        match content {
            ContentItem::Container(container) => {
                ContentItem::Container(Box::new(self.flatten_container(*container)))
            }
            other => other,
        }
    }

    fn newline_object() -> Object {
        let mut obj = Object::with_kind(ObjectKind::Plain);
        let mut runtime = Container::new();
        runtime.AddContent(ink_runtime::Value::StringValue::new("\n".to_string()));
        obj.set_runtimeObject(Some(runtime));
        obj
    }
}
