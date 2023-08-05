use crate::numbers::Number;
use crate::types::{Expression, Value};

#[allow(dead_code)]
pub fn calculate_arity(expression: &Expression) -> Result<(usize, usize), String> {
    log::debug!("calculate_arity({expression})");

    match expression {
        Expression::Identifier(id) => match id.as_str() {
            "+" | "-" | "*" | "/" | "%" => Ok((2, 1)),
            "<" | "<=" | ">" | ">=" | "==" | "!=" => Ok((2, 1)),
            _ => panic!(
                "unknown id to calculate arity of: {} (may need to explicitly specify it)",
                id
            ),
        },
        Expression::DottedIdentifier(ids) => {
            unimplemented!("calculate_arity for dotted identifiers: {:?}", ids)
        }
        Expression::Literal(_) => Ok((0, 1)),
        Expression::Block(children) => {
            // Set the arity based on the At and Bang nodes
            // If these are present, they must be the first 1 or 2 children and must be before any other kinds of children

            // @name means the block takes 1 input named name
            // @# means the block takes # inputs
            // @[] means the block takes the number of inputs in the list
            // !# means the block returns # outputs
            // No !# means the block returns 1 output
            // TODO: Named output?

            let mut arity_in = None;
            let mut arity_out = None;

            // If the first child is an @ set the arity in from that
            if let Some(Expression::At(body)) = children.first() {
                match body.as_ref() {
                    Expression::Literal(Value::Number(Number::Integer(v))) => {
                        arity_in = Some(*v as usize)
                    }
                    Expression::Identifier(_) => arity_in = Some(1),
                    Expression::List(values) => {
                        arity_in = Some(values.len());
                    }
                    _ => {}
                }

                if let Some(Expression::Bang(body)) = children.get(1) {
                    match body.as_ref() {
                        Expression::Literal(Value::Number(Number::Integer(v))) => {
                            arity_out = Some(*v as usize);
                        }
                        _ => {}
                    }
                }
            }

            // If the first child is an ! set the arity out from that
            if let Some(Expression::Bang(body)) = children.first() {
                match body.as_ref() {
                    Expression::Literal(Value::Number(Number::Integer(v))) => {
                        arity_out = Some(*v as usize);
                    }
                    _ => {}
                }

                if let Some(Expression::At(body)) = children.get(1) {
                    match body.as_ref() {
                        Expression::Literal(Value::Number(Number::Integer(v))) => {
                            arity_in = Some(*v as usize);
                        }
                        Expression::Identifier(_) => {
                            arity_in = Some(1);
                        }
                        Expression::List(values) => {
                            arity_in = Some(values.len());
                        }
                        _ => {}
                    }
                }
            }

            // If we have set either of these, return them (with defaults for missing values)
            if arity_in.is_some() || arity_out.is_some() {
                log::debug!("Custom explicit arity: in={arity_in:?} out={arity_out:?}");
                return Ok((arity_in.unwrap_or(0), arity_out.unwrap_or(1)));
            }

            // Otherwise, we're attempting to determine a 'simple' arity, no custom functions
            // This is done by counting the number of inputs and outputs
            // In this case, the output arity is always 1
            // TODO: Handle the !# case better here?

            let mut depth = 0;
            let mut min_depth = 0;

            for child in children {
                if let Ok((child_in, child_out)) = calculate_arity(child) {
                    depth -= child_in as isize;
                    min_depth = min_depth.min(depth);
                    depth += child_out as isize;
                } else {
                    return Err(format!("Cannot derive the arity of {}", expression.clone()));
                }
            }

            log::debug!("Custom implicity arity: in={min_depth} out=1");
            Ok(((0 - min_depth) as usize, 1))
        }
        Expression::List(_) => Err(format!(
            "Cannot calculate the arity of a list: {}",
            expression.clone()
        )),
        Expression::Group(_) => Err(format!(
            "Cannot calculate the arity of a group: {}",
            expression.clone()
        )),
        Expression::At(body) => match body.as_ref() {
            Expression::Identifier(_) => Ok((0, 1)),
            _ => Err(format!(
                "Cannot calculate the arity of a non-named @ expression: {}",
                expression
            )
            .clone()),
        },
        Expression::Bang(body) => match body.as_ref() {
            Expression::Identifier(_) => Ok((1, 0)),
            _ => Err(format!(
                "Cannot calculate the arity of a non-named ! expression: {}",
                expression.clone()
            )),
        },
        Expression::Dollar(body) => match body.as_ref() {
            Expression::Identifier(_) => Ok((0, 1)),
            _ => Err(format!(
                "Cannot calculate the arity of a non-named $ expression: {}",
                expression.clone()
            )),
        },
    }
}
