// Source: ink-c-sharp/compiler/InkParser/InkParser_Choices.cs

use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::Choice::Choice;
use crate::ParsedHierarchy::ContentList::{ContentList, ContentListItem};
use crate::ParsedHierarchy::Expression::{Expression, MultipleConditionExpression};
use crate::ParsedHierarchy::Gather::Gather;
use crate::ParsedHierarchy::Identifier::Identifier;

impl InkParser {
    // C# signature: protected Choice Choice()
    pub fn Choice(&mut self) -> Option<Choice> {
        let mut once_only_choice = true;
        let bullets = if self
            .Peek(|parser| parser.parse_choice_bullets("*"))
            .is_some()
        {
            self.parse_choice_bullets("*")?
        } else if self
            .Peek(|parser| parser.parse_choice_bullets("+"))
            .is_some()
        {
            once_only_choice = false;
            self.parse_choice_bullets("+")?
        } else {
            return None;
        };

        let optional_name = self.BracketedName();

        self.Whitespace();
        if optional_name.is_some() {
            let _ = self.Newline();
        }

        let condition_expr = self.ChoiceCondition();

        self.Whitespace();

        let mut start_content = None;
        let mut option_only_content = None;
        let mut inner_content = None;

        if let Some(start_text_and_logic) = self.MixedTextAndLogic() {
            start_content = Some(ContentList::new(start_text_and_logic));
        }

        let has_weave_style_inline_brackets = self.ParseString("[".to_string()).is_some();
        if has_weave_style_inline_brackets {
            let option_only_text_and_logic = self.MixedTextAndLogic();
            if let Some(option_only_text_and_logic) = option_only_text_and_logic {
                option_only_content = Some(ContentList::new(option_only_text_and_logic));
            }

            if self.ParseString("]".to_string()).is_none() {
                self.Error("Expected closing ']' for weave-style option".to_string());
                return None;
            }

            if let Some(option_only) = option_only_content.as_mut() {
                option_only.TrimTrailingWhitespace();
            }

            if let Some(inner_text_and_logic) = self.MixedTextAndLogic() {
                inner_content = Some(ContentList::new(inner_text_and_logic));
            }
        }

        self.Whitespace();

        if let Some(content) = inner_content.as_mut() {
            self.EndTagIfNecessary_overload_2(content);
        } else if let Some(content) = start_content.as_mut() {
            self.EndTagIfNecessary_overload_2(content);
        }

        let diverts = self.MultiDivert();

        self.Whitespace();

        let empty_content =
            start_content.is_none() && inner_content.is_none() && option_only_content.is_none();
        if empty_content && diverts.is_none() {
            self.Warning("Choice is completely empty. Interpretting as a default fallback choice. Add a divert arrow to remove this warning: * ->".to_string());
        } else if start_content.is_none()
            && has_weave_style_inline_brackets
            && option_only_content.is_none()
        {
            self.Warning(
                "Blank choice - if you intended a default fallback choice, use the `* ->` syntax"
                    .to_string(),
            );
        }

        let mut inner_content = inner_content.unwrap_or_default();
        if let Some(diverts) = diverts {
            self.EndTagIfNecessary_overload_2(&mut inner_content);

            for div_obj in diverts {
                match div_obj {
                    crate::InkParser::InkParser_Divert::DivertPiece::Divert(divert) => {
                        if !divert.get_isEmpty() {
                            inner_content.AddContent(ContentListItem::Divert(divert));
                        }
                    }
                    crate::InkParser::InkParser_Divert::DivertPiece::TunnelOnwards(divert) => {
                        inner_content.AddContent(ContentListItem::Divert(divert));
                    }
                }
            }
        }

        self.EndTagIfNecessary_overload_2(&mut inner_content);
        inner_content.AddContent(ContentListItem::from(
            crate::ParsedHierarchy::Text::Text::new("\n".to_string()),
        ));

        let mut choice = Choice::new(start_content, option_only_content, Some(inner_content));
        choice.set_identifier(optional_name);
        choice.set_indentationDepth(bullets);
        choice.set_hasWeaveStyleInlineBrackets(has_weave_style_inline_brackets);
        choice.set_condition(condition_expr);
        choice.set_onceOnly(once_only_choice);
        choice.set_isInvisibleDefault(empty_content);

        Some(choice)
    }

