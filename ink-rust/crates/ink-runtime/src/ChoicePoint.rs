// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/ChoicePoint.cs

use crate::Container::Container;
use crate::Path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChoicePoint {
    pathOnChoice: Option<Path>,
    pub hasCondition: bool,
    pub hasStartContent: bool,
    pub hasChoiceOnlyContent: bool,
    pub onceOnly: bool,
    pub isInvisibleDefault: bool,
}

impl ChoicePoint {
    // C# signature: public ChoicePoint (bool onceOnly)
    pub fn new(onceOnly: bool) -> Self {
        Self {
            pathOnChoice: None,
            hasCondition: false,
            hasStartContent: false,
            hasChoiceOnlyContent: false,
            onceOnly,
            isInvisibleDefault: false,
        }
    }

    // C# signature: public ChoicePoint()
    pub fn new_overload_2() -> Self {
        Self::new(true)
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&mut self) -> String {
        match self.pathOnChoice.as_ref() {
            Some(path) => format!("Choice: -> {}", path),
            None => "Choice: -> ".to_string(),
        }
    }

    // C# signature: Path pathOnChoice { get; }
    pub fn get_pathOnChoice(&self) -> Option<&Path> {
        self.pathOnChoice.as_ref()
    }

    // C# signature: Container choiceTarget { get; }
    pub fn get_choiceTarget(&mut self) -> Option<Container> {
        todo!(
            "port runtime ChoicePoint.choiceTarget after Runtime.Object.ResolvePath is translated"
        );
    }

    // C# signature: string pathStringOnChoice { get; }
    pub fn get_pathStringOnChoice(&mut self) -> String {
        self.pathOnChoice
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default()
    }

    pub fn set_pathStringOnChoice(&mut self, value: String) {
        self.pathOnChoice = Some(Path::new_overload_4(value));
    }

    // C# signature: bool hasCondition { get; }
    pub fn get_hasCondition(&self) -> bool {
        self.hasCondition
    }

    pub fn set_hasCondition(&mut self, value: bool) {
        self.hasCondition = value;
    }

    // C# signature: bool hasStartContent { get; }
    pub fn get_hasStartContent(&self) -> bool {
        self.hasStartContent
    }

    pub fn set_hasStartContent(&mut self, value: bool) {
        self.hasStartContent = value;
    }

    // C# signature: bool hasChoiceOnlyContent { get; }
    pub fn get_hasChoiceOnlyContent(&self) -> bool {
        self.hasChoiceOnlyContent
    }

    pub fn set_hasChoiceOnlyContent(&mut self, value: bool) {
        self.hasChoiceOnlyContent = value;
    }

    // C# signature: bool onceOnly { get; }
    pub fn get_onceOnly(&self) -> bool {
        self.onceOnly
    }

    pub fn set_onceOnly(&mut self, value: bool) {
        self.onceOnly = value;
    }

    // C# signature: bool isInvisibleDefault { get; }
    pub fn get_isInvisibleDefault(&self) -> bool {
        self.isInvisibleDefault
    }

    pub fn set_isInvisibleDefault(&mut self, value: bool) {
        self.isInvisibleDefault = value;
    }

    // C# signature: int flags { get; }
    pub fn get_flags(&self) -> i32 {
        let mut flags = 0;
        if self.hasCondition {
            flags |= 1;
        }
        if self.hasStartContent {
            flags |= 2;
        }
        if self.hasChoiceOnlyContent {
            flags |= 4;
        }
        if self.isInvisibleDefault {
            flags |= 8;
        }
        if self.onceOnly {
            flags |= 16;
        }
        flags
    }

    pub fn set_flags(&mut self, value: i32) {
        self.hasCondition = (value & 1) > 0;
        self.hasStartContent = (value & 2) > 0;
        self.hasChoiceOnlyContent = (value & 4) > 0;
        self.isInvisibleDefault = (value & 8) > 0;
        self.onceOnly = (value & 16) > 0;
    }
}

#[cfg(test)]
mod tests {
    use super::ChoicePoint;

    #[test]
    fn tracks_choice_flags_and_path_strings() {
        let mut choice = ChoicePoint::new(false);
        choice.set_pathStringOnChoice("knot.stitch".to_string());
        choice.set_hasCondition(true);
        choice.set_hasChoiceOnlyContent(true);

        assert_eq!(choice.get_onceOnly(), false);
        assert_eq!(choice.get_pathStringOnChoice(), "knot.stitch");
        assert_eq!(choice.get_flags(), 1 | 4);
        assert_eq!(choice.ToString(), "Choice: -> knot.stitch");
    }
}
