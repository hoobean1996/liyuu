use super::expr::Expr;

#[derive(PartialEq, Debug)]
pub enum Unary {
    /// +2
    Plus(Box<Expr>),
    /// -2
    Minus(Box<Expr>),
    /// !false
    Bang(Box<Expr>),
    /// ~a
    BitNot(Box<Expr>),
}
