#![allow(dead_code)]

use crate::types::Value;
use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

/// A stack in the context of the VM
///
/// This will actually have a stack of data, and a map of names to stack indices
/// These are also nested by block; when a new block is entered, a new stack is created
#[derive(Debug, Clone, Default)]
pub struct Stack {
    // The values on the stack
    data: Rc<RefCell<Vec<Value>>>,
    // A mapping of names to indices in the data
    names: HashMap<String, usize>,
    // The parent of this stack for name lookups
    parent: Option<Rc<RefCell<Stack>>>,
}

impl Stack {
    /// Creates a new top level stack
    pub fn new() -> Self {
        Stack::default()
    }

    /// Creates a new stack with the current stack as its parent
    ///
    /// n is the number of values to pop from the parent stack and push onto this one
    pub fn extend(&mut self, n: usize) -> Self {
        let mut values = vec![];
        for _ in 0..n {
            values.push(self.pop().unwrap());
        }
        values.reverse();

        Stack {
            data: Rc::new(RefCell::new(values)),
            names: HashMap::new(),
            parent: Some(Rc::new(RefCell::new(self.clone()))),
        }
    }

    /// Pushes a value onto the stack
    pub fn push(&mut self, value: Value) {
        self.data.clone().borrow_mut().push(value);
    }

    /// Pops a value off the stack
    ///
    /// TODO: Handle popping a named value
    pub fn pop(&mut self) -> Option<Value> {
        self.data.clone().borrow_mut().pop()
    }

    /// Assign a new name to the top value on the stack
    ///
    /// A single stack can have multiple names for the same value
    pub fn name(&mut self, name: String) {
        self.names
            .insert(name, self.data.clone().borrow().len() - 1);
    }

    /// Assigns a new name to the top N values of the stack (from bottom to top)
    ///
    /// If the stack is [8, 6, 7, 5], name_many("A", "B") would result in [8, 6, 7@A, 5@B]
    pub fn name_many(&mut self, names: Vec<String>) {
        for (i, name) in names.iter().enumerate() {
            self.names.insert(
                name.clone(),
                self.data.clone().borrow().len() - names.len() + i,
            );
        }
    }

    /// Get a named value from this stack (including the parent) if it exists
    ///
    /// If this stack doesn't have it, check the parent
    pub fn get_named(&self, name: String) -> Option<Value> {
        log::debug!("get_named({}) from {}", name, self);

        if self.names.contains_key(&name) {
            Some(self.data.clone().borrow()[self.names[&name]].clone())
        } else if self.parent.is_some() {
            self.parent.as_ref().unwrap().borrow().get_named(name)
        } else {
            None
        }
    }

    /// Set a named value on this stack (including the parent)
    ///
    /// If this stack doesn't have it, check the parent
    /// If it's not found, panic!
    pub fn set_named(&mut self, name: String, value: Value) {
        log::debug!("set_named({}, {}) on {}", name, value, self);

        if self.names.contains_key(&name) {
            self.data
                .clone()
                .borrow_mut()
                .insert(self.names[&name], value);
            // self.data[self.names[&name]] = value;
        } else if self.parent.is_some() {
            self.parent
                .as_ref()
                .unwrap()
                .clone()
                .borrow_mut()
                .set_named(name, value);
        } else {
            panic!("set_named({}, {}) on {}", name, value, self);
        }
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        if self.parent.is_some() {
            s.push_str(self.parent.as_ref().unwrap().borrow().to_string().as_str());
            s.push_str(" : ");
        }

        s.push('[');
        for (i, v) in self.data.clone().borrow().iter().enumerate() {
            s.push_str(format!("{}", v).as_str());

            for (k, v) in self.names.iter() {
                if *v == i {
                    s.push_str(&format!("@{}", k));
                }
            }

            if i != self.data.clone().borrow().len() - 1 {
                s.push(' ');
            }
        }
        s.push(']');
        write!(f, "{}", s)
    }
}
