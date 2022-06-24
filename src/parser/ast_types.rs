use crate::token::{LiteralType, Token};

pub trait Expr {
    fn accept(&self, visitor: &impl Visitor);
}

pub trait Visitor {
    type Item;

    fn visit_binary(&self, expr: &Binary) -> Self::Item;
    fn visit_literal(&self, expr: &Literal) -> Self::Item;
    fn visit_grouping(&self, expr: &Grouping) -> Self::Item;
    fn visit_unary(&self, expr: &Unary) -> Self::Item;
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Expr for Binary {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_binary(self)
    }
}

#[derive(Debug)]
pub struct Grouping {
    pub expression: Box<dyn Expr>,
}

impl Expr for Grouping {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_grouping(self)
    }
}

#[derive(Debug)]
pub struct Literal(pub LiteralType);

impl Expr for Literal {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_literal(self)
    }
}

#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Expr for Unary {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_unary(self)
    }
}
