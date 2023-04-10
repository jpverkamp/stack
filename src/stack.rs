use std::{collections::HashMap, rc::Rc};

use crate::types::{Value};

#[derive(Debug, Default)]
pub struct Stack {
    data: Vec<Value>,
    names: HashMap<String, usize>,
    parent: Option<Rc<Stack>>,
}

impl Stack {
    pub fn new() -> Self {
        Stack::default()
    }

    pub fn extend(self) -> Self {
        Stack {
            data: vec![],
            names: HashMap::new(),
            parent: Some(Rc::new(self)),
        }
    }

    pub fn push(&mut self, value: Value) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.data.pop()
    }

    pub fn name(&mut self, name: String) {
        self.names.insert(name, self.data.len() - 1);
    }

    pub fn name_many(&mut self, names: Vec<String>) {
        for (i, name) in names.iter().enumerate() {
            self.names.insert(name.clone(), self.data.len() - 1 - i);
        }
    }

    pub fn get_named(&self, name: String) -> Option<Value> {
        if self.names.contains_key(&name) {
            Some(self.data[self.names[&name]].clone())
        } else if self.parent.is_some() {
            self.parent.as_ref().unwrap().get_named(name)
        } else {
            None
        }
    }
}