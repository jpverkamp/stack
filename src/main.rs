use std::io::BufRead;
use std::{io, iter};

#[derive(Debug)]
struct Token {
    row: usize,
    column: usize,
    token: String,
}

#[derive(Debug)]
enum Expression {
    Singleton(String),
    Block(String, Vec<Expression>),
    At(Box<Expression>),
}

const SINGLE_CHAR_TOKENS: &str = "{}[]().";
const BLOCKS: &[(&str, &str); 3] = &[
    ("[", "]"),
    ("{", "}"),
    ("(", ")"),   
];

fn tokenize(reader: impl BufRead) -> Vec<Token> {
    let mut tokens = vec![];

    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut chars = line.chars().enumerate().peekable();

        fn is_symbolic(c: &char) -> bool {
            !c.is_alphabetic() && !c.is_whitespace()
        }

        while let Some((column, next)) = chars.next() {
            // TODO: numbers(?)

            if next.is_whitespace() {
                // skip
                continue;
            }
            
            if next == '#' {
                break;
            }
            
            if SINGLE_CHAR_TOKENS.contains(next) {
                tokens.push(Token {
                    row,
                    column,
                    token: String::from(next),
                });
                continue;
            } 
            
            if next.is_alphanumeric() {
                let mut token = String::from(next);
                loop {
                    if chars.peek().is_some() && chars.peek().unwrap().1.is_alphabetic() {
                        token.push(chars.next().unwrap().1);
                    } else {
                        break;
                    }
                }
                tokens.push(Token { row, column, token });
                continue;
            }
            
            if is_symbolic(&next) {
                let mut token = String::from(next);
                loop {
                    if chars.peek().is_some() && is_symbolic(&chars.peek().unwrap().1) {
                        token.push(chars.next().unwrap().1);
                    } else {
                        break;
                    }
                }
                tokens.push(Token { row, column, token });
                continue;
            }

            panic!("unhandled character class!");
        }
    }

    tokens
}

fn parse(tokens: Vec<Token>) -> Expression {
    fn parse_one(tokens: &[Token]) -> (Expression, &[Token]) {
        if tokens[0].token == "@" {
            let (next, tokens) = parse_one(&tokens[1..]);
            (Expression::At(Box::new(next)), tokens)
        } else if let Some((start, end)) = BLOCKS.iter().find(|(start, _)| String::from(*start) == tokens[0].token) {
            let (children, tokens) = parse_until(&tokens[1..], Some(String::from(*end)));
            (Expression::Block(String::from(*start), children), tokens)
        } else {
            // TODO: ident vs literal
            (Expression::Singleton(tokens[0].token.clone()), &tokens[1..])
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

    Expression::Block(String::from("MAIN"), parse_until(tokens.as_slice(), None).0)
}

fn main() {
    let tokens = tokenize(io::stdin().lock());

    for token in tokens.iter() {
        print!("{} ", token.token);
    }
    println!();

    let ast = parse(tokens);

    println!("{:#?}", ast);
}
