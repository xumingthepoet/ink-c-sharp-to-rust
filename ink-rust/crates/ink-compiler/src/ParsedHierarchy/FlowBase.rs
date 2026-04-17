// Source: ink-c-sharp/compiler/ParsedHierarchy/FlowBase.cs

use crate::ParsedHierarchy::FlowLevel::FlowLevel;
use crate::ParsedHierarchy::Gather::Gather;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Object::{Object, ObjectKind};
use crate::ParsedHierarchy::Story::Story;
use crate::ParsedHierarchy::VariableAssignment::VariableAssignment;
use ink_runtime::Container::{Container, ContentItem};
use ink_runtime::ControlCommand::ControlCommand;
use ink_runtime::Divert::Divert as RuntimeDivert;
use ink_runtime::VariableAssignment::VariableAssignment as RuntimeVariableAssignment;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Argument {
    pub identifier: Option<Identifier>,
    pub isByReference: bool,
    pub isDivertTarget: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VariableResolveResult {
    pub found: bool,
    pub isGlobal: bool,
    pub isArgument: bool,
    pub isTemporary: bool,
    pub ownerFlow: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlowBase {
    pub base: Object,
    pub identifier: Option<Identifier>,
    pub arguments: Vec<Argument>,
    pub variableDeclarations: HashMap<String, VariableAssignment>,
    pub flow_level: FlowLevel,
    pub isFunction: bool,
    pub isIncludedStory: bool,
    parent_flow: Option<Box<FlowBase>>,
    startingSubFlowDivert: Option<RuntimeDivert>,
    startingSubFlowRuntime: Option<Container>,
    rootWeave: Option<Box<crate::ParsedHierarchy::Weave::Weave>>,
    subFlowsByName: HashMap<String, Box<FlowBase>>,
    firstChildFlow: Option<Object>,
}

impl FlowBase {
    // C# signature: public FlowBase (Identifier name = null, List<Parsed.Object> topLevelObjects = null, List<Argument> arguments = null, bool isFunction = false, bool isIncludedStory = false)
    pub fn new(
        name: Identifier,
        topLevelObjects: Vec<Object>,
        arguments: Vec<Argument>,
        isFunction: bool,
        isIncludedStory: bool,
    ) -> Self {
        let mut base = Object::with_kind(ObjectKind::FlowBase);
        base.set_identifier(Some(name.clone()));
        base.content = topLevelObjects;

        let mut flow = Self {
            base,
            identifier: Some(name),
            arguments,
            variableDeclarations: HashMap::new(),
            flow_level: FlowLevel::Story,
            isFunction,
            isIncludedStory,
            parent_flow: None,
            startingSubFlowDivert: None,
            startingSubFlowRuntime: None,
            rootWeave: None,
            subFlowsByName: HashMap::new(),
            firstChildFlow: None,
        };

        flow.PreProcessTopLevelObjects(flow.base.content.clone());
        let split_content = flow.SplitWeaveAndSubFlowContent(flow.base.content.clone(), false);
        flow.base.content = split_content;
        flow
    }

    pub fn set_flowLevel(&mut self, flow_level: FlowLevel) {
        self.flow_level = flow_level;
        self.base.set_kind(match flow_level {
            FlowLevel::Story => ObjectKind::Story,
            FlowLevel::Knot => ObjectKind::Knot,
            FlowLevel::Stitch => ObjectKind::Stitch,
            FlowLevel::WeavePoint => ObjectKind::FlowBase,
        });
    }

    pub fn set_parent_flow(&mut self, parent_flow: Option<Box<FlowBase>>) {
        self.parent_flow = parent_flow;
    }

    pub fn set_startingSubFlowDivert(&mut self, divert: Option<RuntimeDivert>) {
        self.startingSubFlowDivert = divert;
    }

    // C# signature: protected virtual void PreProcessTopLevelObjects(List<Parsed.Object> topLevelObjects)
    pub fn PreProcessTopLevelObjects(&mut self, _topLevelObjects: Vec<Object>) {}

    fn SplitWeaveAndSubFlowContent(
        &mut self,
        contentObjs: Vec<Object>,
        isRootStory: bool,
    ) -> Vec<Object> {
        let mut weaveObjs = Vec::new();
        let mut subFlowObjs = Vec::new();

        self.subFlowsByName.clear();
        self.firstChildFlow = None;

        for obj in contentObjs {
            let is_sub_flow = matches!(obj.kind, ObjectKind::Knot | ObjectKind::Stitch);
            if is_sub_flow {
                if self.firstChildFlow.is_none() {
                    self.firstChildFlow = Some(obj.clone());
                }

                if let Some(name) = obj
                    .identifier
                    .as_ref()
                    .and_then(|identifier| identifier.name.clone())
                {
                    self.subFlowsByName
                        .insert(name, Box::new(FlowBase::from_object(&obj)));
                }
                subFlowObjs.push(obj);
            } else {
                weaveObjs.push(obj);
            }
        }

        if isRootStory {
            let mut gather = Gather::new(
                Identifier {
                    name: None,
                    debugMetadata: None,
                },
                1,
            );
            let mut gather_obj = Object::with_kind(ObjectKind::WeavePoint);
            gather_obj.set_indentationDepth(gather.get_indentationDepth());
            gather_obj.set_identifier(gather.get_identifier().cloned());
            if let ContentItem::Container(container) = gather.GenerateRuntimeObject() {
                gather_obj.set_runtimeObject(Some(*container));
            }
            weaveObjs.push(gather_obj);

            let mut done_obj = Object::with_kind(ObjectKind::Plain);
            let mut done_container = Container::new();
            done_container.AddContent(ControlCommand::Done());
            done_obj.set_runtimeObject(Some(done_container));
            weaveObjs.push(done_obj);
        }

        if !weaveObjs.is_empty() {
            self.rootWeave = Some(Box::new(crate::ParsedHierarchy::Weave::Weave::new(
                weaveObjs.clone(),
                0,
            )));
        }

        let mut final_content = weaveObjs;
        final_content.extend(subFlowObjs);
        final_content
    }

    fn from_object(object: &Object) -> Self {
        let mut base = Object::with_kind(object.kind.clone());
        base.identifier = object.identifier.clone();
        base.indentationDepth = object.indentationDepth;
        base.isFunction = object.isFunction;
        base.content = object.content.clone();
        base.set_runtimeObject(object.get_runtimeObject().cloned());
        base.set_debugMetadata(object.get_debugMetadata().cloned());

        Self {
            base,
            identifier: object.identifier.clone(),
            arguments: Vec::new(),
            variableDeclarations: HashMap::new(),
            flow_level: match object.kind {
                ObjectKind::Story => FlowLevel::Story,
                ObjectKind::Knot => FlowLevel::Knot,
                ObjectKind::Stitch => FlowLevel::Stitch,
                _ => FlowLevel::WeavePoint,
            },
            isFunction: object.isFunction,
            isIncludedStory: false,
            parent_flow: None,
            startingSubFlowDivert: None,
            startingSubFlowRuntime: None,
            rootWeave: None,
            subFlowsByName: HashMap::new(),
            firstChildFlow: None,
        }
    }

    // C# signature: public VariableResolveResult ResolveVariableWithName(string varName, Parsed.Object fromNode)
    pub fn ResolveVariableWithName(
        &self,
        varName: String,
        fromNode: &Object,
    ) -> VariableResolveResult {
        let mut result = VariableResolveResult::default();

        let ownerFlow = fromNode
            .ClosestFlowBase()
            .unwrap_or_else(|| self.base.clone());

        if self.arguments.iter().any(|arg| {
            arg.identifier.as_ref().and_then(|id| id.name.as_deref()) == Some(varName.as_str())
        }) {
            result.found = true;
            result.isArgument = true;
            result.ownerFlow = ownerFlow.identifier.as_ref().and_then(|id| id.name.clone());
            return result;
        }

        if self.variableDeclarations.contains_key(&varName) {
            result.found = true;
            result.isTemporary = true;
            result.ownerFlow = self
                .identifier
                .as_ref()
                .and_then(|identifier| identifier.name.clone());
            return result;
        }

        if self.flow_level == FlowLevel::Story {
            result.found = true;
            result.isGlobal = true;
            result.ownerFlow = self
                .identifier
                .as_ref()
                .and_then(|identifier| identifier.name.clone());
            return result;
        }

        if let Some(parent) = &self.parent_flow {
            return parent.ResolveVariableWithName(varName, fromNode);
        }

        result
    }

    // C# signature: public void TryAddNewVariableDeclaration(VariableAssignment varDecl)
    pub fn TryAddNewVariableDeclaration(&mut self, varDecl: VariableAssignment) {
        let varName = varDecl.get_variableName();
        if varName.is_empty() {
            return;
        }

        if self.variableDeclarations.contains_key(&varName) {
            self.base.Error(
                format!(
                    "found declaration variable '{}' that was already declared",
                    varName
                ),
                None,
                false,
            );
            return;
        }

        self.variableDeclarations.insert(varName, varDecl);
    }

    // C# signature: public void ResolveWeavePointNaming ()
    pub fn ResolveWeavePointNaming(&mut self) {
        if let Some(root_weave) = self.rootWeave.as_mut() {
            root_weave.ResolveWeavePointNaming();
        } else {
            let mut named = HashMap::<String, usize>::new();
            let mut duplicate: Option<(String, usize, usize)> = None;
            for (idx, obj) in self.base.content.iter().enumerate() {
                if obj.kind == ObjectKind::WeavePoint {
                    if let Some(identifier) = &obj.identifier {
                        if let Some(name) = &identifier.name {
                            if let Some(existing_idx) = named.insert(name.clone(), idx) {
                                duplicate = Some((name.clone(), existing_idx, idx));
                            }
                        }
                    }
                }
            }

            if let Some((name, existing_idx, _idx)) = duplicate {
                let existing = self.base.content[existing_idx].clone();
                self.base.Error(
                    format!(
                        "A weave point with the same label name '{}' already exists in this context",
                        name
                    ),
                    Some(existing),
                    false,
                );
            }
        }

        for sub_flow in self.subFlowsByName.values_mut() {
            sub_flow.ResolveWeavePointNaming();
        }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> Container {
        if self.isFunction {
            self.CheckForDisallowedFunctionFlowControl();
        }

        let mut container = Container::new();
        container.set_name(
            self.identifier
                .as_ref()
                .and_then(|identifier| identifier.name.clone()),
        );

        if self.flow_level == FlowLevel::Story || self.isIncludedStory {
            container.set_countFlags(1);
        }

        self.GenerateArgumentVariableAssignments(&mut container);

        if let Some(root_weave) = self.rootWeave.as_mut() {
            if let ContentItem::Container(root_container) = root_weave.GenerateRuntimeObject() {
                container.AddContent(*root_container);
            }
        }

        let mut contentIdx = 0;
        let has_parameters = self.get_hasParameters();
        for obj in &mut self.base.content {
            if matches!(obj.kind, ObjectKind::Knot | ObjectKind::Stitch) {
                if let Some(runtime_object) = obj.get_runtimeObject().cloned() {
                    if contentIdx == 0 && !has_parameters && self.flow_level == FlowLevel::Knot {
                        self.startingSubFlowDivert = Some(RuntimeDivert::new());
                        if let Some(divert) = self.startingSubFlowDivert.as_ref() {
                            container.AddContent(divert.clone());
                        }
                        self.startingSubFlowRuntime = Some(runtime_object.clone());
                    }

                    container.AddToNamedContentOnly(runtime_object);
                }
            }
            contentIdx += 1;
        }

        container
    }

    fn GenerateArgumentVariableAssignments(&self, container: &mut Container) {
        if self.arguments.is_empty() {
            return;
        }

        for argument in self.arguments.iter().rev() {
            let param_name = argument
                .identifier
                .as_ref()
                .and_then(|identifier| identifier.name.clone())
                .unwrap_or_default();
            container.AddContent(RuntimeVariableAssignment::new(param_name, true));
        }
    }

    // C# signature: public Parsed.Object ContentWithNameAtLevel(string name, FlowLevel? level = null, bool deepSearch = false)
    pub fn ContentWithNameAtLevel(
        &self,
        name: String,
        level: Option<FlowLevel>,
        deepSearch: bool,
    ) -> Option<Object> {
        if level.is_none() || level == Some(self.flow_level) {
            if self
                .identifier
                .as_ref()
                .and_then(|identifier| identifier.name.as_deref())
                == Some(name.as_str())
            {
                return Some(self.base.clone());
            }
        }

        if level == Some(FlowLevel::WeavePoint) || level.is_none() {
            if let Some(root_weave) = &self.rootWeave {
                if let Some(weave_point) = root_weave.WeavePointNamed(name.clone()) {
                    return Some(weave_point);
                }
            } else {
                for obj in &self.base.content {
                    if obj.kind == ObjectKind::WeavePoint
                        && obj
                            .identifier
                            .as_ref()
                            .and_then(|identifier| identifier.name.as_deref())
                            == Some(name.as_str())
                    {
                        return Some(obj.clone());
                    }
                }
            }

            if level == Some(FlowLevel::WeavePoint) {
                return if deepSearch {
                    self.DeepSearchForAnyLevelContent(name)
                } else {
                    None
                };
            }
        }

        if let Some(sub_flow) = self.subFlowsByName.get(&name) {
            if level.is_none() || level == Some(sub_flow.flow_level) {
                return Some(sub_flow.base.clone());
            }
        }

        if let Some(level) = level {
            if level != self.flow_level
                && Self::flow_level_rank(level) < Self::flow_level_rank(self.flow_level)
            {
                return None;
            }
        }

        if deepSearch {
            self.DeepSearchForAnyLevelContent(name)
        } else {
            None
        }
    }

    fn DeepSearchForAnyLevelContent(&self, name: String) -> Option<Object> {
        if let Some(weaveResultSelf) =
            self.ContentWithNameAtLevel(name.clone(), Some(FlowLevel::WeavePoint), false)
        {
            return Some(weaveResultSelf);
        }

        for sub_flow in self.subFlowsByName.values() {
            if sub_flow
                .base
                .identifier
                .as_ref()
                .and_then(|identifier| identifier.name.as_deref())
                == Some(name.as_str())
            {
                return Some(sub_flow.base.clone());
            }

            if let Some(found) = sub_flow.base.FindAll(None).into_iter().find(|candidate| {
                candidate
                    .identifier
                    .as_ref()
                    .and_then(|identifier| identifier.name.as_deref())
                    == Some(name.as_str())
            }) {
                return Some(found);
            }
        }

        None
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        if let Some(divert) = self.startingSubFlowDivert.as_mut() {
            if let Some(starting_runtime) = self.startingSubFlowRuntime.as_ref() {
                divert.set_targetPathString(Some(starting_runtime.get_path().ToString()));
            }
        }

        if let Some(root_weave) = self.rootWeave.as_mut() {
            root_weave.ResolveReferences(context);
        }

        self.base.ResolveReferences(context);

        for sub_flow in self.subFlowsByName.values_mut() {
            sub_flow.ResolveReferences(context);
        }

        if let Some(identifier) = &self.identifier {
            if identifier
                .name
                .as_ref()
                .map(|name| !name.is_empty())
                .unwrap_or(false)
            {
                context.CheckForNamingCollisions(
                    Default::default(),
                    identifier.clone(),
                    crate::ParsedHierarchy::Story::SymbolType::SubFlowAndWeave,
                    String::new(),
                );
            }
        }
    }

    pub fn ToString(&self) -> String {
        format!(
            "{} '{}'",
            self.get_typeName(),
            self.identifier
                .as_ref()
                .and_then(|id| id.name.as_deref())
                .unwrap_or("")
        )
    }

    pub fn get_name(&self) -> Option<&str> {
        self.identifier
            .as_ref()
            .and_then(|identifier| identifier.name.as_deref())
    }

    pub fn get_identifier(&self) -> Option<&Identifier> {
        self.identifier.as_ref()
    }

    pub fn get_arguments(&self) -> &[Argument] {
        &self.arguments
    }

    pub fn get_hasParameters(&self) -> bool {
        !self.arguments.is_empty()
    }

    pub fn get_flowLevel(&self) -> FlowLevel {
        self.flow_level
    }

    pub fn get_isFunction(&self) -> bool {
        self.isFunction
    }

    pub fn get_typeName(&self) -> String {
        if self.isFunction {
            "Function".to_string()
        } else {
            match self.flow_level {
                FlowLevel::Story => "Story",
                FlowLevel::Knot => "Knot",
                FlowLevel::Stitch => "Stitch",
                FlowLevel::WeavePoint => "Weave point",
            }
            .to_string()
        }
    }

    fn CheckForDisallowedFunctionFlowControl(&self) {
        let _ = self.flow_level;
    }

    fn flow_level_rank(level: FlowLevel) -> i32 {
        match level {
            FlowLevel::Story => 0,
            FlowLevel::Knot => 1,
            FlowLevel::Stitch => 2,
            FlowLevel::WeavePoint => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Argument, FlowBase, VariableResolveResult};
    use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
    use crate::ParsedHierarchy::FlowLevel::FlowLevel;
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::Number::{Number, NumberValue};
    use crate::ParsedHierarchy::Object::{Object, ObjectKind};
    use crate::ParsedHierarchy::VariableAssignment::VariableAssignment;

    #[test]
    fn resolves_arguments_and_temporaries_in_current_flow() {
        let mut flow = FlowBase::new(
            Identifier {
                name: Some("story".to_string()),
                debugMetadata: None,
            },
            vec![],
            vec![Argument {
                identifier: Some(Identifier {
                    name: Some("x".to_string()),
                    debugMetadata: None,
                }),
                isByReference: false,
                isDivertTarget: false,
            }],
            false,
            false,
        );
        flow.set_flowLevel(FlowLevel::Story);
        flow.TryAddNewVariableDeclaration(VariableAssignment::new(
            Identifier {
                name: Some("temp".to_string()),
                debugMetadata: None,
            },
            Expression::from_kind(ExpressionKind::Number(Number::new(NumberValue::Int(1)))),
        ));

        let result =
            flow.ResolveVariableWithName("x".to_string(), &Object::with_kind(ObjectKind::Plain));
        assert_eq!(
            result,
            VariableResolveResult {
                found: true,
                isGlobal: false,
                isArgument: true,
                isTemporary: false,
                ownerFlow: Some("story".to_string()),
            }
        );

        let temp =
            flow.ResolveVariableWithName("temp".to_string(), &Object::with_kind(ObjectKind::Plain));
        assert!(temp.found);
        assert!(temp.isTemporary);
    }
}
