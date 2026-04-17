// Source: ink-c-sharp/compiler/InkParser/InkParser_Logic.cs

use crate::CharacterSet::CharacterSet;
use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::ConstantDeclaration::ConstantDeclaration;
use crate::ParsedHierarchy::ContentList::{ContentList, ContentListItem};
use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::ListDefinition::{ListDefinition, ListElementDefinition};
use crate::ParsedHierarchy::Number::{Number, NumberValue};
use crate::ParsedHierarchy::Return::Return;
use crate::ParsedHierarchy::Text::Text;
use crate::ParsedHierarchy::VariableAssignment::VariableAssignment;
use std::any::Any;

impl InkParser {
    // C# signature: protected Parsed.Object LogicLine()
    pub fn LogicLine(&mut self) -> Option<Box<dyn Any>> {
        self.Whitespace();

        self.ParseString("~".to_string())?;

        self.Whitespace();

        let mut result: Option<Box<dyn Any>> = self.OneOf(vec![
            Box::new(|parser: &mut InkParser| {
                parser
                    .ReturnStatement()
                    .map(|statement| Box::new(statement) as Box<dyn Any>)
            }),
            Box::new(|parser: &mut InkParser| parser.TempDeclarationOrAssignment()),
            Box::new(|parser: &mut InkParser| {
                parser
                    .Expression()
                    .map(|expression| Box::new(expression) as Box<dyn Any>)
            }),
        ]);

        let Some(mut result_value) = result else {
            self.Error("expression after '~'".to_string());
            let _ = self.SkipToNextLine();
            return Some(Box::new(ContentList::new_overload_2()));
        };

        if let Some(expression) = result_value.downcast_ref::<Expression>() {
            match &expression.kind {
                ExpressionKind::FunctionCall(_) | ExpressionKind::IncDec(_) => {}
                ExpressionKind::VariableReference(variable_reference) => {
                    if variable_reference.get_name() == "include" {
                        self.Error("'~ include' is no longer the correct syntax - please use 'INCLUDE your_filename.ink', without the tilda, and in block capitals.".to_string());
                    } else {
                        self.Error("Logic following a '~' can't be that type of expression. It can only be something like:\n\t~ return\n\t~ var x = blah\n\t~ x++\n\t~ myFunction()".to_string());
                    }
                }
                _ => {
                    self.Error("Logic following a '~' can't be that type of expression. It can only be something like:\n\t~ return\n\t~ var x = blah\n\t~ x++\n\t~ myFunction()".to_string());
                }
            }

            if Self::expression_contains_function_call(expression) {
                let list = ContentList::new(vec![
                    ContentListItem::from(expression.clone()),
                    ContentListItem::from(Text::new("\n".to_string())),
                ]);
                result_value = Box::new(list);
            }
        }

        if self.EndOfLine().is_none() {
            self.Error("end of line".to_string());
            let _ = self.SkipToNextLine();
        }

        result = Some(result_value);
        result
    }

    // C# signature: protected Parsed.Object VariableDeclaration()
    pub fn VariableDeclaration(&mut self) -> Option<Box<dyn Any>> {
        self.Whitespace();

        if self.ParseObject(|parser| parser.Identifier())?.as_str() != "VAR" {
            return None;
        }

        self.Whitespace();

        let varName = self.ParseObject(|parser| parser.IdentifierWithMetadata())?;

        self.Whitespace();

        self.ParseString("=".to_string())?;

        self.Whitespace();

        let expr = self.ParseObject(|parser| parser.Expression())?;

        if !matches!(
            expr.kind,
            ExpressionKind::Number(_)
                | ExpressionKind::Text(_)
                | ExpressionKind::DivertTarget(_)
                | ExpressionKind::VariableReference(_)
                | ExpressionKind::List(_)
        ) {
            self.Error(
                "initial value for a variable must be a number, constant, list or divert target"
                    .to_string(),
            );
        }

        if self
            .ParseObject(|parser| parser.ListElementDefinitionSeparator())
            .is_some()
        {
            self.Error("Unexpected ','. If you're trying to declare a new list, use the LIST keyword, not VAR".to_string());
        }

        let mut result = VariableAssignment::new(varName, expr);
        result.set_isGlobalDeclaration(true);
        Some(Box::new(result))
    }

