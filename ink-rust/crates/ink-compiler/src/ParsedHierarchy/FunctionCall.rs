// Source: ink-c-sharp/compiler/ParsedHierarchy/FunctionCall.cs

use crate::ParsedHierarchy::Divert::Divert;
use crate::ParsedHierarchy::DivertTarget::DivertTarget;
use crate::ParsedHierarchy::Expression::{Expression, ExpressionKind};
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Number::NumberValue;
use crate::ParsedHierarchy::VariableReference::VariableReference;
use crate::ParsedHierarchy::{ListDefinition::ListDefinition, Story::Story};
use ink_runtime::Container::{Container as RuntimeContainer, ContentItem};
use ink_runtime::ControlCommand::ControlCommand;
use ink_runtime::InkList::InkList;
use ink_runtime::NativeFunctionCall::NativeFunctionCall as RuntimeNativeFunctionCall;
use ink_runtime::Value::{ListValue, Value};

#[derive(Clone, Debug, Default)]
pub struct FunctionCall {
    proxyDivert: Divert,
    arguments: Vec<Expression>,
    runtimeDivert: Option<ink_runtime::Divert::Divert>,
    resolvedList: Option<ListDefinition>,
    divertTargetToCount: Option<Box<DivertTarget>>,
    variableReferenceToCount: Option<Box<VariableReference>>,
    pub shouldPopReturnedValue: bool,
}

impl PartialEq for FunctionCall {
    fn eq(&self, other: &Self) -> bool {
        self.proxyDivert == other.proxyDivert
            && self.arguments == other.arguments
            && self.shouldPopReturnedValue == other.shouldPopReturnedValue
    }
}

impl FunctionCall {
    // C# signature: public FunctionCall (Identifier functionName, List<Expression> arguments)
    pub fn new(functionName: Identifier, arguments: Vec<Expression>) -> Self {
        let mut proxyDivert = Divert::new(
            crate::ParsedHierarchy::Path::Path::new_overload_3(functionName),
            arguments.clone(),
        );
        proxyDivert.isFunctionCall = true;

        Self {
            proxyDivert,
            arguments,
            runtimeDivert: None,
            resolvedList: None,
            divertTargetToCount: None,
            variableReferenceToCount: None,
            shouldPopReturnedValue: false,
        }
    }

    fn name_str(&self) -> String {
        self.proxyDivert
            .target
            .as_ref()
            .and_then(|path| path.get_firstComponent())
            .unwrap_or("")
            .to_string()
    }

    fn arguments_mut(&mut self) -> &mut [Expression] {
        &mut self.arguments
    }

