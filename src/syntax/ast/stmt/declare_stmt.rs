use core::fmt;

use crate::syntax::{ast::expr::expr::Expr, typing::types::Type};

use super::stmt::Stmt;

#[derive(PartialEq, Debug)]
pub enum Declare {
    DeclareIdentifier(String, Type, Option<Expr>),
    DeclareFunction {
        name: String,
        parameters: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Stmt>,
    },
    DeclareVector(String, Type, i32),
    DeclarePointer(String, Type),
    DeclareStruct(String, Vec<(String, Type)>),
    DeclareUnion(String, Vec<(String, Type)>),
    DeclareEnum(String, Vec<(String, Option<i32>)>),
    DeclareTypedef(String, Type),
}

impl fmt::Display for Declare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Declare::DeclareIdentifier(name, typ, expr) => {
                write!(f, "int {}", name)
            }
            _ => {
                write!(f, "not implemented yet")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::typing::types::Type;

    use super::Declare;

    #[test]
    pub fn test_declare_identifier() {
        let s1 = Declare::DeclareIdentifier(String::from("a"), Type::Int, None);
        assert_eq!(s1.to_string(), "int a");
    }
}
