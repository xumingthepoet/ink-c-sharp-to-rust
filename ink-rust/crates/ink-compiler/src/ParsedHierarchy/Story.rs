// Source: ink-c-sharp/compiler/ParsedHierarchy/Story.cs

use crate::ParsedHierarchy::ConstantDeclaration::ConstantDeclaration;
use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::ExternalDeclaration::ExternalDeclaration;
use crate::ParsedHierarchy::FlowBase::VariableResolveResult;
use crate::ParsedHierarchy::FlowLevel::FlowLevel;
use crate::ParsedHierarchy::FunctionCall::FunctionCall;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::ListDefinition::{ListDefinition, ListElementDefinition};
use crate::ParsedHierarchy::Object::{Object, ObjectKind, ObjectPayload};
use ink_runtime::Container::{Container, ContentItem};
use ink_runtime::ControlCommand::ControlCommand;
use ink_runtime::Error::{ErrorHandler, ErrorType};
use ink_runtime::Story::Story as RuntimeStory;
use ink_runtime::VariableAssignment::VariableAssignment as RuntimeVariableAssignment;
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
            if let Some(ObjectPayload::IncludedFile(_)) = obj.payload.as_ref() {
                let mut included_content = Vec::<Object>::new();
                if let Some(ObjectPayload::IncludedFile(included_file)) = obj.payload.as_ref() {
                    if let Some(sub_story) = included_file.get_includedStory() {
                        for sub_obj in sub_story.content.iter().cloned() {
                            if matches!(sub_obj.kind, ObjectKind::Knot | ObjectKind::Stitch) {
                                flowsFromOtherFiles.push(sub_obj);
                            } else {
                                included_content.push(sub_obj);
                            }
                        }

                        processed.extend(included_content);
                        processed.push(Self::newline_object());
                    }
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
        if std::env::var_os("INK_DEBUG_STORY_EXPORT").is_some() {
            eprintln!("export start self.content.len()={}", self.content.len());
        }
        self.errorHandler = errorHandler;
        self.ResetError();

        let content = std::mem::take(&mut self.content);
        self.PreProcessTopLevelObjects(content);
        self.attach_parents();

        self.constants.clear();
        self.externals.clear();
        self.listDefs.clear();
        self.dontFlattenContainers.clear();
        self.variableDeclarations.clear();
        self.CollectDeclarations();

        let mut content = std::mem::take(&mut self.content);
        self.content = content.clone();
        for obj in &mut content {
            obj.ResolveReferences(self);
        }

        if self.hadError {
            self.content = content;
            return None;
        }

        if std::env::var_os("INK_DEBUG_STORY_EXPORT").is_some() {
            eprintln!(
                "export top-level kinds={:?}",
                content
                    .iter()
                    .map(|obj| {
                        format!(
                            "{:?}:{:?}",
                            obj.kind,
                            obj.payload.as_ref().map(|_| "payload")
                        )
                    })
                    .collect::<Vec<_>>()
            );
        }

        let mut rootContainer = Container::new();
        for obj in &mut content {
            if let Some(runtime_object) = obj.EnsureRuntimeObject() {
                if matches!(obj.kind, ObjectKind::Knot | ObjectKind::Stitch) {
                    rootContainer.AddToNamedContentOnly(runtime_object);
                } else if runtime_object.get_hasValidName() {
                    rootContainer.AddContent(runtime_object);
                } else {
                    rootContainer.AddContentsOfContainer(runtime_object);
                }
            }
        }

        let mut runtimeLists = Vec::new();
        let mut variableInitialisation = Container::new();
        variableInitialisation.AddContent(ControlCommand::EvalStart());

        for (varName, varDecl) in self.variableDeclarations.iter_mut() {
            if !varDecl.get_isGlobalDeclaration() {
                continue;
            }

            if let Some(listDefinition) = varDecl.get_listDefinition().cloned() {
                let mut listDefinition = listDefinition;
                self.listDefs
                    .insert(varName.clone(), listDefinition.clone());
                variableInitialisation.AddContent(listDefinition.GenerateRuntimeObject());
                runtimeLists.push(listDefinition.get_runtimeListDefinition());
            } else if let Some(expression) = varDecl.get_expression() {
                expression.GenerateIntoContainer(&mut variableInitialisation);
            }

            let mut runtimeVarAss = RuntimeVariableAssignment::new(varName.clone(), true);
            runtimeVarAss.set_isGlobal(true);
            variableInitialisation.AddContent(runtimeVarAss);
        }

        variableInitialisation.AddContent(ControlCommand::EvalEnd());
        variableInitialisation.AddContent(ControlCommand::End());

        if !self.variableDeclarations.is_empty() {
            variableInitialisation.set_name(Some("global decl".to_string()));
            rootContainer.AddToNamedContentOnly(variableInitialisation);
        }

        for list_def in self.listDefs.values_mut() {
            let runtime_list = list_def.get_runtimeListDefinition();
            if !runtimeLists.iter().any(|known| known == &runtime_list) {
                runtimeLists.push(runtime_list);
            }
        }

        rootContainer.AddContent(ControlCommand::Done());

        self.flatten_containers_in(&mut rootContainer);
        self.content = content;
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
        source: Object,
    ) -> Option<ListElementDefinition> {
        if !listName.is_empty() {
            if let Some(list) = self.listDefs.get(&listName) {
                let mut list = list.clone();
                return list.ItemNamed(itemName).cloned();
            }

            return None;
        }

        let mut found_item: Option<ListElementDefinition> = None;
        let mut found_list_name: Option<String> = None;

        for (_named_list, list) in &self.listDefs {
            let mut list = list.clone();
            let list_name = list.get_name().unwrap_or("").to_string();
            if let Some(item) = list.ItemNamed(itemName.clone()) {
                if found_item.is_some() {
                    self.Error(
                        format!(
                            "Ambiguous item name '{}' found in multiple sets, including {} and {}",
                            itemName,
                            found_list_name.clone().unwrap_or_default(),
                            list_name
                        ),
                        source.clone(),
                        false,
                    );
                    return None;
                }

                found_list_name = Some(list_name);
                found_item = Some(item.clone());
            }
        }

        found_item
    }

    // C# signature: public override void Error(string message, Parsed.Object source, bool isWarning)
    pub fn Error(&mut self, message: String, source: Object, isWarning: bool) {
        let mut formatted = String::new();
        if matches!(
            source.payload.as_ref(),
            Some(ObjectPayload::AuthorWarning(_))
        ) {
            formatted.push_str("TODO: ");
        } else if isWarning {
            formatted.push_str("WARNING: ");
        } else {
            formatted.push_str("ERROR: ");
        }

        if let Some(debug_metadata) = source.get_debugMetadata() {
            if debug_metadata.startLineNumber >= 1 {
                if let Some(file_name) = &debug_metadata.fileName {
                    formatted.push_str(&format!("'{}' ", file_name));
                }
                formatted.push_str(&format!("line {}: ", debug_metadata.startLineNumber));
            }
        }

        formatted.push_str(&message);

        self.hadWarning = isWarning;
        self.hadError = !isWarning;
        if let Some(handler) = &self.errorHandler {
            let mut handler = handler.borrow_mut();
            handler(
                &formatted,
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
        _obj: Object,
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

    pub fn CollectDeclarations(&mut self) {
        let content = self.content.clone();
        for obj in &content {
            self.CollectDeclarationsFromObject(obj);
        }
    }

    fn CollectDeclarationsFromObject(&mut self, obj: &Object) {
        match obj.payload.as_ref() {
            Some(ObjectPayload::ConstantDeclaration(declaration)) => {
                if let (Some(name), Some(expression)) =
                    (declaration.get_constantName(), declaration.get_expression())
                {
                    if let Some(existing) = self.constants.get(name) {
                        if existing != expression {
                            self.Error(
                                format!(
                                    "CONST '{}' has been redefined with a different value.",
                                    name
                                ),
                                obj.clone(),
                                false,
                            );
                        }
                    }
                    self.constants.insert(name.to_string(), expression.clone());
                }
            }
            Some(ObjectPayload::ExternalDeclaration(declaration)) => {
                self.AddExternal((**declaration).clone());
            }
            Some(ObjectPayload::VariableAssignment(assignment)) => {
                if assignment.get_isDeclaration() {
                    self.TryAddNewVariableDeclaration((**assignment).clone());
                    if let Some(list_definition) = assignment.get_listDefinition().cloned() {
                        if let Some(name) = list_definition.get_name().map(|name| name.to_string())
                        {
                            self.listDefs.insert(name, list_definition);
                        }
                    }
                }
            }
            _ => {}
        }

        for child in &obj.content {
            self.CollectDeclarationsFromObject(child);
        }
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
        _fromNode: Object,
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
            let content = Self::detach_content_parent(content);
            match content {
                ContentItem::Container(child) => {
                    let child_container = self.flatten_container(child.as_ref().clone());
                    let should_flatten = !child_container.get_hasValidName()
                        && child_container.get_namedContent().is_empty()
                        && !self
                            .dontFlattenContainers
                            .iter()
                            .any(|known| known == &child_container);
                    if should_flatten {
                        flattened.AddContentsOfContainer(child_container);
                    } else {
                        flattened.AddContent(ContentItem::Container(Rc::new(child_container)));
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
        match Self::detach_content_parent(content) {
            ContentItem::Container(container) => {
                ContentItem::Container(Rc::new(self.flatten_container(container.as_ref().clone())))
            }
            other => other,
        }
    }

    fn detach_content_parent(mut content: ContentItem) -> ContentItem {
        match &mut content {
            ContentItem::ChoicePoint(choice_point) => choice_point.set_parent(None),
            ContentItem::Divert(divert) => divert.set_parent(None),
            ContentItem::VariableReference(variable_reference) => {
                variable_reference.set_parent(None)
            }
            _ => {}
        }
        content
    }

    fn newline_object() -> Object {
        let mut obj = Object::with_kind(ObjectKind::Plain);
        let mut runtime = Container::new();
        runtime.AddContent(ink_runtime::Value::StringValue::new("\n".to_string()));
        obj.set_runtimeObject(Some(runtime));
        obj
    }
}

#[cfg(test)]
mod tests {
    use super::Story;
    use crate::ParsedHierarchy::ConstantDeclaration::ConstantDeclaration;
    use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
    use crate::ParsedHierarchy::ExternalDeclaration::ExternalDeclaration;
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::IncludedFile::IncludedFile;
    use crate::ParsedHierarchy::ListDefinition::{ListDefinition, ListElementDefinition};
    use crate::ParsedHierarchy::Number::{Number, NumberValue};
    use crate::ParsedHierarchy::Object::{Object, ObjectKind};
    use crate::ParsedHierarchy::VariableAssignment::VariableAssignment;
    use ink_runtime::Container::ContentItem;
    use ink_runtime::DebugMetadata::DebugMetadata;
    use ink_runtime::Error::ErrorType;
    use ink_runtime::Value::Value;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn content_with_name_finds_top_level_matches() {
        let mut obj = Object::with_kind(ObjectKind::Plain);
        obj.set_identifier(Some(Identifier {
            name: Some("label".to_string()),
            debugMetadata: None,
        }));
        let story = Story::new(vec![obj], false);

        assert!(story
            .ContentWithNameAtLevel("label".to_string(), None, false)
            .is_some());
    }

    #[test]
    fn collect_declarations_preserves_typed_payloads() {
        let const_decl = ConstantDeclaration::new(
            Identifier {
                name: Some("MAX".to_string()),
                debugMetadata: None,
            },
            Some(Expression::from_kind(ExpressionKind::Number(Number::new(
                NumberValue::Int(10),
            )))),
        );

        let mut var_decl = VariableAssignment::new(
            Identifier {
                name: Some("score".to_string()),
                debugMetadata: None,
            },
            Expression::from_kind(ExpressionKind::Number(Number::new(NumberValue::Int(0)))),
        );
        var_decl.set_isGlobalDeclaration(true);

        let mut list_definition = ListDefinition::new(vec![ListElementDefinition::new(
            Identifier {
                name: Some("apple".to_string()),
                debugMetadata: None,
            },
            true,
            None,
        )]);
        list_definition.identifier = Some(Identifier {
            name: Some("food".to_string()),
            debugMetadata: None,
        });
        let list_decl = VariableAssignment::new_overload_2(
            Identifier {
                name: Some("food".to_string()),
                debugMetadata: None,
            },
            list_definition,
        );

        let external = ExternalDeclaration::new(
            Identifier {
                name: Some("host_func".to_string()),
                debugMetadata: None,
            },
            vec![],
        );

        let mut story = Story::new(
            vec![
                Object::from_constant_declaration(const_decl),
                Object::from_variable_assignment(var_decl),
                Object::from_variable_assignment(list_decl),
                Object::from_external_declaration(external),
            ],
            false,
        );

        story.CollectDeclarations();

        assert!(story.constants.contains_key("MAX"));
        assert!(story.variableDeclarations.contains_key("score"));
        assert!(story.variableDeclarations.contains_key("food"));
        assert!(story.ResolveList("food".to_string()).is_some());
        assert!(story.IsExternal("host_func".to_string()));
    }

    #[test]
    fn export_runtime_hoists_global_variable_initialisation() {
        let mut var_decl = VariableAssignment::new(
            Identifier {
                name: Some("score".to_string()),
                debugMetadata: None,
            },
            Expression::from_kind(ExpressionKind::Number(Number::new(NumberValue::Int(3)))),
        );
        var_decl.set_isGlobalDeclaration(true);

        let mut story = Story::new(vec![Object::from_variable_assignment(var_decl)], false);
        let mut runtime_story = story.ExportRuntime(None).expect("runtime story");
        let root = runtime_story.get_mainContentContainer();

        assert!(root.get_namedContent().contains_key("global decl"));
    }

    #[test]
    fn resolve_list_item_reports_ambiguity_and_singles_search_all_lists() {
        let mut first_list = ListDefinition::new(vec![ListElementDefinition::new(
            Identifier {
                name: Some("apple".to_string()),
                debugMetadata: None,
            },
            true,
            None,
        )]);
        first_list.identifier = Some(Identifier {
            name: Some("food".to_string()),
            debugMetadata: None,
        });

        let mut second_list = ListDefinition::new(vec![ListElementDefinition::new(
            Identifier {
                name: Some("apple".to_string()),
                debugMetadata: None,
            },
            true,
            None,
        )]);
        second_list.identifier = Some(Identifier {
            name: Some("drinks".to_string()),
            debugMetadata: None,
        });

        let mut story = Story::new(vec![], false);
        story.register_list_definition(first_list.clone());
        story.register_list_definition(second_list);

        let captured = Rc::new(RefCell::new(Vec::<(String, ErrorType)>::new()));
        let handler = {
            let captured = captured.clone();
            Rc::new(RefCell::new(
                Box::new(move |message: &str, error_type: ErrorType| {
                    captured
                        .borrow_mut()
                        .push((message.to_string(), error_type));
                }) as ink_runtime::Error::ErrorHandler,
            ))
        };
        story.errorHandler = Some(handler);

        assert!(story
            .ResolveListItem(
                "".to_string(),
                "apple".to_string(),
                Object::with_kind(ObjectKind::Plain),
            )
            .is_none());
        assert!(captured
            .borrow()
            .first()
            .map(|(message, error_type)| {
                message.contains("Ambiguous item name 'apple'") && *error_type == ErrorType::Error
            })
            .unwrap_or(false));
    }

    #[test]
    fn flow_only_includes_leave_a_newline_at_the_include_site() {
        let mut included_story = Story::default();
        included_story
            .content
            .push(Object::with_kind(ObjectKind::Knot));

        let include = Object::from_included_file(IncludedFile::new(Some(included_story)));
        let mut story = Story::new(vec![include], false);

        let content = story.content.clone();
        story.PreProcessTopLevelObjects(content);

        assert_eq!(story.content.len(), 2);
        assert_eq!(story.content[1].kind, ObjectKind::Knot);

        let newline_container = story.content[0]
            .get_runtimeObject()
            .expect("newline object should carry a runtime container");
        match newline_container.get_content().first() {
            Some(ContentItem::Value(Value::String(text))) => assert_eq!(text.value, "\n"),
            other => panic!("expected newline string content, got {:?}", other),
        }
    }

    #[test]
    fn error_prefixes_source_location() {
        let mut story = Story::new(vec![], false);
        let captured = Rc::new(RefCell::new(Vec::<(String, ErrorType)>::new()));
        let handler = {
            let captured = captured.clone();
            Rc::new(RefCell::new(
                Box::new(move |message: &str, error_type: ErrorType| {
                    captured
                        .borrow_mut()
                        .push((message.to_string(), error_type));
                }) as ink_runtime::Error::ErrorHandler,
            ))
        };
        story.errorHandler = Some(handler);

        let mut source = Object::with_kind(ObjectKind::Plain);
        source.set_debugMetadata(Some(DebugMetadata {
            startLineNumber: 12,
            endLineNumber: 12,
            startCharacterNumber: 0,
            endCharacterNumber: 1,
            fileName: Some("story.ink".to_string()),
            sourceName: None,
        }));

        story.Error("boom".to_string(), source, false);

        let (message, error_type) = captured.borrow().first().cloned().unwrap();
        assert_eq!(error_type, ErrorType::Error);
        assert!(message.starts_with("ERROR: 'story.ink' line 12: boom"));
    }
}
