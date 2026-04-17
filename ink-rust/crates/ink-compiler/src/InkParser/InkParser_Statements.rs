// Source: ink-c-sharp/compiler/InkParser/InkParser_Statements.cs

use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::ContentList::{ContentList, ContentListItem};
use crate::ParsedHierarchy::Divert::Divert;
use crate::ParsedHierarchy::FlowLevel::FlowLevel;
use crate::ParsedHierarchy::Object::{Object, ObjectKind};
use crate::ParsedHierarchy::Return::Return;
use crate::ParsedHierarchy::Tag::Tag;
use crate::ParsedHierarchy::Text::Text;
use crate::ParsedHierarchy::TunnelOnwards::TunnelOnwards;
use crate::ParsedHierarchy::VariableAssignment::VariableAssignment;
use ink_runtime::Container::Container;
use ink_runtime::Container::ContentItem;
use std::any::Any;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatementLevel {
    InnerBlock,
    Stitch,
    Knot,
    Top,
}

impl InkParser {
    // C# signature: protected List<Parsed.Object> StatementsAtLevel(StatementLevel level)
    pub fn StatementsAtLevel(&mut self, level: StatementLevel) -> Option<Vec<Object>> {
        if level == StatementLevel::InnerBlock {
            if self.GatherDashes().is_some() {
                self.Error("You can't use a gather (the dashes) within the { curly braces } context. For multi-line sequences and conditions, you should only use one dash.".to_string());
            }
        }

        let mut results = Vec::<Object>::new();
        loop {
            let _ = self.Whitespace();

            if self.StatementsBreakForLevel(level).is_some() {
                break;
            }

            let Some(statement) = self.StatementAtLevel(level) else {
                break;
            };
            results.push(statement);
        }

        Some(results)
    }

    // C# signature: protected object StatementAtLevel(StatementLevel level)
    pub fn StatementAtLevel(&mut self, level: StatementLevel) -> Option<Object> {
        let statement = self
            .ParseObject(|parser| parser.MultiDivert())
            .map(|pieces| Self::wrap_divert_line(pieces))
            .or_else(|| {
                if level >= StatementLevel::Top {
                    self.ParseObject(|parser| parser.KnotDefinition())
                } else {
                    None
                }
            })
            .or_else(|| {
                self.ParseObject(|parser| parser.Choice())
                    .map(Self::wrap_choice)
            })
            .or_else(|| {
                self.ParseObject(|parser| parser.Gather())
                    .map(Self::wrap_gather)
            })
            .or_else(|| {
                if level >= StatementLevel::Knot {
                    self.ParseObject(|parser| parser.StitchDefinition())
                } else {
                    None
                }
            })
            .or_else(|| {
                self.ParseObject(|parser| parser.AuthorWarning())
                    .map(Self::wrap_author_warning)
            })
            .or_else(|| {
                self.ParseObject(|parser| parser.IncludeStatement())
                    .map(Self::wrap_included_file)
            })
            .or_else(|| {
                self.ParseObject(|parser| parser.LogicLine())
                    .map(Self::wrap_logic_line)
            })
            .or_else(|| {
                self.ParseObject(|parser| parser.LineOfMixedTextAndLogic())
                    .map(Self::wrap_content_line)
            });

        statement
    }

    // C# signature: protected object StatementsBreakForLevel(StatementLevel level)
    pub fn StatementsBreakForLevel(&mut self, level: StatementLevel) -> Option<Object> {
        let _ = self.Whitespace();

        let break_match = if level <= StatementLevel::Knot {
            self.Peek(|parser| parser.KnotDeclaration().map(|_| Object::new()))
        } else {
            None
        }
        .or_else(|| {
            if level <= StatementLevel::Stitch {
                self.Peek(|parser| parser.StitchDeclaration().map(|_| Object::new()))
            } else {
                None
            }
        })
        .or_else(|| {
            if level <= StatementLevel::InnerBlock {
                self.Peek(|parser| parser.ParseDashNotArrow().map(|_| Object::new()))
                    .or_else(|| {
                        self.Peek(|parser| {
                            parser.ParseString("}".to_string()).map(|_| Object::new())
                        })
                    })
            } else {
                None
            }
        });

        break_match
    }

    pub fn SkipToNextLine(&mut self) -> Option<()> {
        self.ParseUntilCharactersFromString("\n\r".to_string());
        let _ = self.ParseNewline();
        Some(())
    }

    // C# signature: protected ParseRule Line(ParseRule inlineRule)
    pub fn Line<T, R>(&mut self, mut inlineRule: R) -> impl FnMut(&mut Self) -> Option<T>
    where
        R: FnMut(&mut Self) -> Option<T> + 'static,
        T: std::any::Any + 'static,
    {
        move |parser| {
            let result = parser.ParseObject(&mut inlineRule);
            if result.is_none() {
                return None;
            }

            if parser.ParseNewline().is_none() {
                parser.Error("Expected end of line".to_string());
                let _ = parser.SkipToNextLine();
            }

            result
        }
    }

