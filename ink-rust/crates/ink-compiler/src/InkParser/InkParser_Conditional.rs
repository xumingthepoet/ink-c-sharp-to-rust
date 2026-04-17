// Source: ink-c-sharp/compiler/InkParser/InkParser_Conditional.cs

use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::Conditional::Conditional;
use crate::ParsedHierarchy::ConditionalSingleBranch::ConditionalSingleBranch;
use crate::ParsedHierarchy::ContentList::{ContentList, ContentListItem};
use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::Object::Object;
use crate::ParsedHierarchy::Object::ObjectKind;

impl InkParser {
    // C# signature: protected Conditional InnerConditionalContent()
    pub fn InnerConditionalContent(&mut self) -> Option<Conditional> {
        let initial_query_expression = self.ParseObject(|parser| parser.ConditionExpression());
        self.InnerConditionalContent_overload_2(initial_query_expression)
    }

    // C# signature: protected Conditional InnerConditionalContent(Expression initialQueryExpression)
    pub fn InnerConditionalContent_overload_2(
        &mut self,
        initialQueryExpression: Option<Expression>,
    ) -> Option<Conditional> {
        let canBeInline = initialQueryExpression.is_some();
        let isInline = self.Newline().is_none();

        if isInline && !canBeInline {
            return None;
        }

        let mut alternatives = if isInline {
            self.InlineConditionalBranches()
        } else {
            self.MultilineConditionalBranches()
        };

        if !isInline && alternatives.is_none() {
            if initialQueryExpression.is_some() {
                let sole_content = self.StatementsAtLevel(
                    crate::InkParser::InkParser_Statements::StatementLevel::InnerBlock,
                );
                if let Some(sole_content) = sole_content {
                    let mut sole_branch = ConditionalSingleBranch::new(sole_content);
                    sole_branch.set_isInline(false);
                    alternatives = Some(vec![sole_branch]);

                    if let Some(mut else_branch) =
                        self.ParseObject(|parser| parser.SingleMultilineCondition())
                    {
                        if !else_branch.get_isElse() {
                            self.Error(
                                "Expected an '- else:' clause here rather than an extra condition"
                                    .to_string(),
                            );
                            else_branch.set_isElse(true);
                        }

                        if let Some(alternatives) = alternatives.as_mut() {
                            alternatives.push(else_branch);
                        }
                    }
                }
            }

            if alternatives.is_none() {
                return None;
            }
        } else if !isInline {
            if let Some(alternatives_ref) = alternatives.as_mut() {
                if alternatives_ref.len() == 1
                    && alternatives_ref[0].get_isElse()
                    && initialQueryExpression.is_some()
                {
                    let mut empty_true_branch = ConditionalSingleBranch::new(Vec::new());
                    empty_true_branch.set_isTrueBranch(true);
                    alternatives_ref.insert(0, empty_true_branch);
                }
            }

            if initialQueryExpression.is_some() {
                let mut earlier_branches_have_own_expression = false;
                if let Some(alternatives_ref) = alternatives.as_mut() {
                    let branch_count = alternatives_ref.len();
                    for (index, branch) in alternatives_ref.iter_mut().enumerate() {
                        let is_last = index == branch_count.saturating_sub(1);

                        if branch.get_ownExpression().is_some() {
                            branch.set_matchingEquality(true);
                            earlier_branches_have_own_expression = true;
                        } else if earlier_branches_have_own_expression && is_last {
                            branch.set_matchingEquality(true);
                            branch.set_isElse(true);
                        } else if !is_last && branch_count > 2 {
                            self.Error(
                                "Only final branch can be an 'else'. Did you miss a ':'?"
                                    .to_string(),
                            );
                        } else if index == 0 {
                            branch.set_isTrueBranch(true);
                        } else {
                            branch.set_isElse(true);
                        }
                    }
                }
            } else if let Some(alternatives_ref) = alternatives.as_mut() {
                let branch_count = alternatives_ref.len();
                let final_clause = alternatives_ref
                    .last()
                    .cloned()
                    .unwrap_or_else(|| ConditionalSingleBranch::new(Vec::new()));
                for (index, alt) in alternatives_ref.iter_mut().enumerate() {
                    let is_last = index == branch_count.saturating_sub(1);
                    if alt.get_ownExpression().is_none() {
                        if is_last {
                            alt.set_isElse(true);
                        } else if alt.get_isElse() {
                            if final_clause.get_isElse() {
                                self.Error(
                                    "Multiple 'else' cases. Can have a maximum of one, at the end."
                                        .to_string(),
                                );
                            } else {
                                self.Error(
                                    "'else' case in conditional should always be the final one"
                                        .to_string(),
                                );
                            }
                        } else {
                            self.Error(
                                "Branch doesn't have condition. Are you missing a ':'? "
                                    .to_string(),
                            );
                        }
                    }
                }

                if alternatives_ref.len() == 1 && alternatives_ref[0].get_ownExpression().is_none()
                {
                    self.Error("Condition block with no conditions".to_string());
                }
            }
        }

        let mut alternatives = alternatives?;

        for branch in &mut alternatives {
            branch.set_isInline(isInline);
        }

        Some(Conditional::new(initialQueryExpression, alternatives))
    }

