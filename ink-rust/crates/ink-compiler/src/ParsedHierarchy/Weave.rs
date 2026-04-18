// Source: ink-c-sharp/compiler/ParsedHierarchy/Weave.cs

use crate::ParsedHierarchy::Choice::Choice;
use crate::ParsedHierarchy::ContentList::ContentListItem;
use crate::ParsedHierarchy::FlowBase::FlowBase;
use crate::ParsedHierarchy::Gather::Gather;
use crate::ParsedHierarchy::Object::{Object, ObjectKind, ObjectPayload};
use crate::ParsedHierarchy::Story::Story;
use ink_runtime::Container::{Container, ContentItem};
use ink_runtime::ControlCommand::ControlCommand;
use ink_runtime::Divert::Divert as RuntimeDivert;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GatherPointToResolve {
    pub divert: RuntimeDivert,
    pub targetRuntimeObj: Container,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Weave {
    pub base: Object,
    looseEnds: Vec<Object>,
    gatherPointsToResolve: Vec<GatherPointToResolve>,
    rootContainer: Option<Container>,
    currentContainer: Option<Container>,
    baseIndentIndex: i32,
    previousWeavePoint: Option<Object>,
    addContentToPreviousWeavePoint: bool,
    hasSeenChoiceInSection: bool,
    unnamedGatherCount: usize,
    choiceCount: usize,
    namedWeavePoints: HashMap<String, Object>,
}

impl Weave {
    // C# signature: public Weave(List<Parsed.Object> cont, int indentIndex=-1)
    pub fn new(cont: Vec<Object>, indentIndex: i32) -> Self {
        let mut base = Object::with_kind(ObjectKind::Weave);
        base.content = cont;

        let baseIndentIndex = if indentIndex == -1 {
            Self::determine_base_indentation_from_content(&base.content)
        } else {
            indentIndex
        };

        let mut weave = Self {
            base,
            baseIndentIndex,
            ..Default::default()
        };

        weave.ResolveWeavePointNaming();
        weave.ConstructWeaveHierarchyFromIndentation();
        weave
    }

    // C# signature: public void ResolveWeavePointNaming ()
    pub fn ResolveWeavePointNaming(&mut self) {
        self.namedWeavePoints.clear();

        let weave_points: Vec<Object> = self
            .base
            .FindAll(None)
            .into_iter()
            .filter(|obj| obj.kind == ObjectKind::WeavePoint)
            .collect();

        for obj in weave_points {
            let Some(name) = obj
                .identifier
                .as_ref()
                .and_then(|identifier| identifier.name.clone())
            else {
                continue;
            };

            if let Some(existing) = self.namedWeavePoints.get(&name).cloned() {
                self.base.Error(
                    format!(
                        "A {} with the same label name '{}' already exists in this context",
                        "weave point", name
                    ),
                    Some(existing),
                    false,
                );
            } else {
                self.namedWeavePoints.insert(name, obj);
            }
        }
    }

    // C# signature: void ConstructWeaveHierarchyFromIndentation()
    fn ConstructWeaveHierarchyFromIndentation(&mut self) {
        let mut contentIdx = 0;
        while contentIdx < self.base.content.len() {
            let obj = self.base.content[contentIdx].clone();
            if obj.kind == ObjectKind::WeavePoint {
                let weaveIndentIdx = obj.indentationDepth.saturating_sub(1);

                if weaveIndentIdx > self.baseIndentIndex {
                    let innerWeaveStartIdx = contentIdx;
                    while contentIdx < self.base.content.len() {
                        let innerWeaveObj = self.base.content[contentIdx].clone();
                        if innerWeaveObj.kind == ObjectKind::WeavePoint {
                            let innerIndentIdx = innerWeaveObj.indentationDepth.saturating_sub(1);
                            if innerIndentIdx <= self.baseIndentIndex {
                                break;
                            }
                        }

                        contentIdx += 1;
                    }

                    let weaveContent: Vec<Object> = self
                        .base
                        .content
                        .drain(innerWeaveStartIdx..contentIdx)
                        .collect();
                    let nestedWeave = Weave::new(weaveContent, weaveIndentIdx);
                    let mut nestedWeaveObj = Object::with_kind(ObjectKind::Weave);
                    nestedWeaveObj.content = nestedWeave.base.content.clone();
                    self.base
                        .InsertContent(innerWeaveStartIdx as i32, nestedWeaveObj);
                    contentIdx = innerWeaveStartIdx;
                }
            }

            contentIdx += 1;
        }
    }

    // C# signature: public int DetermineBaseIndentationFromContent(List<Parsed.Object> contentList)
    pub fn DetermineBaseIndentationFromContent(&mut self, contentList: Vec<Object>) -> i32 {
        Self::determine_base_indentation_from_content(&contentList)
    }

    fn determine_base_indentation_from_content(contentList: &[Object]) -> i32 {
        for obj in contentList {
            if obj.kind == ObjectKind::WeavePoint {
                return obj.indentationDepth.saturating_sub(1);
            }
        }
        0
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> ContentItem {
        let mut root_container = Container::new();
        root_container.set_debugMetadata(self.base.get_debugMetadata().cloned());
        self.rootContainer = Some(root_container.clone());
        self.currentContainer = Some(root_container.clone());
        self.looseEnds.clear();
        self.gatherPointsToResolve.clear();

        for mut obj in self.base.content.clone() {
            if obj.kind == ObjectKind::Weave {
                let nested = Weave::new(obj.content.clone(), -1);
                self.AddRuntimeForNestedWeave(nested.clone());
                self.gatherPointsToResolve
                    .extend(nested.gatherPointsToResolve.clone());
            } else if obj.kind == ObjectKind::WeavePoint {
                self.AddRuntimeForWeavePoint(&mut obj);
            } else if let Some(runtime_object) = obj.EnsureRuntimeObject() {
                self.AddGeneralRuntimeContent(Some(runtime_object.into()));
            }
        }

        self.PassLooseEndsToAncestors();

        ContentItem::Container(Rc::new(
            self.rootContainer.clone().unwrap_or(root_container),
        ))
    }

    // C# signature: public void AddRuntimeForNestedWeave(Weave nestedResult)
    pub fn AddRuntimeForNestedWeave(&mut self, mut nestedResult: Weave) {
        if let Some(nested_root) = nestedResult.get_rootContainer() {
            self.AddGeneralRuntimeContent(Some(ContentItem::Container(Rc::new(nested_root))));
        }

        if let Some(previous) = &self.previousWeavePoint {
            self.looseEnds.retain(|loose_end| loose_end != previous);
            self.addContentToPreviousWeavePoint = false;
        }
    }

    // C# signature: public override void ResolveReferences(Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        self.base.ResolveReferences(context);

        for obj in &mut self.base.content {
            if obj.kind == ObjectKind::Weave {
                let mut nested = Weave::new(obj.content.clone(), -1);
                nested.ResolveReferences(context);
            } else {
                obj.ResolveReferences(context);
            }
        }

        if self.looseEnds.is_empty() == false {
            let mut is_nested_weave = false;
            let mut ancestor = self.base.get_parent();
            while let Some(current) = ancestor {
                if matches!(current.kind, ObjectKind::Sequence | ObjectKind::Conditional) {
                    is_nested_weave = true;
                    break;
                }
                ancestor = current.get_parent();
            }

            if is_nested_weave {
                self.ValidateTermination(Self::BadNestedTerminationHandler);
            }
        }

        for gatherPoint in &mut self.gatherPointsToResolve {
            gatherPoint
                .divert
                .set_targetPathString(Some(gatherPoint.targetRuntimeObj.get_path().ToString()));
        }

        self.CheckForWeavePointNamingCollisions();
    }

    // C# signature: public IWeavePoint WeavePointNamed(string name)
    pub fn WeavePointNamed(&self, name: String) -> Option<Object> {
        self.namedWeavePoints.get(&name).cloned()
    }

    // C# signature: public void ValidateTermination (BadTerminationHandler badTerminationHandler)
    pub fn ValidateTermination(&mut self, _badTerminationHandler: fn(&mut Object)) {
        if let Some(last_object) = self.get_lastParsedSignificantObject() {
            if matches!(
                last_object.payload.as_ref(),
                Some(ObjectPayload::AuthorWarning(_))
            ) {
                return;
            }
        }
    }

    // C# signature: Runtime.Container rootContainer { get; }
    pub fn get_rootContainer(&mut self) -> Option<Container> {
        if self.rootContainer.is_none() {
            let _ = self.GenerateRuntimeObject();
        }
        self.rootContainer.clone()
    }

    // C# signature: int baseIndentIndex { get; }
    pub fn get_baseIndentIndex(&mut self) -> i32 {
        self.baseIndentIndex
    }

    // C# signature: Parsed.Object lastParsedSignificantObject { get; }
    pub fn get_lastParsedSignificantObject(&mut self) -> Option<Object> {
        if self.base.content.is_empty() {
            return None;
        }

        let mut last_object: Option<Object> = None;
        for obj in self.base.content.iter().rev() {
            last_object = Some(obj.clone());

            if Self::is_terminal_newline(obj) {
                continue;
            }

            if Self::is_global_declaration(obj) {
                continue;
            }

            break;
        }

        if let Some(last_weave) = last_object
            .as_ref()
            .filter(|obj| obj.kind == ObjectKind::Weave)
        {
            let mut nested = Weave::new(last_weave.content.clone(), -1);
            return nested.get_lastParsedSignificantObject();
        }

        last_object
    }

    fn is_terminal_newline(obj: &Object) -> bool {
        obj.get_runtimeObject()
            .and_then(|runtime| runtime.get_content().last())
            .is_some_and(|content| {
                matches!(
                    content,
                    ink_runtime::Container::ContentItem::Value(ink_runtime::Value::Value::String(
                        string_value
                    )) if string_value.value == "\n"
                )
            })
    }

    fn is_global_declaration(obj: &Object) -> bool {
        match obj.payload.as_ref() {
            Some(ObjectPayload::ConstantDeclaration(_)) => true,
            Some(ObjectPayload::VariableAssignment(assignment)) => {
                assignment.get_isGlobalDeclaration() && assignment.get_isDeclaration()
            }
            _ => false,
        }
    }

    fn AddRuntimeForWeavePoint(&mut self, weavePoint: &mut Object) {
        let is_gather = matches!(weavePoint.payload.as_ref(), Some(ObjectPayload::Gather(_)));
        let is_choice = matches!(weavePoint.payload.as_ref(), Some(ObjectPayload::Choice(_)));

        if is_gather {
            if let Some(ObjectPayload::Gather(gather)) = weavePoint.payload.as_mut() {
                self.AddRuntimeForGather(gather);
            }
        } else if is_choice {
            if let Some(previous) = &self.previousWeavePoint {
                if matches!(previous.payload.as_ref(), Some(ObjectPayload::Gather(_))) {
                    self.looseEnds.retain(|loose_end| loose_end != previous);
                }
            }

            if let Some(runtime_object) = weavePoint.EnsureRuntimeObject() {
                self.AddGeneralRuntimeContent(Some(runtime_object.into()));
            }

            self.hasSeenChoiceInSection = true;
        }

        self.addContentToPreviousWeavePoint = false;
        if Self::WeavePointHasLooseEnd(weavePoint) {
            self.looseEnds.push(weavePoint.clone());
            if is_choice {
                self.addContentToPreviousWeavePoint = true;
            }
        }

        self.previousWeavePoint = Some(weavePoint.clone());
    }

    fn AddRuntimeForGather(&mut self, gather: &mut Gather) {
        let auto_enter = !self.hasSeenChoiceInSection;
        self.hasSeenChoiceInSection = false;

        let gather_container = match gather.GenerateRuntimeObject() {
            ContentItem::Container(container) => container.as_ref().clone(),
            _ => Container::new(),
        };

        if auto_enter {
            self.AddGeneralRuntimeContent(Some(ContentItem::Container(Rc::new(
                gather_container.clone(),
            ))));
        } else if let Some(root) = self.rootContainer.as_mut() {
            root.AddToNamedContentOnly(gather_container.clone());
        }

        for mut loose_end in self.looseEnds.clone().into_iter().rev() {
            if matches!(loose_end.payload.as_ref(), Some(ObjectPayload::Gather(_)))
                && loose_end.indentationDepth == gather.get_indentationDepth()
            {
                continue;
            }

            let mut divert = RuntimeDivert::new();
            if let Some(target_runtime) = loose_end
                .get_runtimeObject()
                .cloned()
                .or_else(|| loose_end.EnsureRuntimeObject())
            {
                let mut target_container = target_runtime;
                target_container.AddContent(ControlCommand::Done());
                divert.set_targetPathString(Some(target_container.get_path().ToString()));
            }
            self.gatherPointsToResolve.push(GatherPointToResolve {
                divert,
                targetRuntimeObj: gather_container.clone(),
            });
        }

        self.looseEnds.clear();
        self.currentContainer = Some(gather_container.clone());
        self.rootContainer = self.currentContainer.clone();
    }

    fn AddGeneralRuntimeContent(&mut self, content: Option<ContentItem>) {
        let Some(content) = content else {
            return;
        };
        if let Some(current) = self.currentContainer.as_mut() {
            current.AddContent(content);
            self.rootContainer = self.currentContainer.clone();
        }
    }

    fn PassLooseEndsToAncestors(&mut self) {
        if self.looseEnds.is_empty() {
            return;
        }
        if let Some(root) = self.rootContainer.as_mut() {
            for loose_end in &self.looseEnds {
                if let Some(runtime_object) = loose_end.get_runtimeObject().cloned() {
                    root.AddContent(runtime_object);
                }
            }
        }
    }

    fn WeavePointHasLooseEnd(weavePoint: &Object) -> bool {
        weavePoint.content.is_empty()
    }

    fn ContentThatFollowsWeavePoint(&self, weavePoint: &Object) -> Vec<Object> {
        let mut result = Vec::new();

        for content_obj in &weavePoint.content {
            if Self::is_global_declaration(content_obj) {
                continue;
            }
            result.push(content_obj.clone());
        }

        let Some(parent_weave) = weavePoint.get_parent() else {
            return result;
        };

        let mut weave_point_idx = None;
        for (idx, obj) in parent_weave.content.iter().enumerate() {
            if obj == weavePoint {
                weave_point_idx = Some(idx);
                break;
            }
        }

        let Some(start_idx) = weave_point_idx else {
            return result;
        };

        for later_obj in parent_weave.content.iter().skip(start_idx + 1) {
            if Self::is_global_declaration(later_obj) {
                continue;
            }

            if matches!(later_obj.kind, ObjectKind::WeavePoint | ObjectKind::Weave) {
                break;
            }

            result.push(later_obj.clone());
        }

        result
    }

    fn ValidateFlowOfObjectsTerminates(
        &self,
        objFlow: &[Object],
        defaultObj: &Object,
    ) -> Option<Object> {
        if objFlow.is_empty() {
            None
        } else {
            Some(defaultObj.clone())
        }
    }

    fn BadNestedTerminationHandler(terminatingObj: &mut Object) {
        terminatingObj.Error(
            "Choices nested in conditionals or sequences need to explicitly divert afterwards."
                .to_string(),
            None,
            false,
        );
    }

    fn CheckForWeavePointNamingCollisions(&mut self) {
        if self.namedWeavePoints.is_empty() {
            return;
        }

        let mut ancestor_flows = Vec::<Object>::new();
        let mut ancestor = self.base.get_parent();
        while let Some(current) = ancestor {
            if matches!(
                current.kind,
                ObjectKind::FlowBase | ObjectKind::Story | ObjectKind::Knot | ObjectKind::Stitch
            ) {
                ancestor_flows.push(current.clone());
            } else {
                break;
            }
            ancestor = current.get_parent();
        }

        for (weave_point_name, weave_point) in self.namedWeavePoints.clone() {
            for flow in &ancestor_flows {
                let flow_base = FlowBase::from_object(flow);
                if let Some(other_content_with_name) =
                    flow_base.ContentWithNameAtLevel(weave_point_name.clone(), None, false)
                {
                    if other_content_with_name != weave_point {
                        let error_msg = format!(
                            "{} '{}' has the same label name as a {}",
                            weave_point.get_typeName(),
                            weave_point_name,
                            other_content_with_name.get_typeName()
                        );
                        self.base.Error(error_msg, Some(weave_point.clone()), false);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Weave;
    use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::Number::{Number, NumberValue};
    use crate::ParsedHierarchy::Object::{Object, ObjectKind};
    use crate::ParsedHierarchy::VariableAssignment::VariableAssignment;
    use ink_runtime::Container::Container;
    use ink_runtime::Value::StringValue;

    #[test]
    fn determine_base_indentation_uses_first_weave_point() {
        let mut weave = Weave::new(
            vec![Object::new(), {
                let mut weave_point = Object::with_kind(ObjectKind::WeavePoint);
                weave_point.indentationDepth = 3;
                weave_point
            }],
            -1,
        );

        assert_eq!(weave.get_baseIndentIndex(), 2);
        assert_eq!(
            weave.DetermineBaseIndentationFromContent(weave.base.content.clone()),
            2
        );
        assert!(weave.get_lastParsedSignificantObject().is_some());
    }

    #[test]
    fn nested_indentation_wraps_inner_weave() {
        let mut outer_choice = Object::with_kind(ObjectKind::WeavePoint);
        outer_choice.indentationDepth = 1;
        let mut inner_choice = Object::with_kind(ObjectKind::WeavePoint);
        inner_choice.indentationDepth = 3;

        let weave = Weave::new(vec![outer_choice, inner_choice], -1);
        assert!(weave
            .base
            .content
            .iter()
            .any(|obj| obj.kind == ObjectKind::Weave));
    }

    #[test]
    fn last_significant_object_skips_terminal_newline_runtime_content() {
        let mut normal = Object::with_kind(ObjectKind::Plain);
        let mut normal_runtime = Container::new();
        normal_runtime.AddContent(StringValue::new("hello".to_string()));
        normal.set_runtimeObject(Some(normal_runtime));

        let mut newline = Object::with_kind(ObjectKind::Plain);
        let mut newline_runtime = Container::new();
        newline_runtime.AddContent(StringValue::new("\n".to_string()));
        newline.set_runtimeObject(Some(newline_runtime));

        let mut weave = Weave::new(vec![normal.clone(), newline], -1);

        assert_eq!(
            weave
                .get_lastParsedSignificantObject()
                .as_ref()
                .map(|obj| obj.get_typeName()),
            Some(normal.get_typeName())
        );
    }

    #[test]
    fn last_significant_object_skips_global_declarations() {
        let mut normal = Object::with_kind(ObjectKind::Plain);
        let mut normal_runtime = Container::new();
        normal_runtime.AddContent(StringValue::new("hello".to_string()));
        normal.set_runtimeObject(Some(normal_runtime));

        let mut declaration = VariableAssignment::new(
            Identifier {
                name: Some("score".to_string()),
                debugMetadata: None,
            },
            Expression::from_kind(ExpressionKind::Number(Number::new(NumberValue::Int(1)))),
        );
        declaration.set_isGlobalDeclaration(true);

        let mut weave = Weave::new(
            vec![
                normal.clone(),
                Object::from_variable_assignment(declaration),
            ],
            -1,
        );

        assert_eq!(
            weave
                .get_lastParsedSignificantObject()
                .as_ref()
                .map(|obj| obj.get_typeName()),
            Some(normal.get_typeName())
        );
    }
}
