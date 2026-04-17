// Source: ink-c-sharp/compiler/ParsedHierarchy/Choice.cs

use crate::ParsedHierarchy::ContentList::ContentList;
use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Story::{Story, SymbolType};
use ink_runtime::ChoicePoint::ChoicePoint;
use ink_runtime::Container::CountFlags;
use ink_runtime::Container::{Container, ContentItem};
use ink_runtime::ControlCommand::ControlCommand;
use ink_runtime::Divert::Divert as RuntimeDivert;
use ink_runtime::Value::{DivertTargetValue, Value};
use ink_runtime::VariableAssignment::VariableAssignment as RuntimeVariableAssignment;

#[derive(Clone, Debug, Default)]
pub struct Choice {
    startContent: Option<ContentList>,
    choiceOnlyContent: Option<ContentList>,
    innerContent: Option<ContentList>,
    identifier: Option<Identifier>,
    condition: Option<Expression>,
    onceOnly: bool,
    isInvisibleDefault: bool,
    indentationDepth: i32,
    hasWeaveStyleInlineBrackets: bool,
    runtimeChoice: Option<ChoicePoint>,
    innerContentContainer: Option<Container>,
    outerContainer: Option<Container>,
    startContentRuntimeContainer: Option<Container>,
    divertToStartContentOuter: Option<RuntimeDivert>,
    divertToStartContentInner: Option<RuntimeDivert>,
    r1Label: Option<Container>,
    r2Label: Option<Container>,
    returnToR1: Option<DivertTargetValue>,
    returnToR2: Option<DivertTargetValue>,
}

impl Choice {
    // C# signature: public Choice (ContentList startContent, ContentList choiceOnlyContent, ContentList innerContent)
    pub fn new(
        startContent: Option<ContentList>,
        choiceOnlyContent: Option<ContentList>,
        innerContent: Option<ContentList>,
    ) -> Self {
        Self {
            startContent,
            choiceOnlyContent,
            innerContent,
            onceOnly: true,
            indentationDepth: 1,
            ..Default::default()
        }
    }

    pub fn set_identifier(&mut self, value: Option<Identifier>) {
        self.identifier = value;
    }

    pub fn set_condition(&mut self, value: Option<Expression>) {
        self.condition = value;
    }

    pub fn set_onceOnly(&mut self, value: bool) {
        self.onceOnly = value;
    }

    pub fn set_isInvisibleDefault(&mut self, value: bool) {
        self.isInvisibleDefault = value;
    }

    pub fn set_indentationDepth(&mut self, value: i32) {
        self.indentationDepth = value;
    }