    fn wrap_choice(mut choice: crate::ParsedHierarchy::Choice::Choice) -> Object {
        let mut obj = Object::with_kind(ObjectKind::WeavePoint);
        obj.set_identifier(choice.get_name().map(|name| {
            crate::ParsedHierarchy::Identifier::Identifier {
                name: Some(name.to_string()),
                debugMetadata: None,
            }
        }));
        obj.set_indentationDepth(choice.get_indentationDepth());
        if let ContentItem::Container(container) = choice.GenerateRuntimeObject() {
            obj.set_runtimeObject(Some(*container));
        }
        obj
    }

    fn wrap_gather(mut gather: crate::ParsedHierarchy::Gather::Gather) -> Object {
        let mut obj = Object::with_kind(ObjectKind::WeavePoint);
        obj.set_identifier(gather.get_identifier().cloned());
        obj.set_indentationDepth(gather.get_indentationDepth());
        if let ContentItem::Container(container) = gather.GenerateRuntimeObject() {
            obj.set_runtimeObject(Some(*container));
        }
        obj
    }

    fn wrap_included_file(included: crate::ParsedHierarchy::IncludedFile::IncludedFile) -> Object {
        let mut obj = Object::with_kind(ObjectKind::Plain);
        obj.content = included.get_includedStory().content.clone();
        let _ = included.GenerateRuntimeObject();
        obj
    }

    fn wrap_author_warning(
        _warning: crate::ParsedHierarchy::AuthorWarning::AuthorWarning,
    ) -> Object {
        Object::with_kind(ObjectKind::Plain)
    }

    fn wrap_logic_line(result: Box<dyn Any>) -> Object {
        let result = match result.downcast::<ContentList>() {
            Ok(mut content_list) => {
                let mut obj = Object::with_kind(ObjectKind::Plain);
                obj.set_runtimeObject(Some(content_list.GenerateRuntimeObject()));
                return obj;
            }
            Err(result) => result,
        };

        let result = match result.downcast::<crate::ParsedHierarchy::Expression::Expression>() {
            Ok(expression) => {
                let mut obj = Object::with_kind(ObjectKind::Plain);
                obj.set_runtimeObject(Some(expression.GenerateRuntimeObject()));
                return obj;
            }
            Err(result) => result,
        };

        let result = match result.downcast::<Return>() {
            Ok(returned) => {
                let mut obj = Object::with_kind(ObjectKind::Plain);
                obj.set_runtimeObject(Some(returned.GenerateRuntimeObject()));
                return obj;
            }
            Err(result) => result,
        };

        if let Ok(assignment) = result.downcast::<VariableAssignment>() {
            let mut obj = Object::with_kind(ObjectKind::Plain);
            if let Some(runtime_object) = assignment.clone().GenerateRuntimeObject() {
                if let ContentItem::Container(container) = runtime_object {
                    obj.set_runtimeObject(Some(*container));
                }
            }
            return obj;
        }

        Object::with_kind(ObjectKind::Plain)
    }

    fn wrap_content_line(items: Vec<ContentListItem>) -> Object {
        let mut content_list = ContentList::new(items);
        let runtime = content_list.GenerateRuntimeObject();
        let mut obj = Object::with_kind(ObjectKind::Plain);
        obj.set_runtimeObject(Some(runtime));
        obj
    }

    fn wrap_divert_line(pieces: Vec<crate::InkParser::InkParser_Divert::DivertPiece>) -> Object {
        let mut items = Vec::new();
        for piece in pieces {
            match piece {
                crate::InkParser::InkParser_Divert::DivertPiece::Divert(divert) => {
                    items.push(ContentListItem::Divert(divert));
                }
                crate::InkParser::InkParser_Divert::DivertPiece::TunnelOnwards(tunnel) => {
                    let mut parsed_tunnel = TunnelOnwards::new();
                    parsed_tunnel.set_divertAfter(Some(tunnel));
                    items.push(ContentListItem::TunnelOnwards(parsed_tunnel));
                }
            }
        }
        Self::wrap_content_line(items)
    }
}

#[cfg(test)]
mod tests {
    use super::StatementLevel;

    #[test]
    fn statement_levels_sort_in_expected_order() {
        assert!(StatementLevel::InnerBlock < StatementLevel::Stitch);
        assert!(StatementLevel::Stitch < StatementLevel::Knot);
        assert!(StatementLevel::Knot < StatementLevel::Top);
    }
}
