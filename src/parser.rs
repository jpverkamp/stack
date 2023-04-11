use crate::types::{Expression, Token, Value};

/// Parses a vector of tokens into a vector of expressions.
pub fn parse(tokens: Vec<Token>) -> Expression {
    log::debug!("parse({:?})", tokens);

    // A helper to parse a single expression from the current position in the token stream
    fn parse_one(tokens: &[Token]) -> (Expression, &[Token]) {
        if tokens[0].token == "@" {
            // @ expressions prefix the next value (naming)
            let (next, tokens) = parse_one(&tokens[1..]);
            (Expression::At(Box::new(next)), tokens)
        } else if tokens[0].token == "!" {
            // ! expressions prefix the next value (assignment)
            let (next, tokens) = parse_one(&tokens[1..]);
            (Expression::Bang(Box::new(next)), tokens)
        } else if tokens[0].token == "$" {
            // $ expressions allow pushing a block to the stack
            let (next, tokens) = parse_one(&tokens[1..]);
            (Expression::Dollar(Box::new(next)), tokens)
        } else if tokens[0].token == "{" {
            // { expressions are blocks
            let (children, tokens) = parse_until(&tokens[1..], Some(String::from("}")));
            (Expression::Block(children), tokens)
        } else if tokens[0].token == "[" {
            // [ expressions are lists
            let (children, tokens) = parse_until(&tokens[1..], Some(String::from("]")));
            (Expression::List(children), tokens)
        } else if tokens[0].token == "(" {
            // ( expressions are groups
            let (children, tokens) = parse_until(&tokens[1..], Some(String::from(")")));
            (Expression::Group(children), tokens)
        } else {
            // Try to parse each literal value, if none match assume it's an identifier
            if let Some(v) = tokens[0].token.parse::<i64>().ok() {
                (Expression::Literal(Value::Integer(v)), &tokens[1..])
            } else if let Some(v) = tokens[0].token.parse::<f64>().ok() {
                (Expression::Literal(Value::Float(v)), &tokens[1..])
            } else if tokens[0].token.starts_with("\"") {
                (
                    Expression::Literal(Value::String(tokens[0].token.clone())),
                    &tokens[1..],
                )
            } else if tokens[0].token == "true" || tokens[0].token == "false" {
                (
                    Expression::Literal(Value::Boolean(tokens[0].token == "true")),
                    &tokens[1..],
                )
            } else {
                (
                    Expression::Identifier(tokens[0].token.clone()),
                    &tokens[1..],
                )
            }
        }
    }

    // A helper to parse a list of expressions until a given ending token
    // If ending is not set, parse until end of stream
    fn parse_until(tokens: &[Token], ending: Option<String>) -> (Vec<Expression>, &[Token]) {
        let mut tokens = tokens;
        let mut expressions = vec![];

        while !tokens.is_empty() && Some(tokens[0].token.clone()) != ending {
            let (expression, next_tokens) = parse_one(tokens);
            expressions.push(expression);
            tokens = next_tokens;
        }

        if !tokens.is_empty() && Some(tokens[0].token.clone()) == ending {
            (expressions, &tokens[1..])
        } else {
            (expressions, tokens)
        }
    }

    // Parse the entire stream
    // TODO: This should be an exception if the stream is not empty after this
    Expression::Group(parse_until(tokens.as_slice(), None).0)
}
