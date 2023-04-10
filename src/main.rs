use std::io;

mod types;
mod stack;

mod lexer;
mod parser;
mod vm;


fn main() {
    let tokens = lexer::tokenize(io::stdin().lock());
    for token in tokens.iter() {
        print!("{} ", token.token);
    }
    println!();

    let ast = parser::parse(tokens);
    println!("{:#?}", ast);

    vm::evaluate(ast);
}
