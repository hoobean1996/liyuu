use std::error::Error;

use crate::syntax::ast::{
    ast_module::AstModule,
    expr::{expr::Expr, literal_expr::Literal},
};

use super::{env::TypingEnv, types::Types};

pub struct TypeChecker {
    env: TypingEnv,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            env: TypingEnv::new(),
        }
    }

    pub fn type_check_expr(&mut self, expr: &Expr) -> Result<Types, Box<dyn Error>> {
        match expr {
            Expr::LiteralExpr(le) => match le {
                Literal::Bool(_) => Ok(Types::Bool),
                Literal::ID(_) => panic!("not implemented id's type"),
                Literal::Char(_) => Ok(Types::Char),
                Literal::Int(_) => Ok(Types::Int),
                Literal::String(_) => Ok(Types::String),
            },
            _ => panic!("not implemented yet"),
        }
    }

    pub fn type_check(&mut self, ast: AstModule) -> Option<Box<dyn Error>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::{
        ast::expr::{expr::Expr, literal_expr::Literal},
        typing::types::Types,
    };

    use super::TypeChecker;

    #[test]
    pub fn test_literal_types() {
        {
            let mut typechecker = TypeChecker::new();
            let e = Expr::LiteralExpr(Literal::Bool(false));
            let result = typechecker.type_check_expr(&e);
            if let Ok(typ) = result {
                assert_eq!(typ, Types::Bool);
            }
        }

        {
            let mut typechecker = TypeChecker::new();
            let e = Expr::LiteralExpr(Literal::Int(32));
            let result = typechecker.type_check_expr(&e);
            if let Ok(typ) = result {
                assert_eq!(typ, Types::Int);
            }
        }

        {
            let mut typechecker = TypeChecker::new();
            let e = Expr::LiteralExpr(Literal::Char('a'));
            let result = typechecker.type_check_expr(&e);
            if let Ok(typ) = result {
                assert_eq!(typ, Types::Char);
            }
        }
    }
}
