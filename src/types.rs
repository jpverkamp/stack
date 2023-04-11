#![allow(dead_code)]

use std::fmt::Display;

/// A token is a single unit of a program.
#[derive(Clone, Debug)]
pub struct Token {
    /// The row of the token in the source code.
    pub row: usize,
    /// The column of the token in the source code.
    pub column: usize,
    /// The token itself.
    pub token: String,
}

/// A value is a literal value that has been evaluated.
#[derive(Clone, Debug)]
pub enum Value {
    /// An empty value
    Null,
    /// An integer value
    Integer(i64),
    /// A floating point value
    Float(f64),
    /// A literal string value
    String(String),
    /// A boolean value
    Boolean(bool),
    /// An executable block with stored arity
    Block {
        /// The number of parameters popped from the stack when this block is called
        arity_in: usize,
        /// The number of values pushed to the stack when this block is done
        arity_out: usize,
        /// The block itself
        expression: Box<Expression>,
    },
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Value::Null => "null".to_string(),
            Value::Integer(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::String(v) => v.to_string().trim_matches('"').to_string(),
            Value::Boolean(v) => v.to_string(),
            Value::Block { arity_in, arity_out, .. } => format!("{{{}->{}}}", arity_in, arity_out),
        })
    }
}

/// An expression is a single unit of a program, part of the AST
#[derive(Clone, Debug)]
pub enum Expression {
    /// An identifier/variable, used to lookup a named value or global
    Identifier(String),
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
    ($f:ident $prefix:literal $children:ident $suffix:literal) => {
        {
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
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(id) => write!(f, "{}", id),
            Expression::Literal(value) => write!(f, "{}", value),
            Expression::Block(children) => write_children!{f '{' children '}'},
            Expression::List(children) => write_children!{f '[' children ']'},
            Expression::Group(children) => write_children!{f '(' children ')'},
            Expression::At(expr) => write!(f, "@{}", expr),
            Expression::Bang(expr) => write!(f, "!{}", expr),
            Expression::Dollar(expr) => write!(f, "${}", expr),
        }
    }
}