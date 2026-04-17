// Source: ink-c-sharp/compiler/InkParser/InkParser_Knot.cs

use crate::CharacterSet::CharacterSet;
use crate::InkParser::InkParser::InkParser;
use crate::InkParser::InkParser_Statements::StatementLevel;
use crate::ParsedHierarchy::FlowBase::Argument;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Knot::Knot;
use crate::ParsedHierarchy::Object::{Object, ObjectKind};
use crate::ParsedHierarchy::Stitch::Stitch;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlowDecl {
    pub name: Identifier,
    pub arguments: Vec<Argument>,
    pub isFunction: bool,
}

impl InkParser {
    // C# signature: protected Knot KnotDefinition()
    pub fn KnotDefinition(&mut self) -> Option<Object> {
        let knotDecl = self.KnotDeclaration()?;
        if self.EndOfLine().is_none() {
            self.Error("Expected end of line after knot name definition".to_string());
            let _ = self.SkipToNextLine();
        }

        let content = self
            .StatementsAtLevel(StatementLevel::Knot)
            .or_else(|| self.KnotStitchNoContentRecoveryRule())
            .unwrap_or_default();

        let knot = Knot::new(
            knotDecl.name.clone(),
            content,
            knotDecl.arguments.clone(),
            knotDecl.isFunction,
        );
        Some(Object::from_knot(knot))
    }

    // C# signature: protected FlowDecl KnotDeclaration()
    pub fn KnotDeclaration(&mut self) -> Option<FlowDecl> {
        self.Whitespace();

        if self.KnotTitleEquals().is_none() {
            return None;
        }

        self.Whitespace();

        let identifier = self.IdentifierWithMetadata();
        let knotName;

        let isFunc = identifier
            .as_ref()
            .and_then(|identifier| identifier.name.as_deref())
            == Some("function");
        if isFunc {
            if self.Whitespace().is_none() {
                self.Error("whitespace after the 'function' keyword".to_string());
            }
            knotName = self.IdentifierWithMetadata();
        } else {
            knotName = identifier;
        }

        let knotName = knotName.unwrap_or_else(|| {
            self.Error("Expected the name of the knot".to_string());
            Identifier::default()
        });

        self.Whitespace();

        let parameterNames = self.BracketedKnotDeclArguments();

        self.Whitespace();

        let _ = self.KnotTitleEquals();

        Some(FlowDecl {
            name: knotName,
            arguments: parameterNames.unwrap_or_default(),
            isFunction: isFunc,
        })
    }

    // C# signature: protected string KnotTitleEquals()
    pub fn KnotTitleEquals(&mut self) -> Option<String> {
        let multiEquals = self.ParseCharactersFromString("=".to_string(), -1)?;
        if multiEquals.len() <= 1 {
            None
        } else {
            Some(multiEquals)
        }
    }

    // C# signature: protected object StitchDefinition()
    pub fn StitchDefinition(&mut self) -> Option<Object> {
        let decl = self.StitchDeclaration()?;
        if self.EndOfLine().is_none() {
            self.Error("Expected end of line after stitch name".to_string());
            let _ = self.SkipToNextLine();
        }

        let content = self
            .StatementsAtLevel(StatementLevel::Stitch)
            .or_else(|| self.KnotStitchNoContentRecoveryRule())
            .unwrap_or_default();

        let stitch = Stitch::new(
            decl.name.clone(),
            content,
            decl.arguments.clone(),
            decl.isFunction,
        );
        Some(Object::from_stitch(stitch))
    }

    // C# signature: protected FlowDecl StitchDeclaration()
    pub fn StitchDeclaration(&mut self) -> Option<FlowDecl> {
        self.Whitespace();

        if self.ParseString("=".to_string()).is_none() {
            return None;
        }

        if self.ParseString("=".to_string()).is_some() {
            return None;
        }

        self.Whitespace();

        let isFunc = self.ParseString("function".to_string()).is_some();
        if isFunc {
            self.Whitespace();
        }

        let stitchName = self.IdentifierWithMetadata()?;

        self.Whitespace();

        let flowArgs = self.BracketedKnotDeclArguments().unwrap_or_default();

        self.Whitespace();

        Some(FlowDecl {
            name: stitchName,
            arguments: flowArgs,
            isFunction: isFunc,
        })
    }

    // C# signature: protected object KnotStitchNoContentRecoveryRule()
    pub fn KnotStitchNoContentRecoveryRule(
        &mut self,
    ) -> Option<Vec<crate::ParsedHierarchy::Object::Object>> {
        let _ = self.ParseUntil(
            |parser| parser.KnotDeclaration(),
            Some(CharacterSet::new_overload_2("=".to_string())),
            None,
        );
        let mut recoveredFlowContent = Vec::new();
        let mut content_list = crate::ParsedHierarchy::ContentList::ContentList::new(vec![
            crate::ParsedHierarchy::ContentList::ContentListItem::from(
                crate::ParsedHierarchy::Text::Text::new("<ERROR IN FLOW>".to_string()),
            ),
        ]);
        let runtime = content_list.GenerateRuntimeObject();
        let mut placeholder = Object::with_kind(ObjectKind::Plain);
        placeholder.set_runtimeObject(Some(runtime));
        recoveredFlowContent.push(placeholder);
        Some(recoveredFlowContent)
    }

