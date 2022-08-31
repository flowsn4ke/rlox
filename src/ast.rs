use crate::expr::*;
use crate::token::*;

pub struct Ast {
    expression: Expr,
}

impl Visitor for Ast {
    type Result = String;

    fn visit_expr(&self, expr: &Expr) -> Self::Result {
        match expr {
            Expr::Binary(binary) => self.visit_binary_expr(binary),
            Expr::Grouping(grouping) => self.visit_grouping_expr(grouping),
            Expr::Literal(literal) => self.visit_literal_expr(literal),
            Expr::Unary(unary) => self.visit_unary_expr(unary),
        }
    }

    fn visit_binary_expr(&self, expr: &Binary) -> String {
        let mut result = String::new();
        let binary_expr = &*expr;
        let operator_lexeme = get_token_lexeme(&binary_expr.operator);

        result.push('(');
        result.push_str(operator_lexeme);
        result.push_str(" ");
        result.push_str(&self.visit_expr(&binary_expr.left));
        result.push_str(" ");
        result.push_str(&self.visit_expr(&binary_expr.right));
        result.push(')');

        result
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        let mut result = String::new();
        let grouping_expr = &*expr;
        result.push_str("(");
        result.push_str(&grouping_expr.expression.accept(self));
        result.push_str(")");
        result
    }

    fn visit_literal_expr(&self, expr: &Literal) -> String {
        let mut result = String::new();
        let literal_expr = &*expr;
        result.push_str(&literal_expr.value.to_string());
        result
    }

    fn visit_unary_expr(&self, expr: &Unary) -> String {
        let mut result = String::new();
        let unary_expr = &*expr;
        let operator_lexeme = get_token_lexeme(&unary_expr.operator);

        result.push_str(operator_lexeme);
        result.push_str(&unary_expr.right.accept(self));
        result
    }
}

impl Ast {
    pub fn new_test_ast_to_str() -> String {
        let ast = Ast {
            expression: Expr::Binary(Box::new(Binary {
                left: Expr::Binary(Box::new(Binary {
                    left: Expr::Literal(Box::new(Literal { value: 1.0 })),
                    operator: Token::Plus("+", "+", 1),
                    right: Expr::Literal(Box::new(Literal { value: 2.0 })),
                })),
                right: Expr::Binary(Box::new(Binary {
                    left: Expr::Literal(Box::new(Literal { value: 1.0 })),
                    operator: Token::Plus("+", "+", 1),
                    right: Expr::Literal(Box::new(Literal { value: 2.0 })),
                })),
                operator: Token::Minus("-", "-", 1),
            })),
        };
        ast.visit_expr(&ast.expression)
    }
}

fn get_token_lexeme(token: &Token) -> &str {
    match token {
        Token::And(lexeme, _, _) => *lexeme,
        Token::Bang(lexeme, _, _) => *lexeme,
        Token::BangEqual(lexeme, _, _) => *lexeme,
        Token::Class(lexeme, _, _) => *lexeme,
        Token::Comma(lexeme, _, _) => *lexeme,
        Token::Dot(lexeme, _, _) => *lexeme,
        Token::Else(lexeme, _, _) => *lexeme,
        Token::Equal(lexeme, _, _) => *lexeme,
        Token::EqualEqual(lexeme, _, _) => *lexeme,
        Token::False(lexeme, _, _) => *lexeme,
        Token::For(lexeme, _, _) => *lexeme,
        Token::Fun(lexeme, _, _) => *lexeme,
        Token::Greater(lexeme, _, _) => *lexeme,
        Token::GreaterEqual(lexeme, _, _) => *lexeme,
        Token::Identifier(lexeme, _, _) => lexeme,
        Token::If(lexeme, _, _) => *lexeme,
        Token::LeftBracket(lexeme, _, _) => *lexeme,
        Token::LeftBrace(lexeme, _, _) => *lexeme,
        Token::LeftParen(lexeme, _, _) => *lexeme,
        Token::Less(lexeme, _, _) => *lexeme,
        Token::LessEqual(lexeme, _, _) => *lexeme,
        Token::Nil(lexeme, _, _) => *lexeme,
        Token::Number(lexeme, _, _) => lexeme,
        Token::Or(lexeme, _, _) => *lexeme,
        Token::Minus(lexeme, _, _) => *lexeme,
        Token::Plus(lexeme, _, _) => *lexeme,
        Token::Print(lexeme, _, _) => *lexeme,
        Token::Return(lexeme, _, _) => *lexeme,
        Token::RightParen(lexeme, _, _) => *lexeme,
        Token::RightBracket(lexeme, _, _) => *lexeme,
        Token::RightBrace(lexeme, _, _) => *lexeme,
        Token::Semicolon(lexeme, _, _) => *lexeme,
        Token::Slash(lexeme, _, _) => *lexeme,
        Token::Star(lexeme, _, _) => *lexeme,
        Token::String(lexeme, _, _) => lexeme,
        Token::Super(lexeme, _, _) => *lexeme,
        Token::This(lexeme, _, _) => *lexeme,
        Token::True(lexeme, _, _) => *lexeme,
        Token::Var(lexeme, _, _) => *lexeme,
        Token::While(lexeme, _, _) => *lexeme,
    }
}
