// Source: ink-c-sharp/compiler/InkParser/InkParser_Content.cs

use crate::CharacterSet::CharacterSet;
use crate::InkParser::InkParser::InkParser;
use crate::InkParser::InkParser_Divert::DivertPiece;
use crate::ParsedHierarchy::ContentList::{ContentList, ContentListItem};
use crate::ParsedHierarchy::Divert::Divert;
use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
use crate::ParsedHierarchy::Tag::Tag;
use crate::ParsedHierarchy::Text::Text;
use crate::ParsedHierarchy::TunnelOnwards::TunnelOnwards;
use crate::ParsedHierarchy::Wrap::Glue as ParsedGlue;
use std::any::Any;

impl InkParser {
    fn trim_end_whitespace(
        &mut self,
        mixedTextAndLogicResults: &mut Vec<ContentListItem>,
        terminateWithSpace: bool,
    ) {
        if let Some(ContentListItem::Text(text)) = mixedTextAndLogicResults.last_mut() {
            text.text = text.text.trim_end_matches([' ', '\t']).to_string();
            if terminateWithSpace {
                text.text.push(' ');
            } else if text.text.is_empty() {
                mixedTextAndLogicResults.pop();
                self.trim_end_whitespace(mixedTextAndLogicResults, false);
            }
        }
    }

    // C# signature: protected List<Parsed.Object> LineOfMixedTextAndLogic()
    pub fn LineOfMixedTextAndLogic(&mut self) -> Option<Vec<ContentListItem>> {
        let _ = self.Whitespace();

        let mut result = self.MixedTextAndLogic()?;
        if result.is_empty() {
            return None;
        }

        if let Some(ContentListItem::Text(first_text)) = result.first() {
            if first_text.text.starts_with("return") {
                self.Warning("Do you need a '~' before 'return'? If not, perhaps use a glue: <> (since it's lowercase) or rewrite somehow?".to_string());
            }
        }

        if let Some(last_obj) = result.last() {
            if !matches!(last_obj, ContentListItem::Divert(_)) {
                self.trim_end_whitespace(&mut result, false);
            }
        }

        self.EndTagIfNecessary(&mut result);

        let line_is_pure_tag = matches!(
            result.first(),
            Some(ContentListItem::Tag(tag)) if tag.isStart
        );
        if !line_is_pure_tag {
            result.push(ContentListItem::Text(Text::new("\n".to_string())));
        }

        if self.EndOfLine().is_none() {
            let line_remainder = self.LineRemainder();
            let but_saw = if line_remainder.is_empty() {
                "end of line".to_string()
            } else {
                format!("'{}'", line_remainder)
            };
            self.Error(format!("Expected end of line but saw {}", but_saw));
            let _ = self.SkipToNextLine();
        }

        Some(result)
    }

    // C# signature: protected List<Parsed.Object> MixedTextAndLogic()
    pub fn MixedTextAndLogic(&mut self) -> Option<Vec<ContentListItem>> {
        let disallowed_tilda = self.ParseObject(|parser| {
            parser.Whitespace();
            parser.ParseString("~".to_string())?;
            parser.Whitespace();
            Some(())
        });
        if disallowed_tilda.is_some() {
            self.Error("You shouldn't use a '~' here - tildas are for logic that's on its own line. To do inline logic, use { curly braces } instead".to_string());
        }

        let mut results = Vec::new();
        loop {
            let mut progressed = false;

            if let Some(text) = self.ContentText() {
                results.push(ContentListItem::Text(text));
                progressed = true;
            }

            if let Some(item) = self.InlineLogicOrGlueOrStartTag() {
                results.push(item);
                progressed = true;
            }

            if !progressed {
                break;
            }
        }

        if !self.get_parsingChoice() {
            if let Some(diverts) = self.MultiDivert() {
                self.EndTagIfNecessary(&mut results);
                self.trim_end_whitespace(&mut results, true);
                results.extend(diverts.into_iter().map(|piece| match piece {
                    DivertPiece::Divert(divert) => ContentListItem::Divert(divert),
                    DivertPiece::TunnelOnwards(divert) => {
                        let mut tunnel = TunnelOnwards::new();
                        if !divert.get_isEmpty() {
                            tunnel.set_divertAfter(Some(divert));
                        }
                        ContentListItem::TunnelOnwards(tunnel)
                    }
                }));
            }
        }

        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }

    // C# signature: protected Parsed.Text ContentText()
    pub fn ContentText(&mut self) -> Option<Text> {
        self.ContentTextAllowingEcapeChar()
    }