    pub fn set_hasWeaveStyleInlineBrackets(&mut self, value: bool) {
        self.hasWeaveStyleInlineBrackets = value;
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> ContentItem {
        self.outerContainer = Some(Container::new());
        let outer = self.outerContainer.as_mut().unwrap();

        self.runtimeChoice = Some(ChoicePoint::new(self.onceOnly));
        if let Some(choice) = self.runtimeChoice.as_mut() {
            choice.set_isInvisibleDefault(self.isInvisibleDefault);
        }

        if self.startContent.is_some()
            || self.choiceOnlyContent.is_some()
            || self.condition.is_some()
        {
            outer.AddContent(ControlCommand::EvalStart());
        }

        if let Some(start_content) = self.startContent.as_mut() {
            self.returnToR1 = Some(DivertTargetValue::new(None));
            outer.AddContent(ContentItem::Value(Value::DivertTarget(
                self.returnToR1.clone().unwrap(),
            )));
            outer.AddContent(RuntimeVariableAssignment::new("$r".to_string(), true));

            outer.AddContent(ControlCommand::BeginString());

            self.divertToStartContentOuter = Some(RuntimeDivert::new());
            outer.AddContent(self.divertToStartContentOuter.clone().unwrap());

            let mut start_container = start_content.GenerateRuntimeObject();
            start_container.set_name(Some("s".to_string()));

            let mut var_divert = RuntimeDivert::new();
            var_divert.set_variableDivertName(Some("$r".to_string()));
            start_container.AddContent(var_divert);

            outer.AddToNamedContentOnly(start_container.clone());
            self.startContentRuntimeContainer = Some(start_container);

            let mut r1_label = Container::new();
            r1_label.set_name(Some("$r1".to_string()));
            outer.AddContent(r1_label.clone());
            self.r1Label = Some(r1_label);

            outer.AddContent(ControlCommand::EndString());

            if let Some(choice) = self.runtimeChoice.as_mut() {
                choice.set_hasStartContent(true);
            }
        }

        if let Some(choice_only) = self.choiceOnlyContent.as_mut() {
            outer.AddContent(ControlCommand::BeginString());
            let choice_only_runtime = choice_only.GenerateRuntimeObject();
            outer.AddContentsOfContainer(choice_only_runtime);
            outer.AddContent(ControlCommand::EndString());

            if let Some(choice) = self.runtimeChoice.as_mut() {
                choice.set_hasChoiceOnlyContent(true);
            }
        }

        if let Some(condition) = &self.condition {
            condition.GenerateIntoContainer(outer);
            if let Some(choice) = self.runtimeChoice.as_mut() {
                choice.set_hasCondition(true);
            }
        }

        if self.startContent.is_some()
            || self.choiceOnlyContent.is_some()
            || self.condition.is_some()
        {
            outer.AddContent(ControlCommand::EvalEnd());
        }

        if let Some(choice) = self.runtimeChoice.clone() {
            outer.AddContent(choice);
        }

        self.innerContentContainer = Some(Container::new());
        let inner = self.innerContentContainer.as_mut().unwrap();

        if self.startContent.is_some() {
            self.returnToR2 = Some(DivertTargetValue::new(None));
            inner.AddContent(ControlCommand::EvalStart());
            inner.AddContent(ContentItem::Value(Value::DivertTarget(
                self.returnToR2.clone().unwrap(),
            )));
            inner.AddContent(ControlCommand::EvalEnd());
            inner.AddContent(RuntimeVariableAssignment::new("$r".to_string(), true));

            self.divertToStartContentInner = Some(RuntimeDivert::new());
            inner.AddContent(self.divertToStartContentInner.clone().unwrap());

            let mut r2_label = Container::new();
            r2_label.set_name(Some("$r2".to_string()));
            inner.AddContent(r2_label.clone());
            self.r2Label = Some(r2_label);
        }

        if let Some(inner_content) = self.innerContent.as_mut() {
            let inner_runtime = inner_content.GenerateRuntimeObject();
            inner.AddContentsOfContainer(inner_runtime);
        }

        inner.set_countFlags(CountFlags::CountStartOnly as i32);

        ContentItem::Container(Box::new(outer.clone()))
    }

    // C# signature: public override void ResolveReferences(Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        if let Some(inner) = &self.innerContentContainer {
            if let Some(choice) = self.runtimeChoice.as_mut() {
                choice.set_pathStringOnChoice(inner.get_path().ToString());
            }
        }

        if let (Some(return_to_r1), Some(r1_label)) =
            (self.returnToR1.as_mut(), self.r1Label.as_ref())
        {
            *return_to_r1 = DivertTargetValue::new(Some(r1_label.get_path().clone()));
        }

        if let (Some(return_to_r2), Some(r2_label)) =
            (self.returnToR2.as_mut(), self.r2Label.as_ref())
        {
            *return_to_r2 = DivertTargetValue::new(Some(r2_label.get_path().clone()));
        }

        if let (Some(divert), Some(start_container)) = (
            self.divertToStartContentOuter.as_mut(),
            self.startContentRuntimeContainer.as_ref(),
        ) {
            divert.set_targetPathString(Some(start_container.get_path().ToString()));
        }

        if let (Some(divert), Some(start_container)) = (
            self.divertToStartContentInner.as_mut(),
            self.startContentRuntimeContainer.as_ref(),
        ) {
            divert.set_targetPathString(Some(start_container.get_path().ToString()));
        }

