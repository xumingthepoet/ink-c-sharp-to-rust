// Source: ink-c-sharp/compiler/ParsedHierarchy/Knot.cs

use crate::ParsedHierarchy::FlowBase::FlowBase;
use crate::ParsedHierarchy::FlowLevel::FlowLevel;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Story::Story;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Knot {
    base: FlowBase,
}

impl Knot {
    // C# signature: public Knot (Identifier name, List<Parsed.Object> topLevelObjects, List<Argument> arguments, bool isFunction)
    pub fn new(
        name: Identifier,
        topLevelObjects: Vec<crate::ParsedHierarchy::Object::Object>,
        arguments: Vec<crate::ParsedHierarchy::FlowBase::Argument>,
        isFunction: bool,
    ) -> Self {
        let mut base = FlowBase::new(name, topLevelObjects, arguments, isFunction, false);
        base.set_flowLevel(FlowLevel::Knot);
        Self { base }
    }

    pub fn ResolveReferences(&mut self, context: &mut Story) {
        self.base.ResolveReferences(context);
    }

    pub fn GenerateRuntimeObject(&mut self) -> ink_runtime::Container::Container {
        self.base.GenerateRuntimeObject()
    }

    pub fn get_flowLevel(&self) -> FlowLevel {
        FlowLevel::Knot
    }

    pub fn get_name(&self) -> Option<&str> {
        self.base.get_name()
    }

    pub fn get_identifier(&self) -> Option<&Identifier> {
        self.base.get_identifier()
    }

    pub fn get_base(&self) -> &FlowBase {
        &self.base
    }

    pub fn get_base_mut(&mut self) -> &mut FlowBase {
        &mut self.base
    }
}

#[cfg(test)]
mod tests {
    use super::Knot;
    use crate::ParsedHierarchy::FlowBase::Argument;
    use crate::ParsedHierarchy::FlowLevel::FlowLevel;
    use crate::ParsedHierarchy::Identifier::Identifier;

    #[test]
    fn knot_reports_flow_level_and_name() {
        let knot = Knot::new(
            Identifier {
                name: Some("intro".to_string()),
                debugMetadata: None,
            },
            vec![],
            vec![Argument::default()],
            false,
        );

        assert_eq!(knot.get_flowLevel(), FlowLevel::Knot);
        assert_eq!(knot.get_name(), Some("intro"));
    }
}
