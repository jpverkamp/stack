use crate::stack::Stack;
use crate::types::{Expression, Value};

/// A helper macro to generate functions that operate on two integers and floats
macro_rules! numeric_binop {
    ($stack:expr, $f:expr) => {{
        // TODO: Check we have enough values
        let b = $stack.pop().unwrap();
        let a = $stack.pop().unwrap();

        let result = match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer($f(a, b)),
            (Value::Float(a), Value::Float(b)) => Value::Float($f(a, b)),
            (Value::Integer(a), Value::Float(b)) => Value::Float($f(a as f64, b)),
            (Value::Float(a), Value::Integer(b)) => Value::Float($f(a, b as f64)),
            _ => unimplemented!(),
        };

        $stack.push(result);
    }};
}

/// Evaluates a vector of expressions
/// This does not actually return anything, but instead mutates the stack
pub fn evaluate(ast: Vec<Expression>) {
    // Internal eval function, carries the stack with it and mutates it
    fn eval(node: Expression, stack: &mut Stack) {
        log::debug!("eval({node:?}, {stack:?})");

        // Cloned for debug printing
        match node.clone() {
            // Identifiers are globals are named expressions
            // TODO: Extract globals into their own module
            Expression::Identifier(id) => match id.as_str() {
                "+" => numeric_binop!(stack, |a, b| { a + b }),
                "-" => numeric_binop!(stack, |a, b| { a - b }),
                "*" => numeric_binop!(stack, |a, b| { a * b }),
                "/" => numeric_binop!(stack, |a, b| { a / b }),
                "%" => numeric_binop!(stack, |a, b| { a % b }),
                "writeln" => {
                    println!("{:?}", stack.pop().unwrap());
                }
                _ => todo!(), // TODO: named expressions
            },
            // Literal values are just pushed onto the stack
            Expression::Literal(value) => stack.push(value.clone()),
            // TODO: Blocks are parsed into block values, arity is calculated here
            Expression::Block(_) => {}
            // TODO: Lists shouldn't currently be directly called
            // TODO: This could be list expressions
            Expression::List(_) => todo!(),
            // TODO: Groups evaluate their children one at a time
            Expression::Group(_) => todo!(),
            // @ expressions name the top value on the stack
            // @[] expressions name multiple values
            Expression::At(_) => todo!(),
            // ! expressions set (or update) the value of named expressions
            Expression::Bang(_) => todo!(),
        };
    }

    // At the top level, create the stack and evaluate each expression
    let mut stack = Stack::new();
    for node in ast {
        eval(node, &mut stack);
    }
}
