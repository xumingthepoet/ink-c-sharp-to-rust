// Source: ink-c-sharp/compiler/InkParser/InkParser_Knot.cs

use crate::CharacterSet::CharacterSet;
use crate::InkParser::InkParser::InkParser;
use crate::InkParser::InkParser_Statements::StatementLevel;
use crate::ParsedHierarchy::FlowBase::Argument;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Knot::Knot;
use crate::ParsedHierarchy::Object::{Object, ObjectKind};
use crate::ParsedHierarchy::Stitch::Stitch;
use ink_runtime::Container::Container;
use ink_runtime::Container::ContentItem;

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

        let mut knot = Knot::new(
            knotDecl.name.clone(),
            content.clone(),
            knotDecl.arguments.clone(),
            knotDecl.isFunction,
        );
        let runtime_container = knot.GenerateRuntimeObject();
        Some(Self::wrap_flow_object(
            ObjectKind::Knot,
            knot.get_name().map(|name| name.to_string()),
            knot.get_base().base.content.clone(),
            runtime_container,
            knot.get_base().get_isFunction(),
        ))
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

        let mut stitch = Stitch::new(
            decl.name.clone(),
            content.clone(),
            decl.arguments.clone(),
            decl.isFunction,
        );
        let runtime_container = stitch.get_base_mut().GenerateRuntimeObject();
        Some(Self::wrap_flow_object(
            ObjectKind::Stitch,
            stitch.get_name().map(|name| name.to_string()),
            stitch.get_base().base.content.clone(),
            runtime_container,
            stitch.get_base().get_isFunction(),
        ))
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

        let parameterNames = self.BracketedKnotDeclArguments().unwrap_or_default();
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

    fn wrap_flow_object(
        kind: ObjectKind,
        name: Option<String>,
        content: Vec<Object>,
        runtime_container: Container,
        isFunction: bool,
    ) -> Object {
        let mut obj = Object::with_kind(kind);
        obj.isFunction = isFunction;
        obj.set_identifier(name.map(|name| Identifier {
            name: Some(name),
            debugMetadata: None,
        }));
        obj.content = content;
        obj.set_runtimeObject(Some(runtime_container));
        obj
    }
}
