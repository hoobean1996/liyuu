use super::expr::Expr;

pub enum UnaryExpr {
    Plus(Box<Expr>),
    Minus(Box<Expr>),
}
