use log;
use std::io;

mod stack;
mod types;

mod lexer;
mod parser;
mod vm;
mod arity;
mod compile_c;

fn main() {
    pretty_env_logger::init();

    let tokens = lexer::tokenize(io::stdin().lock());
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

    // vm::evaluate(ast);

    let c = compile_c::compile(ast);
    println!("{}", c);
}
