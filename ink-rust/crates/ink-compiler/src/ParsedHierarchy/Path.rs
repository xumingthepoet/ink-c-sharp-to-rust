// Source: ink-c-sharp/compiler/ParsedHierarchy/Path.cs

use crate::ParsedHierarchy::FlowLevel::FlowLevel;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Object::{Object, ObjectKind};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Path {
    _baseTargetLevel: Option<FlowLevel>,
    pub components: Vec<Identifier>,
    dotSeparatedComponents: Option<String>,
}

impl Path {
    // C# signature: public Path(FlowLevel baseFlowLevel, List<Identifier> components)
    pub fn new(baseFlowLevel: FlowLevel, components: Vec<Identifier>) -> Self {
        Self {
            _baseTargetLevel: Some(baseFlowLevel),
            components,
            dotSeparatedComponents: None,
        }
    }

    // C# signature: public Path(List<Identifier> components)
    pub fn new_overload_2(components: Vec<Identifier>) -> Self {
        Self {
            _baseTargetLevel: None,
            components,
            dotSeparatedComponents: None,
        }
    }

    // C# signature: public Path(Identifier ambiguousName)
    pub fn new_overload_3(ambiguousName: Identifier) -> Self {
        Self {
            _baseTargetLevel: None,
            components: vec![ambiguousName],
            dotSeparatedComponents: None,
        }
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        if self.components.is_empty() {
            if self.get_baseTargetLevel() == FlowLevel::WeavePoint {
                "-> <next gather point>".to_string()
            } else {
                "<invalid Path>".to_string()
            }
        } else {
            format!("-> {}", self.get_dotSeparatedComponents())
        }
    }

    // C# signature: public Parsed.Object ResolveFromContext(Parsed.Object context)
    pub fn ResolveFromContext(&self, context: &Object) -> Option<Object> {
        if self.components.is_empty() {
            return None;
        }

        let base_target_object = self.ResolveBaseTarget(context)?;
        if self.components.len() > 1 {
            return self.ResolveTailComponents(&base_target_object);
        }

        Some(base_target_object)
    }

    // C# signature: FlowLevel baseTargetLevel { get; }
    pub fn get_baseTargetLevel(&self) -> FlowLevel {
        if self.get_baseLevelIsAmbiguous() {
            FlowLevel::Story
        } else {
            self._baseTargetLevel.unwrap()
        }
    }

    // C# signature: bool baseLevelIsAmbiguous { get; }
    pub fn get_baseLevelIsAmbiguous(&self) -> bool {
        self._baseTargetLevel.is_none()
    }

    // C# signature: string firstComponent { get; }
    pub fn get_firstComponent(&self) -> Option<&str> {
        self.components
            .first()
            .and_then(|identifier| identifier.name.as_deref())
    }

    // C# signature: int numberOfComponents { get; }
    pub fn get_numberOfComponents(&self) -> i32 {
        self.components.len() as i32
    }

    // C# signature: string dotSeparatedComponents { get; }
    pub fn get_dotSeparatedComponents(&self) -> String {
        if let Some(value) = &self.dotSeparatedComponents {
            return value.clone();
        }

        self.components
            .iter()
            .map(|component| component.name.as_deref().unwrap_or(""))
            .collect::<Vec<_>>()
            .join(".")
    }

    // C# signature: List<Identifier> components { get; }
    pub fn get_components(&self) -> &[Identifier] {
        &self.components
    }

    fn ResolveBaseTarget(&self, originalContext: &Object) -> Option<Object> {
        let first_comp = self.get_firstComponent()?;

        let mut ancestor_context: Option<&Object> = Some(originalContext);
        while let Some(current_context) = ancestor_context {
            let deep_search = std::ptr::eq(current_context, originalContext);
            if let Some(found_base) =
                self.TryGetChildFromContext(current_context, first_comp, None, deep_search)
            {
                return Some(found_base);
            }

            ancestor_context = current_context.get_parent();
        }

        None
    }

    fn ResolveTailComponents(&self, rootTarget: &Object) -> Option<Object> {
        let mut found_component = rootTarget.clone();
        for index in 1..self.components.len() {
            let comp_name = self.components[index]
                .name
                .as_deref()
                .unwrap_or("")
                .to_string();

            let minimum_expected_level = match found_component.kind {
                ObjectKind::Story => Some(FlowLevel::Knot),
                ObjectKind::Knot => Some(FlowLevel::Stitch),
                ObjectKind::Stitch => Some(FlowLevel::WeavePoint),
                ObjectKind::FlowBase => Some(FlowLevel::WeavePoint),
                _ => Some(FlowLevel::WeavePoint),
            };

            found_component = self.TryGetChildFromContext(
                &found_component,
                &comp_name,
                minimum_expected_level,
                false,
            )?;
        }

        Some(found_component)
    }

