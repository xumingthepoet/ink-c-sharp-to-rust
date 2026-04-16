// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/ControlCommand.cs

#[derive(Clone, Debug, Default)]
pub struct ControlCommand {
    pub commandType: CommandType,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub enum CommandType {
    #[default]
    NotSet = -1,
    EvalStart,
    EvalOutput,
    EvalEnd,
    Duplicate,
    PopEvaluatedValue,
    PopFunction,
    PopTunnel,
    BeginString,
    EndString,
    NoOp,
    ChoiceCount,
    Turns,
    TurnsSince,
    ReadCount,
    Random,
    SeedRandom,
    VisitIndex,
    SequenceShuffleIndex,
    StartThread,
    Done,
    End,
    ListFromInt,
    ListRange,
    ListRandom,
    BeginTag,
    EndTag,
    TOTAL_VALUES,
}

impl ControlCommand {
    // C# signature: public ControlCommand (CommandType commandType)
    pub fn new(_commandType: CommandType) -> Self {
        Self {
            commandType: _commandType,
        }
    }

    // C# signature: public ControlCommand()
    pub fn new_overload_2() -> Self {
        Self::new(CommandType::NotSet)
    }

    // C# signature: public override Object Copy()
    pub fn Copy(&self) -> ControlCommand {
        Self::new(self.commandType)
    }

    // C# signature: public static ControlCommand EvalStart()
    pub fn EvalStart() -> ControlCommand {
        Self::new(CommandType::EvalStart)
    }

    // C# signature: public static ControlCommand EvalOutput()
    pub fn EvalOutput() -> ControlCommand {
        Self::new(CommandType::EvalOutput)
    }

    // C# signature: public static ControlCommand EvalEnd()
    pub fn EvalEnd() -> ControlCommand {
        Self::new(CommandType::EvalEnd)
    }

    // C# signature: public static ControlCommand Duplicate()
    pub fn Duplicate() -> ControlCommand {
        Self::new(CommandType::Duplicate)
    }

    // C# signature: public static ControlCommand PopEvaluatedValue()
    pub fn PopEvaluatedValue() -> ControlCommand {
        Self::new(CommandType::PopEvaluatedValue)
    }

    // C# signature: public static ControlCommand PopFunction()
    pub fn PopFunction() -> ControlCommand {
        Self::new(CommandType::PopFunction)
    }

    // C# signature: public static ControlCommand PopTunnel()
    pub fn PopTunnel() -> ControlCommand {
        Self::new(CommandType::PopTunnel)
    }

    // C# signature: public static ControlCommand BeginString()
    pub fn BeginString() -> ControlCommand {
        Self::new(CommandType::BeginString)
    }

    // C# signature: public static ControlCommand EndString()
    pub fn EndString() -> ControlCommand {
        Self::new(CommandType::EndString)
    }

    // C# signature: public static ControlCommand NoOp()
    pub fn NoOp() -> ControlCommand {
        Self::new(CommandType::NoOp)
    }

    // C# signature: public static ControlCommand ChoiceCount()
    pub fn ChoiceCount() -> ControlCommand {
        Self::new(CommandType::ChoiceCount)
    }

    // C# signature: public static ControlCommand Turns ()
    pub fn Turns() -> ControlCommand {
        Self::new(CommandType::Turns)
    }

    // C# signature: public static ControlCommand TurnsSince()
    pub fn TurnsSince() -> ControlCommand {
        Self::new(CommandType::TurnsSince)
    }

    // C# signature: public static ControlCommand ReadCount ()
    pub fn ReadCount() -> ControlCommand {
        Self::new(CommandType::ReadCount)
    }

    // C# signature: public static ControlCommand Random ()
    pub fn Random() -> ControlCommand {
        Self::new(CommandType::Random)
    }

    // C# signature: public static ControlCommand SeedRandom ()
    pub fn SeedRandom() -> ControlCommand {
        Self::new(CommandType::SeedRandom)
    }

    // C# signature: public static ControlCommand VisitIndex()
    pub fn VisitIndex() -> ControlCommand {
        Self::new(CommandType::VisitIndex)
    }

    // C# signature: public static ControlCommand SequenceShuffleIndex()
    pub fn SequenceShuffleIndex() -> ControlCommand {
        Self::new(CommandType::SequenceShuffleIndex)
    }

    // C# signature: public static ControlCommand StartThread()
    pub fn StartThread() -> ControlCommand {
        Self::new(CommandType::StartThread)
    }

    // C# signature: public static ControlCommand Done()
    pub fn Done() -> ControlCommand {
        Self::new(CommandType::Done)
    }

    // C# signature: public static ControlCommand End()
    pub fn End() -> ControlCommand {
        Self::new(CommandType::End)
    }

    // C# signature: public static ControlCommand ListFromInt ()
    pub fn ListFromInt() -> ControlCommand {
        Self::new(CommandType::ListFromInt)
    }

    // C# signature: public static ControlCommand ListRange ()
    pub fn ListRange() -> ControlCommand {
        Self::new(CommandType::ListRange)
    }

    // C# signature: public static ControlCommand ListRandom ()
    pub fn ListRandom() -> ControlCommand {
        Self::new(CommandType::ListRandom)
    }

    // C# signature: public static ControlCommand BeginTag ()
    pub fn BeginTag() -> ControlCommand {
        Self::new(CommandType::BeginTag)
    }

    // C# signature: public static ControlCommand EndTag ()
    pub fn EndTag() -> ControlCommand {
        Self::new(CommandType::EndTag)
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        self.to_string()
    }

    // C# signature: CommandType commandType { get; }
    pub fn get_commandType(&self) -> CommandType {
        self.commandType
    }
}

impl std::fmt::Display for ControlCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.commandType)
    }
}
