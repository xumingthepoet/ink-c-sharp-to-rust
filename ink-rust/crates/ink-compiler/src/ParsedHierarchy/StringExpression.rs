// Source: ink-c-sharp/compiler/ParsedHierarchy/StringExpression.cs

use crate::ParsedHierarchy::ContentList::ContentListItem;
use crate::ParsedHierarchy::Text::Text;
use ink_runtime::Container::{Container as RuntimeContainer, ContentItem};
use ink_runtime::ControlCommand::ControlCommand;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Default)]
pub struct StringExpression {
    content: Vec<ContentListItem>,
}

impl PartialEq for StringExpression {
    fn eq(&self, other: &Self) -> bool {
        self.get_isSingleString()
            && other.get_isSingleString()
            && self.ToString() == other.ToString()
    }
}

impl StringExpression {
    // C# signature: public StringExpression (List<Parsed.Object> content)
    pub fn new(content: Vec<ContentListItem>) -> Self {
        Self { content }
    }

    // C# signature: public override void GenerateIntoContainer (Runtime.Container container)
    pub fn GenerateIntoContainer(&self, container: &mut RuntimeContainer) {
        container.AddContent(ControlCommand::BeginString());

        for content in &self.content {
            Self::append_content_to_runtime(container, content);
        }

        container.AddContent(ControlCommand::EndString());
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        self.content
            .iter()
            .map(|content| content.to_string())
            .collect()
    }

    // C# signature: public override bool Equals (object obj)
    pub fn Equals(&self, obj: &StringExpression) -> bool {
        if !self.get_isSingleString() || !obj.get_isSingleString() {
            return false;
        }

        self.ToString() == obj.ToString()
    }

    // C# signature: public override int GetHashCode ()
    pub fn GetHashCode(&self) -> i32 {
        let mut hasher = DefaultHasher::new();
        self.ToString().hash(&mut hasher);
        hasher.finish() as i32
    }

    // C# signature: bool isSingleString { get; }
    pub fn get_isSingleString(&self) -> bool {
        if self.content.len() != 1 {
            return false;
        }

        matches!(self.content.first(), Some(ContentListItem::Text(_)))
    }

    pub fn ResolveReferences(&mut self, _context: &mut crate::ParsedHierarchy::Story::Story) {
        for content in &mut self.content {
            match content {
                ContentListItem::Expression(expression) => expression.ResolveReferences(_context),
                ContentListItem::ContentList(content_list) => {
                    content_list.ResolveReferences(_context);
                }
                ContentListItem::Divert(_) => {}
                ContentListItem::List(list) => list.ResolveReferences(_context),
                ContentListItem::VariableAssignment(variable_assignment) => {
                    variable_assignment.ResolveReferences(_context)
                }
                ContentListItem::Gather(gather) => gather.ResolveReferences(_context),
                ContentListItem::ConstantDeclaration(declaration) => {
                    declaration.ResolveReferences(_context)
                }
                ContentListItem::Object(object) => object.ResolveReferences(_context),
                _ => {}
            }
        }
    }

    fn append_content_to_runtime(container: &mut RuntimeContainer, content: &ContentListItem) {
        match content.clone() {
            ContentListItem::Text(text) => container.AddContent(text.GenerateRuntimeObject()),
            ContentListItem::Tag(tag) => container.AddContent(tag.GenerateRuntimeObject()),
            ContentListItem::Glue(glue) => container.AddContent(glue.GenerateRuntimeObject()),
            ContentListItem::LegacyTag(tag) => container.AddContent(tag.GenerateRuntimeObject()),
            ContentListItem::Expression(expression) => {
                container.AddContent(expression.GenerateRuntimeObject());
            }
            ContentListItem::Divert(mut divert) => {
                container.AddContent(divert.GenerateRuntimeObject())
            }
            ContentListItem::TunnelOnwards(mut tunnel) => {
                container.AddContent(tunnel.GenerateRuntimeObject());
            }
            ContentListItem::List(list) => {
                let mut temp = RuntimeContainer::new();
                list.GenerateIntoContainer(&mut temp);
                container.AddContentsOfContainer(temp);
            }
            ContentListItem::VariableAssignment(mut variable_assignment) => {
                if let Some(runtime_object) = variable_assignment.GenerateRuntimeObject() {
                    container.AddContent(runtime_object);
                }
            }
            ContentListItem::Return(returned) => {
                container.AddContent(returned.GenerateRuntimeObject())
            }
            ContentListItem::Gather(mut gather) => {
                container.AddContent(gather.GenerateRuntimeObject())
            }
            ContentListItem::ContentList(mut content_list) => {
                let generated = content_list.GenerateRuntimeObject();
                container.AddContent(generated);
            }
            ContentListItem::AuthorWarning(warning) => {
                let _ = warning.GenerateRuntimeObject();
            }
            ContentListItem::ConstantDeclaration(mut declaration) => {
                let _ = declaration.GenerateRuntimeObject();
            }
            ContentListItem::IncludedFile(included) => {
                let _ = included.GenerateRuntimeObject();
            }
            ContentListItem::Object(object) => {
                if let Some(runtime_object) = object.get_runtimeObject().cloned() {
                    container.AddContent(runtime_object);
                }
            }
        }
    }

    pub fn get_content(&self) -> &[ContentListItem] {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::StringExpression;
    use crate::ParsedHierarchy::ContentList::ContentListItem;
    use crate::ParsedHierarchy::Text::Text;
    use ink_runtime::Container::{Container, ContentItem};
    use ink_runtime::ControlCommand::CommandType;
    use ink_runtime::Value::{StringValue, Value};

    #[test]
    fn generates_string_delimiters_and_content() {
        let expr =
            StringExpression::new(vec![ContentListItem::from(Text::new("hello".to_string()))]);

        let mut container = Container::new();
        expr.GenerateIntoContainer(&mut container);

        assert_eq!(container.get_content().len(), 3);
        assert!(matches!(
            container.get_content()[0],
            ContentItem::ControlCommand(ref command)
                if command.get_commandType() == CommandType::BeginString
        ));
        assert!(matches!(
            container.get_content()[1],
            ContentItem::Value(Value::String(StringValue { .. }))
        ));
        assert!(matches!(
            container.get_content()[2],
            ContentItem::ControlCommand(ref command)
                if command.get_commandType() == CommandType::EndString
        ));
    }

    #[test]
    fn compares_single_string_expressions_by_text() {
        let lhs =
            StringExpression::new(vec![ContentListItem::from(Text::new("hello".to_string()))]);
        let rhs =
            StringExpression::new(vec![ContentListItem::from(Text::new("hello".to_string()))]);
        let composite = StringExpression::new(vec![
            ContentListItem::from(Text::new("he".to_string())),
            ContentListItem::from(Text::new("llo".to_string())),
        ]);

        assert!(lhs.get_isSingleString());
        assert!(lhs.Equals(&rhs));
        assert!(!lhs.Equals(&composite));
        assert_eq!(lhs.ToString(), "hello");
    }
}
