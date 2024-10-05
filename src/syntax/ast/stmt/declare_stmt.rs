use core::fmt;

use crate::syntax::{ast::expr::expr::Expr, typing::types::Types};

#[derive(PartialEq, Debug)]
pub enum Declare {
    DeclareIdentifier(String, Types, Option<Expr>),
    DeclareFunction(String, Vec<Types>, Types),
    DeclareVector(String, Types, i32),
    DeclarePointer(String, Types),
    DeclareStruct(String, Vec<(String, Types)>),
    DeclareUnion(String, Vec<(String, Types)>),
    DeclareEnum(String, Vec<(String, Option<i32>)>),
    DeclareTypedef(String, Types),
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
    use crate::syntax::typing::types::Types;

    use super::Declare;

    #[test]
    pub fn test_declare_identifier() {
        let s1 = Declare::DeclareIdentifier(String::from("a"), Types::Int, None);
        assert_eq!(s1.to_string(), "int a");
    }
}
