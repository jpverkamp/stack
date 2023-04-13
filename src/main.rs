use clap::*;
use log;
use std::io::BufReader;

mod numbers;
mod stack;
mod types;

mod arity;
mod lexer;
mod parser;
mod vm;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: String,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();

    let file = std::fs::File::open(args.file).unwrap();
    let tokens = lexer::tokenize(BufReader::new(file));
    
    log::info!(
        "Tokens: {}",
        tokens
            .iter()
            .map(|token| token.token.clone())
            .collect::<Vec<String>>()
            .join(" ")
    );

    let ast = parser::parse(tokens);
    log::info!("AST:\n{:#?}", ast);

    vm::evaluate(ast);
}