    // C# signature: public override void GenerateIntoContainer (Runtime.Container container)
    pub fn GenerateIntoContainer(&self, container: &mut RuntimeContainer) {
        let name = self.name_str();
        let foundList = self.resolvedList.clone();
        let mut usingProxyDivert = false;
        let mut proxyDivert = self.proxyDivert.clone();

        if self.get_isChoiceCount() {
            if !self.arguments.is_empty() {
                panic!("The CHOICE_COUNT() function shouldn't take any arguments");
            }

            container.AddContent(ControlCommand::ChoiceCount());
        } else if self.get_isTurns() {
            if !self.arguments.is_empty() {
                panic!("The TURNS() function shouldn't take any arguments");
            }

            container.AddContent(ControlCommand::Turns());
        } else if self.get_isTurnsSince() || self.get_isReadCount() {
            if self.arguments.len() != 1 {
                panic!(
                    "The {}() function should take one argument: a divert target to the target knot, stitch, gather or choice you want to check. e.g. TURNS_SINCE(-> myKnot)",
                    name
                );
            }

            if let Some(divertTarget) = self.divertTargetToCount.as_ref() {
                divertTarget.GenerateIntoContainer(container);
            } else if let Some(variableReference) = self.variableReferenceToCount.as_ref() {
                variableReference.GenerateIntoContainer(container);
            } else {
                self.arguments[0].GenerateIntoContainer(container);
            }

            if self.get_isTurnsSince() {
                container.AddContent(ControlCommand::TurnsSince());
            } else {
                container.AddContent(ControlCommand::ReadCount());
            }
        } else if self.get_isRandom() {
            if self.arguments.len() != 2 {
                panic!("RANDOM should take 2 parameters: a minimum and a maximum integer");
            }

            for argument in &self.arguments {
                argument.GenerateIntoContainer(container);
            }

            container.AddContent(ControlCommand::Random());
        } else if self.get_isSeedRandom() {
            if self.arguments.len() != 1 {
                panic!("SEED_RANDOM should take 1 parameter - an integer seed");
            }

            self.arguments[0].GenerateIntoContainer(container);
            container.AddContent(ControlCommand::SeedRandom());
        } else if self.get_isListRange() {
            if self.arguments.len() != 3 {
                panic!("LIST_RANGE should take 3 parameters - a list, a min and a max");
            }

            for argument in &self.arguments {
                argument.GenerateIntoContainer(container);
            }

            container.AddContent(ControlCommand::ListRange());
        } else if self.get_isListRandom() {
            if self.arguments.len() != 1 {
                panic!("LIST_RANDOM should take 1 parameter - a list");
            }

            self.arguments[0].GenerateIntoContainer(container);
            container.AddContent(ControlCommand::ListRandom());
        } else if RuntimeNativeFunctionCall::CallExistsWithName(name.clone()) {
            let nativeCall = RuntimeNativeFunctionCall::CallWithName(name.clone());
            if nativeCall.get_numberOfParameters() != self.arguments.len() as i32 {
                let mut msg = format!(
                    "{} should take {} parameter",
                    name,
                    nativeCall.get_numberOfParameters()
                );
                if nativeCall.get_numberOfParameters() != 1 {
                    msg.push('s');
                }
                panic!("{}", msg);
            }

            for argument in &self.arguments {
                argument.GenerateIntoContainer(container);
            }

            container.AddContent(RuntimeNativeFunctionCall::CallWithName(name));
        } else if let Some(listDef) = foundList {
            if self.arguments.len() > 1 {
                panic!(
                    "Can currently only construct a list from one integer (or an empty list from a given list definition)"
                );
            }

            if self.arguments.len() == 1 {
                container.AddContent(Value::new_string(name.clone()));
                self.arguments[0].GenerateIntoContainer(container);
                container.AddContent(ControlCommand::ListFromInt());
            } else {
                let mut list_value = ListValue::new();
                list_value.originNames =
                    Some(vec![listDef.get_name().unwrap_or_default().to_string()]);
                list_value.origins = Some(vec![listDef.get_runtimeListDefinition()]);
                container.AddContent(Value::new_list(list_value));
            }
        } else {
            let runtime_object = proxyDivert.GenerateRuntimeObject();
            container.AddContent(runtime_object);
            usingProxyDivert = true;
        }

        if !usingProxyDivert {
            // In the C# implementation, the proxy divert is removed from the
            // content tree when a built-in/native form is emitted instead.
            // This port keeps the proxy divert as a detached helper.
        }

        if self.shouldPopReturnedValue {
            container.AddContent(ControlCommand::PopEvaluatedValue());
        }
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        let function_name = self.name_str();
        self.proxyDivert.ResolveReferences(context);

        self.resolvedList = context.ResolveList(function_name.clone());

        for argument in self.arguments_mut() {
            argument.ResolveReferences(context);
        }

        if self.get_isTurnsSince() || self.get_isReadCount() {
            if let Some(argument) = self.arguments.first() {
                match &argument.kind {
                    ExpressionKind::DivertTarget(divert_target) => {
                        self.divertTargetToCount = Some(Box::new((**divert_target).clone()));
                    }
                    ExpressionKind::VariableReference(variable_reference) => {
                        self.variableReferenceToCount =
                            Some(Box::new((**variable_reference).clone()));
                    }
                    _ => {}
                }
            }
        }

        if self.get_isRandom() {
            if self.arguments.len() != 2 {
                context.Error(
                    "RANDOM should take 2 parameters: a minimum and a maximum integer".to_string(),
                    Default::default(),
                    false,
                );
            }

            for (index, argument) in self.arguments.iter().enumerate() {
                if let ExpressionKind::Number(number) = &argument.kind {
                    if !matches!(number.value, NumberValue::Int(_)) {
                        let param_name = if index == 0 { "minimum" } else { "maximum" };
                        context.Error(
                            format!("RANDOM's {} parameter should be an integer", param_name),
                            Default::default(),
                            false,
                        );
                    }
                }
            }
        } else if self.get_isSeedRandom() {
            if self.arguments.len() != 1 {
                context.Error(
                    "SEED_RANDOM should take 1 parameter - an integer seed".to_string(),
                    Default::default(),
                    false,
                );
            }

            if let Some(argument) = self.arguments.first() {
                if let ExpressionKind::Number(number) = &argument.kind {
                    if !matches!(number.value, NumberValue::Int(_)) {
                        context.Error(
                            "SEED_RANDOM's parameter should be an integer seed".to_string(),
                            Default::default(),
                            false,
                        );
                    }
                }
            }
        }

        if let Some(divertTarget) = self.divertTargetToCount.as_mut() {
            divertTarget.ResolveReferences(context);
        }

        if let Some(variableReference) = self.variableReferenceToCount.as_mut() {
            variableReference.ResolveReferences(context);
            if variableReference
                .get_runtimeVarRef()
                .map(|var_ref| var_ref.get_pathForCount())
                .flatten()
                .is_some()
            {
                context.Error(
                    format!(
                        "Should be {}(-> {}). Usage without the '->' only makes sense for variable targets.",
                        function_name,
                        variableReference.get_name()
                    ),
                    Default::default(),
                    false,
                );
            }
        }
    }

