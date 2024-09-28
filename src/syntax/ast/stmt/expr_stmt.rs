use crate::syntax::ast::expr::expr::Expr;

pub enum ExprStmt {
    Expr(Box<Expr>),
}
