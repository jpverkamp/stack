use crate::numbers::Number;
use crate::types::{Expression, Value};

#[allow(dead_code)]
pub fn calculate_arity(expression: &Expression) -> (usize, usize) {
    log::debug!("calculate_arity({expression})");

    match expression {
        Expression::Identifier(id) => match id.as_str() {
            "+" | "-" | "*" | "/" | "%" => (2, 1),
            "<" | "<=" | ">" | ">=" | "==" | "!=" => (2, 1),
            _ => panic!(
                "unknown id to calculate arity of: {} (may need to explicitly specify it)",
                id
            ),
        },
        Expression::DottedIdentifier(ids) => {
            unimplemented!("calculate_arity for dotted identifiers: {:?}", ids)
        }
        Expression::Literal(_) => (0, 1),
        Expression::Block(children) => {
            // Set the arity based on the At and Bang nodes
            // If these are present, they must be the first 1 or 2 children and must be before any other kinds of children

            // @name means the block takes 1 input named name
            // @# means the block takes # inputs
            // @[] means the block takes the number of inputs in the list
            // !# means the block returns # outputs
            // No !# means the block returns 1 output
            // TODO: Named output?

            let mut arity_in = 0;
            let mut arity_out = 1;

            // If the first child is an @ set the arity in from that
            if let Some(Expression::At(body)) = children.first() {
                match body.as_ref() {
                    Expression::Literal(Value::Number(Number::Integer(v))) => {
                        arity_in = *v as usize
                    }
                    Expression::Identifier(_) => arity_in = 1,
                    Expression::List(values) => {
                        arity_in = values.len();
                    }
                    _ => {}
                }

                if let Some(Expression::Bang(body)) = children.get(1) {
                    match body.as_ref() {
                        Expression::Literal(Value::Number(Number::Integer(v))) => {
                            arity_out = *v as usize;
                        }
                        _ => {}
                    }
                }
            }

            // If the first child is an ! set the arity out from that
            if let Some(Expression::Bang(body)) = children.first() {
                match body.as_ref() {
                    Expression::Literal(Value::Number(Number::Integer(v))) => {
                        arity_out = *v as usize;
                    }
                    _ => {}
                }

                if let Some(Expression::At(body)) = children.get(1) {
                    match body.as_ref() {
                        Expression::Literal(Value::Number(Number::Integer(v))) => {
                            arity_in = *v as usize;
                        }
                        Expression::Identifier(_) => {
                            arity_in = 1;
                        }
                        Expression::List(values) => {
                            arity_in = values.len();
                        }
                        _ => {}
                    }
                }
            }

            log::debug!("calculate_arity({expression}) is ({arity_in}, {arity_out})");

            (arity_in, arity_out)
        }
        Expression::List(_) => panic!("Cannot calculate the arity of a list: {}", expression),
        Expression::Group(_) => panic!("Cannot calculate the arity of a group: {}", expression),
        Expression::At(body) => match body.as_ref() {
            Expression::Identifier(_) => (0, 1),
            _ => panic!(
                "Cannot calculate the arity of a non-named @ expression: {}",
                expression
            ),
        },
        Expression::Bang(body) => match body.as_ref() {
            Expression::Identifier(_) => (1, 0),
            _ => panic!(
                "Cannot calculate the arity of a non-named ! expression: {}",
                expression
            ),
        },
        Expression::Dollar(body) => match body.as_ref() {
            Expression::Identifier(_) => (0, 1),
            _ => panic!(
                "Cannot calculate the arity of a non-named $ expression: {}",
                expression
            ),
        },
    }
}
