#![allow(dead_code)]

use crate::numbers::Number;
use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

/// A span is a location in the source code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    pub name: Option<String>,
    pub row: usize,
    pub column: usize,
    pub length: usize,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}",
            self.name.as_ref().unwrap_or(&"<unknown>".to_string()),
            self.row,
            self.column,
            self.length,
        )
    }
}

/// A token is a single unit of a program.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub span: Span,
    pub token: String,
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Number::Integer(v) => v.to_string(),
                Number::Rational {
                    numerator,
                    denominator,
                } => format!("{}/{}", numerator, denominator),
                Number::Float(v) => v.to_string(),
                Number::Complex { real, imaginary } =>
                    if *imaginary < 0.0 {
                        format!("{}{}i", real, imaginary)
                    } else {
                        format!("{}+{}i", real, imaginary)
                    },
            }
        )
    }
}

/// A value is a literal value that has been evaluated.
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Value {
    Number(Number),
    String(String),
    Boolean(bool),
    Block {
        arity_in: usize,
        arity_out: usize,
        expression: Box<Expression>,
    },
    List(Rc<RefCell<Vec<Value>>>),
    Hash(Rc<RefCell<HashMap<String, Value>>>),
    IntHash(Rc<RefCell<HashMap<i64, Value>>>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Number(v) => v.to_string(),
                Value::String(v) => v.to_string().trim_matches('"').to_string(),
                Value::Boolean(v) => v.to_string(),
                Value::Block {
                    arity_in,
                    arity_out,
                    ..
                } => format!("{{{}->{}}}", arity_in, arity_out),
                Value::List(v) => {
                    format!(
                        "[{}]",
                        v.clone()
                            .borrow()
                            .iter()
                            .map(|v| format!("{}", v))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
                Value::Hash(v) => {
                    format!(
                        "Hash<{}>",
                        v.clone()
                            .borrow()
                            .iter()
                            .map(|(k, v)| format!("{}:{}", k, v))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
                Value::IntHash(v) => {
                    format!(
                        "IntHash<{}>",
                        v.clone()
                            .borrow()
                            .iter()
                            .map(|(k, v)| format!("{}:{}", k, v))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
            }
        )
    }
}

/// An expression is a single unit of a program, part of the AST
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    /// An identifier/variable, used to lookup a named value or global
    Identifier(String),
    /// A dotted expression, used to lookup fields in structs
    DottedIdentifier(Vec<String>),
    /// A literal value
    Literal(Value),

    /// A function definition, generally delimited with {}
    Block(Vec<Expression>),
    /// A list of values, generally delimited with []
    List(Vec<Expression>),
    /// A group of values, generally delimited with (), currently used only for clean code
    Group(Vec<Expression>),

    /// An @ prefixed expression, used to name values on the stack
    /// If followed by [], an @list is multiple names
    At(Box<Expression>),
    /// A ! prefixed expression, used to set values by name
    Bang(Box<Expression>),
    /// A $ prefixed expression, used to pass to the stack (only really needed for blocks)
    Dollar(Box<Expression>),
}

macro_rules! write_children {
    ($f:ident $prefix:literal $children:ident $suffix:literal) => {{
        let mut s = String::new();
        s.push($prefix);
        for (i, child) in $children.iter().enumerate() {
            s.push_str(&format!("{}", child));
            if i != $children.len() - 1 {
                s.push(' ');
            }
        }
        s.push($suffix);
        write!($f, "{}", s)
    }};
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(id) => write!(f, "{}", id),
            Expression::DottedIdentifier(ids) => write!(f, "{}", ids.join(".")),
            Expression::Literal(value) => write!(f, "{}", value),
            Expression::Block(children) => write_children! {f '{' children '}'},
            Expression::List(children) => write_children! {f '[' children ']'},
            Expression::Group(children) => write_children! {f '(' children ')'},
            Expression::At(expr) => write!(f, "@{}", expr),
            Expression::Bang(expr) => write!(f, "!{}", expr),
            Expression::Dollar(expr) => write!(f, "${}", expr),
        }
    }
}
