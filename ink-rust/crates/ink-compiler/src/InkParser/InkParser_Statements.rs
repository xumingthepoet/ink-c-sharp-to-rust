// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/compiler/InkParser/InkParser_Statements.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct InkParser {
    pub _port_marker: (),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatementLevel {
    PortPlaceholder,
}

impl Default for StatementLevel {
    fn default() -> Self {
        Self::PortPlaceholder
    }
}

impl InkParser {
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: protected List<Parsed.Object> StatementsAtLevel(StatementLevel level)
    pub fn StatementsAtLevel(
        &mut self,
        _level: crate::stub::StatementLevel,
    ) -> Vec<crate::stub::PortStub> {
        Default::default()
    }

    // C# signature: protected object StatementAtLevel(StatementLevel level)
    pub fn StatementAtLevel(
        &mut self,
        _level: crate::stub::StatementLevel,
    ) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected object StatementsBreakForLevel(StatementLevel level)
    pub fn StatementsBreakForLevel(
        &mut self,
        _level: crate::stub::StatementLevel,
    ) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected object SkipToNextLine()
    pub fn SkipToNextLine(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: protected ParseRule Line(ParseRule inlineRule)
    pub fn Line(&mut self, _inlineRule: crate::stub::ParseRule) -> crate::stub::ParseRule {
        Default::default()
    }
}
