use clap::*;
use log;
use std::io::BufReader;

mod numbers;
mod stack;
mod types;

mod arity;
mod compile_c;
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

    // If we should compile or interpret
    #[arg(short, long)]
    compile: bool,
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

    if args.compile {
        let c_code = compile_c::compile(ast);
        println!("{}", c_code);
    } else {
        vm::evaluate(ast);
    }
}
