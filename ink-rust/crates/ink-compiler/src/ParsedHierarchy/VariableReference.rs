// Source: ink-c-sharp/compiler/ParsedHierarchy/VariableReference.cs

use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Path::Path;
use crate::ParsedHierarchy::Story::Story;
use ink_runtime::VariableReference::VariableReference as RuntimeVariableReference;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VariableReference {
    pub pathIdentifiers: Vec<Identifier>,
    path: Vec<String>,
    name: String,
    singleIdentifier: Option<Identifier>,
    runtimeVarRef: Option<RuntimeVariableReference>,
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
            runtimeVarRef: None,
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

        container.AddContent(RuntimeVariableReference::new(self.name.clone()));
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

        self.runtimeVarRef = Some(RuntimeVariableReference::new(self.name.clone()));

        // Full read-count and variable-resolution parity still depends on
        // the unported object/path ancestry pipeline.
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
    pub fn get_runtimeVarRef(&self) -> Option<&RuntimeVariableReference> {
        self.runtimeVarRef.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::VariableReference;
    use crate::ParsedHierarchy::Identifier::Identifier;

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
}
