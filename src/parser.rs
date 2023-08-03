use crate::numbers::Number;
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
                (
                    Expression::Literal(Value::Number(Number::Integer(v))),
                    &tokens[1..],
                )
            } else if let Some(v) = tokens[0].token.parse::<f64>().ok() {
                (
                    Expression::Literal(Value::Number(Number::Float(v))),
                    &tokens[1..],
                )
            } else if tokens[0].token.starts_with("\"") {
                (
                    Expression::Literal(Value::String(
                        tokens[0].token.trim_matches('"').to_string(),
                    )),
                    &tokens[1..],
                )
            } else if tokens[0].token == "true" || tokens[0].token == "false" {
                (
                    Expression::Literal(Value::Boolean(tokens[0].token == "true")),
                    &tokens[1..],
                )
            } else if tokens[0].token.contains(".") {
                (
                    Expression::DottedIdentifier(
                        tokens[0]
                            .token
                            .split(".")
                            .map(|s| s.to_string())
                            .collect(),
                    ),
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

#[cfg(test)]
mod test {
    use crate::lexer::tokenize;
    use crate::numbers::Number;
    use crate::parser::parse;
    use crate::types::{Expression, Value};

    #[test]
    fn test_integer() {
        let input = tokenize("123".as_bytes());
        let output = parse(input);
        assert_eq!(
            output,
            Expression::Group(vec![Expression::Literal(Value::Number(Number::Integer(
                123
            )))])
        );
    }

    #[test]
    fn test_float() {
        let input = tokenize("123.456".as_bytes());
        let output = parse(input);
        assert_eq!(
            output,
            Expression::Group(vec![Expression::Literal(Value::Number(Number::Float(
                123.456
            )))])
        );
    }

    #[test]
    fn test_string_literal() {
        let input = tokenize("\"hello world\"".as_bytes());
        let output = parse(input);
        assert_eq!(
            output,
            Expression::Group(vec![Expression::Literal(Value::String(String::from(
                "hello world"
            )))])
        );
    }

    #[test]
    fn test_boolean_literal() {
        let input = tokenize("true".as_bytes());
        let output = parse(input);
        assert_eq!(
            output,
            Expression::Group(vec![Expression::Literal(Value::Boolean(true))])
        );
    }

    #[test]
    fn test_simple_addition() {
        let input = tokenize("1 2 +".as_bytes());
        let output = parse(input);
        assert_eq!(
            output,
            Expression::Group(vec![
                Expression::Literal(Value::Number(Number::Integer(1))),
                Expression::Literal(Value::Number(Number::Integer(2))),
                Expression::Identifier(String::from("+")),
            ])
        );
    }

    #[test]
    fn test_identifier() {
        let input = tokenize("a".as_bytes());
        let output = parse(input);
        assert_eq!(
            output,
            Expression::Group(vec![Expression::Identifier(String::from("a"))])
        );
    }

    #[test]
    fn test_symbolic_identifier() {
        let input = tokenize("<=".as_bytes());
        let output = parse(input);
        assert_eq!(
            output,
            Expression::Group(vec![Expression::Identifier(String::from("<="))])
        );
    }

    #[test]
    fn test_dotted_identifier() {
        let input = tokenize("a.b.c".as_bytes());
        let output = parse(input);
        assert_eq!(
            output,
            Expression::Group(vec![Expression::DottedIdentifier(vec![
                String::from("a"),
                String::from("b"),
                String::from("c"),
            ])])
        );
    }

    #[test]
    fn test_naming() {
        let input = tokenize("@a".as_bytes());
        let output = parse(input);
        assert_eq!(
            output,
            Expression::Group(vec![Expression::At(Box::new(Expression::Identifier(
                String::from("a")
            )))])
        );
    }

    #[test]
    fn test_list_naming() {
        let input = tokenize("@[a b c]".as_bytes());
        let output = parse(input);
        assert_eq!(
            output,
            Expression::Group(vec![Expression::At(Box::new(Expression::List(vec![
                Expression::Identifier(String::from("a")),
                Expression::Identifier(String::from("b")),
                Expression::Identifier(String::from("c")),
            ])))])
        );
    }

    #[test]
    fn test_simple_block() {
        let input = tokenize("{ 1 2 + }".as_bytes());
        let output = parse(input);
        assert_eq!(
            output,
            Expression::Group(vec![Expression::Block(vec![
                Expression::Literal(Value::Number(Number::Integer(1))),
                Expression::Literal(Value::Number(Number::Integer(2))),
                Expression::Identifier(String::from("+")),
            ])])
        );
    }

    #[test]
    fn test_assignment_bang() {
        let input = tokenize("1 !a a a +".as_bytes());
        let output = parse(input);
        assert_eq!(
            output,
            Expression::Group(vec![
                Expression::Literal(Value::Number(Number::Integer(1))),
                Expression::Bang(Box::new(Expression::Identifier(String::from("a")))),
                Expression::Identifier(String::from("a")),
                Expression::Identifier(String::from("a")),
                Expression::Identifier(String::from("+")),
            ])
        );
    }

    #[test]
    fn test_factorial() {
        let input = tokenize(
            "
{
  @[n fact]
  1
  { @0 n 1 - $fact fact n * }
  n 1 < if
} @fact

5 $fact fact writeln"
                .as_bytes(),
        );
        let output = parse(input);
        assert_eq!(
            output,
            Expression::Group(vec![
                Expression::Block(vec![
                    Expression::At(Box::new(Expression::List(vec![
                        Expression::Identifier(String::from("n")),
                        Expression::Identifier(String::from("fact")),
                    ]))),
                    Expression::Literal(Value::Number(Number::Integer(1))),
                    Expression::Block(vec![
                        Expression::At(Box::new(Expression::Literal(Value::Number(
                            Number::Integer(0)
                        )),)),
                        Expression::Identifier(String::from("n")),
                        Expression::Literal(Value::Number(Number::Integer(1))),
                        Expression::Identifier(String::from("-")),
                        Expression::Dollar(Box::new(Expression::Identifier(String::from("fact")))),
                        Expression::Identifier(String::from("fact")),
                        Expression::Identifier(String::from("n")),
                        Expression::Identifier(String::from("*")),
                    ]),
                    Expression::Identifier(String::from("n")),
                    Expression::Literal(Value::Number(Number::Integer(1))),
                    Expression::Identifier(String::from("<")),
                    Expression::Identifier(String::from("if")),
                ]),
                Expression::At(Box::new(Expression::Identifier(String::from("fact")))),
                Expression::Literal(Value::Number(Number::Integer(5))),
                Expression::Dollar(Box::new(Expression::Identifier(String::from("fact")))),
                Expression::Identifier(String::from("fact")),
                Expression::Identifier(String::from("writeln")),
            ])
        );
    }
}
