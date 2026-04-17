use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StringBuilder {
    buf: Rc<RefCell<String>>,
}

impl StringBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn Append(&mut self, value: impl AsRef<str>) {
        self.buf.borrow_mut().push_str(value.as_ref());
    }

    pub fn AppendChar(&mut self, value: char) {
        self.buf.borrow_mut().push(value);
    }

    pub fn AppendLine(&mut self, value: impl AsRef<str>) {
        self.Append(value);
        self.AppendChar('\n');
    }

    pub fn ToString(&self) -> String {
        self.buf.borrow().clone()
    }
}
