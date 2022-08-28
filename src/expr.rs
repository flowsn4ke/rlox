use crate::token::Token;

pub trait Acceptor {
    fn accept<V: Visitor>(&self, visitor: &V) -> V::Result;
}

pub trait Visitor {
    type Result;

    fn visit_binary_expr(&self, expr: &Binary) -> Self::Result;
    fn visit_expr(&self, name: &Expr) -> Self::Result;
    fn visit_grouping_expr(&self, name: &Grouping) -> Self::Result;
    fn visit_literal_expr(&self, expr: &Literal) -> Self::Result;
    fn visit_unary_expr(&self, expr: &Unary) -> Self::Result;
}

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Binary>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
    Grouping(Box<Grouping>),
}

impl Acceptor for Expr {
    fn accept<V: Visitor>(&self, visitor: &V) -> V::Result {
        visitor.visit_expr(self)
    }
}

#[derive(Debug)]
pub struct Grouping {
    pub expression: Expr,
}

impl Acceptor for Grouping {
    fn accept<V: Visitor>(&self, visitor: &V) -> V::Result {
        visitor.visit_grouping_expr(self)
    }
}

#[derive(Debug)]
pub struct Binary {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

impl Acceptor for Binary {
    fn accept<V: Visitor>(&self, visitor: &V) -> V::Result {
        visitor.visit_binary_expr(self)
    }
}

#[derive(Debug)]
pub struct Literal {
    pub value: f64,
}

impl Acceptor for Literal {
    fn accept<V: Visitor>(&self, visitor: &V) -> V::Result {
        visitor.visit_literal_expr(self)
    }
}

#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Expr,
}

impl Acceptor for Unary {
    fn accept<V: Visitor>(&self, visitor: &V) -> V::Result {
        visitor.visit_unary_expr(self)
    }
}
