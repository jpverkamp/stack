#![allow(dead_code)]

use crate::types::Value;
use std::{collections::HashMap, rc::Rc};

/// A stack in the context of the VM
///
/// This will actually have a stack of data, and a map of names to stack indices
/// These are also nested by block; when a new block is entered, a new stack is created
#[derive(Debug, Default)]
pub struct Stack {
    // The values on the stack
    data: Vec<Value>,
    // A mapping of names to indices in the data
    names: HashMap<String, usize>,
    // The parent of this stack for name lookups
    parent: Option<Rc<Stack>>,
}

impl Stack {
    /// Creates a new top level stack
    pub fn new() -> Self {
        Stack::default()
    }

    /// Creates a new stack with the current stack as its parent
    pub fn extend(self) -> Self {
        Stack {
            data: vec![],
            names: HashMap::new(),
            parent: Some(Rc::new(self)),
        }
    }

    /// Pushes a value onto the stack
    pub fn push(&mut self, value: Value) {
        self.data.push(value);
    }

    /// Pops a value off the stack
    pub fn pop(&mut self) -> Option<Value> {
        self.data.pop()
    }

    /// Assign a new name to the top value on the stack
    ///
    /// A single stack can have multiple names for the same value
    pub fn name(&mut self, name: String) {
        self.names.insert(name, self.data.len() - 1);
    }

    /// Assigns a new name to the top N values of the stack (from bottom to top)
    ///
    /// If the stack is [8, 6, 7, 5], name_many("A", "B") would result in [8, 6, 7@A, 5@B]
    pub fn name_many(&mut self, names: Vec<String>) {
        for (i, name) in names.iter().enumerate() {
            self.names
                .insert(name.clone(), self.data.len() - 1 - names.len() + i);
        }
    }

    /// Get a named value from this stack (including the parent) if it exists
    ///
    /// If this stack doesn't have it, check the parent
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
