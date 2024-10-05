use super::expr::Expr;

#[derive(PartialEq, Debug)]
pub enum GroupExpr {
    GroupExpr(Box<Expr>),
}
