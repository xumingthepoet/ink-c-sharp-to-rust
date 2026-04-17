// Source: ink-c-sharp/compiler/ParsedHierarchy/Stitch.cs

use crate::ParsedHierarchy::FlowBase::FlowBase;
use crate::ParsedHierarchy::FlowLevel::FlowLevel;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Story::Story;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Stitch {
    base: FlowBase,
}

impl Stitch {
    // C# signature: public Stitch (Identifier name, List<Parsed.Object> topLevelObjects, List<Argument> arguments, bool isFunction)
    pub fn new(
        name: Identifier,
        topLevelObjects: Vec<crate::ParsedHierarchy::Object::Object>,
        arguments: Vec<crate::ParsedHierarchy::FlowBase::Argument>,
        isFunction: bool,
    ) -> Self {
        let mut base = FlowBase::new(name, topLevelObjects, arguments, isFunction, false);
        base.set_flowLevel(FlowLevel::Stitch);
        Self { base }
    }

    pub fn get_flowLevel(&self) -> FlowLevel {
        FlowLevel::Stitch
    }

    pub fn get_name(&self) -> Option<&str> {
        self.base.get_name()
    }

    pub fn get_identifier(&self) -> Option<&Identifier> {
        self.base.get_identifier()
    }

    pub fn ResolveReferences(&mut self, context: &mut Story) {
        self.base.ResolveReferences(context);
    }

    pub fn GenerateRuntimeObject(&mut self) -> ink_runtime::Container::Container {
        self.base.GenerateRuntimeObject()
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
    use super::Stitch;
    use crate::ParsedHierarchy::FlowLevel::FlowLevel;
    use crate::ParsedHierarchy::Identifier::Identifier;

    #[test]
    fn stitch_reports_flow_level_and_name() {
        let stitch = Stitch::new(
            Identifier {
                name: Some("scene".to_string()),
                debugMetadata: None,
            },
            vec![],
            vec![],
            false,
        );

        assert_eq!(stitch.get_flowLevel(), FlowLevel::Stitch);
        assert_eq!(stitch.get_name(), Some("scene"));
    }
}
