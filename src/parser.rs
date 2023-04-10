use crate::types::{Token, Value, Expression};

pub fn parse(tokens: Vec<Token>) -> Vec<Expression> {
    fn parse_one(tokens: &[Token]) -> (Expression, &[Token]) {
        if tokens[0].token == "@" {
            let (next, tokens) = parse_one(&tokens[1..]);
            (Expression::At(Box::new(next)), tokens)
        } else if tokens[0].token == "!" {
            let (next, tokens) = parse_one(&tokens[1..]);
            (Expression::Bang(Box::new(next)), tokens)
        } else if tokens[0].token == "{" {
            let (children, tokens) = parse_until(&tokens[1..], Some(String::from("}")));
            (Expression::Block(children), tokens)
        } else if tokens[0].token == "[" {
            let (children, tokens) = parse_until(&tokens[1..], Some(String::from("]")));
            (Expression::List(children), tokens)
        } else if tokens[0].token == "(" {
            let (children, tokens) = parse_until(&tokens[1..], Some(String::from(")")));
            (Expression::Group(children), tokens)
        } else {
            // Try to parse each literal value, if none match assume it's an identifier
            if let Some(v) = tokens[0].token.parse::<i64>().ok() {
                (Expression::Literal(Value::Integer(v)), &tokens[1..])
            } else if let Some(v) = tokens[0].token.parse::<f64>().ok() {
                (Expression::Literal(Value::Float(v)), &tokens[1..])
            } else if tokens[0].token.starts_with("\"") {
                (Expression::Literal(Value::String(tokens[0].token.clone())), &tokens[1..])
            } else if tokens[0].token == "true" || tokens[0].token == "false" {
                (Expression::Literal(Value::Boolean(tokens[0].token == "true")), &tokens[1..])
            } else {
                (Expression::Identifier(tokens[0].token.clone()), &tokens[1..])
            }
        }
    }

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

    parse_until(tokens.as_slice(), None).0
}