    // C# signature: protected Parsed.VariableAssignment ListDeclaration ()
    pub fn ListDeclaration(&mut self) -> Option<Box<dyn Any>> {
        self.Whitespace();

        if self.ParseObject(|parser| parser.Identifier())?.as_str() != "LIST" {
            return None;
        }

        self.Whitespace();

        let varName = self.ParseObject(|parser| parser.IdentifierWithMetadata())?;

        self.Whitespace();

        self.ParseString("=".to_string())?;

        self.Whitespace();

        let mut definition = self.ParseObject(|parser| parser.ListDefinition())?;
        definition.identifier = Some(varName.clone());

        Some(Box::new(VariableAssignment::new_overload_2(
            varName, definition,
        )))
    }

    // C# signature: protected Parsed.ListDefinition ListDefinition ()
    pub fn ListDefinition(&mut self) -> Option<ListDefinition> {
        let allElements = self.SeparatedList(
            |parser| parser.ListElementDefinition(),
            |parser| parser.ListElementDefinitionSeparator().map(|_| ()),
        )?;

        Some(ListDefinition::new(allElements))
    }

    // C# signature: protected string ListElementDefinitionSeparator ()
    pub fn ListElementDefinitionSeparator(&mut self) -> Option<String> {
        self.AnyWhitespace();

        self.ParseString(",".to_string())?;

        self.AnyWhitespace();

        Some(",".to_string())
    }

    // C# signature: protected Parsed.ListElementDefinition ListElementDefinition ()
    pub fn ListElementDefinition(&mut self) -> Option<ListElementDefinition> {
        let inInitialList = self.ParseString("(".to_string()).is_some();
        let mut needsToCloseParen = inInitialList;

        self.Whitespace();

        let name = self.ParseObject(|parser| parser.IdentifierWithMetadata())?;

        self.Whitespace();

        if inInitialList && self.ParseString(")".to_string()).is_some() {
            needsToCloseParen = false;
            self.Whitespace();
        }

        let mut elementValue = None;
        if self.ParseString("=".to_string()).is_some() {
            self.Whitespace();

            let elementValueExpr = self.ParseObject(|parser| parser.ExpressionInt())?;
            match &elementValueExpr.kind {
                ExpressionKind::Number(Number {
                    value: NumberValue::Int(value),
                }) => {
                    elementValue = Some(*value);
                }
                _ => {
                    self.Error("value to be assigned to list item".to_string());
                }
            }

            if needsToCloseParen {
                self.Whitespace();
                if self.ParseString(")".to_string()).is_some() {
                    needsToCloseParen = false;
                }
            }
        }

        if needsToCloseParen {
            self.Error("Expected closing ')'".to_string());
        }

        Some(ListElementDefinition::new(
            name,
            inInitialList,
            elementValue,
        ))
    }

    // C# signature: protected Parsed.Object ConstDeclaration()
    pub fn ConstDeclaration(&mut self) -> Option<Box<dyn Any>> {
        self.Whitespace();

        if self.ParseObject(|parser| parser.Identifier())?.as_str() != "CONST" {
            return None;
        }

        self.Whitespace();

        let varName = self.ParseObject(|parser| parser.IdentifierWithMetadata())?;

        self.Whitespace();

        self.ParseString("=".to_string())?;

        self.Whitespace();

        let expr = self.ParseObject(|parser| parser.Expression())?;
        if !matches!(
            expr.kind,
            ExpressionKind::Number(_) | ExpressionKind::DivertTarget(_) | ExpressionKind::Text(_)
        ) {
            self.Error(
                "initial value for a constant must be a number or divert target".to_string(),
            );
        }

        Some(Box::new(ConstantDeclaration::new(varName, Some(expr))))
    }

