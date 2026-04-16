// Source: ink-c-sharp/compiler/ParsedHierarchy/Path.cs

use crate::ParsedHierarchy::FlowLevel::FlowLevel;
use crate::ParsedHierarchy::Identifier::Identifier;

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
    pub fn ResolveFromContext(&mut self, _context: crate::stub::PortStub) -> crate::stub::PortStub {
        todo!("port ParsedHierarchy.Path.ResolveFromContext after ParsedHierarchy.Object/FlowBase are translated");
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
}

#[cfg(test)]
mod tests {
    use super::Path;
    use crate::ParsedHierarchy::FlowLevel::FlowLevel;
    use crate::ParsedHierarchy::Identifier::Identifier;

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
}
