// Source: ink-c-sharp/compiler/ParsedHierarchy/Sequence.cs

use crate::ParsedHierarchy::ContentList::ContentList;
use crate::ParsedHierarchy::Story::Story;
use ink_runtime::Container::{Container, ContentItem};
use ink_runtime::ControlCommand::ControlCommand;
use ink_runtime::Divert::Divert;
use ink_runtime::NativeFunctionCall::NativeFunctionCall;
use ink_runtime::Path::{Component, Path};
use ink_runtime::Value::Value;
use std::ops::{BitAnd, BitOr, BitOrAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SequenceType(i32);

impl SequenceType {
    pub const STOPPING: Self = Self(1);
    pub const CYCLE: Self = Self(2);
    pub const SHUFFLE: Self = Self(4);
    pub const ONCE: Self = Self(8);

    pub fn bits(self) -> i32 {
        self.0
    }

    pub fn from_bits(bits: i32) -> Self {
        Self(bits)
    }

    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}

impl Default for SequenceType {
    fn default() -> Self {
        Self::STOPPING
    }
}

impl BitOr for SequenceType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for SequenceType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for SequenceType {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::fmt::Display for SequenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if self.contains(Self::STOPPING) {
            parts.push("Stopping");
        }
        if self.contains(Self::CYCLE) {
            parts.push("Cycle");
        }
        if self.contains(Self::SHUFFLE) {
            parts.push("Shuffle");
        }
        if self.contains(Self::ONCE) {
            parts.push("Once");
        }
        if parts.is_empty() {
            write!(f, "0")
        } else {
            write!(f, "{}", parts.join(" | "))
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SequenceDivertToResolve {
    pub divert: Divert,
    pub targetPath: Path,
}

#[derive(Clone, Debug, Default)]
pub struct Sequence {
    sequenceElements: Vec<ContentList>,
    sequenceType: SequenceType,
    sequenceDivertsToResolve: Vec<SequenceDivertToResolve>,
}

impl Sequence {
    // C# signature: public Sequence (List<ContentList> elementContentLists, SequenceType sequenceType)
    pub fn new(elementContentLists: Vec<ContentList>, sequenceType: SequenceType) -> Self {
        Self {
            sequenceElements: elementContentLists,
            sequenceType,
            sequenceDivertsToResolve: Vec::new(),
        }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&mut self) -> ContentItem {
        self.sequenceDivertsToResolve.clear();

        let mut container = Container::new();
        container.set_countFlags(1 | 4);

        container.AddContent(ControlCommand::EvalStart());
        container.AddContent(ControlCommand::VisitIndex());

        let once = self.sequenceType.contains(SequenceType::ONCE);
        let cycle = self.sequenceType.contains(SequenceType::CYCLE);
        let stopping = self.sequenceType.contains(SequenceType::STOPPING);
        let shuffle = self.sequenceType.contains(SequenceType::SHUFFLE);

        let seqBranchCount = self.sequenceElements.len() + usize::from(once);

        if stopping || once {
            container.AddContent(Value::new_int((seqBranchCount.saturating_sub(1)) as i32));
            container.AddContent(NativeFunctionCall::CallWithName("MIN".to_string()));
        } else if cycle {
            container.AddContent(Value::new_int(self.sequenceElements.len() as i32));
            container.AddContent(NativeFunctionCall::CallWithName("%".to_string()));
        }

        if shuffle {
            let mut skip_shuffle = Container::new();
            skip_shuffle.set_name(Some("__seq_shuffle_exit".to_string()));
            skip_shuffle.set_path(Path::new_overload_3(
                vec![Component::new_overload_2(
                    skip_shuffle.get_name().to_string(),
                )],
                false,
            ));
            let shuffleExitPath = skip_shuffle.get_path().clone();

            if once || stopping {
                let last_idx = if stopping {
                    self.sequenceElements.len().saturating_sub(1)
                } else {
                    self.sequenceElements.len()
                };
                container.AddContent(ControlCommand::Duplicate());
                container.AddContent(Value::new_int(last_idx as i32));
                container.AddContent(NativeFunctionCall::CallWithName("==".to_string()));

                let mut skip_shuffle_divert = Divert::new();
                skip_shuffle_divert.set_isConditional(true);
                container.AddContent(skip_shuffle_divert.clone());
                self.sequenceDivertsToResolve.push(SequenceDivertToResolve {
                    divert: skip_shuffle_divert,
                    targetPath: shuffleExitPath.clone(),
                });
            }

            let mut element_count_to_shuffle = self.sequenceElements.len();
            if stopping {
                element_count_to_shuffle = element_count_to_shuffle.saturating_sub(1);
            }
            container.AddContent(Value::new_int(element_count_to_shuffle as i32));
            container.AddContent(ControlCommand::SequenceShuffleIndex());

            // Sequence shuffle needs a place to continue after the chosen index is emitted.
            let mut shuffle_exit = Container::new();
            shuffle_exit.set_name(Some("__seq_shuffle_exit".to_string()));
            shuffle_exit.set_path(shuffleExitPath);
            container.AddToNamedContentOnly(shuffle_exit);
        }

        container.AddContent(ControlCommand::EvalEnd());

        let mut postSequenceNoOp = Container::new();
        postSequenceNoOp.set_name(Some("__seq_end".to_string()));
        postSequenceNoOp.set_path(Path::new_overload_3(
            vec![Component::new_overload_2(
                postSequenceNoOp.get_name().to_string(),
            )],
            false,
        ));

        for (el_index, element) in self.sequenceElements.iter_mut().enumerate() {
            container.AddContent(ControlCommand::EvalStart());
            container.AddContent(ControlCommand::Duplicate());
            container.AddContent(Value::new_int(el_index as i32));
            container.AddContent(NativeFunctionCall::CallWithName("==".to_string()));
            container.AddContent(ControlCommand::EvalEnd());

            let mut sequence_divert = Divert::new();
            sequence_divert.set_isConditional(true);
            container.AddContent(sequence_divert.clone());

            let mut branch_container = element.GenerateRuntimeObject();
            branch_container.set_name(Some(format!("s{}", el_index)));
            branch_container.set_path(Path::new_overload_3(
                vec![Component::new_overload_2(
                    branch_container.get_name().to_string(),
                )],
                false,
            ));

            let branch_path = branch_container.get_path().clone();
            branch_container.InsertContent(ControlCommand::PopEvaluatedValue(), 0);

            let seq_branch_complete_divert = Divert::new();
            branch_container.AddContent(seq_branch_complete_divert.clone());
            container.AddToNamedContentOnly(branch_container.clone());

            self.sequenceDivertsToResolve.push(SequenceDivertToResolve {
                divert: sequence_divert,
                targetPath: branch_path,
            });
            self.sequenceDivertsToResolve.push(SequenceDivertToResolve {
                divert: seq_branch_complete_divert,
                targetPath: postSequenceNoOp.get_path().clone(),
            });
        }

        container.AddToNamedContentOnly(postSequenceNoOp);

        ContentItem::Container(Box::new(container))
    }

    // C# signature: public override void ResolveReferences(Story context)
    pub fn ResolveReferences(&mut self, context: &mut Story) {
        for content_list in &mut self.sequenceElements {
            content_list.ResolveReferences(context);
        }

        for to_resolve in &mut self.sequenceDivertsToResolve {
            to_resolve
                .divert
                .set_targetPathString(Some(to_resolve.targetPath.ToString()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Sequence, SequenceType};
    use crate::ParsedHierarchy::ContentList::{ContentList, ContentListItem};
    use crate::ParsedHierarchy::Text::Text;
    use ink_runtime::Container::ContentItem;

    #[test]
    fn generates_runtime_container_for_simple_sequence() {
        let mut sequence = Sequence::new(
            vec![
                ContentList::new(vec![ContentListItem::from(Text::new("a".to_string()))]),
                ContentList::new(vec![ContentListItem::from(Text::new("b".to_string()))]),
            ],
            SequenceType::default(),
        );

        let runtime = sequence.GenerateRuntimeObject();
        assert!(matches!(runtime, ContentItem::Container(_)));
        assert!(sequence.sequenceType.contains(SequenceType::STOPPING));
    }
}
