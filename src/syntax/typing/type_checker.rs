use std::error::Error;

use crate::syntax::ast::ast_module::AstModule;

pub struct TypeChecker {}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {}
    }

    pub fn type_check(&mut self, ast: AstModule) -> Option<Box<dyn Error>> {
        None
    }
}
