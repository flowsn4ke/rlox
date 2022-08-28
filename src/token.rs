#[derive(Debug)]
pub enum Token {
    And(&'static str, &'static str, usize),
    Bang(&'static str, &'static str, usize),
    BangEqual(&'static str, &'static str, usize),
    Class(&'static str, &'static str, usize),
    Comma(&'static str, &'static str, usize),
    Dot(&'static str, &'static str, usize),
    Else(&'static str, &'static str, usize),
    Equal(&'static str, &'static str, usize),
    EqualEqual(&'static str, &'static str, usize),
    False(&'static str, &'static str, usize),
    For(&'static str, &'static str, usize),
    Fun(&'static str, &'static str, usize),
    Greater(&'static str, &'static str, usize),
    GreaterEqual(&'static str, &'static str, usize),
    Identifier(String, String, usize),
    If(&'static str, &'static str, usize),
    LeftBracket(&'static str, &'static str, usize),
    LeftBrace(&'static str, &'static str, usize),
    LeftParen(&'static str, &'static str, usize),
    Less(&'static str, &'static str, usize),
    LessEqual(&'static str, &'static str, usize),
    Nil(&'static str, &'static str, usize),
    Number(String, f64, usize),
    Or(&'static str, &'static str, usize),
    Minus(&'static str, &'static str, usize),
    Plus(&'static str, &'static str, usize),
    Print(&'static str, &'static str, usize),
    Return(&'static str, &'static str, usize),
    RightParen(&'static str, &'static str, usize),
    RightBracket(&'static str, &'static str, usize),
    RightBrace(&'static str, &'static str, usize),
    Semicolon(&'static str, &'static str, usize),
    Slash(&'static str, &'static str, usize),
    Star(&'static str, &'static str, usize),
    String(String, String, usize),
    Super(&'static str, &'static str, usize),
    This(&'static str, &'static str, usize),
    True(&'static str, &'static str, usize),
    Var(&'static str, &'static str, usize),
    While(&'static str, &'static str, usize),
}

/*
You should always be using to_owned(). to_string() is the generic conversion
to a String from any type implementing the ToString trait. It uses the formatting
functions and therefor might end up doing multiple allocations and running much more
code than a simple to_owned() which just allocates a buffer and copies the literal
into the buffer.
*/

impl Token {
    pub fn get_token_from_symbol(lexeme: &str, line: usize) -> Token {
        match lexeme {
            "=" => Token::Equal("=", "=", line),
            "==" => Token::EqualEqual("==", "==", line),
            "!=" => Token::BangEqual("!=", "!=", line),
            ">=" => Token::GreaterEqual(">=", ">=", line),
            "<=" => Token::LessEqual("<=", "<=", line),
            "!" => Token::Bang("!", "!", line),
            "-" => Token::Minus("-", "-", line),
            "+" => Token::Plus("+", "+", line),
            ";" => Token::Semicolon(";", ";", line),
            "," => Token::Comma(",", ",", line),
            "." => Token::Dot(".", ".", line),
            "<" => Token::Less("<", "<", line),
            ">" => Token::Greater(">", ">", line),
            "*" => Token::Star("*", "*", line),
            "/" => Token::Slash("/", "/", line),
            "{" => Token::LeftBrace("{", "{", line),
            "}" => Token::RightBrace("}", "}", line),
            "[" => Token::LeftBracket("[", "[", line),
            "]" => Token::RightBracket("]", "]", line),
            "(" => Token::LeftParen("(", "(", line),
            ")" => Token::RightParen(")", ")", line),
            _ => panic!("Unexpected symbol."),
        }
    }
    pub fn get_token_from_identifier(lexeme: &str, line: usize) -> Token {
        match lexeme {
            "and" => Token::And("and", "and", line),
            "class" => Token::Class("class", "class", line),
            "else" => Token::Else("else", "else", line),
            "false" => Token::False("false", "false", line),
            "for" => Token::For("for", "for", line),
            "fun" => Token::Fun("fun", "fun", line),
            "if" => Token::If("if", "if", line),
            "nil" => Token::Nil("nil", "nil", line),
            "or" => Token::Or("or", "or", line),
            "print" => Token::Print("print", "print", line),
            "return" => Token::Return("return", "return", line),
            "super" => Token::Super("super", "super", line),
            "this" => Token::This("this", "this", line),
            "true" => Token::True("true", "true", line),
            "var" => Token::Var("var", "var", line),
            "while" => Token::While("while", "while", line),
            _ => Token::Identifier(lexeme.to_owned(), lexeme.to_owned(), line),
        }
    }
    pub fn get_token_from_number(lexeme: &str, line: usize) -> Token {
        Token::Number(lexeme.to_owned(), lexeme.parse::<f64>().unwrap(), line)
    }
    pub fn get_token_from_string(lexeme: &str, line: usize) -> Token {
        Token::String(lexeme.to_owned(), lexeme.to_owned(), line)
    }
}
