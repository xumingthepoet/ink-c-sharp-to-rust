// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Profiler.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Profiler {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct StepDetails {
    pub _port_marker: (),
}

#[derive(Clone, Debug, Default)]
pub struct ProfileNode {
    pub _port_marker: (),
}

impl Profiler {
    // C# signature: public Profiler()
    pub fn new() -> Self {
        Default::default()
    }

    // C# signature: public string Report()
    pub fn Report(&mut self) -> String {
        Default::default()
    }

    // C# signature: public void PreContinue()
    pub fn PreContinue(&mut self) {}

    // C# signature: public void PostContinue()
    pub fn PostContinue(&mut self) {}

    // C# signature: public void PreStep()
    pub fn PreStep(&mut self) {}

    // C# signature: public void Step(CallStack callstack)
    pub fn Step(&mut self, _callstack: crate::stub::CallStack) {}

    // C# signature: public void PostStep()
    pub fn PostStep(&mut self) {}

    // C# signature: public string StepLengthReport()
    pub fn StepLengthReport(&mut self) -> String {
        Default::default()
    }

    // C# signature: public string Megalog()
    pub fn Megalog(&mut self) -> String {
        Default::default()
    }

    // C# signature: public void PreSnapshot()
    pub fn PreSnapshot(&mut self) {}

    // C# signature: public void PostSnapshot()
    pub fn PostSnapshot(&mut self) {}

    // C# signature: public static string FormatMillisecs(double num)
    pub fn FormatMillisecs(_num: f64) -> String {
        Default::default()
    }

    // C# signature: public void AddSample(string[] stack, double duration)
    pub fn AddSample(&mut self, _stack: Vec<String>, _duration: f64) {}

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        Default::default()
    }

    // C# signature: ProfileNode rootNode { get; }
    pub fn get_rootNode(&mut self) -> crate::stub::ProfileNode {
        Default::default()
    }

    // C# signature: bool hasChildren { get; }
    pub fn get_hasChildren(&mut self) -> bool {
        Default::default()
    }

    // C# signature: int totalMillisecs { get; }
    pub fn get_totalMillisecs(&mut self) -> i32 {
        Default::default()
    }

    // C# signature: string ownReport { get; }
    pub fn get_ownReport(&mut self) -> String {
        Default::default()
    }
}