    // C# signature: protected List<FlowBase.Argument> BracketedKnotDeclArguments()
    pub fn BracketedKnotDeclArguments(&mut self) -> Option<Vec<Argument>> {
        if self.ParseString("(".to_string()).is_none() {
            return None;
        }

        let mut flowArguments = Vec::new();
        if self.Whitespace().is_some() {
            if let Some(arg) = self.FlowDeclArgument() {
                flowArguments.push(arg);
                loop {
                    self.Whitespace();
                    if self.ParseString(",".to_string()).is_none() {
                        break;
                    }
                    self.Whitespace();
                    if let Some(arg) = self.FlowDeclArgument() {
                        flowArguments.push(arg);
                    } else {
                        break;
                    }
                }
            }
        }

        if self.ParseString(")".to_string()).is_none() {
            self.Error("closing ')' for parameter list".to_string());
        }

        Some(flowArguments)
    }

    // C# signature: protected FlowBase.Argument FlowDeclArgument()
    pub fn FlowDeclArgument(&mut self) -> Option<Argument> {
        let firstIden = self.IdentifierWithMetadata();
        self.Whitespace();
        let divertArrow = self.ParseDivertArrow();
        self.Whitespace();
        let secondIden = self.IdentifierWithMetadata();

        if firstIden.is_none() && secondIden.is_none() {
            return None;
        }

        let mut flowArg = Argument::default();
        if divertArrow.is_some() {
            flowArg.isDivertTarget = true;
        }

        if firstIden.as_ref().and_then(|id| id.name.as_deref()) == Some("ref") {
            if secondIden.is_none() {
                self.Error("Expected an parameter name after 'ref'".to_string());
            }

            flowArg.identifier = secondIden;
            flowArg.isByReference = true;
        } else {
            if flowArg.isDivertTarget {
                flowArg.identifier = secondIden;
            } else {
                flowArg.identifier = firstIden;
            }

            if flowArg.identifier.is_none() {
                self.Error("Expected an parameter name".to_string());
            }

            flowArg.isByReference = false;
        }

        Some(flowArg)
    }

    // C# signature: protected ExternalDeclaration ExternalDeclaration()
    pub fn ExternalDeclaration(
        &mut self,
    ) -> Option<crate::ParsedHierarchy::ExternalDeclaration::ExternalDeclaration> {
        self.Whitespace();

        let external = self.IdentifierWithMetadata();
        if external
            .as_ref()
            .and_then(|identifier| identifier.name.as_deref())
            != Some("EXTERNAL")
        {
            return None;
        }

        self.Whitespace();

        let funcIdentifier = self.IdentifierWithMetadata().unwrap_or_default();

        self.Whitespace();

        let parameterNames = match self.BracketedKnotDeclArguments() {
            Some(parameter_names) => parameter_names,
            None => {
                self.Error(format!(
                    "declaration of arguments for EXTERNAL, even if empty, i.e. 'EXTERNAL {}()'",
                    funcIdentifier.name.clone().unwrap_or_default()
                ));
                return None;
            }
        };
        let argNames = parameterNames
            .iter()
            .filter_map(|arg| arg.identifier.as_ref().and_then(|id| id.name.clone()))
            .collect::<Vec<_>>();

        Some(
            crate::ParsedHierarchy::ExternalDeclaration::ExternalDeclaration::new(
                funcIdentifier,
                argNames,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::InkParser;
    use crate::ParsedHierarchy::Object::ObjectPayload;

    #[test]
    fn knot_and_stitch_definitions_remain_typed_payloads() {
        let mut knot_parser = InkParser::new(
            "== intro ==\nHello\n= scene\nWorld\n".to_string(),
            None,
            None,
            None,
        );

        let knot = knot_parser.KnotDefinition().expect("knot");
        assert!(matches!(
            knot.payload.as_ref(),
            Some(ObjectPayload::Knot(_))
        ));

        let knot_payload = match knot.payload.as_ref() {
            Some(ObjectPayload::Knot(knot)) => knot,
            _ => unreachable!("checked above"),
        };
        assert_eq!(knot_payload.get_name(), Some("intro"));
        assert!(knot_payload
            .get_base()
            .base
            .content
            .iter()
            .any(|obj| matches!(obj.payload.as_ref(), Some(ObjectPayload::Stitch(_)))));
    }

    #[test]
    fn top_level_statements_keep_divert_and_following_knot() {
        let mut parser = InkParser::new(
            "-> intro\n== intro ==\nHello\n".to_string(),
            None,
            None,
            None,
        );

        let story = parser.Parse();

        assert_eq!(story.content.len(), 2);
        assert!(matches!(
            story.content[1].payload.as_ref(),
            Some(ObjectPayload::Knot(_))
        ));
    }

    #[test]
    fn external_declaration_requires_argument_parentheses() {
        let mut parser = InkParser::new("EXTERNAL host()\n".to_string(), None, None, None);
        assert!(parser.ExternalDeclaration().is_some());

        let mut invalid = InkParser::new("EXTERNAL host\n".to_string(), None, None, None);
        assert!(invalid.ExternalDeclaration().is_none());
    }
}
