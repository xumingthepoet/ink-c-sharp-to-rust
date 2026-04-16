// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/ControlCommand.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct ControlCommand {
    pub _port_marker: (),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandType {
    PortPlaceholder,
}

impl Default for CommandType {
    fn default() -> Self {
        Self::PortPlaceholder
    }
}

impl ControlCommand {
    // C# signature: public ControlCommand (CommandType commandType)
    pub fn new(_commandType: crate::stub::CommandType) -> Self {
        Default::default()
    }

    // C# signature: public ControlCommand()
    pub fn new_overload_2() -> Self {
        Default::default()
    }

    // C# signature: public override Object Copy()
    pub fn Copy(&mut self) -> crate::stub::PortStub {
        Default::default()
    }

    // C# signature: public static ControlCommand EvalStart()
    pub fn EvalStart() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand EvalOutput()
    pub fn EvalOutput() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand EvalEnd()
    pub fn EvalEnd() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand Duplicate()
    pub fn Duplicate() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand PopEvaluatedValue()
    pub fn PopEvaluatedValue() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand PopFunction()
    pub fn PopFunction() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand PopTunnel()
    pub fn PopTunnel() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand BeginString()
    pub fn BeginString() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand EndString()
    pub fn EndString() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand NoOp()
    pub fn NoOp() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand ChoiceCount()
    pub fn ChoiceCount() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand Turns ()
    pub fn Turns() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand TurnsSince()
    pub fn TurnsSince() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand ReadCount ()
    pub fn ReadCount() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand Random ()
    pub fn Random() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand SeedRandom ()
    pub fn SeedRandom() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand VisitIndex()
    pub fn VisitIndex() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand SequenceShuffleIndex()
    pub fn SequenceShuffleIndex() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand StartThread()
    pub fn StartThread() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand Done()
    pub fn Done() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand End()
    pub fn End() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand ListFromInt ()
    pub fn ListFromInt() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand ListRange ()
    pub fn ListRange() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand ListRandom ()
    pub fn ListRandom() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand BeginTag ()
    pub fn BeginTag() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public static ControlCommand EndTag ()
    pub fn EndTag() -> crate::stub::ControlCommand {
        Default::default()
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: CommandType commandType { get; }
    pub fn get_commandType(&mut self) -> crate::stub::CommandType {
        Default::default()
    }
}
