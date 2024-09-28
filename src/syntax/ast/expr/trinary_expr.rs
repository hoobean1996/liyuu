use super::expr::Expr;

pub enum TrinaryExpr {
    TrinaryExpr(Box<Expr>, Box<Expr>, Box<Expr>),
}
