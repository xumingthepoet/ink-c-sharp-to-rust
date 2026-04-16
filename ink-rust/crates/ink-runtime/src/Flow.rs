// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Flow.cs

use crate::stub::*;

#[derive(Clone, Debug, Default)]
pub struct Flow {
    pub _port_marker: (),
}

impl Flow {
    // C# signature: public Flow(string name, Story story)
    pub fn new(_name: String, _story: crate::stub::Story) -> Self {
        Default::default()
    }

    // C# signature: public Flow(string name, Story story, Dictionary<string, object> jObject)
    pub fn new_overload_2(
        _name: String,
        _story: crate::stub::Story,
        _jObject: std::collections::HashMap<String, crate::stub::PortStub>,
    ) -> Self {
        Default::default()
    }

    // C# signature: public void WriteJson(SimpleJson.Writer writer)
    pub fn WriteJson(&mut self, _writer: crate::stub::Writer) {}

    // C# signature: public void LoadFlowChoiceThreads(Dictionary<string, object> jChoiceThreads, Story story)
    pub fn LoadFlowChoiceThreads(
        &mut self,
        _jChoiceThreads: std::collections::HashMap<String, crate::stub::PortStub>,
        _story: crate::stub::Story,
    ) {
    }
}
