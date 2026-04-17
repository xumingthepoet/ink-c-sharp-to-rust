// Source: ink-c-sharp/compiler/InkParser/InkParser_CommandLineInput.cs

use crate::CharacterSet::CharacterSet;
use crate::CommandLineInput::CommandLineInput;
use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::Divert::Divert;
use crate::ParsedHierarchy::Expression::Expression;
use std::any::Any;

impl InkParser {
    pub fn CommandLineUserInput(&mut self) -> Option<CommandLineInput> {
        let mut result = CommandLineInput::new();

        self.Whitespace();

        if self.ParseString("help".to_string()).is_some() {
            result.isHelp = true;
            return Some(result);
        }

        if self.ParseString("exit".to_string()).is_some()
            || self.ParseString("quit".to_string()).is_some()
        {
            result.isExit = true;
            return Some(result);
        }

        self.OneOf(vec![
            Box::new(|parser: &mut InkParser| parser.DebugSource()),
            Box::new(|parser: &mut InkParser| parser.DebugPathLookup()),
            Box::new(|parser: &mut InkParser| parser.UserChoiceNumber()),
            Box::new(|parser: &mut InkParser| parser.UserImmediateModeStatement()),
        ])
    }

    fn DebugSource(&mut self) -> Option<CommandLineInput> {
        self.Whitespace();

        self.ParseString("DebugSource".to_string())?;

        self.Whitespace();

        let expectMsg = "character offset in parentheses, e.g. DebugSource(5)";
        if self.ParseString("(".to_string()).is_none() {
            self.Error(expectMsg.to_string());
            return None;
        }

        self.Whitespace();

        let characterOffset = match self.parser_mut().ParseInt() {
            Some(value) => value,
            None => {
                self.Error(expectMsg.to_string());
                return None;
            }
        };

        self.Whitespace();
        self.ParseString(")".to_string());

        let mut inputStruct = CommandLineInput::new();
        inputStruct.debugSource = Some(characterOffset);
        Some(inputStruct)
    }

    fn DebugPathLookup(&mut self) -> Option<CommandLineInput> {
        self.Whitespace();

        self.ParseString("DebugPath".to_string())?;

        self.Whitespace();

        let pathStr = self.RuntimePath()?;
        let mut inputStruct = CommandLineInput::new();
        inputStruct.debugPathLookup = Some(pathStr);
        Some(inputStruct)
    }

    fn RuntimePath(&mut self) -> Option<String> {
        let mut runtimePathCharacterSet = CharacterSet::new_overload_2("-.".to_string());
        self.ExtendIdentifierCharacterRanges(&mut runtimePathCharacterSet);
        self.ParseCharactersFromCharSet(runtimePathCharacterSet, true, -1)
    }

    fn UserChoiceNumber(&mut self) -> Option<CommandLineInput> {
        self.Whitespace();

        let number = self.parser_mut().ParseInt()?;

        self.Whitespace();

        self.ParseNewline()?;

        let mut inputStruct = CommandLineInput::new();
        inputStruct.choiceInput = Some(number);
        Some(inputStruct)
    }

    fn UserImmediateModeStatement(&mut self) -> Option<CommandLineInput> {
        let statement = self.OneOf(vec![
            Box::new(|parser: &mut InkParser| {
                parser
                    .SingleDivert()
                    .map(|divert| Box::new(divert) as Box<dyn Any>)
            }),
            Box::new(|parser: &mut InkParser| parser.TempDeclarationOrAssignment()),
            Box::new(|parser: &mut InkParser| {
                parser
                    .Expression()
                    .map(|expression| Box::new(expression) as Box<dyn Any>)
            }),
        ]);

        statement.map(|statement| {
            let mut inputStruct = CommandLineInput::new();
            inputStruct.userImmediateModeStatement = Some(statement);
            inputStruct
        })
    }
}

#[cfg(test)]
mod tests {
    use super::InkParser;
    use crate::ParsedHierarchy::Divert::Divert;
    use crate::ParsedHierarchy::Expression::Expression;

    #[test]
    fn parses_help_exit_and_choice_number_inputs() {
        let mut help_parser = InkParser::new("help".to_string(), None, None, None);
        assert!(help_parser.CommandLineUserInput().unwrap().isHelp);

        let mut exit_parser = InkParser::new("quit".to_string(), None, None, None);
        assert!(exit_parser.CommandLineUserInput().unwrap().isExit);

        let mut choice_parser = InkParser::new("42\n".to_string(), None, None, None);
        assert_eq!(
            choice_parser.CommandLineUserInput().unwrap().choiceInput,
            Some(42)
        );
    }

    #[test]
    fn parses_debug_source_and_path_inputs() {
        let mut debug_source_parser =
            InkParser::new("DebugSource(5)".to_string(), None, None, None);
        assert_eq!(
            debug_source_parser
                .CommandLineUserInput()
                .unwrap()
                .debugSource,
            Some(5)
        );

        let mut debug_path_parser =
            InkParser::new("DebugPath knot.stitch".to_string(), None, None, None);
        assert_eq!(
            debug_path_parser
                .CommandLineUserInput()
                .unwrap()
                .debugPathLookup
                .as_deref(),
            Some("knot.stitch")
        );
    }

    #[test]
    fn parses_immediate_mode_statement() {
        let mut divert_parser = InkParser::new("-> knot".to_string(), None, None, None);
        let input = divert_parser.CommandLineUserInput().unwrap();
        assert!(input
            .userImmediateModeStatement
            .as_ref()
            .expect("statement")
            .is::<Divert>());

        let mut expr_parser = InkParser::new("1 + 2".to_string(), None, None, None);
        let input = expr_parser.CommandLineUserInput().unwrap();
        assert!(input
            .userImmediateModeStatement
            .as_ref()
            .expect("statement")
            .is::<Expression>());
    }
}