    fn parse_choice_bullets(&mut self, bullet: &str) -> Option<i32> {
        let mut count = 0;
        loop {
            let _ = self.Whitespace();
            if self.ParseString(bullet.to_string()).is_some() {
                count += 1;
            } else {
                break;
            }
        }

        if count == 0 {
            None
        } else {
            Some(count)
        }
    }

    // C# signature: protected Expression ChoiceCondition()
    pub fn ChoiceCondition(&mut self) -> Option<Expression> {
        let mut conditions = Vec::new();
        if let Some(first) = self.ChoiceSingleCondition() {
            conditions.push(first);
        } else {
            return None;
        }

        loop {
            let rule_id = self.parser_mut().BeginRule();
            if self.ChoiceConditionsSpace().is_none() {
                self.parser_mut().CancelRule(rule_id);
                break;
            }
            match self.ChoiceSingleCondition() {
                Some(condition) => {
                    self.parser_mut().SucceedRule(rule_id, ());
                    conditions.push(condition);
                }
                None => {
                    self.parser_mut().CancelRule(rule_id);
                    break;
                }
            }
        }

        if conditions.len() == 1 {
            Some(conditions.remove(0))
        } else {
            Some(Expression::from_kind(
                crate::ParsedHierarchy::Expression::ExpressionKind::MultipleCondition(
                    MultipleConditionExpression::new(conditions),
                ),
            ))
        }
    }

    // C# signature: protected object ChoiceConditionsSpace()
    pub fn ChoiceConditionsSpace(&mut self) -> Option<()> {
        let _ = self.Newline();
        let _ = self.Whitespace();
        Some(())
    }

    // C# signature: protected Expression ChoiceSingleCondition()
    pub fn ChoiceSingleCondition(&mut self) -> Option<Expression> {
        self.ParseString("{".to_string())?;

        let cond_expr = self.Expression();
        if cond_expr.is_none() {
            self.Error("choice condition inside { }".to_string());
            return None;
        }

        let cond_expr = cond_expr.unwrap();
        self.DisallowIncrement(&cond_expr);

        if self.ParseString("}".to_string()).is_none() {
            self.Error("closing '}' for choice condition".to_string());
            return None;
        }

        Some(cond_expr)
    }

    // C# signature: protected Gather Gather()
    pub fn Gather(&mut self) -> Option<Gather> {
        let gather_dash_count = self.GatherDashes()?;
        let optional_name = self.BracketedName();

        let gather = Gather::new(optional_name.unwrap_or_default(), gather_dash_count);
        let _ = gather.get_name();
        let _ = self.Newline();
        Some(gather)
    }

    // C# signature: protected object GatherDashes()
    pub fn GatherDashes(&mut self) -> Option<i32> {
        let _ = self.Whitespace();

        let mut gather_dash_count = 0;
        while self.ParseDashNotArrow().is_some() {
            gather_dash_count += 1;
            let _ = self.Whitespace();
        }

        if gather_dash_count == 0 {
            None
        } else {
            Some(gather_dash_count)
        }
    }

    // C# signature: protected object ParseDashNotArrow()
    pub fn ParseDashNotArrow(&mut self) -> Option<()> {
        if self
            .Peek(|parser| parser.ParseString("->".to_string()))
            .is_some()
        {
            return None;
        }

        if self.ParseString("-".to_string()).is_some() {
            Some(())
        } else {
            None
        }
    }

    // C# signature: protected Identifier BracketedName()
    pub fn BracketedName(&mut self) -> Option<Identifier> {
        self.ParseString("(".to_string())?;
        let _ = self.Whitespace();

        let name = self.IdentifierWithMetadata()?;

        let _ = self.Whitespace();
        if self.ParseString(")".to_string()).is_none() {
            self.Error("closing ')' for bracketed name".to_string());
            return None;
        }

        Some(name)
    }
}

#[cfg(test)]
mod tests {
    use super::InkParser;

    #[test]
    fn parses_gather_dashes_and_bracketed_names() {
        let mut parser = InkParser::new("-- (label)\n".to_string(), None, None, None);
        let gather = parser.Gather().expect("gather");
        assert_eq!(gather.get_indentationDepth(), 2);
        assert_eq!(gather.get_name(), Some("label"));
    }
}
