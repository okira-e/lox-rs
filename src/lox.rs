// use std::{fs, io};
// use std::io::Write;
// use crate::lexing::lexer::Lexer;
// use crate::lexing::token::Token;
//
// pub struct Lox {
//     had_errors: bool,
// }
//
// impl Lox {
//     pub fn new() -> Lox {
//         return Lox {
//             had_errors: false,
//         };
//     }
//
//     pub fn run_file(&self, file_name: &String) {
//         let content = fs::read_to_string(file_name).unwrap_or_else(|err| {
//             println!("Error reading source file: {}", err);
//             std::process::exit(1);
//         });
//
//         self.run(content);
//
//         if self.had_errors {
//             std::process::exit(65);
//         }
//     }
//
//     pub fn run_prompt(&mut self) {
//         loop {
//             print!("Lox> ");
//             io::stdout().flush().unwrap();
//
//             let mut input = String::new();
//             io::stdin().read_line(&mut input).unwrap();
//
//             self.run(input);
//             self.had_errors = false;
//         }
//     }
//
//     fn run(&self, input: String) {
//         let mut lexer = Lexer::new(input);
//         let tokens = lexer.scan_tokens();
//
//         for token in tokens {
//             println!("{:?}", token);
//         }
//
//         // if self.had_errors {
//         //     std::process::exit(65);
//         // }
//     }
//
//     fn error(token: Token, message: &str) {
//         report(token.line, "", message);
//     }
//
//     fn report(&mut self, line: usize, location: &str, msg: &str) {
//         println!("[line {}] Error {}: {}", line, location, msg);
//         self.had_errors = true;
//     }
// }
