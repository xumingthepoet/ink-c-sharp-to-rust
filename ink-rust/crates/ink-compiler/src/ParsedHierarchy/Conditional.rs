// Source: ink-c-sharp/compiler/ParsedHierarchy/Conditional.cs

use crate::ParsedHierarchy::ConditionalSingleBranch::ConditionalSingleBranch;
use crate::ParsedHierarchy::Expression::Expression;
use crate::ParsedHierarchy::Object::Object;
use crate::ParsedHierarchy::Story::Story;
use ink_runtime::Container::Container as RuntimeContainer;
use ink_runtime::ControlCommand::ControlCommand;
use ink_runtime::Path::{Component, Path};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Conditional {
    base: Object,
    initialCondition: Option<Expression>,
    branches: Vec<ConditionalSingleBranch>,
    reJoinTarget: Option<Path>,
}

impl Conditional {
    // C# signature: public Conditional (Expression condition, List<ConditionalSingleBranch> branches)
    pub fn new(condition: Option<Expression>, branches: Vec<ConditionalSingleBranch>) -> Self {
        let mut base = Object::new();
        if let Some(condition) = condition.as_ref() {
            base.AddContent(Object::from_expression(condition.clone()));
        }

        for branch in &branches {
            base.AddContent(branch.get_base().clone());
        }

        Self {
            base,
            initialCondition: condition,
            branches,
            reJoinTarget: None,
        }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> RuntimeContainer {
        let mut container = RuntimeContainer::new();

        if let Some(initial_condition) = &self.initialCondition {
            container.AddContent(initial_condition.GenerateRuntimeObject());
        }

        for branch in &mut self.branches {
            let branch_container = branch.GenerateRuntimeObject();
            container.AddContent(branch_container);
        }

        if self.initialCondition.is_some()
            && self
                .branches
                .first()
                .and_then(|branch| branch.get_ownExpression())
                .is_some()
            && self
                .branches
                .last()
                .map(|branch| !branch.get_isElse())
                .unwrap_or(false)
        {
            container.AddContent(ControlCommand::PopEvaluatedValue());
        }

        container.AddContent(ControlCommand::NoOp());
        let rejoin_path = container
            .get_path()
            .PathByAppendingComponent(Component::new((container.get_content().len() - 1) as i32));
        self.reJoinTarget = Some(rejoin_path);

        container
    }

    // C# signature: public override void ResolveReferences (Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        let rejoin_path = self.reJoinTarget.clone();

        for branch in &mut self.branches {
            branch.set_returnDivertTargetPath(rejoin_path.clone());
            branch.ResolveReferences(context);
        }
        if let Some(condition) = &mut self.initialCondition {
            condition.ResolveReferences(context);
        }
        self.base.ResolveReferences(context);
    }

    // C# signature: Expression initialCondition { get; }
    pub fn get_initialCondition(&self) -> Option<&Expression> {
        self.initialCondition.as_ref()
    }

    // C# signature: List<ConditionalSingleBranch> branches { get; }
    pub fn get_branches(&self) -> &[ConditionalSingleBranch] {
        &self.branches
    }
}
