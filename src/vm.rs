use crate::arity::calculate_arity;
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

/// A helper macro to generate functions that operate on two integers and floats
macro_rules! comparison_binop {
    ($stack:expr, $f:expr) => {{
        // TODO: Check we have enough values
        let b = $stack.pop().unwrap();
        let a = $stack.pop().unwrap();

        let result = match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => Value::Boolean($f(a, b)),
            (Value::Float(a), Value::Float(b)) => Value::Boolean($f(a, b)),
            (Value::Integer(a), Value::Float(b)) => Value::Boolean($f(a as f64, b)),
            (Value::Float(a), Value::Integer(b)) => Value::Boolean($f(a, b as f64)),
            _ => unimplemented!(),
        };

        $stack.push(result);
    }};
}

/// Evaluates a vector of expressions
/// This does not actually return anything, but instead mutates the stack
pub fn evaluate(ast: Expression) {
    // Internal eval function, carries the stack with it and mutates it
    fn eval(node: Expression, stack: &mut Stack) {
        log::info!("eval({node}, {stack})");

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
                "<" => comparison_binop!(stack, |a, b| { a < b }),
                "<=" => comparison_binop!(stack, |a, b| { a <= b }),
                "==" => comparison_binop!(stack, |a, b| { a == b }),
                ">=" => comparison_binop!(stack, |a, b| { a >= b }),
                ">" => comparison_binop!(stack, |a, b| { a > b }),
                "writeln" => {
                    println!("{}", stack.pop().unwrap());
                },
                "if" => {
                    let condition = stack.pop().unwrap();
                    let false_branch = stack.pop().unwrap();
                    let true_branch = stack.pop().unwrap();

                    log::debug!("if condition: {}, true: {}, false: {}", condition, true_branch, false_branch);

                    let branch = match condition {
                        Value::Boolean(value) => if value { true_branch } else { false_branch },
                        _ => panic!("if condition must be a boolean, got {}", condition),
                    };

                    log::debug!("if selected: {}", branch);
                    
                    match branch {
                        // Blocks get evaluated lazily (now)
                        Value::Block { arity_in, arity_out, expression } => {
                            // TODO: Refactor this

                            // Extend the stack with arity_in values
                            let mut substack = stack.extend(arity_in);                           
                            log::debug!("substack: {}", substack);

                            // Evaluate the block with that new stack context
                            eval(expression.as_ref().clone(), &mut substack);
                            log::debug!("substack after eval: {}", substack);
                            
                            // Copy arity_out values to return, drop the rest of the substack
                            // TODO: should this be a stack method?
                            let mut results = vec![];
                            for _ in 0..arity_out {
                                results.push(substack.pop())
                            }

                            for result in results {
                                stack.push(result.unwrap());
                            }
                        },
                        // All literal values just get directly pushed
                        _ => {
                            stack.push(branch);
                        }
                    }
                }
                name => {
                    if let Some(value) = stack.get_named(String::from(name)) {
                        if let Value::Block { arity_in, arity_out, expression } = value {
                            // Extend the stack with arity_in values
                            let mut substack = stack.extend(arity_in);                           
                            log::debug!("substack: {}", substack);

                            // Evaluate the block with that new stack context
                            eval(expression.as_ref().clone(), &mut substack);
                            log::debug!("substack after eval: {}", substack);
                            
                            // Copy arity_out values to return, drop the rest of the substack
                            // TODO: should this be a stack method?
                            let mut results = vec![];
                            for _ in 0..arity_out {
                                results.push(substack.pop())
                            }

                            for result in results {
                                stack.push(result.unwrap());
                            }
                        } else {
                            stack.push(value);
                        }
                    } else {
                        panic!("Unknown identifier {:?}", name);
                    }
                },
            },
            // Literal values are just pushed onto the stack
            Expression::Literal(value) => stack.push(value.clone()),
            // Blocks are parsed into block values, arity is calculated here
            Expression::Block(children) => {
                let (arity_in, arity_out) = calculate_arity(&node.clone()).unwrap();

                // TODO: Actually calculate arity
                stack.push(Value::Block {
                    arity_in,
                    arity_out,
                    expression: Box::new(Expression::Group(children)),
                });
            }
            // TODO: Lists shouldn't currently be directly called
            // TODO: This could be list expressions
            Expression::List(_) => todo!(),
            // TODO: Groups evaluate their children one at a time
            Expression::Group(children) => {
                for node in children {
                    eval(node, stack);
                }
            },
            // @ expressions name the top value on the stack
            // @[] expressions name multiple values
            Expression::At(subnode) => {
                match subnode.as_ref() {
                    // Specifying input arity, ignore
                    Expression::Literal(Value::Integer(_)) => {},
                    // Naming the top of the stack
                    Expression::Identifier(name) => {
                        stack.name(name.clone());
                    },
                    // Naming several values at once on top of the stack
                    Expression::List(exprs) => {
                        let mut names = vec![];
                        for expr in exprs {
                            match expr {
                                Expression::Identifier(name) => names.push(name.clone()),
                                _ => panic!("Invalid @ expression, @[list] must contain only names, got {:?}", node)
                            }
                        }
                        stack.name_many(names.clone())
                    },
                    _ => panic!("Invalid @ expression, must be @name or @[list], got {:?}", node)
                }
            },
            // ! expressions set (or update) the value of named expressions
            Expression::Bang(subnode) => {
                match subnode.as_ref() {
                    // Output expression, ignore
                    Expression::Literal(Value::Integer(_)) => {},

                    // Write to a named variable
                    Expression::Identifier(_name) => todo!(),
                    
                    // Anything else doesn't currently make sense
                    _ => panic!("Invalid ! expression, must be !# or !name, got {:?}", node)
                }
            },
            // $ expressions are used to access named expressions without evaluating
            Expression::Dollar(subnode) => {
                match subnode.as_ref() {
                    // Push to stack (don't evaluate)
                    Expression::Identifier(name) => {
                        if let Some(value) = stack.get_named(name.clone()) {
                            stack.push(value.clone());
                        } else {
                            panic!("Unknown identifier {:?}", name);
                        }
                    },
                    
                    // Anything else doesn't currently make sense
                    _ => panic!("Invalid $ expression, must be $name, got {:?}", node)
                }
            },
        };
    }

    // At the top level, create the stack and evaluate each expression
    let mut stack = Stack::new();
    eval(ast, &mut stack);
}
