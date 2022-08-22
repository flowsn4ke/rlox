#[derive(Debug)]
pub enum Token {
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
    pub fn get_token_from_symbol(lexeme: &str, line: usize) -> Token {
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
    pub fn get_token_from_identifier(lexeme: &str, line: usize) -> Token {
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
    pub fn get_token_from_number(lexeme: &str, line: usize) -> Token {
        Token::Number(lexeme.to_owned(), lexeme.parse::<f64>().unwrap(), line)
    }
    pub fn get_token_from_string(lexeme: &str, line: usize) -> Token {
        Token::String(lexeme.to_owned(), lexeme.to_owned(), line)
    }
}