    // C# signature: public static bool IsBuiltIn(string name)
    pub fn IsBuiltIn(name: String) -> bool {
        if RuntimeNativeFunctionCall::CallExistsWithName(name.clone()) {
            return true;
        }

        matches!(
            name.as_str(),
            "CHOICE_COUNT"
                | "TURNS_SINCE"
                | "TURNS"
                | "RANDOM"
                | "SEED_RANDOM"
                | "LIST_VALUE"
                | "LIST_RANDOM"
                | "LIST_RANGE"
                | "READ_COUNT"
        )
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        let args = self
            .arguments
            .iter()
            .map(|expr| expr.ToString())
            .collect::<Vec<_>>()
            .join(", ");
        format!("{}({})", self.name_str(), args)
    }

    // C# signature: string name { get; }
    pub fn get_name(&self) -> String {
        self.name_str()
    }

    // C# signature: Divert proxyDivert { get; }
    pub fn get_proxyDivert(&self) -> &Divert {
        &self.proxyDivert
    }

    // C# signature: List<Expression> arguments { get; }
    pub fn get_arguments(&self) -> &[Expression] {
        &self.arguments
    }

    // C# signature: Runtime.Divert runtimeDivert { get; }
    pub fn get_runtimeDivert(&self) -> Option<&ink_runtime::Divert::Divert> {
        self.runtimeDivert.as_ref()
    }

    // C# signature: bool isChoiceCount { get; }
    pub fn get_isChoiceCount(&self) -> bool {
        self.name_str() == "CHOICE_COUNT"
    }

    // C# signature: bool isTurns { get; }
    pub fn get_isTurns(&self) -> bool {
        self.name_str() == "TURNS"
    }

    // C# signature: bool isTurnsSince { get; }
    pub fn get_isTurnsSince(&self) -> bool {
        self.name_str() == "TURNS_SINCE"
    }

    // C# signature: bool isRandom { get; }
    pub fn get_isRandom(&self) -> bool {
        self.name_str() == "RANDOM"
    }

    // C# signature: bool isSeedRandom { get; }
    pub fn get_isSeedRandom(&self) -> bool {
        self.name_str() == "SEED_RANDOM"
    }

    // C# signature: bool isListRange { get; }
    pub fn get_isListRange(&self) -> bool {
        self.name_str() == "LIST_RANGE"
    }

    // C# signature: bool isListRandom { get; }
    pub fn get_isListRandom(&self) -> bool {
        self.name_str() == "LIST_RANDOM"
    }

    // C# signature: bool isReadCount { get; }
    pub fn get_isReadCount(&self) -> bool {
        self.name_str() == "READ_COUNT"
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionCall;
    use crate::ParsedHierarchy::Identifier::Identifier;

    #[test]
    fn recognizes_builtin_function_names() {
        assert!(FunctionCall::IsBuiltIn("RANDOM".to_string()));
        assert!(FunctionCall::IsBuiltIn("LIST_RANGE".to_string()));
        assert!(!FunctionCall::IsBuiltIn("not_a_builtin".to_string()));
    }

    #[test]
    fn formats_proxy_name_and_arguments() {
        let call = FunctionCall::new(
            Identifier {
                name: Some("FOO".to_string()),
                debugMetadata: None,
            },
            Vec::new(),
        );

        assert_eq!(call.get_name(), "FOO");
        assert_eq!(call.ToString(), "FOO()");
    }
}
