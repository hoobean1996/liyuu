use super::expr::Expr;

#[derive(PartialEq, Debug)]
pub enum Trinary {
    TrinaryExpr(Box<Expr>, Box<Expr>, Box<Expr>),
}
