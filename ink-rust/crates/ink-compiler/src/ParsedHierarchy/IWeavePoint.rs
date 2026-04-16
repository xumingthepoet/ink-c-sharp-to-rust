// Source: ink-c-sharp/compiler/ParsedHierarchy/IWeavePoint.cs

use crate::ParsedHierarchy::Identifier::Identifier;
use crate::ParsedHierarchy::Object::Object;
use ink_runtime::Container::Container;

pub trait IWeavePoint {
    fn get_indentationDepth(&self) -> i32;
    fn get_runtimeContainer(&self) -> Option<&Container>;
    fn get_content(&self) -> &[Object];
    fn get_name(&self) -> Option<&str>;
    fn get_identifier(&self) -> Option<&Identifier>;
}
