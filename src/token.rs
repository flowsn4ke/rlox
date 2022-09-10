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

impl Clone for Token {
    fn clone(&self) -> Token {
        match self {
            Token::And(lx, lt, ln) => Token::And(*lx, *lt, *ln),
            Token::Bang(lx, lt, ln) => Token::Bang(*lx, *lt, *ln),
            Token::BangEqual(lx, lt, ln) => Token::BangEqual(*lx, *lt, *ln),
            Token::Class(lx, lt, ln) => Token::Class(*lx, *lt, *ln),
            Token::Comma(lx, lt, ln) => Token::Comma(*lx, *lt, *ln),
            Token::Dot(lx, lt, ln) => Token::Dot(*lx, *lt, *ln),
            Token::Else(lx, lt, ln) => Token::Else(*lx, *lt, *ln),
            Token::Equal(lx, lt, ln) => Token::Equal(*lx, *lt, *ln),
            Token::EqualEqual(lx, lt, ln) => Token::EqualEqual(*lx, *lt, *ln),
            Token::False(lx, lt, ln) => Token::False(*lx, *lt, *ln),
            Token::For(lx, lt, ln) => Token::For(*lx, *lt, *ln),
            Token::Fun(lx, lt, ln) => Token::Fun(*lx, *lt, *ln),
            Token::Greater(lx, lt, ln) => Token::Greater(*lx, *lt, *ln),
            Token::GreaterEqual(lx, lt, ln) => Token::GreaterEqual(*lx, *lt, *ln),
            Token::Identifier(lx, lt, ln) => Token::Identifier(lx.clone(), lt.clone(), *ln),
            Token::If(lx, lt, ln) => Token::If(*lx, *lt, *ln),
            Token::LeftBracket(lx, lt, ln) => Token::LeftBracket(*lx, *lt, *ln),
            Token::LeftBrace(lx, lt, ln) => Token::LeftBrace(*lx, *lt, *ln),
            Token::LeftParen(lx, lt, ln) => Token::LeftParen(*lx, *lt, *ln),
            Token::Less(lx, lt, ln) => Token::Less(*lx, *lt, *ln),
            Token::LessEqual(lx, lt, ln) => Token::LessEqual(*lx, *lt, *ln),
            Token::Nil(lx, lt, ln) => Token::Nil(*lx, *lt, *ln),
            Token::Number(lx, lt, ln) => Token::Number(lx.clone(), *lt, *ln),
            Token::Or(lx, lt, ln) => Token::Or(*lx, *lt, *ln),
            Token::Minus(lx, lt, ln) => Token::Minus(*lx, *lt, *ln),
            Token::Plus(lx, lt, ln) => Token::Plus(*lx, *lt, *ln),
            Token::Print(lx, lt, ln) => Token::Print(*lx, *lt, *ln),
            Token::Return(lx, lt, ln) => Token::Return(*lx, *lt, *ln),
            Token::RightParen(lx, lt, ln) => Token::RightParen(*lx, *lt, *ln),
            Token::RightBracket(lx, lt, ln) => Token::RightBracket(*lx, *lt, *ln),
            Token::RightBrace(lx, lt, ln) => Token::RightBrace(*lx, *lt, *ln),
            Token::Semicolon(lx, lt, ln) => Token::Semicolon(*lx, *lt, *ln),
            Token::Slash(lx, lt, ln) => Token::Slash(*lx, *lt, *ln),
            Token::Star(lx, lt, ln) => Token::Star(*lx, *lt, *ln),
            Token::String(lx, lt, ln) => Token::String(lx.clone(), lt.clone(), *ln),
            Token::Super(lx, lt, ln) => Token::Super(*lx, *lt, *ln),
            Token::This(lx, lt, ln) => Token::This(*lx, *lt, *ln),
            Token::True(lx, lt, ln) => Token::True(*lx, *lt, *ln),
            Token::Var(lx, lt, ln) => Token::Var(*lx, *lt, *ln),
            Token::While(lx, lt, ln) => Token::While(*lx, *lt, *ln),
        }
    }
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
    pub fn get_info(&self) -> (String, String, usize) {
        match self {
            Token::And(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Bang(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::BangEqual(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Class(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Comma(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Dot(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Else(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Equal(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::EqualEqual(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::False(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::For(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Fun(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Greater(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::GreaterEqual(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Identifier(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::If(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::LeftBracket(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::LeftBrace(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::LeftParen(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Less(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::LessEqual(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Nil(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Number(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Or(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Minus(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Plus(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Print(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Return(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::RightParen(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::RightBracket(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::RightBrace(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Semicolon(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Slash(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Star(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::String(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Super(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::This(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::True(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::Var(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
            Token::While(lx, lt, ln) => (lx.to_string(), lt.to_string(), *ln),
        }
    }
}
