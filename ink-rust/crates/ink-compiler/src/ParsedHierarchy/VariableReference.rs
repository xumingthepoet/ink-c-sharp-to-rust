// Source: ink-c-sharp/compiler/ParsedHierarchy/VariableReference.cs

use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Object::{Object, ObjectKind};
use crate::ParsedHierarchy::Path::Path;
use crate::ParsedHierarchy::Story::Story;
use ink_runtime::VariableReference::VariableReference as RuntimeVariableReference;
use std::cell::RefCell;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VariableReference {
    pub pathIdentifiers: Vec<Identifier>,
    path: Vec<String>,
    name: String,
    singleIdentifier: Option<Identifier>,
    runtimeVarRef: RefCell<Option<RuntimeVariableReference>>,
    constantExpression: Option<Box<Expression>>,
    pub isConstantReference: bool,
    pub isListItemReference: bool,
}

impl VariableReference {
    // C# signature: public VariableReference (List<Identifier> pathIdentifiers)
    pub fn new(pathIdentifiers: Vec<Identifier>) -> Self {
        let path = pathIdentifiers
            .iter()
            .map(|id| id.name.clone().unwrap_or_default())
            .collect::<Vec<_>>();

        Self {
            name: path.join("."),
            pathIdentifiers,
            path,
            singleIdentifier: None,
            runtimeVarRef: RefCell::new(None),
            constantExpression: None,
            isConstantReference: false,
            isListItemReference: false,
        }
    }

    // C# signature: public override void GenerateIntoContainer (Runtime.Container container)
    pub fn GenerateIntoContainer(&self, container: &mut ink_runtime::Container::Container) {
        if let Some(constantValue) = self.constantExpression.as_ref() {
            constantValue.GenerateConstantIntoContainer(container);
            return;
        }

        let runtime_var_ref = RuntimeVariableReference::new(self.name.clone());
        *self.runtimeVarRef.borrow_mut() = Some(runtime_var_ref.clone());
        container.AddContent(runtime_var_ref);
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        if let Some(constantValue) = context.constants.get(&self.name).cloned() {
            self.constantExpression = Some(Box::new(constantValue));
            self.isConstantReference = true;
            return;
        }

        if self.path.len() == 1 || self.path.len() == 2 {
            let (listName, listItemName) = if self.path.len() == 1 {
                (String::new(), self.path[0].clone())
            } else {
                (self.path[0].clone(), self.path[1].clone())
            };

            if context
                .ResolveListItem(listName, listItemName, Default::default())
                .is_some()
            {
                self.isListItemReference = true;
                return;
            }
        }

        if self.isConstantReference || self.isListItemReference {
            return;
        }

        if self.runtimeVarRef.borrow().is_none() {
            *self.runtimeVarRef.borrow_mut() =
                Some(RuntimeVariableReference::new(self.name.clone()));
        }

        let parsedPath = Path::new_overload_2(self.pathIdentifiers.clone());
        let mut synthetic_root = Object::with_kind(ObjectKind::Story);
        synthetic_root.content = context.content.clone();

        if let Some(targetForCount) = parsedPath.ResolveFromContext(&synthetic_root) {
            if let Some(runtime_var_ref) = &mut *self.runtimeVarRef.borrow_mut() {
                runtime_var_ref.set_pathForCount(Some(targetForCount.get_runtimePath()));
                runtime_var_ref.set_name(None);
            }
            return;
        }

        if self.path.len() > 1 {
            context.Error(
                format!(
                    "Could not find target for read count: {}",
                    parsedPath.ToString()
                ),
                Default::default(),
                false,
            );
            return;
        }

        if !context
            .ResolveVariableWithName(self.name.clone(), Default::default())
            .found
        {
            context.Error(
                format!("Unresolved variable: {}", self.ToString()),
                Default::default(),
                false,
            );
        }
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        self.path.join(".")
    }

    // C# signature: string name { get; }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    // C# signature: Identifier identifier { get; }
    pub fn get_identifier(&self) -> Option<Identifier> {
        if !self.pathIdentifiers.is_empty() {
            let name = self.path.join(".");
            let debugMetadata = self
                .pathIdentifiers
                .iter()
                .filter_map(|identifier| identifier.debugMetadata.clone())
                .reduce(|acc, dm| acc.Merge(&dm));
            Some(Identifier {
                name: Some(name),
                debugMetadata,
            })
        } else {
            self.singleIdentifier.clone()
        }
    }

    // C# signature: List<string> path { get; }
    pub fn get_path(&self) -> &[String] {
        &self.path
    }

    // C# signature: Runtime.VariableReference runtimeVarRef { get; }
    pub fn get_runtimeVarRef(&self) -> Option<RuntimeVariableReference> {
        self.runtimeVarRef.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::VariableReference;
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::Knot::Knot;
    use crate::ParsedHierarchy::Object::Object;
    use crate::ParsedHierarchy::Story::Story;

    #[test]
    fn builds_name_and_identifier() {
        let var = VariableReference::new(vec![
            Identifier {
                name: Some("alpha".to_string()),
                debugMetadata: None,
            },
            Identifier {
                name: Some("beta".to_string()),
                debugMetadata: None,
            },
        ]);

        assert_eq!(var.get_name(), "alpha.beta");
        assert_eq!(var.ToString(), "alpha.beta");
        assert_eq!(var.get_path(), &["alpha".to_string(), "beta".to_string()]);
    }

    #[test]
    fn stores_runtime_variable_reference_when_generated() {
        let var = VariableReference::new(vec![Identifier {
            name: Some("score".to_string()),
            debugMetadata: None,
        }]);
        let mut container = ink_runtime::Container::Container::new();

        var.GenerateIntoContainer(&mut container);

        assert!(var.get_runtimeVarRef().is_some());
        assert_eq!(container.get_content().len(), 1);
    }

    #[test]
    fn resolves_read_count_targets_from_story_content() {
        let mut knot_obj = Object::from_knot(Knot::new(
            Identifier {
                name: Some("intro".to_string()),
                debugMetadata: None,
            },
            vec![],
            vec![],
            false,
        ));
        let _ = knot_obj.EnsureRuntimeObject();

        let mut story = Story::new(vec![knot_obj], false);
        let mut reference = VariableReference::new(vec![Identifier {
            name: Some("intro".to_string()),
            debugMetadata: None,
        }]);

        reference.ResolveReferences(&mut story);

        let runtime_ref = reference
            .get_runtimeVarRef()
            .expect("runtime variable reference");
        assert!(runtime_ref.get_pathForCount().is_some());
        assert!(runtime_ref.get_name().is_none());
    }
}
