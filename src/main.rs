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

        while self.is_not_at_end() {
            let c = self.get_current_char();

            match c {
                '(' => tokens.push(self.scan_symbol(1)),
                ')' => tokens.push(self.scan_symbol(1)),
                '{' => tokens.push(self.scan_symbol(1)),
                '}' => tokens.push(self.scan_symbol(1)),
                '[' => tokens.push(self.scan_symbol(1)),
                ']' => tokens.push(self.scan_symbol(1)),
                ',' => tokens.push(self.scan_symbol(1)),
                '.' => tokens.push(self.scan_symbol(1)),
                '-' => tokens.push(self.scan_symbol(1)),
                '+' => tokens.push(self.scan_symbol(1)),
                ';' => tokens.push(self.scan_symbol(1)),
                '*' => tokens.push(self.scan_symbol(1)),
                ' ' | '\r' | '\t' => self.advance(),
                '!' => match self.peek() {
                    '=' => tokens.push(self.scan_symbol(2)),
                    _ => tokens.push(self.scan_symbol(1)),
                },
                '=' => match self.peek() {
                    '=' => tokens.push(self.scan_symbol(2)),
                    _ => tokens.push(self.scan_symbol(1)),
                },
                '<' => match self.peek() {
                    '=' => tokens.push(self.scan_symbol(2)),
                    _ => tokens.push(self.scan_symbol(1)),
                },
                '>' => match self.peek() {
                    '=' => tokens.push(self.scan_symbol(2)),
                    _ => tokens.push(self.scan_symbol(1)),
                },
                '"' => tokens.push(self.scan_string()),
                '/' =>  match self.peek() {
                    '/' => self.skip_comment(),
                    _ => tokens.push(self.scan_symbol(1))
                },
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

        println!("{:?}", tokens);

        tokens
    }
    fn get_current_char(&self) -> char {
        self.source[self.current]
    }
    fn advance(&mut self) {
        // TODO: Put error handling here and in peek?
        self.current += 1;
    }
    fn peek(&self) -> char {
        self.source[self.current + 1]
    }
    fn is_not_at_end(&self) -> bool {
        self.current < self.source.len()
    }
    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }
    fn skip_comment(&mut self) {
        while self.is_not_at_end() && self.get_current_char() != '\n' {
            self.advance();
        }
    }
    fn scan_symbol(&mut self, len: usize) -> TokenTypes {
        let start = self.current;

        for i in 0..len {
            self.advance();
        }

        let lexeme_chars = &self.source[start..self.current];
        let lexeme = String::from_iter(lexeme_chars);
        let typ = get_symbol_type(&lexeme);

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

        TokenTypes::Identifier(Token::new(typ, lexeme.clone(), lexeme.clone(), self.line))
    }
    fn scan_number(&mut self) -> TokenTypes {
        let start = self.current;

        while self.get_current_char().is_digit(10) {
            if !self.peek().is_digit(10) && self.peek().is_alphabetic() {
                panic!("Invalid syntax at line {}", self.line);
            }

            self.advance();

            if self.get_current_char() == '.' {
                self.advance(); // add the point to the lexeme to make a float
            }
        }

        let lexeme_chars = &self.source[start..self.current];
        let lexeme = String::from_iter(lexeme_chars);
        let number = lexeme.parse::<f64>().unwrap();

        TokenTypes::Number(Token::new(TokenType::Number, lexeme, number, self.line))
    }
    fn scan_string(&mut self) -> TokenTypes {
        let start = self.current;

        self.advance(); // skip the first " char

        while self.get_current_char() != '"' && self.is_not_at_end() {
            if self.peek() == '\n' {
                panic!("Multiline strings are forbidden. Fix line {}", self.line);
            }

            self.advance();
        }

        self.advance(); // get rid of the trailing " char

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

#[derive(Debug)]
enum TokenTypes {
    Symbol(Token<String>),
    Identifier(Token<String>),
    Number(Token<f64>),
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
        "==" => TokenType::EqualEqual,
        "!=" => TokenType::BangEqual,
        ">=" => TokenType::GreaterEqual,
        "<=" => TokenType::LessEqual,
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
        "fun" => TokenType::Fun,
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
