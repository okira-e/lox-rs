mod lexing;
mod lox;

use std::io::Write;
use std::{fs, io};
use crate::lexing::lexer::{Lexer};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    match args.len() {
        2 => {
            run_file(args.get(1).unwrap());
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
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        run(input.as_str());
    }
}

fn run<'a>(input: &'a str) {
    let mut lexer = Lexer::new(input);
    let (tokens, errors) = lexer.scan_tokens();

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
}
