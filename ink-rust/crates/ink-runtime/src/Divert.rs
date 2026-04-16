// Source: ink-c-sharp/ink-engine-runtime/Divert.cs

use crate::Path::Path;

#[derive(Clone, Debug, PartialEq)]
pub struct Divert {
    targetPath: Option<Path>,
    variableDivertName: Option<String>,
    pushesToStack: bool,
    stackPushType: crate::PushPop::PushPopType,
    isExternal: bool,
    externalArgs: i32,
    isConditional: bool,
}

impl Default for Divert {
    fn default() -> Self {
        Self {
            targetPath: None,
            variableDivertName: None,
            pushesToStack: false,
            stackPushType: crate::PushPop::PushPopType::Tunnel,
            isExternal: false,
            externalArgs: 0,
            isConditional: false,
        }
    }
}

impl Divert {
    // C# signature: public Divert ()
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public Divert(PushPopType stackPushType)
    pub fn new_overload_2(stackPushType: crate::PushPop::PushPopType) -> Self {
        Self {
            pushesToStack: true,
            stackPushType,
            ..Default::default()
        }
    }

    // C# signature: public override bool Equals (object obj)
    pub fn Equals(&self, obj: &Divert) -> bool {
        if self.hasVariableTarget() == obj.hasVariableTarget() {
            if self.hasVariableTarget() {
                self.variableDivertName == obj.variableDivertName
            } else {
                self.targetPath == obj.targetPath
            }
        } else {
            false
        }
    }

    // C# signature: public override int GetHashCode ()
    pub fn GetHashCode(&self) -> i32 {
        if self.hasVariableTarget() {
            self.variableDivertName.as_deref().unwrap_or("").len() as i32 + 12345
        } else {
            self.targetPath
                .as_ref()
                .map(|path| path.GetHashCode())
                .unwrap_or(0)
                + 54321
        }
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        if let Some(name) = &self.variableDivertName {
            format!("Divert(variable: {})", name)
        } else if self.targetPath.is_none() {
            "Divert(null)".to_string()
        } else {
            let mut result = String::from("Divert");
            if self.isConditional {
                result.push('?');
            }
            if self.pushesToStack {
                match self.stackPushType {
                    crate::PushPop::PushPopType::Function => result.push_str(" function"),
                    crate::PushPop::PushPopType::Tunnel => result.push_str(" tunnel"),
                    _ => {}
                }
            }

            result.push_str(" -> ");
            result.push_str(&self.get_targetPathString());
            result.push_str(" (");
            result.push_str(&self.targetPath.as_ref().unwrap().ToString());
            result.push(')');
            result
        }
    }

    // C# signature: Path targetPath { get; }
    pub fn get_targetPath(&self) -> Option<&Path> {
        self.targetPath.as_ref()
    }

    // C# signature: Pointer targetPointer { get; }
    pub fn get_targetPointer(&self) -> crate::Pointer::Pointer {
        todo!("port runtime Divert.targetPointer after Runtime.Object.ResolvePath is translated");
    }

    // C# signature: string targetPathString { get; }
    pub fn get_targetPathString(&self) -> String {
        self.targetPath
            .as_ref()
            .map(|path| path.ToString())
            .unwrap_or_default()
    }

    pub fn set_targetPathString(&mut self, value: Option<String>) {
        self.targetPath = value.map(Path::new_overload_4);
    }

    // C# signature: string variableDivertName { get; }
    pub fn get_variableDivertName(&self) -> Option<&str> {
        self.variableDivertName.as_deref()
    }

    pub fn set_variableDivertName(&mut self, value: Option<String>) {
        self.variableDivertName = value;
    }

    // C# signature: bool hasVariableTarget { get; }
    pub fn hasVariableTarget(&self) -> bool {
        self.variableDivertName.is_some()
    }

    // C# signature: bool pushesToStack { get; }
    pub fn get_pushesToStack(&self) -> bool {
        self.pushesToStack
    }

    pub fn set_pushesToStack(&mut self, value: bool) {
        self.pushesToStack = value;
    }

    pub fn get_stackPushType(&self) -> crate::PushPop::PushPopType {
        self.stackPushType
    }

    // C# signature: bool isExternal { get; }
    pub fn get_isExternal(&self) -> bool {
        self.isExternal
    }

    pub fn set_isExternal(&mut self, value: bool) {
        self.isExternal = value;
    }

    // C# signature: int externalArgs { get; }
    pub fn get_externalArgs(&self) -> i32 {
        self.externalArgs
    }

    pub fn set_externalArgs(&mut self, value: i32) {
        self.externalArgs = value;
    }

    // C# signature: bool isConditional { get; }
    pub fn get_isConditional(&self) -> bool {
        self.isConditional
    }

    pub fn set_isConditional(&mut self, value: bool) {
        self.isConditional = value;
    }
}

#[cfg(test)]
mod tests {
    use super::Divert;
    use crate::PushPop::PushPopType;

    #[test]
    fn stringifies_variable_and_stack_diverts() {
        let mut divert = Divert::new();
        divert.set_variableDivertName(Some("score".to_string()));
        assert_eq!(divert.ToString(), "Divert(variable: score)");

        let mut call = Divert::new_overload_2(PushPopType::Function);
        call.set_targetPathString(Some("knot.stitch".to_string()));
        assert!(call.ToString().contains("Divert function -> knot.stitch"));
    }
}
