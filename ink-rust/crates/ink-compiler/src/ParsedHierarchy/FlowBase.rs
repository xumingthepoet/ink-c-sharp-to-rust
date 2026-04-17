// Source: ink-c-sharp/compiler/ParsedHierarchy/FlowBase.cs

use crate::ParsedHierarchy::FlowLevel::FlowLevel;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Object::{Object, ObjectKind};
use crate::ParsedHierarchy::Story::Story;
use crate::ParsedHierarchy::VariableAssignment::VariableAssignment;
use ink_runtime::Container::{Container, ContentItem};
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

        Self {
            base,
            identifier: Some(name),
            arguments,
            variableDeclarations: HashMap::new(),
            flow_level: FlowLevel::Story,
            isFunction,
            isIncludedStory,
            parent_flow: None,
            startingSubFlowDivert: None,
        }
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

        if self
            .parent_flow
            .as_ref()
            .map(|parent| parent.flow_level == FlowLevel::Story)
            .unwrap_or(self.flow_level == FlowLevel::Story)
        {
            if self.base.get_ancestry().is_empty() {
                container.set_countFlags(1);
            }
        }

        self.GenerateArgumentVariableAssignments(&mut container);

        for obj in &mut self.base.content {
            if let Some(runtime_object) = obj.get_runtimeObject().cloned() {
                container.AddContent(runtime_object);
            }
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

            if level == Some(FlowLevel::WeavePoint) {
                return if deepSearch {
                    self.DeepSearchForAnyLevelContent(name)
                } else {
                    None
                };
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

        for obj in &self.base.content {
            if obj
                .identifier
                .as_ref()
                .and_then(|identifier| identifier.name.as_deref())
                == Some(name.as_str())
            {
                return Some(obj.clone());
            }

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

        None
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        if let Some(divert) = self.startingSubFlowDivert.as_mut() {
            if let Some(first_child) = self.base.content.first() {
                divert.set_targetPathString(Some(first_child.get_runtimePath().ToString()));
            }
        }

        self.base.ResolveReferences(context);

        if let Some(root_identifier) = &self.identifier {
            let _ = root_identifier;
            let _ = context;
        }

        for _arg in &self.arguments {
            let _ = _arg;
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
