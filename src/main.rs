use std::io::Write;
use std::{env, fmt, fs, io};

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
    let mut scanner = Scanner::from(source);
    let tokens = scanner.scan_tokens();
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

struct Scanner {
    source: String,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    fn from(source: String) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }
    fn scan_tokens(&mut self) -> Vec<Token> {
        let tokens: Vec<Token> = Vec::new();

        let mut iter = self.source.chars().peekable();

        while let Some(chr) = iter.next() {
            println!("{} at position {}", chr, self.current);
            self.current += 1;

            // match chr {
            //     '(' => tokens.push(Token {
            //         typ: TokenType::LEFT_PAREN,
            //         line: self.line,
            //         lexeme: "lexeme",
            //         literal: "literal",
            //     }),
            //     _ => {}
            // };

            match iter.peek() {
                Some(x) => {
                    println!(" Next is {:?}", *x)
                }
                _ => {
                    break;
                }
            };
        }

        println!("Tokens: {:?}", tokens);

        tokens
    }
}

#[derive(Debug)]
struct Token<'a> {
    typ: TokenType,
    lexeme: &'a str,
    literal: &'a str,
    line: u32,
}

impl<'a> Token<'a> {
    // This is an associated function
    fn new(typ: TokenType, lexeme: &'a str, literal: &'a str, line: u32) -> Token<'a> {
        Token {
            typ,
            lexeme,
            literal,
            line,
        }
    }
    // This is a method
    fn to_string(&self) -> String {
        format!("{} {} {}", self.typ, self.lexeme, self.literal)
    }
}

// TODO: Implement a function to return a value for each token type?
// That way we can implement the Add trait on TokenType

fn get_token_type(identifier: &str) -> TokenType {
    match identifier {
        "and" => TokenType::AND,
        "class" => TokenType::CLASS,
        "else" => TokenType::ELSE,
        "false" => TokenType::FALSE,
        "for" => TokenType::FOR,
        "if" => TokenType::IF,
        "nil" => TokenType::NIL,
        "or" => TokenType::OR,
        "print" => TokenType::PRINT,
        "return" => TokenType::RETURN,
        "super" => TokenType::SUPER,
        "this" => TokenType::THIS,
        "true" => TokenType::TRUE,
        "var" => TokenType::VAR,
        "while" => TokenType::WHILE,
        _ => TokenType::IDENTIFIER,
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum TokenType {
    AND,
    BANG,
    BANG_EQUAL,
    CLASS,
    COMMA,
    DOT,
    ELSE,
    EQUAL,
    EQUAL_EQUAL,
    FALSE,
    FOR,
    FUN,
    GREATER,
    GREATER_EQUAL,
    IDENTIFIER,
    IF,
    LEFT_BRACE,
    LEFT_PAREN,
    LESS,
    LESS_EQUAL,
    NIL,
    NUMBER,
    OR,
    MINUS,
    PLUS,
    PRINT,
    RETURN,
    RIGHT_PAREN,
    RIGHT_BRACE,
    SEMICOLON,
    SLASH,
    STAR,
    STRING,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
}
