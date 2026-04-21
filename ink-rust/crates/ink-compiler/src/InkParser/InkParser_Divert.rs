// Source: ink-c-sharp/compiler/InkParser/InkParser_Divert.cs

use crate::InkParser::InkParser::InkParser;
use crate::ParsedHierarchy::Divert::Divert;
use crate::ParsedHierarchy::Path::Path;

#[derive(Clone, Debug, PartialEq)]
pub enum DivertPiece {
    Divert(Divert),
    TunnelOnwards(Divert),
}

#[derive(Clone, Debug, PartialEq)]
enum DivertParseItem {
    Arrow(String),
    Divert(Divert),
}

impl InkParser {
    // C# signature: protected List<Parsed.Object> MultiDivert()
    pub fn MultiDivert(&mut self) -> Option<Vec<DivertPiece>> {
        self.Whitespace();

        if let Some(thread_divert) = self.StartThread() {
            return Some(vec![DivertPiece::Divert(thread_divert)]);
        }

        let arrows_and_diverts = self.Interleave(
            |parser| {
                parser
                    .ParseDivertArrowOrTunnelOnwards()
                    .map(DivertParseItem::Arrow)
            },
            |parser| {
                parser
                    .DivertIdentifierWithArguments()
                    .map(DivertParseItem::Divert)
            },
            None,
            false,
        )?;

        let mut diverts = Vec::new();

        for i in 0..arrows_and_diverts.len() {
            match &arrows_and_diverts[i] {
                DivertParseItem::Arrow(arrow) => {
                    if arrow == "->->" {
                        let tunnel_onwards_placement_valid = i == 0
                            || i == arrows_and_diverts.len() - 1
                            || i == arrows_and_diverts.len() - 2;
                        if !tunnel_onwards_placement_valid {
                            self.Error("Tunnel onwards '->->' must only come at the begining or the start of a divert".to_string());
                        }

                        let mut tunnel_onwards_divert = Divert::default();
                        if i < arrows_and_diverts.len() - 1 {
                            if let DivertParseItem::Divert(tunnel_onward_divert) =
                                &arrows_and_diverts[i + 1]
                            {
                                tunnel_onwards_divert = tunnel_onward_divert.clone();
                            }
                        } else {
                            tunnel_onwards_divert.isEmpty = true;
                        }

                        diverts.push(DivertPiece::TunnelOnwards(tunnel_onwards_divert));
                        break;
                    }
                }
                DivertParseItem::Divert(divert) => {
                    let mut divert = divert.clone();
                    if i < arrows_and_diverts.len() - 1 {
                        divert.isTunnel = true;
                    }
                    diverts.push(DivertPiece::Divert(divert));
                }
            }
        }

        if diverts.is_empty() && arrows_and_diverts.len() == 1 {
            let mut gather_divert = Divert::default();
            gather_divert.isEmpty = true;

            if !self.get_parsingChoice() {
                self.Error("Empty diverts (->) are only valid on choices".to_string());
            }

            diverts.push(DivertPiece::Divert(gather_divert));
        }

        Some(diverts)
    }

    // C# signature: protected Divert StartThread()
    pub fn StartThread(&mut self) -> Option<Divert> {
        self.Whitespace();

        self.ParseThreadArrow()?;

        self.Whitespace();

        let mut divert = self.DivertIdentifierWithArguments()?;
        divert.isThread = true;
        Some(divert)
    }

    // C# signature: protected Divert DivertIdentifierWithArguments()
    pub fn DivertIdentifierWithArguments(&mut self) -> Option<Divert> {
        self.Whitespace();

        let target_components = self.DotSeparatedDivertPathComponents()?;

        self.Whitespace();

        let arguments = self.ExpressionFunctionCallArguments().unwrap_or_default();

        self.Whitespace();

        let target_path = Path::new_overload_2(target_components);
        Some(Divert::new(target_path, arguments))
    }

    // C# signature: protected Divert SingleDivert()
    pub fn SingleDivert(&mut self) -> Option<Divert> {
        let diverts = self.MultiDivert()?;
        if diverts.len() != 1 {
            return None;
        }

        match diverts.into_iter().next()? {
            DivertPiece::Divert(divert) if !divert.isTunnel => Some(divert),
            _ => None,
        }
    }

    fn DotSeparatedDivertPathComponents(
        &mut self,
    ) -> Option<Vec<crate::ParsedHierarchy::Identifier::Identifier>> {
        let mut components = Vec::new();
        let first = self.IdentifierWithMetadata()?;
        components.push(first);

        loop {
            self.Whitespace();
            if self.ParseString(".".to_string()).is_none() {
                break;
            }
            self.Whitespace();
            components.push(self.IdentifierWithMetadata()?);
        }

        Some(components)
    }

    // C# signature: protected string ParseDivertArrowOrTunnelOnwards()
    pub fn ParseDivertArrowOrTunnelOnwards(&mut self) -> Option<String> {
        let mut num_arrows = 0;
        while self.ParseString("->".to_string()).is_some() {
            num_arrows += 1;
        }

        match num_arrows {
            0 => None,
            1 => Some("->".to_string()),
            2 => Some("->->".to_string()),
            _ => {
                self.Error(
                    "Unexpected number of arrows in divert. Should only have '->' or '->->'"
                        .to_string(),
                );
                Some("->->".to_string())
            }
        }
    }

    // C# signature: protected string ParseDivertArrow()
    pub fn ParseDivertArrow(&mut self) -> Option<String> {
        self.ParseString("->".to_string())
    }

    // C# signature: protected string ParseThreadArrow()
    pub fn ParseThreadArrow(&mut self) -> Option<String> {
        self.ParseString("<-".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{DivertPiece, InkParser};

    #[test]
    fn parses_tunnel_onwards_with_target() {
        let mut parser = InkParser::new("->-> knot".to_string(), None, None, None);
        let pieces = parser.MultiDivert().expect("expected tunnel divert");
        assert_eq!(pieces.len(), 1);
        match &pieces[0] {
            DivertPiece::TunnelOnwards(divert) => {
                assert!(!divert.get_isEmpty());
                assert_eq!(
                    divert.get_target().unwrap().ToString(),
                    "-> knot".to_string()
                );
            }
            other => panic!("unexpected piece: {:?}", other),
        }
    }

    #[test]
    fn parses_bare_tunnel_onwards_as_empty_divert() {
        let mut parser = InkParser::new("->->".to_string(), None, None, None);
        let pieces = parser.MultiDivert().expect("expected empty tunnel divert");
        assert_eq!(pieces.len(), 1);
        match &pieces[0] {
            DivertPiece::TunnelOnwards(divert) => assert!(divert.get_isEmpty()),
            other => panic!("unexpected piece: {:?}", other),
        }
    }
}
