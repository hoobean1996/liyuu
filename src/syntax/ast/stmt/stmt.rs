use crate::syntax::ast::expr::expr::Expr;

use super::{declare_stmt::Declare, directive_stmt::Directive};

#[derive(PartialEq, Debug)]
pub enum Stmt {
    // 1
    // 'a'
    // "abc"
    // a
    // 1 + 1
    // 1 - 2
    // 1 * 2
    // 1 / 2
    Expr(Box<Expr>),
    Directive(Directive),
    // a = b
    Assign(Box<Expr>, Box<Expr>),
    // return a
    Return(Box<Expr>),
    // {
    //   a = 1;
    //   return a;
    // }
    Block(Vec<Stmt>),
    // int a;
    Declare(Declare),
}
