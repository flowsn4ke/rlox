use crate::{expr::*, token::*};

pub struct ParseError {
    pub message: String,
    pub line: usize,
}

pub struct Parser {
    tokens: Vec<Token>,
    source: String,
    current: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, source: String) -> Parser {
        Parser {
            tokens,
            source,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.is_not_at_end() {
            match self.tokens[self.current] {
                Token::BangEqual(lx, lt, ln) => {
                    let operator = self.tokens[self.current].clone();

                    self.advance();

                    expr = Expr::Binary(Box::new(Binary {
                        left: expr,
                        operator,
                        right: self.comparison(),
                    }))
                }
                Token::EqualEqual(lx, lt, ln) => {
                    let operator = self.tokens[self.current].clone();

                    self.advance();
                    expr = Expr::Binary(Box::new(Binary {
                        left: expr,
                        operator,
                        right: self.comparison(),
                    }))
                }
                _ => break,
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.is_not_at_end() {
            match self.tokens[self.current] {
                Token::Greater(lx, lt, ln) => {
                    let operator = self.tokens[self.current].clone();

                    self.advance();

                    expr = Expr::Binary(Box::new(Binary {
                        left: expr,
                        operator,
                        right: self.term(),
                    }))
                }
                Token::GreaterEqual(lx, lt, ln) => {
                    let operator = self.tokens[self.current].clone();

                    self.advance();

                    expr = Expr::Binary(Box::new(Binary {
                        left: expr,
                        operator,
                        right: self.term(),
                    }))
                }
                Token::Less(lx, lt, ln) => {
                    let operator = self.tokens[self.current].clone();

                    self.advance();

                    expr = Expr::Binary(Box::new(Binary {
                        left: expr,
                        operator,
                        right: self.term(),
                    }))
                }
                Token::LessEqual(lx, lt, ln) => {
                    let operator = self.tokens[self.current].clone();

                    self.advance();

                    expr = Expr::Binary(Box::new(Binary {
                        left: expr,
                        operator,
                        right: self.term(),
                    }))
                }
                _ => break,
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.is_not_at_end() {
            match self.tokens[self.current] {
                Token::Minus(lx, lt, ln) => {
                    let operator = self.tokens[self.current].clone();

                    self.advance();

                    expr = Expr::Binary(Box::new(Binary {
                        left: expr,
                        operator,
                        right: self.factor(),
                    }))
                }
                Token::Plus(lx, lt, ln) => {
                    let operator = self.tokens[self.current].clone();

                    self.advance();

                    expr = Expr::Binary(Box::new(Binary {
                        left: expr,
                        operator,
                        right: self.factor(),
                    }))
                }
                _ => break,
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.is_not_at_end() {
            match self.tokens[self.current] {
                Token::Slash(lx, lt, ln) => {
                    let operator = self.tokens[self.current].clone();

                    self.advance();

                    expr = Expr::Binary(Box::new(Binary {
                        left: expr,
                        operator,
                        right: self.unary(),
                    }))
                }
                Token::Star(lx, lt, ln) => {
                    let operator = self.tokens[self.current].clone();

                    self.advance();

                    expr = Expr::Binary(Box::new(Binary {
                        left: expr,
                        operator,
                        right: self.unary(),
                    }))
                }
                _ => break,
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        match self.tokens[self.current] {
            Token::Bang(lx, lt, ln) => {
                let operator = self.tokens[self.current].clone();

                self.advance();

                let expr = Expr::Unary(Box::new(Unary {
                    operator,
                    right: self.unary(),
                }));

                return expr;
            }
            Token::Minus(lx, lt, ln) => {
                let operator = self.tokens[self.current].clone();

                self.advance();

                return Expr::Unary(Box::new(Unary {
                    operator,
                    right: self.unary(),
                }));
            }
            _ => {}
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        let token = self.tokens[self.current].clone();

        self.advance();

        match token {
            Token::False(lx, lt, ln) => Expr::Literal(Literal::Boolean(false)),
            Token::True(lx, lt, ln) => Expr::Literal(Literal::Boolean(true)),
            Token::Nil(lx, lt, ln) => Expr::Literal(Literal::Nil),
            Token::Number(lx, lt, ln) => Expr::Literal(Literal::Number(lt)),
            Token::String(lx, lt, ln) => Expr::Literal(Literal::String(lt)),
            Token::LeftParen(lx, lt, ln) => {
                let expr = self.expression();

                match self.tokens[self.current] {
                    Token::RightParen(lx, lt, ln) => self.advance(),
                    _ => panic!("Expected ')' after expression started at line {ln}"),
                };

                Expr::Grouping(Box::new(Grouping { expression: expr }))
            }
            _ => {
                let (lx, lt, ln) = token.get_info();

                panic!("Unexpected token: \"{lx}\" at line {ln}");
            }
        }
    }

    fn advance(&mut self) {
        if self.is_not_at_end() {
            self.current += 1;
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= (self.tokens.len() - 1)
    }
    fn is_not_at_end(&self) -> bool {
        self.current < self.tokens.len()
    }
}
