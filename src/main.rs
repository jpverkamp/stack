use clap::*;
use log;
use std::{io::BufReader, env};

mod debug;

mod numbers;
mod stack;
mod types;

mod arity;
mod compile_c;
mod lexer;
mod parser;
mod vm;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    compile: bool,

    #[arg(short, long)]
    debug: bool,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();

    // Debug flag override envs variable
    if args.debug {
        unsafe {
            debug::ENABLED = true;
        }
    } else {
        match env::var("STACKLANG_DEBUG") {
            Ok(s) if s.to_lowercase() == "true" => unsafe {
                debug::ENABLED = true;
            },
            _ => {},
        }
    }
    unsafe {
        env::set_var("RUST_LOG", "trace");
        if debug::ENABLED {
            log::debug!("Debug mode enabled");
        }
    }

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