        if let Some(start_content) = self.startContent.as_mut() {
            start_content.ResolveReferences(context);
        }
        if let Some(choice_only) = self.choiceOnlyContent.as_mut() {
            choice_only.ResolveReferences(context);
        }
        if let Some(inner_content) = self.innerContent.as_mut() {
            inner_content.ResolveReferences(context);
        }
        if let Some(condition) = self.condition.as_mut() {
            condition.ResolveReferences(context);
        }

        if let Some(identifier) = &self.identifier {
            if identifier
                .name
                .as_ref()
                .map(|name| !name.is_empty())
                .unwrap_or(false)
            {
                context.CheckForNamingCollisions(
                    Default::default(),
                    identifier.clone(),
                    SymbolType::SubFlowAndWeave,
                    String::new(),
                );
            }
        }
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        let start = self
            .startContent
            .as_ref()
            .map(|content| content.ToString())
            .unwrap_or_default();
        if let Some(choice_only) = &self.choiceOnlyContent {
            format!("* {}[{}]...", start, choice_only.ToString())
        } else {
            format!("* {}...", start)
        }
    }

    // C# signature: ContentList startContent { get; }
    pub fn get_startContent(&self) -> Option<&ContentList> {
        self.startContent.as_ref()
    }

    pub fn get_choiceOnlyContent(&self) -> Option<&ContentList> {
        self.choiceOnlyContent.as_ref()
    }

    pub fn get_innerContent(&self) -> Option<&ContentList> {
        self.innerContent.as_ref()
    }

    // C# signature: string name { get; }
    pub fn get_name(&self) -> Option<&str> {
        self.identifier
            .as_ref()
            .and_then(|identifier| identifier.name.as_deref())
    }

    // C# signature: Identifier identifier { get; }
    pub fn get_identifier(&self) -> Option<&Identifier> {
        self.identifier.as_ref()
    }

    // C# signature: Expression condition { get; }
    pub fn get_condition(&self) -> Option<&Expression> {
        self.condition.as_ref()
    }

    // C# signature: bool onceOnly { get; }
    pub fn get_onceOnly(&self) -> bool {
        self.onceOnly
    }

    // C# signature: bool isInvisibleDefault { get; }
    pub fn get_isInvisibleDefault(&self) -> bool {
        self.isInvisibleDefault
    }

    // C# signature: int indentationDepth { get; }
    pub fn get_indentationDepth(&self) -> i32 {
        self.indentationDepth
    }

    // C# signature: bool hasWeaveStyleInlineBrackets { get; }
    pub fn get_hasWeaveStyleInlineBrackets(&self) -> bool {
        self.hasWeaveStyleInlineBrackets
    }

    // C# signature: Runtime.Container runtimeContainer { get; }
    pub fn get_runtimeContainer(&self) -> Option<Container> {
        self.innerContentContainer.clone()
    }

    // C# signature: Runtime.Container innerContentContainer { get; }
    pub fn get_innerContentContainer(&self) -> Option<Container> {
        self.innerContentContainer.clone()
    }

    // C# signature: Runtime.Container containerForCounting { get; }
    pub fn get_containerForCounting(&self) -> Option<Container> {
        self.innerContentContainer.clone()
    }

    // C# signature: Runtime.Path runtimePath { get; }
    pub fn get_runtimePath(&self) -> Option<ink_runtime::Path::Path> {
        self.innerContentContainer
            .as_ref()
            .map(|container| container.get_path().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::Choice;
    use crate::ParsedHierarchy::ContentList::{ContentList, ContentListItem};
    use crate::ParsedHierarchy::Text::Text;
    use ink_runtime::Container::ContentItem;

    #[test]
    fn generates_runtime_container_for_simple_choice() {
        let mut choice = Choice::new(
            Some(ContentList::new(vec![ContentListItem::from(Text::new(
                "Hello".to_string(),
            ))])),
            None,
            None,
        );

        let runtime = choice.GenerateRuntimeObject();
        assert!(matches!(runtime, ContentItem::Container(_)));
        assert_eq!(choice.get_onceOnly(), true);
        assert!(choice.ToString().contains("Hello"));
    }
}
