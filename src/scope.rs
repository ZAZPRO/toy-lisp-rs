use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::object::Object;

#[derive(Debug, PartialEq, Default)]
pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    vars: HashMap<String, Object>,
}

impl Scope {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Self {
        Scope {
            parent: Some(parent),
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.vars.get(name) {
            Some(v) => Some(v.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|o| o.borrow().get(name).clone()),
        }
    }

    pub fn set(&mut self, name: &str, val: Object) {
        self.vars.insert(name.to_string(), val);
    }
}
