use std::fmt;

use super::{
    binary_expr::BinaryExpr, group_expr::GroupExpr, trinary_expr::TrinaryExpr,
    unary_expr::UnaryExpr,
};

pub enum Expr {
    BoolLiteralExpr(bool),
    IntLiteralExpr(i64),
    StringLiteralExpr(String),
    IdentifierExpr(String),

    UnaryExpr(UnaryExpr),
    BinaryExpr(BinaryExpr),
    TrinaryExpr(TrinaryExpr),
    GroupExpr(GroupExpr),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::BoolLiteralExpr(b) => {
                write!(f, "{}", b)
            }
            Expr::IntLiteralExpr(i) => {
                write!(f, "{}", i)
            }
            Expr::StringLiteralExpr(s) => {
                write!(f, "{}", s)
            }
            Expr::IdentifierExpr(s) => {
                write!(f, "{}", s)
            }
            Expr::UnaryExpr(ue) => match ue {
                UnaryExpr::Plus(e) => {
                    let se = e.to_string();
                    write!(f, "({})", se)
                }
                UnaryExpr::Minus(e) => {
                    let se = e.to_string();
                    write!(f, "-({})", se)
                }
            },
            _ => {
                write!(f, "not implemented yet")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::ast::expr::unary_expr::UnaryExpr;

    use super::Expr;

    #[test]
    pub fn test_int_literal_expr() {
        let expect_value = 3;
        let e = Expr::IntLiteralExpr(expect_value);
        if let Expr::IntLiteralExpr(actual_value) = e {
            assert_eq!(actual_value, 3);
        }
    }

    #[test]
    pub fn test_unary_expr() {
        {
            let expect_value = 3;
            let e1 = Expr::IntLiteralExpr(expect_value);
            let e2 = Expr::UnaryExpr(UnaryExpr::Plus(Box::new(e1)));
            if let Expr::UnaryExpr(ue) = e2 {
                if let UnaryExpr::Plus(e) = ue {
                    if let Expr::IntLiteralExpr(actual_value) = *e {
                        assert_eq!(actual_value, expect_value);
                    }
                }
            }
        }
        {
            let expect_value = 3;
            let e1 = Expr::IntLiteralExpr(expect_value);
            let e2 = Expr::UnaryExpr(UnaryExpr::Minus(Box::new(e1)));
            if let Expr::UnaryExpr(ue) = e2 {
                if let UnaryExpr::Minus(e) = ue {
                    if let Expr::IntLiteralExpr(actual_value) = *e {
                        assert_eq!(actual_value, expect_value);
                    }
                }
            }
        }
    }

    #[test]
    pub fn test_display() {
        {
            let e = Expr::BoolLiteralExpr(true);
            assert_eq!(e.to_string(), "true");
        }

        {
            let e = Expr::BoolLiteralExpr(false);
            assert_eq!(e.to_string(), "false");
        }

        {
            let e = Expr::IntLiteralExpr(3);
            assert_eq!(e.to_string(), "3");
        }

        {
            let e = Expr::StringLiteralExpr(String::from("abc"));
            assert_eq!(e.to_string(), "abc");
        }

        {
            let e = Expr::IntLiteralExpr(3);
            let ue = Expr::UnaryExpr(UnaryExpr::Minus(Box::new(e)));

            assert_eq!(ue.to_string(), "-(3)");
        }
    }
}
