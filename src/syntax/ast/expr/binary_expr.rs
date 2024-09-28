use super::expr::Expr;

pub enum BinaryExpr {
    Plus(Box<Expr>, Box<Expr>),
    Mins(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}
