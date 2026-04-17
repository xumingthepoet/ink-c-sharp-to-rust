// Source: ink-c-sharp/compiler/InkParser/InkParser_Statements.cs

use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::ContentList::{ContentList, ContentListItem};
use crate::ParsedHierarchy::Divert::Divert;
use crate::ParsedHierarchy::FlowLevel::FlowLevel;
use crate::ParsedHierarchy::Object::{Object, ObjectKind};
use crate::ParsedHierarchy::Tag::Tag;
use crate::ParsedHierarchy::Text::Text;
use crate::ParsedHierarchy::TunnelOnwards::TunnelOnwards;
use ink_runtime::Container::Container;
use ink_runtime::Container::ContentItem;

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
            .MultiDivert()
            .map(|pieces| Self::wrap_divert_line(pieces))
            .or_else(|| {
                if level >= StatementLevel::Top {
                    self.KnotDefinition()
                } else {
                    None
                }
            })
            .or_else(|| self.Choice().map(Self::wrap_choice))
            .or_else(|| self.Gather().map(Self::wrap_gather))
            .or_else(|| {
                if level >= StatementLevel::Knot {
                    self.StitchDefinition()
                } else {
                    None
                }
            })
            .or_else(|| self.IncludeStatement().map(Self::wrap_included_file))
            .or_else(|| self.LineOfMixedTextAndLogic().map(Self::wrap_content_line));

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
        let obj = Object::with_kind(ObjectKind::Plain);
        let _ = included.GenerateRuntimeObject();
        obj
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
