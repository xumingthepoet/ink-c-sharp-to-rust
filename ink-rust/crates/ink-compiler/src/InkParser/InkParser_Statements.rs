// Source: ink-c-sharp/compiler/InkParser/InkParser_Statements.cs

use crate::InkParser::InkParser::InkParser;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatementLevel {
    InnerBlock,
    Stitch,
    Knot,
    Top,
}

impl InkParser {
    // C# signature: protected List<Parsed.Object> StatementsAtLevel(StatementLevel level)
    pub fn StatementsAtLevel(&mut self, _level: StatementLevel) -> Vec<crate::stub::PortStub> {
        todo!("Statement dispatcher requires the unported parsed hierarchy tree");
    }

    // C# signature: protected object StatementAtLevel(StatementLevel level)
    pub fn StatementAtLevel(&mut self, _level: StatementLevel) -> crate::stub::PortStub {
        todo!("Statement dispatcher requires the unported parsed hierarchy tree");
    }

    // C# signature: protected object StatementsBreakForLevel(StatementLevel level)
    pub fn StatementsBreakForLevel(&mut self, _level: StatementLevel) -> crate::stub::PortStub {
        todo!("Statement dispatcher requires the unported parsed hierarchy tree");
    }

    // C# signature: protected object SkipToNextLine()
    pub fn SkipToNextLine(&mut self) -> crate::stub::PortStub {
        self.ParseUntilCharactersFromString("\n\r".to_string());
        self.ParseNewline();
        Default::default()
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
                parser.SkipToNextLine();
            }

            result
        }
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
