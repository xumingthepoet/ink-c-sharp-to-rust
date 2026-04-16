// Source: ink-c-sharp/compiler/ParsedHierarchy/Tag.cs

use ink_runtime::ControlCommand::ControlCommand;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tag {
    pub isStart: bool,
    pub inChoice: bool,
}

impl Tag {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_flags(isStart: bool, inChoice: bool) -> Self {
        Self { isStart, inChoice }
    }

    // C# signature: public override Runtime.Object GenerateRuntimeObject ()
    pub fn GenerateRuntimeObject(&self) -> ControlCommand {
        if self.isStart {
            ControlCommand::BeginTag()
        } else {
            ControlCommand::EndTag()
        }
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        if self.isStart {
            "#StartTag".to_string()
        } else {
            "#EndTag".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tag;
    use ink_runtime::ControlCommand::CommandType;

    #[test]
    fn creates_matching_runtime_command() {
        let start = Tag::new_with_flags(true, false);
        let end = Tag::new_with_flags(false, false);

        assert_eq!(
            start.GenerateRuntimeObject().get_commandType(),
            CommandType::BeginTag
        );
        assert_eq!(
            end.GenerateRuntimeObject().get_commandType(),
            CommandType::EndTag
        );
        assert_eq!(start.ToString(), "#StartTag");
        assert_eq!(end.ToString(), "#EndTag");
    }
}
