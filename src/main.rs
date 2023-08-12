use clap::*;
use log;
use std::{env, io::BufReader, path::PathBuf};

mod debug;

mod numbers;
mod stack;
mod types;

mod arity;
mod compile_c;
mod lexer;
mod parser;
mod vm;

mod example_tests;

// The top-level application
#[derive(Parser, Debug)]
#[clap(name = "stacklang", version = "0.1.0", author = "JP Verkamp")]
struct App {
    #[clap(flatten)]
    globals: GlobalArgs,

    #[clap(subcommand)]
    command: Command,
}

// Global arguments that apply to all subcommands
#[derive(Args, Debug)]
struct GlobalArgs {
    #[clap(long, short = 'd')]
    debug: bool,
}

/// The specific subcommands that can be run
#[derive(Subcommand, Debug)]
enum Command {
    #[clap(name = "vm", about = "Run a StackLang program using the VM")]
    Run {
        /// Input filename
        path: PathBuf,
    },

    #[clap(name = "compile", about = "Compile a StackLang program to C")]
    Compile {
        /// Pass to compile (Clang) and automatically run
        #[clap(long, short = 'r')]
        run: bool,

        /// Output filename, defaults to {path}.c
        #[clap(long, short = 'o')]
        output: Option<PathBuf>,

        /// Input filename
        path: PathBuf,
    },
}

fn main() {
    pretty_env_logger::init();
    let args = App::parse();

    // Debug flag override envs variable
    if args.globals.debug {
        unsafe {
            debug::ENABLED = true;
        }
    } else {
        match env::var("STACKLANG_DEBUG") {
            Ok(s) if s.to_lowercase() == "true" => unsafe {
                debug::ENABLED = true;
            },
            _ => {}
        }
    }
    unsafe {
        env::set_var("RUST_LOG", "trace");
        if debug::ENABLED {
            log::debug!("Debug mode enabled");
        }
    }

    // Run specified subcommand
    match args.command {
        Command::Run { path } => {
            let file = std::fs::File::open(path).unwrap();

            let tokens = lexer::tokenize(BufReader::new(file));
            log::info!("Tokens: {:#?}", tokens);

            let ast = parser::parse(tokens);
            log::info!("AST:\n{:#?}", ast);

            vm::VM::new().evaluate(ast);
        }
        Command::Compile { run, output, path } => {
            let file = std::fs::File::open(path.clone()).unwrap();

            let tokens = lexer::tokenize(BufReader::new(file));
            log::info!("Tokens: {:#?}", tokens);

            let ast = parser::parse(tokens);
            log::info!("AST:\n{:#?}", ast);

            let c_code = compile_c::compile(ast);

            // Set output path if not specified
            let c_path = match output {
                Some(s) => s,
                None => {
                    let mut c_path = path.clone();
                    c_path.set_extension("c");
                    c_path
                }
            };
            log::info!("Writing C code to {}", c_path.to_str().unwrap());
            std::fs::write(c_path.clone(), c_code).unwrap();

            // If run flag is set, compile and run
            if run {
                let exe_path = {
                    let mut exe_path = c_path.clone();
                    exe_path.set_extension("");
                    exe_path
                };
                log::info!("Compiling C code to {}", exe_path.to_str().unwrap());

                let mut cmd = std::process::Command::new("clang");
                cmd.arg(c_path).arg("-o").arg(exe_path.clone());

                let status = cmd.status().unwrap();
                if !status.success() {
                    panic!("clang failed");
                }

                let mut cmd = std::process::Command::new(exe_path);
                let status = cmd.status().unwrap();
                if !status.success() {
                    panic!("program failed");
                }
            }
        }
    }
}
