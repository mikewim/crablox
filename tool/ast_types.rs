use crate::token::Token;

pub trait Expr {
    fn accept(visitor: Visitor);
}

pub trait Visitor<T> {
    fn visit(expr: T);
}

pub struct Binary<T> {
    left: T,
    operator: Token,
    right: T,
}

pub struct Grouping<T> {
    expression: T,
}

pub struct Literal<T> {
    value: T,
}

pub struct Unary<T> {
    operator: Token,
    right: T,
}
