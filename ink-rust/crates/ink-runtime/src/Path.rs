// Auto-generated structural port skeleton. Fill behavior from the matching C# source.
// Source: ink-c-sharp/ink-engine-runtime/Path.cs

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const PARENT_ID: &str = "^";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Component {
    pub index: i32,
    pub name: Option<String>,
}

impl Component {
    pub fn new(index: i32) -> Self {
        debug_assert!(index >= 0);
        Self { index, name: None }
    }

    pub fn new_overload_2(name: String) -> Self {
        debug_assert!(!name.is_empty());
        Self {
            index: -1,
            name: Some(name),
        }
    }

    pub fn ToParent() -> Component {
        Self::new_overload_2(PARENT_ID.to_string())
    }

    pub fn ToString(&self) -> String {
        self.to_string()
    }

    pub fn Equals(&self, otherComp: &Component) -> bool {
        self == otherComp
    }

    pub fn GetHashCode(&self) -> i32 {
        if self.get_isIndex() {
            self.index
        } else {
            hash_to_i32(self.name.as_deref().unwrap_or(""))
        }
    }

    pub fn get_index(&self) -> i32 {
        self.index
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn get_isIndex(&self) -> bool {
        self.index >= 0
    }

    pub fn get_isParent(&self) -> bool {
        self.name.as_deref() == Some(PARENT_ID)
    }
}

impl Default for Component {
    fn default() -> Self {
        Self {
            index: -1,
            name: None,
        }
    }
}

impl std::fmt::Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.get_isIndex() {
            write!(f, "{}", self.index)
        } else {
            f.write_str(self.name.as_deref().unwrap_or(""))
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Path {
    components: Vec<Component>,
    pub isRelative: bool,
}

impl Path {
    // C# signature: public Path()
    pub fn new() -> Self {
        Self::default()
    }

    // C# signature: public Path(Component head, Path tail)
    pub fn new_overload_2(_head: Component, _tail: Path) -> Self {
        let mut components = Vec::with_capacity(1 + _tail.components.len());
        components.push(_head);
        components.extend(_tail.components);
        Self {
            components,
            isRelative: false,
        }
    }

    // C# signature: public Path(IEnumerable<Component> components, bool relative = false)
    pub fn new_overload_3(_components: Vec<Component>, _relative: bool) -> Self {
        Self {
            components: _components,
            isRelative: _relative,
        }
    }

    // C# signature: public Path(string componentsString)
    pub fn new_overload_4(_componentsString: String) -> Self {
        Self::from_components_string(&_componentsString)
    }

    fn from_components_string(components_string: &str) -> Self {
        if components_string.is_empty() {
            return Self::new();
        }

        let (is_relative, raw_components) = if let Some(rest) = components_string.strip_prefix('.')
        {
            (true, rest)
        } else {
            (false, components_string)
        };

        if raw_components.is_empty() {
            return Self {
                components: Vec::new(),
                isRelative: is_relative,
            };
        }

        let components = raw_components
            .split('.')
            .map(|component| match component.parse::<i32>() {
                Ok(index) => Component::new(index),
                Err(_) => Component::new_overload_2(component.to_string()),
            })
            .collect();

        Self {
            components,
            isRelative: is_relative,
        }
    }

    pub fn ToParent() -> Component {
        Component::ToParent()
    }

    pub fn ToString(&self) -> String {
        self.to_string()
    }

    pub fn Equals(&self, otherPath: &Path) -> bool {
        self == otherPath
    }

    pub fn GetHashCode(&self) -> i32 {
        hash_to_i32(&self.to_string())
    }

    // C# signature: public Component GetComponent(int index)
    pub fn GetComponent(&self, _index: i32) -> Option<&Component> {
        if _index < 0 {
            return None;
        }
        self.components.get(_index as usize)
    }

    // C# signature: public Path PathByAppendingPath(Path pathToAppend)
    pub fn PathByAppendingPath(&self, _pathToAppend: &Path) -> Path {
        let upward_moves = _pathToAppend
            .components
            .iter()
            .take_while(|component| component.get_isParent())
            .count();

        let keep_count = self.components.len().saturating_sub(upward_moves);
        let mut components = Vec::new();
        components.extend(self.components.iter().take(keep_count).cloned());
        components.extend(_pathToAppend.components.iter().skip(upward_moves).cloned());

        Path {
            components,
            isRelative: false,
        }
    }

    // C# signature: public Path PathByAppendingComponent (Component c)
    pub fn PathByAppendingComponent(&self, _c: Component) -> Path {
        let mut components = self.components.clone();
        components.push(_c);
        Path {
            components,
            isRelative: false,
        }
    }

    pub fn get_isRelative(&self) -> bool {
        self.isRelative
    }

    pub fn get_head(&self) -> Option<&Component> {
        self.components.first()
    }

    pub fn get_tail(&self) -> Path {
        if self.components.len() >= 2 {
            Path {
                components: self.components[1..].to_vec(),
                isRelative: false,
            }
        } else {
            Self::get_self()
        }
    }

    pub fn get_length(&self) -> i32 {
        self.components.len() as i32
    }

    pub fn get_lastComponent(&self) -> Option<&Component> {
        self.components.last()
    }

    pub fn get_containsNamedComponent(&self) -> bool {
        self.components
            .iter()
            .any(|component| !component.get_isIndex())
    }

    pub fn get_self() -> Path {
        Path {
            components: Vec::new(),
            isRelative: true,
        }
    }

    pub fn get_componentsString(&self) -> String {
        let mut joined = self
            .components
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(".");

        if self.isRelative {
            joined.insert(0, '.');
        }

        joined
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_componentsString())
    }
}

fn hash_to_i32(value: &str) -> i32 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish() as i32
}
