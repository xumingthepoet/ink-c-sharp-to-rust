// Source: ink-c-sharp/compiler/InkParser/InkParser_Statements.cs

use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::ContentList::{ContentList, ContentListItem};
use crate::ParsedHierarchy::Divert::Divert;
use crate::ParsedHierarchy::FlowLevel::FlowLevel;
use crate::ParsedHierarchy::Object::Object;
use crate::ParsedHierarchy::Object::ObjectPayload;
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
            .ParseObject(|parser| {
                let pieces = parser.MultiDivert()?;
                if parser.EndOfLine().is_none() {
                    parser.Error("Expected end of line".to_string());
                    let _ = parser.SkipToNextLine();
                }
                Some(pieces)
            })
            .map(|pieces| self.wrap_divert_line(pieces))
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
            .or_else(|| self.parse_declaration_line(|parser| parser.ListDeclaration()))
            .or_else(|| self.parse_declaration_line(|parser| parser.VariableDeclaration()))
            .or_else(|| self.parse_declaration_line(|parser| parser.ConstDeclaration()))
            .or_else(|| {
                let declaration = self.ParseObject(|parser| parser.ExternalDeclaration())?;
                if self.EndOfLine().is_none() {
                    self.Error("Expected end of line".to_string());
                    let _ = self.SkipToNextLine();
                }
                Some(Object::from_external_declaration(declaration))
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

        if level == StatementLevel::Top {
            if matches!(
                statement.as_ref().and_then(|obj| obj.payload.as_ref()),
                Some(ObjectPayload::Return(_))
            ) {
                self.Error("should not have return statement outside of a knot".to_string());
            }
        }

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

    fn parse_declaration_line<R>(&mut self, mut rule: R) -> Option<Object>
    where
        R: FnMut(&mut InkParser) -> Option<Box<dyn Any>>,
    {
        let result = self.ParseObject(&mut rule)?;
        if self.EndOfLine().is_none() {
            self.Error("Expected end of line".to_string());
            let _ = self.SkipToNextLine();
        }
        Some(Self::wrap_logic_line(result))
    }

    fn wrap_choice(choice: crate::ParsedHierarchy::Choice::Choice) -> Object {
        Object::from_choice(choice)
    }

    fn wrap_gather(gather: crate::ParsedHierarchy::Gather::Gather) -> Object {
        Object::from_gather(gather)
    }

    fn wrap_included_file(included: crate::ParsedHierarchy::IncludedFile::IncludedFile) -> Object {
        Object::from_included_file(included)
    }

    fn wrap_author_warning(
        warning: crate::ParsedHierarchy::AuthorWarning::AuthorWarning,
    ) -> Object {
        Object::from_author_warning(warning)
    }

    fn wrap_logic_line(result: Box<dyn Any>) -> Object {
        let result = match result.downcast::<ContentList>() {
            Ok(content_list) => return Object::from_content_list(*content_list),
            Err(result) => result,
        };

        let result = match result.downcast::<crate::ParsedHierarchy::Expression::Expression>() {
            Ok(expression) => return Object::from_expression(*expression),
            Err(result) => result,
        };

        let result = match result.downcast::<Return>() {
            Ok(returned) => return Object::from_return(*returned),
            Err(result) => result,
        };

        let result = match result.downcast::<VariableAssignment>() {
            Ok(assignment) => return Object::from_variable_assignment(*assignment),
            Err(result) => result,
        };

        if let Ok(declaration) =
            result.downcast::<crate::ParsedHierarchy::ConstantDeclaration::ConstantDeclaration>()
        {
            return Object::from_constant_declaration(*declaration);
        }

        Object::new()
    }

    fn wrap_content_line(items: Vec<ContentListItem>) -> Object {
        Object::from_content_list(ContentList::new(items))
    }

    fn wrap_divert_line(
        &mut self,
        pieces: Vec<crate::InkParser::InkParser_Divert::DivertPiece>,
    ) -> Object {
        let mut items = Vec::new();
        for piece in pieces {
            match piece {
                crate::InkParser::InkParser_Divert::DivertPiece::Divert(divert) => {
                    items.push(ContentListItem::Divert(divert));
                }
                crate::InkParser::InkParser_Divert::DivertPiece::TunnelOnwards(tunnel) => {
                    let mut parsed_tunnel = TunnelOnwards::new();
                    if !tunnel.get_isEmpty() {
                        parsed_tunnel.set_divertAfter(Some(tunnel));
                    }
                    items.push(ContentListItem::TunnelOnwards(parsed_tunnel));
                }
            }
        }
        self.EndTagIfNecessary(&mut items);
        Self::wrap_content_line(items)
    }
}

#[cfg(test)]
mod tests {
    use super::StatementLevel;
    use crate::InkParser::InkParser::CustomFlags;
    use crate::InkParser::InkParser::InkParser;
    use crate::InkParser::InkParser_Divert::DivertPiece;
    use crate::ParsedHierarchy::ContentList::ContentListItem;
    use crate::ParsedHierarchy::Divert::Divert;
    use crate::ParsedHierarchy::Object::ObjectPayload;
    use std::sync::{Arc, Mutex};

    #[test]
    fn statement_levels_sort_in_expected_order() {
        assert!(StatementLevel::InnerBlock < StatementLevel::Stitch);
        assert!(StatementLevel::Stitch < StatementLevel::Knot);
        assert!(StatementLevel::Knot < StatementLevel::Top);
    }

    #[test]
    fn top_level_declarations_remain_typed_objects() {
        let mut parser = InkParser::new(
            "VAR score = 5\nCONST MAX = 10\nEXTERNAL host()\n".to_string(),
            None,
            None,
            None,
        );

        let statements = parser.StatementsAtLevel(StatementLevel::Top).unwrap();

        assert!(matches!(
            statements[0].payload.as_ref(),
            Some(ObjectPayload::VariableAssignment(_))
        ));
        assert!(matches!(
            statements[1].payload.as_ref(),
            Some(ObjectPayload::ConstantDeclaration(_))
        ));
        assert!(matches!(
            statements[2].payload.as_ref(),
            Some(ObjectPayload::ExternalDeclaration(_))
        ));
    }

    #[test]
    fn top_level_return_still_parses_but_reports_error() {
        let seen = Arc::new(Mutex::new(None));
        let captured = Arc::clone(&seen);
        let handler = Arc::new(
            move |message: String, line: i32, character: i32, is_warning: bool| {
                *captured.lock().unwrap() = Some((message, line, character, is_warning));
            },
        );

        let mut parser = InkParser::new("~ return 5\n".to_string(), None, Some(handler), None);
        let statements = parser.StatementsAtLevel(StatementLevel::Top).unwrap();

        assert!(matches!(
            statements[0].payload.as_ref(),
            Some(ObjectPayload::Return(_))
        ));
        assert_eq!(
            *seen.lock().unwrap(),
            Some((
                "should not have return statement outside of a knot".to_string(),
                0,
                0,
                false
            ))
        );
    }

    #[test]
    fn top_level_inline_expression_statement_is_parsed() {
        let mut parser = InkParser::new("{1}\n".to_string(), None, None, None);
        let statements = parser.StatementsAtLevel(StatementLevel::Top).unwrap();

        assert!(!statements.is_empty());
        assert!(matches!(
            statements[0].payload.as_ref(),
            Some(ObjectPayload::ContentList(_))
        ));
    }

    #[test]
    fn divert_lines_close_active_tags() {
        let mut parser = InkParser::new(String::new(), None, None, None);
        parser.set_flag(CustomFlags::TagActive, true);

        let object = parser.wrap_divert_line(vec![DivertPiece::Divert(Divert::default())]);
        let content_list = match object.payload {
            Some(ObjectPayload::ContentList(content_list)) => content_list,
            other => panic!("unexpected object payload: {:?}", other),
        };

        assert_eq!(content_list.get_content().len(), 2);
        assert!(matches!(
            &content_list.get_content()[1],
            ContentListItem::Tag(_)
        ));
        assert!(!parser.get_tagActive());
    }
}
