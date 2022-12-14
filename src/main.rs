use crate::ast::Ast;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::io::Write;
use std::{env, fs, io};

//* modules must be declared here so they can use each other
mod ast;
mod expr;
mod parser;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            run_prompt();
        }
        2 => {
            let path = &args[1];
            run_file(path);
        }
        _ => panic!("Please either provide a file path or no arguments"),
    };
}

fn run(source: String) {
    let mut scanner = Scanner::from(source.clone());
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens, source.clone());
    let expression = parser.parse();

    let ast = Ast::new(expression);
    ast.print();
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("No such file.");
    run(source);
}

fn run_prompt() {
    println!("Start typing your commands...");

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut source = String::new();

        io::stdin()
            .read_line(&mut source)
            .expect("Could not read input.");

        run(source);
    }
}