    // C# signature: protected List<ConditionalSingleBranch> InlineConditionalBranches()
    pub fn InlineConditionalBranches(&mut self) -> Option<Vec<ConditionalSingleBranch>> {
        let mut alternatives = Vec::new();

        let first_branch = self.MixedTextAndLogic()?;
        alternatives.push(ConditionalSingleBranch::new(Self::wrap_inline_branch(
            first_branch,
        )));
        alternatives[0].set_isTrueBranch(true);

        if self.ParseString("|".to_string()).is_some() {
            let second_branch = self.MixedTextAndLogic().unwrap_or_default();
            let mut else_branch =
                ConditionalSingleBranch::new(Self::wrap_inline_branch(second_branch));
            else_branch.set_isElse(true);
            alternatives.push(else_branch);

            if self.ParseString("|".to_string()).is_some() {
                self.Error(
                    "Expected one or two alternatives separated by '|' in inline conditional"
                        .to_string(),
                );
                return None;
            }
        }

        Some(alternatives)
    }

    // C# signature: protected List<ConditionalSingleBranch> MultilineConditionalBranches()
    pub fn MultilineConditionalBranches(&mut self) -> Option<Vec<ConditionalSingleBranch>> {
        self.MultilineWhitespace();

        let mut branches = Vec::new();
        while let Some(branch) = self.SingleMultilineCondition() {
            branches.push(branch);
            self.MultilineWhitespace();
        }

        if branches.is_empty() {
            None
        } else {
            Some(branches)
        }
    }

    // C# signature: protected ConditionalSingleBranch SingleMultilineCondition()
    pub fn SingleMultilineCondition(&mut self) -> Option<ConditionalSingleBranch> {
        self.Whitespace();

        if self.ParseString("->".to_string()).is_some() {
            return None;
        }

        if self.ParseString("-".to_string()).is_none() {
            return None;
        }

        self.Whitespace();

        let mut expr = None;
        let isElse = self.ParseObject(|parser| parser.ElseExpression()).is_some();
        if !isElse {
            expr = self.ParseObject(|parser| parser.ConditionExpression());
        }

        let mut content = self
            .StatementsAtLevel(crate::InkParser::InkParser_Statements::StatementLevel::InnerBlock)
            .unwrap_or_default();

        if expr.is_none() && content.is_empty() {
            self.Error("expected content for the conditional branch following '-'".to_string());
            content.push(Object::with_kind(ObjectKind::Plain));
        }

        self.MultilineWhitespace();

        let mut branch = ConditionalSingleBranch::new(content);
        branch.set_ownExpression(expr);
        branch.set_isElse(isElse);
        Some(branch)
    }

    // C# signature: protected Expression ConditionExpression()
    pub fn ConditionExpression(&mut self) -> Option<Expression> {
        let expr = self.ParseObject(|parser| parser.Expression())?;
        self.DisallowIncrement(&expr as &dyn std::any::Any);

        self.Whitespace();

        if self.ParseString(":".to_string()).is_none() {
            return None;
        }

        Some(expr)
    }

    // C# signature: protected object ElseExpression()
    pub fn ElseExpression(&mut self) -> Option<()> {
        if self.ParseString("else".to_string()).is_none() {
            return None;
        }

        self.Whitespace();

        if self.ParseString(":".to_string()).is_none() {
            return None;
        }

        Some(())
    }

    fn wrap_inline_branch(items: Vec<ContentListItem>) -> Vec<Object> {
        let mut content_list = ContentList::new(items);
        let runtime = content_list.GenerateRuntimeObject();
        let mut obj = Object::with_kind(ObjectKind::Plain);
        obj.set_runtimeObject(Some(runtime));
        vec![obj]
    }
}

#[cfg(test)]
mod tests {
    use super::InkParser;

    #[test]
    fn parses_condition_and_else_markers() {
        let mut condition = InkParser::new("1:".to_string(), None, None, None);
        assert!(condition.ConditionExpression().is_some());

        let mut else_parser = InkParser::new("else:".to_string(), None, None, None);
        assert!(else_parser.ElseExpression().is_some());
    }
}
