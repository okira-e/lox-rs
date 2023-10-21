mod token;
mod token_kinds;
mod tokenizer;
mod literal;
mod expressions;
mod ast_printer;
mod parser;
mod language_error;
mod interpreter;

use std::io::Write;
use std::{fs, io};
use tokenizer::{Tokenizer};
use crate::ast_printer::{print_ast};
use crate::language_error::Error;
use crate::parser::Parser;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    match args.len() {
        2 => {
            run_file(args.get(1).unwrap_or_else(|| {
                println!("Error reading source file");
                std::process::exit(1);
            }));
        }
        1 => {
            run_prompt();
        }
        _ => println!("Usage: lox <filename>"),
    }
}

pub fn run_file(file_name: &String) {
    let content = fs::read_to_string(file_name).unwrap_or_else(|err| {
        println!("Error reading source file: {}", err);
        std::process::exit(1);
    });

    run(content.as_str());
}

pub fn run_prompt() {
    loop {
        print!("Lox> ");
        io::stdout().flush().unwrap_or_else(|err| {
            println!("Error flushing stdout: {}", err);
            std::process::exit(1);
        });

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or_else(|err| {
            println!("Error reading stdin: {}", err);
            std::process::exit(1);
        });

        run(input.as_str());
    }
}

fn run(input: &str) {
    let mut tokenizer = Tokenizer::new(input);
    let (tokens, errors) = tokenizer.scan_tokens();

    if errors.len() > 0 {
        for err in errors {
            report_error(err);
        }

        std::process::exit(1);
    }

    let mut parser = Parser::new(tokens);

    let ast = parser.parse();

    println!("\nAST: {}\n", print_ast(&ast));

    interpreter::interpret(&ast);
}

/// Report a compiler error.
pub fn report_error(err: &Error) {
    if let Some(line) = err.line {
        println!("Found an error at line {}. {}", line, err.msg);
    } else {
        println!("{}", err.msg);
    }

}