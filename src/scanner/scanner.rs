use super::Token;

pub struct Scanner {
    source: Vec<char>,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn from(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
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
                '!' if self.peek() == '='=> tokens.push(self.scan_symbol(2)),
                '!' => tokens.push(self.scan_symbol(1)),
                '=' if self.peek() == '=' => tokens.push(self.scan_symbol(2)),
                '=' => tokens.push(self.scan_symbol(1)),
                '<' if self.peek() == '=' => tokens.push(self.scan_symbol(2)),
                '<' => tokens.push(self.scan_symbol(1)),
                '>' if self.peek() == '=' => tokens.push(self.scan_symbol(2)),
                '>' => tokens.push(self.scan_symbol(1)),
                '"' => tokens.push(self.scan_string()),
                '/' => match self.peek() {
                    '/' => self.skip_line_comment(),
                    '*' => self.skip_block_comment(),
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
        if self.current == self.source.len() - 1 {
            '\0' // escape sequence for the null character
        } else {
            self.source[self.current + 1]
        }
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
    fn scan_symbol(&mut self, len: usize) -> Token {
        let start = self.current;

        for _ in 0..len {
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

        while self.get_current_char().is_digit(10) {
            if self.peek().is_alphabetic() {
                panic!("Invalid syntax at line {}", self.line);
            }

            self.advance();

            if self.get_current_char() == '.' {
                self.advance();

                while self.get_current_char().is_digit(10) {
                    self.advance();
                }
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
    fn skip_line_comment(&mut self) {
        while self.is_not_at_end() && self.get_current_char() != '\n' {
            self.advance();
        }
    }
    fn skip_block_comment(&mut self) {
        let comment_start_line = self.line;

        // skip the "/*" 
        self.advance();
        self.advance();

        while self.is_not_at_end() {
            match self.get_current_char() {
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                '*' if self.peek() == '/' => {
                    self.advance();
                    self.advance();
                    break;
                }
                '/' if self.peek() == '*' => {
                    self.advance();
                    self.advance();
                    self.skip_block_comment(); // support for nested block comments
                }
                _ => self.advance(),
            };

            if self.is_at_end() {
                panic!("Unterminated block comment started at line {}", comment_start_line);
            }
        }
    }
}