    fn TryGetChildFromContext(
        &self,
        context: &Object,
        childName: &str,
        minimumLevel: Option<FlowLevel>,
        forceDeepSearch: bool,
    ) -> Option<Object> {
        let ambiguousChildLevel = minimumLevel.is_none();

        if (context.kind == ObjectKind::Weave || context.kind == ObjectKind::WeavePoint)
            && (ambiguousChildLevel || minimumLevel == Some(FlowLevel::WeavePoint))
        {
            if let Some(found) = context.content.iter().find(|obj| {
                obj.kind == ObjectKind::WeavePoint
                    && obj
                        .identifier
                        .as_ref()
                        .and_then(|identifier| identifier.name.as_deref())
                        == Some(childName)
            }) {
                return Some(found.clone());
            }
        }

        if self.is_flow_like_context(context) {
            if let Some(found) = context.content.iter().find(|obj| {
                obj.identifier
                    .as_ref()
                    .and_then(|identifier| identifier.name.as_deref())
                    == Some(childName)
                    && self.object_meets_minimum_level(obj, minimumLevel)
            }) {
                return Some(found.clone());
            }

            if forceDeepSearch || matches!(context.kind, ObjectKind::Knot) {
                return self.DeepSearchForAnyLevelContent(context, childName);
            }
        }

        if forceDeepSearch {
            return self.DeepSearchForAnyLevelContent(context, childName);
        }

        None
    }

    fn DeepSearchForAnyLevelContent(&self, context: &Object, childName: &str) -> Option<Object> {
        if let Some(found) = context.content.iter().find(|obj| {
            obj.identifier
                .as_ref()
                .and_then(|identifier| identifier.name.as_deref())
                == Some(childName)
                && self.object_meets_minimum_level(obj, None)
        }) {
            return Some(found.clone());
        }

        for obj in &context.content {
            if let Some(found) = self.DeepSearchForAnyLevelContent(obj, childName) {
                return Some(found);
            }
        }

        None
    }

    fn object_meets_minimum_level(&self, obj: &Object, minimumLevel: Option<FlowLevel>) -> bool {
        match minimumLevel {
            None => true,
            Some(level) => {
                self.flow_level_rank(self.flow_level_for_object(obj)) >= self.flow_level_rank(level)
            }
        }
    }

    fn is_flow_like_context(&self, context: &Object) -> bool {
        matches!(
            context.kind,
            ObjectKind::Story | ObjectKind::FlowBase | ObjectKind::Knot | ObjectKind::Stitch
        )
    }

    fn flow_level_for_object(&self, object: &Object) -> FlowLevel {
        match object.kind {
            ObjectKind::Story => FlowLevel::Story,
            ObjectKind::Knot => FlowLevel::Knot,
            ObjectKind::Stitch => FlowLevel::Stitch,
            _ => FlowLevel::WeavePoint,
        }
    }

    fn flow_level_rank(&self, level: FlowLevel) -> i32 {
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
    use super::Path;
    use crate::ParsedHierarchy::FlowLevel::FlowLevel;
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::Object::{Object, ObjectKind};

    #[test]
    fn stringifies_and_reports_components() {
        let path = Path::new_overload_2(vec![
            Identifier {
                name: Some("knot".to_string()),
                debugMetadata: None,
            },
            Identifier {
                name: Some("stitch".to_string()),
                debugMetadata: None,
            },
        ]);

        assert_eq!(path.get_baseTargetLevel(), FlowLevel::Story);
        assert_eq!(path.get_firstComponent(), Some("knot"));
        assert_eq!(path.get_numberOfComponents(), 2);
        assert_eq!(path.get_dotSeparatedComponents(), "knot.stitch");
        assert_eq!(path.ToString(), "-> knot.stitch");
    }

    #[test]
    fn resolves_named_children_through_parent_chain() {
        let mut root = Object::with_kind(ObjectKind::Story);
        root.set_identifier(Some(Identifier {
            name: Some("root".to_string()),
            debugMetadata: None,
        }));

        let mut knot = Object::with_kind(ObjectKind::Knot);
        knot.set_identifier(Some(Identifier {
            name: Some("knot".to_string()),
            debugMetadata: None,
        }));
        knot.set_parent(Some(Box::new(root.clone())));

        let mut stitch = Object::with_kind(ObjectKind::WeavePoint);
        stitch.set_identifier(Some(Identifier {
            name: Some("stitch".to_string()),
            debugMetadata: None,
        }));
        stitch.set_parent(Some(Box::new(knot.clone())));

        knot.content.push(stitch.clone());
        root.content.push(knot.clone());

        let path = Path::new_overload_2(vec![
            Identifier {
                name: Some("knot".to_string()),
                debugMetadata: None,
            },
            Identifier {
                name: Some("stitch".to_string()),
                debugMetadata: None,
            },
        ]);

        let resolved = path.ResolveFromContext(&root).expect("resolve path");
        assert_eq!(
            resolved
                .identifier
                .as_ref()
                .and_then(|identifier| identifier.name.as_deref()),
            Some("stitch")
        );
    }
}
