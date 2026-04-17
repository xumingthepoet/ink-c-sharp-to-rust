// Source: ink-c-sharp/compiler/Stats.cs

use crate::ParsedHierarchy::Object::{Object, ObjectKind};
use crate::ParsedHierarchy::Story::Story;
use ink_runtime::ChoicePoint::ChoicePoint;
use ink_runtime::Container::{Container, ContentItem};
use ink_runtime::ControlCommand::ControlCommand;
use ink_runtime::Value::{StringValue, Value};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    pub words: i32,
    pub knots: i32,
    pub stitches: i32,
    pub functions: i32,
    pub choices: i32,
    pub gathers: i32,
    pub diverts: i32,
}

impl Stats {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public static Stats Generate(Ink.Parsed.Story story)
    pub fn Generate(story: &Story) -> Self {
        let mut stats = Self::default();

        for obj in &story.content {
            stats.accumulate_object(obj);
        }

        stats
    }

    fn accumulate_object(&mut self, obj: &Object) {
        match obj.kind {
            ObjectKind::Knot => {
                self.knots += 1;
                if obj.isFunction {
                    self.functions += 1;
                }
            }
            ObjectKind::Stitch => {
                self.stitches += 1;
            }
            ObjectKind::WeavePoint => {
                if obj
                    .get_runtimeObject()
                    .map(Self::container_has_choice_point)
                    .unwrap_or(false)
                {
                    self.choices += 1;
                } else {
                    self.gathers += 1;
                }
            }
            _ => {}
        }

        if let Some(runtime_object) = obj.get_runtimeObject() {
            self.accumulate_runtime_container(runtime_object);
        }

        for child in obj.get_content() {
            self.accumulate_object(child);
        }
    }

    fn accumulate_runtime_container(&mut self, container: &Container) {
        for content in container.get_content() {
            match content {
                ContentItem::Value(Value::String(StringValue { value, .. })) => {
                    self.words += Self::count_words(value);
                }
                ContentItem::Divert(_) => {
                    self.diverts += 1;
                }
                ContentItem::Container(child) => {
                    self.accumulate_runtime_container(child);
                }
                _ => {}
            }
        }
    }

    fn container_has_choice_point(container: &Container) -> bool {
        container.get_content().iter().any(|content| match content {
            ContentItem::ChoicePoint(_) => true,
            ContentItem::Container(child) => Self::container_has_choice_point(child),
            _ => false,
        })
    }

    fn count_words(text: &str) -> i32 {
        let mut words = 0;
        let mut was_whitespace = true;

        for c in text.chars() {
            if matches!(c, ' ' | '\t' | '\n' | '\r') {
                was_whitespace = true;
            } else if was_whitespace {
                words += 1;
                was_whitespace = false;
            }
        }

        words
    }
}

#[cfg(test)]
mod tests {
    use super::Stats;
    use crate::ParsedHierarchy::Identifier::Identifier;
    use crate::ParsedHierarchy::Object::{Object, ObjectKind};
    use crate::ParsedHierarchy::Story::Story;
    use ink_runtime::ChoicePoint::ChoicePoint;
    use ink_runtime::Container::Container;
    use ink_runtime::ControlCommand::ControlCommand;
    use ink_runtime::DebugMetadata::DebugMetadata;
    use ink_runtime::Divert::Divert;
    use ink_runtime::Value::{StringValue, Value};

    fn string_container(text: &str) -> Container {
        let mut container = Container::new();
        container.AddContent(Value::String(StringValue::new(text.to_string())));
        container
    }

    #[test]
    fn counts_words_and_tree_nodes() {
        let mut story = Story::new(vec![], false);

        let mut function_knot = Object::with_kind(ObjectKind::Knot);
        function_knot.isFunction = true;
        function_knot.set_runtimeObject(Some({
            let mut container = string_container("hello world");
            container.AddContent(Divert::new());
            container
        }));

        let mut knot = Object::with_kind(ObjectKind::Knot);
        knot.set_runtimeObject(Some({
            let mut container = string_container("one two three");
            container.AddContent(ControlCommand::End());
            container
        }));

        let mut stitch = Object::with_kind(ObjectKind::Stitch);
        stitch.set_runtimeObject(Some(Container::new()));

        let mut choice = Object::with_kind(ObjectKind::WeavePoint);
        choice.set_runtimeObject(Some({
            let mut container = Container::new();
            container.AddContent(ChoicePoint::new(false));
            container
        }));

        let mut gather = Object::with_kind(ObjectKind::WeavePoint);
        gather.set_debugMetadata(Some(DebugMetadata::new()));
        gather.set_runtimeObject(Some(Container::new()));

        story.content = vec![function_knot, knot, stitch, choice, gather];

        let stats = Stats::Generate(&story);

        assert_eq!(stats.words, 5);
        assert_eq!(stats.knots, 2);
        assert_eq!(stats.functions, 1);
        assert_eq!(stats.stitches, 1);
        assert_eq!(stats.choices, 1);
        assert_eq!(stats.gathers, 1);
        assert_eq!(stats.diverts, 1);
    }
}
