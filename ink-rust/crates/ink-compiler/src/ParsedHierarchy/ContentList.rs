// Source: ink-c-sharp/compiler/ParsedHierarchy/ContentList.cs

use crate::ParsedHierarchy::AuthorWarning::AuthorWarning;
use crate::ParsedHierarchy::ConstantDeclaration::ConstantDeclaration;
use crate::ParsedHierarchy::Divert::Divert;
use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::Gather::Gather;
use crate::ParsedHierarchy::IncludedFile::IncludedFile;
use crate::ParsedHierarchy::List::List;
use crate::ParsedHierarchy::Return::Return;
use crate::ParsedHierarchy::Tag::Tag;
use crate::ParsedHierarchy::Text::Text;
use crate::ParsedHierarchy::TunnelOnwards::TunnelOnwards;
use crate::ParsedHierarchy::VariableAssignment::VariableAssignment;
use crate::ParsedHierarchy::Wrap::{Glue, LegacyTag};
use ink_runtime::Container::{Container as RuntimeContainer, ContentItem};

#[derive(Clone, Debug)]
pub enum ContentListItem {
    Text(Text),
    Tag(Tag),
    Glue(Glue),
    LegacyTag(LegacyTag),
    Expression(Expression),
    Divert(Divert),
    TunnelOnwards(TunnelOnwards),
    List(List),
    VariableAssignment(VariableAssignment),
    Return(Return),
    Gather(Gather),
    ContentList(Box<ContentList>),
    AuthorWarning(AuthorWarning),
    ConstantDeclaration(ConstantDeclaration),
    IncludedFile(IncludedFile),
}

impl From<Text> for ContentListItem {
    fn from(value: Text) -> Self {
        Self::Text(value)
    }
}

impl From<Tag> for ContentListItem {
    fn from(value: Tag) -> Self {
        Self::Tag(value)
    }
}

impl From<Glue> for ContentListItem {
    fn from(value: Glue) -> Self {
        Self::Glue(value)
    }
}

impl From<LegacyTag> for ContentListItem {
    fn from(value: LegacyTag) -> Self {
        Self::LegacyTag(value)
    }
}

impl From<Expression> for ContentListItem {
    fn from(value: Expression) -> Self {
        Self::Expression(value)
    }
}

impl From<Divert> for ContentListItem {
    fn from(value: Divert) -> Self {
        Self::Divert(value)
    }
}

impl From<TunnelOnwards> for ContentListItem {
    fn from(value: TunnelOnwards) -> Self {
        Self::TunnelOnwards(value)
    }
}

impl From<List> for ContentListItem {
    fn from(value: List) -> Self {
        Self::List(value)
    }
}

impl From<VariableAssignment> for ContentListItem {
    fn from(value: VariableAssignment) -> Self {
        Self::VariableAssignment(value)
    }
}

impl From<Return> for ContentListItem {
    fn from(value: Return) -> Self {
        Self::Return(value)
    }
}

impl From<Gather> for ContentListItem {
    fn from(value: Gather) -> Self {
        Self::Gather(value)
    }
}

impl From<ContentList> for ContentListItem {
    fn from(value: ContentList) -> Self {
        Self::ContentList(Box::new(value))
    }
}

impl From<AuthorWarning> for ContentListItem {
    fn from(value: AuthorWarning) -> Self {
        Self::AuthorWarning(value)
    }
}

impl From<ConstantDeclaration> for ContentListItem {
    fn from(value: ConstantDeclaration) -> Self {
        Self::ConstantDeclaration(value)
    }
}

impl From<IncludedFile> for ContentListItem {
    fn from(value: IncludedFile) -> Self {
        Self::IncludedFile(value)
    }
}

