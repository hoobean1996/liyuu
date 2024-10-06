use std::error::Error;

use crate::syntax::ast::{
    ast_module::CompilationUnit,
    expr::{expr::Expr, literal_expr::Literal},
};

use super::{env::TypingEnv, types::Type};

pub struct TypeChecker {
    env: TypingEnv,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            env: TypingEnv::new(),
        }
    }

    pub fn type_check_expr(&mut self, expr: &Expr) -> Result<Type, Box<dyn Error>> {
        match expr {
            Expr::LiteralExpr(le) => match le {
                Literal::Bool(_) => Ok(Type::Bool),
                Literal::ID(_) => panic!("not implemented id's type"),
                Literal::Char(_) => Ok(Type::Char),
                Literal::Int(_) => Ok(Type::Int),
                Literal::String(_) => Ok(Type::String),
            },
            _ => panic!("not implemented yet"),
        }
    }

    pub fn type_check(&mut self, ast: CompilationUnit) -> Option<Box<dyn Error>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::{
        ast::expr::{expr::Expr, literal_expr::Literal},
        typing::types::Type,
    };

    use super::TypeChecker;

    #[test]
    pub fn test_literal_types() {
        {
            let mut typechecker = TypeChecker::new();
            let e = Expr::LiteralExpr(Literal::Bool(false));
            let result = typechecker.type_check_expr(&e);
            if let Ok(typ) = result {
                assert_eq!(typ, Type::Bool);
            }
        }

        {
            let mut typechecker = TypeChecker::new();
            let e = Expr::LiteralExpr(Literal::Int(32));
            let result = typechecker.type_check_expr(&e);
            if let Ok(typ) = result {
                assert_eq!(typ, Type::Int);
            }
        }

        {
            let mut typechecker = TypeChecker::new();
            let e = Expr::LiteralExpr(Literal::Char('a'));
            let result = typechecker.type_check_expr(&e);
            if let Ok(typ) = result {
                assert_eq!(typ, Type::Char);
            }
        }
    }
}
