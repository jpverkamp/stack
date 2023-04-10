#![allow(dead_code)]

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
    /// If followed by [], an @list is
    At(Box<Expression>),
    Bang(Box<Expression>),
}
