// Source: ink-c-sharp/ink-engine-runtime/ChoicePoint.cs

use crate::Container::Container;
use crate::Container::ContentItem;
use crate::DebugMetadata::DebugMetadata;
use crate::Path::Path;

#[derive(Clone, Debug, PartialEq)]
pub struct ChoicePoint {
    parent: Option<Box<Container>>,
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
            parent: None,
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
    pub fn ToString(&self) -> String {
        let Some(path) = self.get_pathOnChoice() else {
            return "Choice: -> ".to_string();
        };

        let mut target_string = path.ToString();
        if let Some(target_line_num) = self.debug_line_number_of_path(&path) {
            target_string = format!(" line {}({})", target_line_num, target_string);
        }

        format!("Choice: -> {}", target_string)
    }

    // C# signature: Path pathOnChoice { get; }
    pub fn get_pathOnChoice(&self) -> Option<Path> {
        let mut path = self.pathOnChoice.clone()?;
        if path.get_isRelative() {
            if let Some(choice_target) = self.choice_target_for_path(&path) {
                path = choice_target.get_path().clone();
            }
        }
        Some(path)
    }

    // C# signature: Container choiceTarget { get; }
    pub fn get_choiceTarget(&mut self) -> Option<Container> {
        let path = self.pathOnChoice.clone()?;
        self.choice_target_for_path(&path)
    }

    // C# signature: string pathStringOnChoice { get; }
    pub fn get_pathStringOnChoice(&self) -> Option<String> {
        let path = self.pathOnChoice.clone()?;
        Some(self.compact_path_string(path))
    }

    pub fn set_pathStringOnChoice(&mut self, value: String) {
        self.pathOnChoice = Some(Path::new_overload_4(value));
    }

    pub fn set_parent(&mut self, parent: Option<Box<Container>>) {
        self.parent = parent;
    }

    pub fn get_parent(&self) -> Option<&Container> {
        self.parent.as_deref()
    }

    fn debug_line_number_of_path(&self, path: &Path) -> Option<i32> {
        if path.get_length() == 0 {
            return None;
        }

        let mut root = self
            .parent
            .as_ref()
            .and_then(|container| container.get_rootContentContainer())
            .or_else(|| {
                self.parent
                    .as_ref()
                    .map(|container| container.as_ref().clone())
            })?;

        let target_content = root.ContentAtPath(path.clone(), 0, -1).obj?;
        match target_content {
            ContentItem::Container(container) => container
                .get_debugMetadata()
                .map(|debug_metadata| debug_metadata.startLineNumber),
            _ => None,
        }
    }

    fn choice_target_for_path(&self, path: &Path) -> Option<Container> {
        let container = self.parent.as_ref()?;
        let mut search_root = if path.get_isRelative() {
            container.as_ref().clone()
        } else {
            container
                .get_rootContentContainer()
                .unwrap_or_else(|| container.as_ref().clone())
        };

        search_root
            .ContentAtPath(path.clone(), 0, -1)
            .get_container()
            .cloned()
    }

    fn root_content_container(&self) -> Option<Container> {
        let parent = self.parent.as_ref()?;
        parent.get_rootContentContainer()
    }

    fn convert_path_to_relative(&self, global_path: Path) -> Path {
        let own_path = self
            .parent
            .as_ref()
            .map(|container| container.get_path().clone())
            .unwrap_or_default();
        let min_path_length = own_path.get_length().min(global_path.get_length());
        let mut last_shared_path_comp_index = -1;

        for i in 0..min_path_length {
            let Some(own_comp) = own_path.GetComponent(i) else {
                break;
            };
            let Some(other_comp) = global_path.GetComponent(i) else {
                break;
            };

            if own_comp.Equals(other_comp) {
                last_shared_path_comp_index = i;
            } else {
                break;
            }
        }

        if last_shared_path_comp_index == -1 {
            return global_path;
        }

        let num_upwards_moves = (own_path.get_length() - 1) - last_shared_path_comp_index;
        let mut new_path_comps = Vec::new();

        for _ in 0..num_upwards_moves {
            new_path_comps.push(Path::ToParent());
        }

        for down in (last_shared_path_comp_index + 1)..global_path.get_length() {
            if let Some(component) = global_path.GetComponent(down) {
                new_path_comps.push(component.clone());
            }
        }

        Path::new_overload_3(new_path_comps, true)
    }

    fn compact_path_string(&self, other_path: Path) -> String {
        let (relative_path_str, global_path_str) = if other_path.get_isRelative() {
            (
                other_path.get_componentsString(),
                self.parent
                    .as_ref()
                    .map(|container| container.get_path().PathByAppendingPath(&other_path))
                    .unwrap_or_else(|| other_path.clone())
                    .get_componentsString(),
            )
        } else {
            let relative_path = self.convert_path_to_relative(other_path.clone());
            (
                relative_path.get_componentsString(),
                other_path.get_componentsString(),
            )
        };

        if relative_path_str.len() < global_path_str.len() {
            relative_path_str
        } else {
            global_path_str
        }
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
    use crate::Container::{Container, ContentItem};

    #[test]
    fn tracks_choice_flags_and_path_strings() {
        let mut choice = ChoicePoint::new(false);
        choice.set_pathStringOnChoice("knot.stitch".to_string());
        choice.set_hasCondition(true);
        choice.set_hasChoiceOnlyContent(true);

        assert_eq!(choice.get_onceOnly(), false);
        assert_eq!(
            choice.get_pathStringOnChoice(),
            Some("knot.stitch".to_string())
        );
        assert_eq!(choice.get_flags(), 1 | 4);
        assert_eq!(choice.ToString(), "Choice: -> knot.stitch");
    }

    #[test]
    fn resolves_choice_target_from_parent_container() {
        let mut target = Container::new();
        target.set_name(Some("target".to_string()));

        let mut root = Container::new();
        root.AddToNamedContentOnly(target.clone());

        let mut choice = ChoicePoint::new(true);
        choice.set_parent(Some(Box::new(root.clone())));
        choice.set_pathStringOnChoice("target".to_string());

        let resolved = choice.get_choiceTarget().expect("choice target");
        assert_eq!(resolved.get_name(), "target");
        assert!(matches!(
            root.get_namedContent().get("target"),
            Some(ContentItem::Container(_))
        ));
    }
}