impl std::fmt::Display for ContentListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentListItem::Text(text) => write!(f, "{}", text),
            ContentListItem::Tag(tag) => write!(f, "{}", tag.ToString()),
            ContentListItem::Glue(_) => f.write_str("Glue"),
            ContentListItem::LegacyTag(_) => f.write_str("LegacyTag"),
            ContentListItem::Expression(expression) => write!(f, "{}", expression.ToString()),
            ContentListItem::Divert(divert) => write!(f, "{}", divert.ToString()),
            ContentListItem::TunnelOnwards(_) => f.write_str("TunnelOnwards"),
            ContentListItem::List(list) => write!(f, "{}", list.ToString()),
            ContentListItem::VariableAssignment(_) => f.write_str("VariableAssignment"),
            ContentListItem::Return(_) => f.write_str("Return"),
            ContentListItem::Gather(_) => f.write_str("Gather"),
            ContentListItem::ContentList(content_list) => write!(f, "{}", content_list.ToString()),
            ContentListItem::AuthorWarning(_) => f.write_str("AuthorWarning"),
            ContentListItem::ConstantDeclaration(_) => f.write_str("ConstantDeclaration"),
            ContentListItem::IncludedFile(_) => f.write_str("IncludedFile"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ContentList {
    content: Vec<ContentListItem>,
    dontFlatten: bool,
    runtimeContainer: Option<RuntimeContainer>,
}

impl ContentList {
    // C# signature: public ContentList (List<Parsed.Object> objects)
    pub fn new(objects: Vec<ContentListItem>) -> Self {
        Self {
            content: objects,
            ..Default::default()
        }
    }

    // C# signature: public ContentList()
    pub fn new_overload_2() -> Self {
        Default::default()
    }

    // C# signature: public T AddContent<T>(T subContent)
    pub fn AddContent<T>(&mut self, subContent: T) -> T
    where
        T: Into<ContentListItem> + Clone,
    {
        self.content.push(subContent.clone().into());
        subContent
    }

    // C# signature: public void AddContent<T>(List<T> listContent)
    pub fn AddContent_overload_2(&mut self, listContent: Vec<ContentListItem>) {
        for obj in listContent {
            self.content.push(obj);
        }
    }

    // C# signature: public void TrimTrailingWhitespace()
    pub fn TrimTrailingWhitespace(&mut self) {
        while let Some(last) = self.content.last_mut() {
            match last {
                ContentListItem::Text(text) => {
                    text.text = text.text.trim_end_matches([' ', '\t']).to_string();
                    if text.text.is_empty() {
                        self.content.pop();
                        continue;
                    }
                    break;
                }
                _ => break,
            }
        }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> RuntimeContainer {
        let mut container = RuntimeContainer::new();

        for content in &mut self.content {
            Self::append_content_to_runtime(&mut container, content);
        }

        // The original C# code asks the owning Story to remember containers that
        // must not be flattened. That hookup still depends on the higher-level
        // parser/story ownership chain, so the flag is preserved here but the
        // story callback remains the next integration step.
        if self.dontFlatten {
            // intentionally left as a visible compatibility gap
        }

        self.runtimeContainer = Some(container.clone());
        container
    }

    fn append_content_to_runtime(container: &mut RuntimeContainer, content: &mut ContentListItem) {
        match content {
            ContentListItem::Text(text) => {
                container.AddContent(text.GenerateRuntimeObject());
            }
            ContentListItem::Tag(tag) => {
                container.AddContent(tag.GenerateRuntimeObject());
            }
            ContentListItem::Glue(glue) => {
                container.AddContent(glue.GenerateRuntimeObject());
            }
            ContentListItem::LegacyTag(tag) => {
                container.AddContent(tag.GenerateRuntimeObject());
            }
            ContentListItem::Expression(expression) => {
                container.AddContent(expression.GenerateRuntimeObject());
            }
            ContentListItem::Divert(divert) => {
                container.AddContent(divert.GenerateRuntimeObject());
            }
            ContentListItem::TunnelOnwards(tunnel) => {
                container.AddContent(tunnel.GenerateRuntimeObject());
            }
            ContentListItem::List(list) => {
                let mut temp = RuntimeContainer::new();
                list.GenerateIntoContainer(&mut temp);
                container.AddContentsOfContainer(temp);
            }
            ContentListItem::VariableAssignment(variable_assignment) => {
                if let Some(runtime_object) = variable_assignment.GenerateRuntimeObject() {
                    container.AddContent(runtime_object);
                }
            }
            ContentListItem::Return(returned) => {
                container.AddContent(returned.GenerateRuntimeObject());
            }
            ContentListItem::Gather(gather) => {
                container.AddContent(gather.GenerateRuntimeObject());
            }
            ContentListItem::ContentList(content_list) => {
                let generated = content_list.GenerateRuntimeObject();
                container.AddContent(generated);
            }
            ContentListItem::AuthorWarning(warning) => {
                let _ = warning.GenerateRuntimeObject();
            }
            ContentListItem::ConstantDeclaration(declaration) => {
                let _ = declaration.GenerateRuntimeObject();
            }
            ContentListItem::IncludedFile(included) => {
                let _ = included.GenerateRuntimeObject();
            }
        }
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        let items = self
            .content
            .iter()
            .map(|content| content.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!("ContentList({})", items)
    }

    // C# signature: bool dontFlatten { get; }
    pub fn get_dontFlatten(&self) -> bool {
        self.dontFlatten
    }

    pub fn set_dontFlatten(&mut self, value: bool) {
        self.dontFlatten = value;
    }

    // C# signature: Runtime.Container runtimeContainer { get; }
    pub fn get_runtimeContainer(&self) -> Option<RuntimeContainer> {
        self.runtimeContainer.clone()
    }

    pub fn get_content(&self) -> &[ContentListItem] {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::{ContentList, ContentListItem};
    use crate::ParsedHierarchy::Tag::Tag;
    use crate::ParsedHierarchy::Text::Text;
    use ink_runtime::Container::ContentItem;
    use ink_runtime::ControlCommand::CommandType;
    use ink_runtime::Value::{StringValue, Value};

    #[test]
    fn trims_trailing_whitespace_from_terminal_text() {
        let mut list = ContentList::new(vec![
            ContentListItem::from(Text::new("hello ".to_string())),
            ContentListItem::from(Text::new(" \t".to_string())),
        ]);

        list.TrimTrailingWhitespace();

        assert_eq!(list.get_content().len(), 1);
        match &list.get_content()[0] {
            ContentListItem::Text(text) => assert_eq!(text.get_text(), "hello"),
            other => panic!("unexpected item: {:?}", other),
        }
    }

    #[test]
    fn generates_runtime_container_from_mixed_content() {
        let mut list = ContentList::new(vec![
            ContentListItem::from(Text::new("hello".to_string())),
            ContentListItem::from(Tag::new_with_flags(true, false)),
        ]);

        let runtime = list.GenerateRuntimeObject();

        assert_eq!(runtime.get_content().len(), 2);
        assert!(matches!(
            runtime.get_content()[0],
            ContentItem::Value(Value::String(StringValue { .. }))
        ));
        assert!(matches!(
            runtime.get_content()[1],
            ContentItem::ControlCommand(ref command)
                if command.get_commandType() == CommandType::BeginTag
        ));
    }
}
