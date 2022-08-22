use std::io::Write;
use std::{env, fs, io};

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
    line: usize,
}

impl Scanner {
    fn from(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            current: 0,
            line: 1,
        }
    }
    fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

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
                '/' => match self.peek() {
                    '/' => self.skip_comment(),
                    _ => tokens.push(self.scan_symbol(1)),
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
                        // TODO: Don't panic, output error message and keep scanning so we can report multiple errors at once.
                        panic!("Unexpected character at line {}", self.line);
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
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
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
    fn scan_symbol(&mut self, len: usize) -> Token {
        let start = self.current;

        for i in 0..len {
            self.advance();
        }

        let lexeme_chars = &self.source[start..self.current];
        let lexeme = String::from_iter(lexeme_chars);

        Token::get_token_from_symbol(&lexeme, self.line)
    }
    fn scan_identifier(&mut self) -> Token {
        let start = self.current;

        while self.get_current_char().is_alphanumeric() {
            self.advance();
        }

        let lexeme_chars = &self.source[start..self.current];
        let lexeme = String::from_iter(lexeme_chars);

        Token::get_token_from_identifier(&lexeme, self.line)
    }
    fn scan_number(&mut self) -> Token {
        let start = self.current;
        let mut found_decimal = false;

        while self.get_current_char().is_digit(10) {
            if !self.peek().is_digit(10) && self.peek().is_alphabetic() {
                panic!("Invalid syntax at line {}", self.line);
            }

            self.advance();

            if self.get_current_char() == '.' && !found_decimal {
                self.advance(); // add the point to the lexeme to make a float
                found_decimal = true;
            }
        }

        let lexeme_chars = &self.source[start..self.current];
        let lexeme = String::from_iter(lexeme_chars);

        Token::get_token_from_number(&lexeme, self.line)
    }
    fn scan_string(&mut self) -> Token {
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

        Token::get_token_from_string(&lexeme, self.line)
    }
}

#[derive(Debug)]
enum Token {
    And(String, String, usize),
    Bang(String, String, usize),
    BangEqual(String, String, usize),
    Class(String, String, usize),
    Comma(String, String, usize),
    Dot(String, String, usize),
    Else(String, String, usize),
    Equal(String, String, usize),
    EqualEqual(String, String, usize),
    False(String, String, usize),
    For(String, String, usize),
    Fun(String, String, usize),
    Greater(String, String, usize),
    GreaterEqual(String, String, usize),
    Identifier(String, String, usize),
    If(String, String, usize),
    LeftBracket(String, String, usize),
    LeftBrace(String, String, usize),
    LeftParen(String, String, usize),
    Less(String, String, usize),
    LessEqual(String, String, usize),
    Nil(String, String, usize),
    Number(String, f64, usize),
    Or(String, String, usize),
    Minus(String, String, usize),
    Plus(String, String, usize),
    Print(String, String, usize),
    Return(String, String, usize),
    RightParen(String, String, usize),
    RightBracket(String, String, usize),
    RightBrace(String, String, usize),
    Semicolon(String, String, usize),
    Slash(String, String, usize),
    Star(String, String, usize),
    String(String, String, usize),
    Super(String, String, usize),
    This(String, String, usize),
    True(String, String, usize),
    Var(String, String, usize),
    While(String, String, usize),
}

/*
You should always be using to_owned(). to_string() is the generic conversion
to a String from any type implementing the ToString trait. It uses the formatting
functions and therefor might end up doing multiple allocations and running much more
code than a simple to_owned() which just allocates a buffer and copies the literal
into the buffer.
*/

