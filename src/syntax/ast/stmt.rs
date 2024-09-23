use super::expr::expr::Expr;

pub enum Stmt {
    Expr(Expr),
}
