mod token;
mod token_kinds;
mod tokenizer;

mod literal_types;

mod expressions;

mod ast_printer;

mod parser;

use std::io::Write;
use std::{fs, io};
use tokenizer::{Tokenizer};
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
        _ => println!("Usage: pulsar <filename>"),
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

    if !errors.is_empty() {
        for error in errors {
            println!("{}", error.message);
        }

        // std::process::exit(65);
        return;
    }

    for token in tokens {
        println!("{}", token.to_string());
    }

    // let mut parser = Parser::new(tokens);
}
