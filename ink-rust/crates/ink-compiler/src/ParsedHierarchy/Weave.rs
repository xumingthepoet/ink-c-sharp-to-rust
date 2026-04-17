// Source: ink-c-sharp/compiler/ParsedHierarchy/Weave.cs

use crate::ParsedHierarchy::Object::{Object, ObjectKind};
use crate::ParsedHierarchy::Story::Story;
use ink_runtime::Container::{Container, ContentItem};
use ink_runtime::Divert::Divert as RuntimeDivert;
use std::collections::HashMap;

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
        self.rootContainer = Some(root_container.clone());
        self.currentContainer = Some(root_container.clone());
        self.looseEnds.clear();
        self.gatherPointsToResolve.clear();

        for obj in &mut self.base.content {
            if obj.kind == ObjectKind::Weave {
                let mut nested = Weave::new(obj.content.clone(), -1);
                if let ContentItem::Container(nested_root) = nested.GenerateRuntimeObject() {
                    root_container.AddContent(*nested_root);
                }
            } else if let Some(runtime_object) = obj.get_runtimeObject().cloned() {
                root_container.AddContent(runtime_object);
            }
        }

        self.rootContainer = Some(root_container.clone());
        self.currentContainer = Some(root_container.clone());
        ContentItem::Container(Box::new(root_container))
    }

    // C# signature: public void AddRuntimeForNestedWeave(Weave nestedResult)
    pub fn AddRuntimeForNestedWeave(&mut self, _nestedResult: Weave) {
        // The full indentation-driven nested weave reconstruction is still incomplete.
        // The current compiler path only needs the root-level runtime container.
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

        for gatherPoint in &mut self.gatherPointsToResolve {
            gatherPoint
                .divert
                .set_targetPathString(Some(gatherPoint.targetRuntimeObj.get_path().ToString()));
        }
    }

    // C# signature: public IWeavePoint WeavePointNamed(string name)
    pub fn WeavePointNamed(&self, name: String) -> Option<Object> {
        self.namedWeavePoints.get(&name).cloned()
    }

    // C# signature: public void ValidateTermination (BadTerminationHandler badTerminationHandler)
    pub fn ValidateTermination(&mut self, _badTerminationHandler: fn(&mut Object)) {
        // The full nested weave termination analysis is not yet ported.
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
        for obj in self.base.content.iter().rev() {
            if let Some(name) = obj
                .identifier
                .as_ref()
                .and_then(|identifier| identifier.name.clone())
            {
                if !name.is_empty() {
                    return Some(obj.clone());
                }
            }

            if obj.kind != ObjectKind::Plain {
                return Some(obj.clone());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::Weave;
    use crate::ParsedHierarchy::Object::{Object, ObjectKind};

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
}
