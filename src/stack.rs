#![allow(dead_code)]

use crate::types::Value;
use std::{collections::HashMap, fmt::Display};

/// A stack in the context of the VM
///
/// This will actually have a stack of data, and a map of names to stack indices
/// These are also nested by block; when a new block is entered, a new stack is created
#[derive(Debug, Clone, Default)]
pub struct Stack {
    // The values on the stack
    data: Vec<Value>,
    // A stack of return indexes to the stack
    returns: Vec<usize>,
    // A scoped mapping of names to indices in the data
    names: Vec<HashMap<String, usize>>,
}

impl Stack {
    /// Creates a new top level stack
    pub fn new() -> Self {
        let mut s = Stack::default();
        s.extend(0);
        s
    }

    /// Creates a new scope
    ///
    /// arity is the number of values to pop from the parent stack and push onto this one
    pub fn extend(&mut self, arity: usize) {
        self.returns.push(self.data.len() - arity);
        self.names.push(HashMap::new());
    }

    /// Returns from a scope
    /// 
    /// arity is the number of values to pop from this stack and push onto the parent
    pub fn contract(&mut self, arity: usize) {
        // Drop this scope
        let return_index = self.returns.pop().unwrap();
        self.names.pop();

        let to_drop = self.data.len() - return_index - arity;

        // Copy the return values
        let mut to_push = vec![];
        for _ in 0..arity {
            to_push.push(self.data.pop().unwrap());
        }

        // Drop extra values we may have created
        for _ in 0..to_drop {
            self.data.pop();
        }

        // Push copied values back
        for v in to_push.into_iter().rev() {
            self.data.push(v);
        }
    }

    /// Pushes a value onto the stack
    pub fn push(&mut self, value: Value) {
        self.data.push(value);
    }

    /// Pops a value off the stack
    ///
    /// TODO: Handle popping a named value
    pub fn pop(&mut self) -> Option<Value> {
        self.data.pop()
    }

    /// Assign a new name to the top value on the stack
    ///
    /// A single stack can have multiple names for the same value
    pub fn name(&mut self, name: String) {
        self.names
            .last_mut()
            .unwrap()
            .insert(name, self.data.len() - 1);
    }

    /// Assigns a new name to the top N values of the stack (from bottom to top)
    ///
    /// If the stack is [8, 6, 7, 5], name_many("A", "B") would result in [8, 6, 7@A, 5@B]
    pub fn name_many(&mut self, names: Vec<String>) {
        for (i, name) in names.iter().enumerate() {
            self.names
                .last_mut()
                .unwrap()
                .insert(name.clone(), self.data.len() - names.len() + i);
        }
    }

    /// Get a named value from this stack (including the parent) if it exists
    ///
    /// If this stack doesn't have it, check the parent
    pub fn get_named(&self, name: String) -> Option<Value> {
        log::debug!("get_named({}) from {}", name, self);

        for names in self.names.iter().rev() {
            if names.contains_key(&name) {
                return Some(self.data[names[&name]].clone());
            }
        }

        None
    }

    /// Set a named value on this stack (including the parent)
    ///
    /// If this stack doesn't have it, check the parent
    /// If it's not found, panic!
    pub fn set_named(&mut self, name: String, value: Value) {
        log::debug!("set_named({}, {}) on {}", name, value, self);

        for names in self.names.iter_mut().rev() {
            if names.contains_key(&name) {
                let index = names[&name];
                self.data[index] = value;
                return;
            }
        }

        panic!("set_named({}, {}) on {} couldn't find name", name, value, self);
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(" stack<");

        for (i, value) in self.data.iter().enumerate() {
            if self.returns.contains(&i) {
                s.push_str(" | ");
            }

            s.push_str(format!("{}", value).as_str());

            for names in self.names.iter().rev() {
                for (k, v) in names.iter() {
                    if *v == i {
                        s.push_str(&format!("@{}", k));
                    }
                }
            }
        }

        s.push_str(">");
        write!(f, "{}", s)
    }
}
