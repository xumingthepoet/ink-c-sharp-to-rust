// Source: ink-c-sharp/compiler/ParsedHierarchy/StringExpression.cs

use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
use crate::ParsedHierarchy::Text::Text;
use ink_runtime::Container::{Container as RuntimeContainer, ContentItem};
use ink_runtime::ControlCommand::ControlCommand;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StringExpression {
    content: Vec<Expression>,
}

impl StringExpression {
    // C# signature: public StringExpression (List<Parsed.Object> content)
    pub fn new(content: Vec<Expression>) -> Self {
        Self { content }
    }

    // C# signature: public override void GenerateIntoContainer (Runtime.Container container)
    pub fn GenerateIntoContainer(&self, container: &mut RuntimeContainer) {
        container.AddContent(ControlCommand::BeginString());

        for expression in &self.content {
            let mut content_container = RuntimeContainer::new();
            expression.GenerateIntoContainer(&mut content_container);
            container.AddContentsOfContainer(content_container);
        }

        container.AddContent(ControlCommand::EndString());
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        self.content
            .iter()
            .map(|content| content.ToString())
            .collect::<Vec<_>>()
            .join("")
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

        matches!(
            self.content.first().map(|expression| &expression.kind),
            Some(ExpressionKind::Text(_))
        )
    }

    pub fn get_content(&self) -> &[Expression] {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::StringExpression;
    use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
    use crate::ParsedHierarchy::Text::Text;
    use ink_runtime::Container::{Container, ContentItem};
    use ink_runtime::ControlCommand::CommandType;
    use ink_runtime::Value::{StringValue, Value};

    #[test]
    fn generates_string_delimiters_and_content() {
        let expr = StringExpression::new(vec![Expression::from_kind(ExpressionKind::Text(
            Text::new("hello".to_string()),
        ))]);

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
        let lhs = StringExpression::new(vec![Expression::from_kind(ExpressionKind::Text(
            Text::new("hello".to_string()),
        ))]);
        let rhs = StringExpression::new(vec![Expression::from_kind(ExpressionKind::Text(
            Text::new("hello".to_string()),
        ))]);
        let composite = StringExpression::new(vec![
            Expression::from_kind(ExpressionKind::Text(Text::new("he".to_string()))),
            Expression::from_kind(ExpressionKind::Text(Text::new("llo".to_string()))),
        ]);

        assert!(lhs.get_isSingleString());
        assert!(lhs.Equals(&rhs));
        assert!(!lhs.Equals(&composite));
        assert_eq!(lhs.ToString(), "hello");
    }
}
