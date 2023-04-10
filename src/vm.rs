use crate::types::{Value, Expression};
use crate::stack::{Stack};

macro_rules! binop {
    ($stack:expr, $f:expr) => {
        {
            // TODO: Check we have enough values
            let b = $stack.pop().unwrap();
            let a = $stack.pop().unwrap();

            $stack.push(match (a, b) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer($f(a, b)),
                (Value::Float(a), Value::Float(b)) => Value::Float($f(a, b)),
                (Value::Integer(a), Value::Float(b)) => Value::Float($f(a as f64, b)),
                (Value::Float(a), Value::Integer(b)) => Value::Float($f(a, b as f64)),
                _ => unimplemented!()
            });
        }
    }
}

pub fn evaluate(ast: Vec<Expression>) {
    fn eval(node: Expression, stack: &mut Stack) {
        eprintln!("eval({node:?}, {stack:?})");

        match node.clone() {


            Expression::Identifier(id) => match id.as_str() {
                "+" => binop!(stack, |a, b| { a + b }),
                "-" => binop!(stack, |a, b| { a - b }),
                "*" => binop!(stack, |a, b| { a * b }),
                "/" => binop!(stack, |a, b| { a / b }),
                "%" => binop!(stack, |a, b| { a % b }),
                "writeln" => {
                    println!("{:?}", stack.pop().unwrap());
                }
                _ => todo!(),

            },
            Expression::Literal(value) => stack.push(value.clone()),
            Expression::Block(_) => todo!(),
            Expression::List(_) => todo!(),
            Expression::Group(_) => todo!(),
            Expression::At(_) => todo!(),
            Expression::Bang(_) => todo!(),
        };
    }

    let mut stack = Stack::new();

    for node in ast {
        eval(node, &mut stack);
    }
}