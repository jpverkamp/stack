use std::{collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
pub struct Token {
    pub row: usize,
    pub column: usize,
    pub token: String,
}

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Block {
        arity_in: usize,
        arity_out: usize,
        expression: Box<Expression>,
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Identifier(String),
    Literal(Value),

    Block(Vec<Expression>),
    List(Vec<Expression>),
    Group(Vec<Expression>),

    At(Box<Expression>),
    Bang(Box<Expression>),
}
