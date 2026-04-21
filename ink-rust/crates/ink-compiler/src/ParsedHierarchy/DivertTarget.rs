// Source: ink-c-sharp/compiler/ParsedHierarchy/DivertTarget.cs

use crate::ParsedHierarchy::Divert::Divert;
use crate::ParsedHierarchy::FlowBase::FlowBase;
use crate::ParsedHierarchy::Story::Story;
use ink_runtime::Container::ContentItem;
use ink_runtime::Value::{DivertTargetValue, Value};

#[derive(Clone, Debug, Default)]
pub struct DivertTarget {
    pub divert: Divert,
    runtimeDivertTargetValue: Option<DivertTargetValue>,
    runtimeDivert: Option<ink_runtime::Divert::Divert>,
}

impl PartialEq for DivertTarget {
    fn eq(&self, other: &Self) -> bool {
        self.divert == other.divert
    }
}

impl DivertTarget {
    // C# signature: public DivertTarget (Divert divert)
    pub fn new(divert: Divert) -> Self {
        Self {
            divert,
            runtimeDivertTargetValue: None,
            runtimeDivert: None,
        }
    }

    // C# signature: public override void GenerateIntoContainer (Runtime.Container container)
    pub fn GenerateIntoContainer(&self, container: &mut ink_runtime::Container::Container) {
        let mut divert = self.divert.clone();
        divert.GenerateRuntimeObject();
        let runtime_divert_target_value =
            self.runtimeDivertTargetValue.clone().unwrap_or_else(|| {
                DivertTargetValue::new(
                    divert
                        .get_runtimeDivert()
                        .as_ref()
                        .and_then(|runtime_divert| runtime_divert.get_targetPath()),
                )
            });

        container.AddContent(ContentItem::Value(Value::DivertTarget(
            runtime_divert_target_value,
        )));
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        self.divert.ResolveReferences(context);
        let mut divert = self.divert.clone();
        divert.GenerateRuntimeObject();
        self.runtimeDivert = divert.get_runtimeDivert().cloned();
        self.runtimeDivertTargetValue = Some(DivertTargetValue::new(None));

        if self.divert.get_isDone() || self.divert.get_isEnd() {
            context.Error(
                "Can't Can't use -> DONE or -> END as variable divert targets".to_string(),
                Default::default(),
                false,
            );
            return;
        }

        if let Some(runtimeDivert) = &self.runtimeDivert {
            if runtimeDivert.get_variableDivertName().is_some() {
                context.Error(
                    format!(
                        "Since '{}' is a variable, it shouldn't be preceded by '->' here.",
                        self.divert
                            .get_target()
                            .map(|path| path.get_dotSeparatedComponents())
                            .unwrap_or_default()
                    ),
                    Default::default(),
                    false,
                );
            }
        }

        if let Some(target_value) = &mut self.runtimeDivertTargetValue {
            if let Some(runtime_divert) = &self.runtimeDivert {
                target_value.value = runtime_divert.get_targetPath();
            }
        }

        if let Some(target_content) = self.divert.get_targetContent() {
            if let Some(target_flow) = match target_content.kind {
                crate::ParsedHierarchy::Object::ObjectKind::Knot
                | crate::ParsedHierarchy::Object::ObjectKind::Stitch
                | crate::ParsedHierarchy::Object::ObjectKind::Story => {
                    Some(FlowBase::from_object(target_content))
                }
                _ => None,
            } {
                if target_flow
                    .get_arguments()
                    .iter()
                    .any(|argument| argument.isByReference)
                {
                    context.Error(
                        format!(
                            "Can't store a divert target to a knot or function that has by-reference arguments ('{}' has 'ref {}').",
                            target_flow
                                .get_identifier()
                                .and_then(|identifier| identifier.name.as_deref())
                                .unwrap_or_default(),
                            target_flow
                                .get_arguments()
                                .iter()
                                .find(|argument| argument.isByReference)
                                .and_then(|argument| argument.identifier.as_ref())
                                .and_then(|identifier| identifier.name.as_deref())
                                .unwrap_or_default()
                        ),
                        Default::default(),
                        false,
                    );
                }
            }
        }
    }

    // C# signature: public override bool Equals (object obj)
    pub fn Equals(&self, obj: &DivertTarget) -> bool {
        self.divert
            .get_target()
            .map(|path| path.get_dotSeparatedComponents())
            == obj
                .divert
                .get_target()
                .map(|path| path.get_dotSeparatedComponents())
    }

    // C# signature: public override int GetHashCode ()
    pub fn GetHashCode(&self) -> i32 {
        let mut hash = 0i32;
        for byte in self
            .divert
            .get_target()
            .map(|path| path.get_dotSeparatedComponents())
            .unwrap_or_default()
            .bytes()
        {
            hash = hash.wrapping_mul(31).wrapping_add(byte as i32);
        }
        hash
    }

    pub fn ToString(&self) -> String {
        self.divert.ToString()
    }
}

#[cfg(test)]
mod tests {
    use super::DivertTarget;
    use crate::ParsedHierarchy::Divert::Divert;
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::Path::Path;
    use ink_runtime::Container::{Container, ContentItem};

    #[test]
    fn generates_runtime_divert_target_values() {
        let divert = Divert::new(
            Path::new_overload_2(vec![Identifier {
                name: Some("knot".to_string()),
                debugMetadata: None,
            }]),
            Vec::new(),
        );
        let target = DivertTarget::new(divert);
        let mut container = Container::new();

        target.GenerateIntoContainer(&mut container);

        assert!(matches!(container.get_content()[0], ContentItem::Value(_)));
    }

    #[test]
    fn hash_code_depends_on_target_text() {
        let a = DivertTarget::new(Divert::new(
            Path::new_overload_2(vec![Identifier {
                name: Some("knot".to_string()),
                debugMetadata: None,
            }]),
            Vec::new(),
        ));
        let b = DivertTarget::new(Divert::new(
            Path::new_overload_2(vec![Identifier {
                name: Some("knot".to_string()),
                debugMetadata: None,
            }]),
            Vec::new(),
        ));
        let c = DivertTarget::new(Divert::new(
            Path::new_overload_2(vec![Identifier {
                name: Some("elsewhere".to_string()),
                debugMetadata: None,
            }]),
            Vec::new(),
        ));

        assert_eq!(a.GetHashCode(), b.GetHashCode());
        assert_ne!(a.GetHashCode(), c.GetHashCode());
    }

    #[test]
    fn generate_uses_resolved_target_cache_when_available() {
        use ink_runtime::Path::Path as RuntimePath;
        use ink_runtime::Value::{DivertTargetValue, Value};

        let mut target = DivertTarget::new(Divert::new(
            Path::new_overload_2(vec![Identifier {
                name: Some("knot".to_string()),
                debugMetadata: None,
            }]),
            Vec::new(),
        ));
        target.runtimeDivertTargetValue = Some(DivertTargetValue::new(None));
        if let Some(value) = &mut target.runtimeDivertTargetValue {
            value.value = Some(RuntimePath::new_overload_4("resolved".to_string()));
        }

        let mut container = Container::new();
        target.GenerateIntoContainer(&mut container);

        match &container.get_content()[0] {
            ContentItem::Value(Value::DivertTarget(value)) => {
                assert_eq!(
                    value
                        .get_targetPath()
                        .map(|path: &RuntimePath| path.ToString()),
                    Some("resolved".to_string())
                );
            }
            other => panic!("unexpected content: {other:?}"),
        }
    }
}