    fn expression_contains_function_call(expression: &Expression) -> bool {
        match &expression.kind {
            ExpressionKind::FunctionCall(_) => true,
            _ => expression
                .get_subExpressions()
                .iter()
                .any(Self::expression_contains_function_call),
        }
    }

    fn identifier_char_set(&mut self) -> CharacterSet {
        let mut identifierCharSet = CharacterSet::new();
        identifierCharSet
            .AddRange('A', 'Z')
            .AddRange('a', 'z')
            .AddRange('0', '9')
            .AddRange('_', '_');
        self.ExtendIdentifierCharacterRanges(&mut identifierCharSet);
        identifierCharSet
    }

    // C# signature: protected Identifier IdentifierWithMetadata()
    pub fn IdentifierWithMetadata(&mut self) -> Option<Identifier> {
        let name = self.Identifier()?;
        Some(Identifier {
            name: Some(name),
            debugMetadata: None,
        })
    }

    // C# signature: protected string Identifier()
    pub fn Identifier(&mut self) -> Option<String> {
        let identifierCharSet = self.identifier_char_set();
        let name = self.ParseCharactersFromCharSet(identifierCharSet, true, -1)?;

        if name.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }

        Some(name)
    }
}

#[cfg(test)]
mod tests {
    use super::InkParser;
    use crate::ParsedHierarchy::ConstantDeclaration::ConstantDeclaration;
    use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
    use crate::ParsedHierarchy::ListDefinition::ListDefinition;
    use crate::ParsedHierarchy::VariableAssignment::VariableAssignment;
    use std::any::Any;

    #[test]
    fn parses_identifiers_and_rejects_digit_only_names() {
        let mut parser = InkParser::new("alpha123".to_string(), None, None, None);
        assert_eq!(parser.Identifier().as_deref(), Some("alpha123"));

        let mut digit_parser = InkParser::new("12345".to_string(), None, None, None);
        assert!(digit_parser.Identifier().is_none());

        let mut metadata_parser = InkParser::new("name".to_string(), None, None, None);
        let identifier = metadata_parser.IdentifierWithMetadata().unwrap();
        assert_eq!(identifier.name.as_deref(), Some("name"));
        assert!(identifier.debugMetadata.is_none());
    }

    #[test]
    fn parses_variable_and_const_declarations() {
        let mut var_parser = InkParser::new("VAR score = 5".to_string(), None, None, None);
        let var_decl = var_parser
            .VariableDeclaration()
            .unwrap()
            .downcast::<VariableAssignment>()
            .ok()
            .map(|boxed| *boxed)
            .expect("variable assignment");
        assert_eq!(var_decl.get_variableName(), "score");
        assert!(var_decl.get_isGlobalDeclaration());

        let mut const_parser = InkParser::new("CONST max = 10".to_string(), None, None, None);
        let const_decl = const_parser
            .ConstDeclaration()
            .unwrap()
            .downcast::<ConstantDeclaration>()
            .ok()
            .map(|boxed| *boxed)
            .expect("constant declaration");
        assert_eq!(const_decl.get_constantName(), Some("max"));
    }

    #[test]
    fn parses_list_definitions_and_list_declarations() {
        let mut list_parser = InkParser::new("(apple = 2, pear)".to_string(), None, None, None);
        let list_def = list_parser.ListDefinition().expect("list def");
        assert_eq!(list_def.itemDefinitions.len(), 2);

        let mut decl_parser =
            InkParser::new("LIST food = (apple, pear)".to_string(), None, None, None);
        let list_decl = decl_parser
            .ListDeclaration()
            .unwrap()
            .downcast::<VariableAssignment>()
            .ok()
            .map(|boxed| *boxed)
            .expect("list assignment");
        assert_eq!(list_decl.get_variableName(), "food");
        assert!(list_decl.get_listDefinition().is_some());
    }
}
