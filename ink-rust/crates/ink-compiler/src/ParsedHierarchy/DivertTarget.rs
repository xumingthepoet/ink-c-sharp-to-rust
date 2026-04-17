// Source: ink-c-sharp/compiler/ParsedHierarchy/DivertTarget.cs

use crate::ParsedHierarchy::Divert::Divert;
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
        let runtime_divert = divert.get_runtimeDivert().cloned();
        let runtime_divert_target_value = DivertTargetValue::new(
            runtime_divert
                .as_ref()
                .and_then(|runtime_divert| runtime_divert.get_targetPath()),
        );

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
        self.divert
            .get_target()
            .map(|path| path.get_dotSeparatedComponents())
            .unwrap_or_default()
            .len() as i32
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
}
