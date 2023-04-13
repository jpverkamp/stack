use crate::arity::calculate_arity;
use crate::numbers::Number;
use crate::stack::Stack;
use crate::types::{Expression, Value};

/// Evaluates a vector of expressions
/// This does not actually return anything, but instead mutates the stack
#[allow(dead_code)]
pub fn evaluate(ast: Expression) {
    log::debug!("evaluate({})", ast);

    /// A helper macro to generate functions that operate on two integers and floats
    macro_rules! numeric_binop {
        ($stack:expr, $f:expr) => {{
            // TODO: Check we have enough values
            let b = $stack.pop().unwrap();
            let a = $stack.pop().unwrap();

            match (a.clone(), b.clone()) {
                (Value::Number(a), Value::Number(b)) => {
                    $stack.push(Value::Number($f(a, b)));
                }
                _ => panic!(
                    "cannot perform numeric operation on non-numeric values, got {} and {}",
                    a, b
                ),
            };
        }};
    }

    /// A helper macro to generate functions that operate on two integers and floats
    macro_rules! comparison_binop {
        ($stack:expr, $f:expr) => {{
            // TODO: Check we have enough values
            let b = $stack.pop().unwrap();
            let a = $stack.pop().unwrap();

            match (a.clone(), b.clone()) {
                (Value::Number(a), Value::Number(b)) => {
                    $stack.push(Value::Boolean($f(a, b)));
                }
                // TODO: Handle other types
                _ => panic!(
                    "cannot perform comparison operation on non-numeric values, got {} and {}",
                    a, b
                ),
            };
        }};
    }

    // Internal eval function, carries the stack with it and mutates it
    fn eval(node: Expression, stack: &mut Stack) {
        log::info!("eval({node}, {stack})");

        // Handle running a block
        fn eval_block(
            stack: &mut Stack,
            arity_in: usize,
            expression: Box<Expression>,
            arity_out: usize,
        ) {
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
                results.push(substack.pop().unwrap())
            }
            results.reverse();

            for result in results {
                stack.push(result);
            }
        }

        // Cloned for debug printing
        match node.clone() {
            // Identifiers are globals are named expressions
            // TODO: Extract globals into their own module
            Expression::Identifier(id) => {
                match id.as_str() {
                    // Built in numeric functions
                    "+" => numeric_binop!(stack, |a, b| { a + b }),
                    "-" => numeric_binop!(stack, |a, b| { a - b }),
                    "*" => numeric_binop!(stack, |a, b| { a * b }),
                    "/" => numeric_binop!(stack, |a, b| { a / b }),
                    "%" => numeric_binop!(stack, |a, b| { a % b }),
                    // Built in comparisons
                    "<" => comparison_binop!(stack, |a, b| { a < b }),
                    "<=" => comparison_binop!(stack, |a, b| { a <= b }),
                    "==" => comparison_binop!(stack, |a, b| { a == b }),
                    ">=" => comparison_binop!(stack, |a, b| { a >= b }),
                    ">" => comparison_binop!(stack, |a, b| { a > b }),
                    // Convert a value to an int if possible
                    "int" => {
                        let value = stack.pop().unwrap();
                        match value {
                            Value::String(s) => {
                                stack.push(Value::Number(Number::Integer(s.parse().unwrap())))
                            }
                            Value::Number(n) => stack.push(Value::Number(n.to_integer())),
                            _ => panic!("int cannot, got {}", value),
                        }
                    }
                    // Apply a block to the stack
                    "apply" => {
                        let block = stack.pop().unwrap();
                        match block {
                            Value::Block {
                                arity_in,
                                arity_out,
                                expression,
                            } => {
                                eval_block(stack, arity_in, expression, arity_out);
                            }
                            _ => panic!("apply expects a block, got {}", block),
                        }
                    }
                    // Read a line from stdin as a string
                    "read" => {
                        let mut input = String::new();
                        match std::io::stdin().read_line(&mut input) {
                            Ok(_) => {
                                stack.push(Value::String(input.trim_end_matches('\n').to_string()));
                            },
                            Err(e) => {
                                panic!("failed to read from stdin: {e}");
                            },
                        };
                    }
                    // Pop and write a value to stdout
                    "write" => {
                        print!("{}", stack.pop().unwrap());
                    }
                    // Pop and write a value to stdout with a newline
                    "writeln" => {
                        println!("{}", stack.pop().unwrap());
                    }
                    // Write a newline
                    "newline" => {
                        println!();
                    }
                    // Loop over an iterable, expects a block and an iterable
                    "loop" => {
                        let iterable = stack.pop().unwrap();
                        let block = stack.pop().unwrap();

                        match iterable {
                        Value::Number(Number::Integer(n)) => {
                            if n < 0 {
                                panic!("numeric loops must have a positive integer, got {}", n);
                            }

                            for i in 0..n {
                                stack.push(Value::Number(Number::Integer(i)));
                                match block.clone() {
                                    // Blocks get evaluated lazily (now)
                                    Value::Block { arity_in, arity_out, expression } => {
                                        eval_block(stack, arity_in, expression, arity_out);
                                    },
                                    // Loops must have a block
                                    _ => panic!("loop must have a block, got {}", block),
                                }
                            }
                        },
                        Value::String(s) => {
                            for c in s.chars() {
                                stack.push(Value::String(c.to_string()));
                                match block.clone() {
                                    Value::Block { arity_in, arity_out, expression } => {
                                        eval_block(stack, arity_in, expression, arity_out);
                                    },
                                    _ => panic!("loop must have a block, got {}", block),
                                }
                            }
                        },
                        _ => panic!("loop must have an iterable (currently an integer or string), got {}", iterable),
                    };
                    }
                    // If statement, expects two blocks or literals and a conditional (must be boolean)
                    "if" => {
                        let condition = stack.pop().unwrap();
                        let false_branch = stack.pop().unwrap();
                        let true_branch = stack.pop().unwrap();

                        log::debug!(
                            "if condition: {}, true: {}, false: {}",
                            condition,
                            true_branch,
                            false_branch
                        );

                        let branch = match condition {
                            Value::Boolean(value) => {
                                if value {
                                    true_branch
                                } else {
                                    false_branch
                                }
                            }
                            _ => panic!("if condition must be a boolean, got {}", condition),
                        };

                        log::debug!("if selected: {}", branch);

                        match branch {
                            // Blocks get evaluated lazily (now)
                            Value::Block {
                                arity_in,
                                arity_out,
                                expression,
                            } => {
                                eval_block(stack, arity_in, expression, arity_out);
                            }
                            // All literal values just get directly pushed
                            _ => {
                                stack.push(branch);
                            }
                        }
                    }
                    name => {
                        if let Some(value) = stack.get_named(String::from(name)) {
                            if let Value::Block {
                                arity_in,
                                arity_out,
                                expression,
                            } = value
                            {
                                eval_block(stack, arity_in, expression, arity_out);
                            } else {
                                stack.push(value);
                            }
                        } else {
                            panic!("Unknown identifier {:?}", name);
                        }
                    }
                }
            }
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
            }
            // @ expressions name the top value on the stack
            // @[] expressions name multiple values
            Expression::At(subnode) => {
                match subnode.as_ref() {
                    // Specifying input arity, ignore
                    Expression::Literal(Value::Number(Number::Integer(_))) => {}
                    // Naming the top of the stack
                    Expression::Identifier(name) => {
                        stack.name(name.clone());
                    }
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
                    }
                    _ => panic!(
                        "Invalid @ expression, must be @name or @[list], got {:?}",
                        node
                    ),
                }
            }
            // ! expressions set (or update) the value of named expressions
            Expression::Bang(subnode) => {
                match subnode.as_ref() {
                    // Output expression, ignore
                    Expression::Literal(Value::Number(Number::Integer(_))) => {}

                    // Write to a named variable
                    Expression::Identifier(_name) => todo!(),

                    // Anything else doesn't currently make sense
                    _ => panic!("Invalid ! expression, must be !# or !name, got {:?}", node),
                }
            }
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
                    }

                    // Anything else doesn't currently make sense
                    _ => panic!("Invalid $ expression, must be $name, got {:?}", node),
                }
            }
        };
    }

    // At the top level, create the stack and evaluate each expression
    let mut stack = Stack::new();
    eval(ast, &mut stack);
}
