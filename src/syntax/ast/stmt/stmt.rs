use crate::syntax::ast::expr::expr::Expr;

use super::{declare_stmt::DeclareStmt, directive_stmt::DirectiveStmt, expr_stmt::ExprStmt};

pub enum Stmt {
    ExprStmt(ExprStmt),
    DirectiveStmt(DirectiveStmt),
    AssignStmt(Box<Expr>, Box<Expr>),
    ReturnStmt(Box<Expr>),
    BlockStmt(Vec<Stmt>),
    DeclareStmt(DeclareStmt),
}

#[cfg(test)]
mod tests {
    use crate::syntax::ast::expr::expr::Expr;

    use super::Stmt;

    #[test]
    pub fn test_return() {
        let s = Stmt::ReturnStmt(Box::new(Expr::IntLiteralExpr(3)));
    }
}
