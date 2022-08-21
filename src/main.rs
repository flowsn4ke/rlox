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
    source: Vec<char>,
    current: usize,
    line: u32,
}

impl Scanner {
    fn from(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            current: 0,
            line: 1,
        }
    }
    fn scan_tokens(&mut self) -> Vec<TokenTypes> {
        let mut tokens: Vec<TokenTypes> = Vec::new();

        while self.current < self.source.len() {
            let c = self.get_current_char();

            match c {
                '(' => tokens.push(self.scan_symbol()),
                ')' => tokens.push(self.scan_symbol()),
                '{' => tokens.push(self.scan_symbol()),
                '}' => tokens.push(self.scan_symbol()),
                '[' => tokens.push(self.scan_symbol()),
                ']' => tokens.push(self.scan_symbol()),
                ',' => tokens.push(self.scan_symbol()),
                '.' => tokens.push(self.scan_symbol()),
                '-' => tokens.push(self.scan_symbol()),
                '+' => tokens.push(self.scan_symbol()),
                ';' => tokens.push(self.scan_symbol()),
                '*' => tokens.push(self.scan_symbol()),
                '/' => tokens.push(self.scan_symbol()),
                '!' => tokens.push(self.scan_symbol()),
                '=' => tokens.push(self.scan_symbol()),
                '<' => tokens.push(self.scan_symbol()),
                '>' => tokens.push(self.scan_symbol()),
                '"' => {
                    tokens.push(self.scan_string());
                }
                ' ' | '\r' | '\t' => self.advance(),
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => {
                    if c.is_digit(10) {
                        tokens.push(self.scan_number());
                    } else if self.is_alpha(c) {
                        tokens.push(self.scan_identifier());
                    } else {
                        panic!("Unexpected character.");
                    }
                }
            };
        }

        tokens
    }
    fn get_current_char(&self) -> char {
        self.source[self.current]
    }
    fn advance(&mut self) {
        self.current += 1;
    }
    fn peek(&self) -> char {
        self.source[self.current + 1]
    }
    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }
    fn scan_symbol(&mut self) -> TokenTypes {
        let lexeme_char = &self.source[self.current];
        let lexeme = lexeme_char.to_string();
        let typ = get_symbol_type(&lexeme);

        self.advance();

        TokenTypes::Symbol(Token::new(typ, lexeme.clone(), lexeme.clone(), self.line))
    }
    fn scan_identifier(&mut self) -> TokenTypes {
        let start = self.current;

        while self.get_current_char().is_alphanumeric() {
            self.advance();
        }

        let lexeme_chars = &self.source[start..self.current];
        let lexeme = String::from_iter(lexeme_chars);
        let typ = get_identifier_type(&lexeme);

        TokenTypes::String(Token::new(typ, lexeme.clone(), lexeme.clone(), self.line))
    }
    fn scan_number(&mut self) -> TokenTypes {
        let start = self.current;

        while self.get_current_char().is_digit(10) {
            self.advance();
        }

        let lexeme_chars = &self.source[start..self.current];
        let lexeme = String::from_iter(lexeme_chars);
        let number = lexeme.parse::<i64>().unwrap();

        TokenTypes::Number(Token::new(TokenType::Number, lexeme, number, self.line))
    }
    fn scan_string(&mut self) -> TokenTypes {
        let start = self.current;

        while self.get_current_char() != '"' {
            self.advance();
        }
        self.advance(); // get rid of the trailing "

        let lexeme_chars = &self.source[start..self.current];
        let lexeme = String::from_iter(lexeme_chars);

        TokenTypes::String(Token::new(
            TokenType::String,
            lexeme.clone(),
            lexeme.clone(),
            self.line,
        ))
    }
}

#[derive(Debug)]
struct Token<T: fmt::Display + fmt::Debug> {
    typ: TokenType,
    lexeme: String,
    literal: T,
    line: u32,
}

enum TokenTypes {
    Symbol(Token<String>),
    Identifier(Token<String>),
    Number(Token<i64>),
    String(Token<String>),
}

impl<T: fmt::Display + fmt::Debug> Token<T> {
    fn new(typ: TokenType, lexeme: String, literal: T, line: u32) -> Token<T> {
        Token {
            typ,
            lexeme,
            literal,
            line,
        }
    }
}

fn get_symbol_type(lexeme: &str) -> TokenType {
    match lexeme {
        "=" => TokenType::Equal,
        "!" => TokenType::Bang,
        "-" => TokenType::Minus,
        "+" => TokenType::Plus,
        ";" => TokenType::Semicolon,
        "," => TokenType::Comma,
        "." => TokenType::Dot,
        "<" => TokenType::Less,
        ">" => TokenType::Greater,
        "*" => TokenType::Star,
        "/" => TokenType::Slash,
        "{" => TokenType::LeftBrace,
        "}" => TokenType::RightBrace,
        "[" => TokenType::LeftBracket,
        "]" => TokenType::RightBracket,
        "(" => TokenType::LeftParen,
        ")" => TokenType::RightParen,
        _ => panic!("Unexpected symbol."),
    }
}

fn get_identifier_type(identifier: &str) -> TokenType {
    match identifier {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
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
    And,
    Bang,
    BangEqual,
    Class,
    Comma,
    Dot,
    Else,
    Equal,
    EqualEqual,
    False,
    For,
    Fun,
    Greater,
    GreaterEqual,
    Identifier,
    If,
    LeftBracket,
    LeftBrace,
    LeftParen,
    Less,
    LessEqual,
    Nil,
    Number,
    Or,
    Minus,
    Plus,
    Print,
    Return,
    RightParen,
    RightBracket,
    RightBrace,
    Semicolon,
    Slash,
    Star,
    String,
    Super,
    This,
    True,
    Var,
    While,
}

// fn main() {
//     let a = A {
//         name: String::from("a"),
//         value: 1,
//     };
//     let b = A {
//         name: String::from("b"),
//         value: String::from("b"),
//     };

//     let mut v: Vec<ATypes> = Vec::new();

//     v.push(ATypes::u32(a));
//     v.push(ATypes::string(b));

//     match &v[0] {
//         ATypes::u32(item) => {
//             println!("{:?}", item.value)
//         }
//         ATypes::string(item) => {
//             println!("{:?}", item.value)
//         }
//     };
// }

// fn makeA() -> ATypes {
//     ATypes::u32(A {
//         name: String::from("a"),
//         value: 1,
//     })
// }

// fn makeB() -> ATypes {
//     ATypes::string(A {
//         name: String::from("b"),
//         value: String::from("b"),
//     })
// }

// #[derive(Debug)]
// #[allow(non_camel_case_types)]
// enum ATypes {
//     u32(A<u32>),
//     string(A<String>),
// }
// #[derive(Debug)]
// struct A<T> {
//     name: String,
//     value: T,
// }