    // C# signature: protected Parsed.Text ContentTextAllowingEcapeChar()
    pub fn ContentTextAllowingEcapeChar(&mut self) -> Option<Text> {
        let mut sb = String::new();
        let mut saw_any = false;

        loop {
            let str = self.ContentTextNoEscape();
            let got_escape_char = self.ParseString("\\".to_string()).is_some();

            if got_escape_char || str.is_some() {
                saw_any = true;
                if let Some(str) = str {
                    sb.push_str(&str);
                }
                if got_escape_char {
                    sb.push(self.ParseSingleCharacter());
                }
            } else {
                break;
            }
        }

        if saw_any {
            Some(Text::new(sb))
        } else {
            None
        }
    }

    // C# signature: protected string ContentTextNoEscape()
    pub fn ContentTextNoEscape(&mut self) -> Option<String> {
        let mut sb = String::new();

        loop {
            let c = self.get_currentCharacter();
            if c == '\0' {
                break;
            }

            if matches!(c, '-' | '<' | '{' | '}' | '|' | '\n' | '\r' | '\\' | '#')
                || (self.get_parsingChoice() && matches!(c, '[' | ']'))
                || (self.get_parsingStringExpression() && c == '"')
            {
                break;
            }

            sb.push(self.ParseSingleCharacter());
        }

        if sb.is_empty() {
            None
        } else {
            Some(sb)
        }
    }

    pub fn InlineLogicOrGlueOrStartTag(&mut self) -> Option<ContentListItem> {
        self.OneOf(vec![
            Box::new(|parser: &mut InkParser| parser.InlineLogic()),
            Box::new(|parser: &mut InkParser| parser.Glue()),
            Box::new(|parser: &mut InkParser| parser.StartTag()),
        ])
    }

    pub fn Glue(&mut self) -> Option<ContentListItem> {
        if self.ParseString("<>".to_string()).is_some() {
            Some(ContentListItem::from(ParsedGlue::new(
                ink_runtime::Glue::Glue::new(),
            )))
        } else {
            None
        }
    }

    pub fn InlineLogic(&mut self) -> Option<ContentListItem> {
        if self.ParseString("{".to_string()).is_none() {
            return None;
        }

        let was_parsing_string = self.get_parsingStringExpression();
        let was_tag_active = self.get_tagActive();

        let _ = self.Whitespace();

        let logic = self.InnerLogic()?;
        self.DisallowIncrement(&logic as &dyn Any);

        let mut content_list = match logic {
            ContentListItem::ContentList(content_list) => *content_list,
            other => {
                let mut list = ContentList::new_overload_2();
                list.AddContent(other);
                list
            }
        };

        let _ = self.Whitespace();
        if self.ParseString("}".to_string()).is_none() {
            self.Error("closing brace '}' for inline logic".to_string());
            self.set_flag(
                crate::InkParser::InkParser::CustomFlags::ParsingString,
                was_parsing_string,
            );
            return None;
        }

        self.set_flag(
            crate::InkParser::InkParser::CustomFlags::ParsingString,
            was_parsing_string,
        );

        if !was_tag_active {
            self.EndTagIfNecessary_overload_2(&mut content_list);
        }

        Some(ContentListItem::ContentList(Box::new(content_list)))
    }

    pub fn InnerLogic(&mut self) -> Option<ContentListItem> {
        self.Whitespace();
        self.InnerExpression()
    }

    pub fn InnerExpression(&mut self) -> Option<ContentListItem> {
        self.Expression().map(|mut expr| {
            expr.outputWhenComplete = true;
            ContentListItem::Expression(expr)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::InkParser;
    use crate::ParsedHierarchy::ContentList::ContentListItem;

    #[test]
    fn parses_simple_inline_text_and_tag_lines() {
        let mut parser = InkParser::new("hello #world\n".to_string(), None, None, None);
        let line = parser.LineOfMixedTextAndLogic().expect("line");
        assert!(matches!(line[0], ContentListItem::Text(_)));
    }

    #[test]
    fn inner_expression_marks_output_when_complete() {
        let mut parser = InkParser::new("{1}".to_string(), None, None, None);
        let line = parser.LineOfMixedTextAndLogic().expect("line");
        match &line[0] {
            ContentListItem::ContentList(list) => {
                assert!(matches!(list.get_content()[0], ContentListItem::Expression(_)));
                if let ContentListItem::Expression(expr) = &list.get_content()[0] {
                    assert!(expr.get_outputWhenComplete());
                }
            }
            other => panic!("unexpected line item: {other:?}"),
        }
    }
}
