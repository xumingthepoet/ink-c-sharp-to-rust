// Source: ink-c-sharp/compiler/InkParser/InkParser_Whitespace.cs

use crate::CharacterSet::CharacterSet;
use crate::InkParser::InkParser::InkParser;

impl InkParser {
    // Handles both newline and endOfFile
    pub fn EndOfLine(&mut self) -> Option<()> {
        self.Newline().or_else(|| self.EndOfFile())
    }

    // Allow whitespace before the actual newline
    pub fn Newline(&mut self) -> Option<()> {
        self.Whitespace();

        if self.ParseNewline().is_some() {
            Some(())
        } else {
            None
        }
    }

    pub fn EndOfFile(&mut self) -> Option<()> {
        self.Whitespace();

        if !self.get_endOfInput() {
            return None;
        }

        Some(())
    }

    // General purpose space, returns N-count newlines (fails if no newlines)
    pub fn MultilineWhitespace(&mut self) -> Option<()> {
        let mut seen_newline = false;
        while self.Newline().is_some() {
            seen_newline = true;
        }

        if seen_newline {
            Some(())
        } else {
            None
        }
    }

    pub fn Whitespace(&mut self) -> Option<()> {
        if self
            .ParseCharactersFromCharSet(CharacterSet::new_overload_2(" \t".to_string()), true, -1)
            .is_some()
        {
            Some(())
        } else {
            None
        }
    }

    pub fn Spaced<T, R>(
        &mut self,
        mut rule: R,
    ) -> Box<dyn FnMut(&mut InkParser) -> Option<T> + 'static>
    where
        R: FnMut(&mut InkParser) -> Option<T> + 'static,
        T: 'static,
    {
        Box::new(move |parser| {
            parser.Whitespace();
            let result = rule(parser)?;
            parser.Whitespace();
            Some(result)
        })
    }

    pub fn AnyWhitespace(&mut self) -> Option<()> {
        let mut any_whitespace = false;
        while self.Whitespace().is_some() || self.MultilineWhitespace().is_some() {
            any_whitespace = true;
        }
        if any_whitespace {
            Some(())
        } else {
            None
        }
    }

    pub fn MultiSpaced<T, R>(
        &mut self,
        mut rule: R,
    ) -> Box<dyn FnMut(&mut InkParser) -> Option<T> + 'static>
    where
        R: FnMut(&mut InkParser) -> Option<T> + 'static,
        T: 'static,
    {
        Box::new(move |parser| {
            parser.AnyWhitespace();
            let result = rule(parser)?;
            parser.AnyWhitespace();
            Some(result)
        })
    }
}