impl Token {
    fn get_token_from_symbol(lexeme: &str, line: usize) -> Token {
        match lexeme {
            "=" => Token::Equal(lexeme.to_owned(), lexeme.to_owned(), line),
            "==" => Token::EqualEqual(lexeme.to_owned(), lexeme.to_owned(), line),
            "!=" => Token::BangEqual(lexeme.to_owned(), lexeme.to_owned(), line),
            ">=" => Token::GreaterEqual(lexeme.to_owned(), lexeme.to_owned(), line),
            "<=" => Token::LessEqual(lexeme.to_owned(), lexeme.to_owned(), line),
            "!" => Token::Bang(lexeme.to_owned(), lexeme.to_owned(), line),
            "-" => Token::Minus(lexeme.to_owned(), lexeme.to_owned(), line),
            "+" => Token::Plus(lexeme.to_owned(), lexeme.to_owned(), line),
            ";" => Token::Semicolon(lexeme.to_owned(), lexeme.to_owned(), line),
            "," => Token::Comma(lexeme.to_owned(), lexeme.to_owned(), line),
            "." => Token::Dot(lexeme.to_owned(), lexeme.to_owned(), line),
            "<" => Token::Less(lexeme.to_owned(), lexeme.to_owned(), line),
            ">" => Token::Greater(lexeme.to_owned(), lexeme.to_owned(), line),
            "*" => Token::Star(lexeme.to_owned(), lexeme.to_owned(), line),
            "/" => Token::Slash(lexeme.to_owned(), lexeme.to_owned(), line),
            "{" => Token::LeftBrace(lexeme.to_owned(), lexeme.to_owned(), line),
            "}" => Token::RightBrace(lexeme.to_owned(), lexeme.to_owned(), line),
            "[" => Token::LeftBracket(lexeme.to_owned(), lexeme.to_owned(), line),
            "]" => Token::RightBracket(lexeme.to_owned(), lexeme.to_owned(), line),
            "(" => Token::LeftParen(lexeme.to_owned(), lexeme.to_owned(), line),
            ")" => Token::RightParen(lexeme.to_owned(), lexeme.to_owned(), line),
            _ => panic!("Unexpected symbol."),
        }
    }
    fn get_token_from_identifier(lexeme: &str, line: usize) -> Token {
        match lexeme {
            "and" => Token::And(lexeme.to_owned(), lexeme.to_owned(), line),
            "class" => Token::Class(lexeme.to_owned(), lexeme.to_owned(), line),
            "else" => Token::Else(lexeme.to_owned(), lexeme.to_owned(), line),
            "false" => Token::False(lexeme.to_owned(), lexeme.to_owned(), line),
            "for" => Token::For(lexeme.to_owned(), lexeme.to_owned(), line),
            "fun" => Token::Fun(lexeme.to_owned(), lexeme.to_owned(), line),
            "if" => Token::If(lexeme.to_owned(), lexeme.to_owned(), line),
            "nil" => Token::Nil(lexeme.to_owned(), lexeme.to_owned(), line),
            "or" => Token::Or(lexeme.to_owned(), lexeme.to_owned(), line),
            "print" => Token::Print(lexeme.to_owned(), lexeme.to_owned(), line),
            "return" => Token::Return(lexeme.to_owned(), lexeme.to_owned(), line),
            "super" => Token::Super(lexeme.to_owned(), lexeme.to_owned(), line),
            "this" => Token::This(lexeme.to_owned(), lexeme.to_owned(), line),
            "true" => Token::True(lexeme.to_owned(), lexeme.to_owned(), line),
            "var" => Token::Var(lexeme.to_owned(), lexeme.to_owned(), line),
            "while" => Token::While(lexeme.to_owned(), lexeme.to_owned(), line),
            _ => Token::Identifier(lexeme.to_owned(), lexeme.to_owned(), line),
        }
    }
    fn get_token_from_number(lexeme: &str, line: usize) -> Token {
        Token::Number(lexeme.to_owned(), lexeme.parse::<f64>().unwrap(), line)
    }
    fn get_token_from_string(lexeme: &str, line: usize) -> Token {
        Token::String(lexeme.to_owned(), lexeme.to_owned(), line)
    }
}
