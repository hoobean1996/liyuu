use std::fmt;

use super::{binary_expr::Binary, group_expr::GroupExpr, trinary_expr::Trinary, unary_expr::Unary};

#[derive(PartialEq, Debug)]
pub enum Expr {
    Bool(bool),
    Int(i64),
    Char(char),
    String(String),
    ID(String),

    UnaryExpr(Unary),
    BinaryExpr(Binary),

    TrinaryExpr(Trinary),
    GroupExpr(GroupExpr),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Bool(b) => {
                write!(f, "{}", b)
            }
            Expr::Int(i) => {
                write!(f, "{}", i)
            }
            Expr::Char(c) => {
                write!(f, "'{}'", c)
            }
            Expr::String(s) => {
                write!(f, "\"{}\"", s)
            }
            Expr::ID(s) => {
                write!(f, "{}", s)
            }
            Expr::UnaryExpr(ue) => match ue {
                Unary::Plus(e) => {
                    let se = e.to_string();
                    write!(f, "({})", se)
                }
                Unary::Minus(e) => {
                    let se = e.to_string();
                    write!(f, "-({})", se)
                }
                Unary::Bang(e) => {
                    let se = e.to_string();
                    write!(f, "!({})", se)
                }
                Unary::BitNot(e) => {
                    let se = e.to_string();
                    write!(f, "~({})", se)
                }
            },
            Expr::BinaryExpr(be) => match be {
                Binary::Plus(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} + {}", &se1, &se2)
                }
                Binary::Minus(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} - {}", &se1, &se2)
                }
                Binary::Mul(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} * {}", &se1, &se2)
                }
                Binary::Div(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} / {}", &se1, &se2)
                }
                _ => panic!("not supported yet"),
            },
            Expr::TrinaryExpr(te) => match te {
                Trinary::TrinaryExpr(cond, e1, e2) => {
                    write!(
                        f,
                        "{} ? {} : {}",
                        cond.to_string(),
                        e1.to_string(),
                        e2.to_string()
                    )
                }
            },
            Expr::GroupExpr(ge) => match ge {
                GroupExpr::GroupExpr(e) => {
                    write!(f, "({})", e.to_string())
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::ast::expr::unary_expr::Unary;

    use super::Expr;

    #[test]
    pub fn test_bool() {
        let expect_value = false;
        let e = Expr::Bool(false);
        assert_eq!(&e.to_string(), "false");
        if let Expr::Bool(actual_value) = e {
            assert_eq!(actual_value, expect_value);
        }
    }

    #[test]
    pub fn test_int_literal_expr() {
        let expect_value = 3;
        let e = Expr::Int(expect_value);
        assert_eq!(&e.to_string(), "3");
        if let Expr::Int(actual_value) = e {
            assert_eq!(actual_value, expect_value);
        }
    }

    #[test]
    pub fn test_char() {
        let expect_value = 'a';
        let e = Expr::Char(expect_value);
        assert_eq!(&e.to_string(), "'a'");
        if let Expr::Char(actual_value) = e {
            assert_eq!(actual_value, expect_value);
        }
    }

    #[test]
    pub fn test_string() {
        let expect_value = "abc";
        let e = Expr::String(String::from(expect_value));
        assert_eq!(&e.to_string(), "\"abc\"");
        if let Expr::String(actual_value) = e {
            assert_eq!(actual_value, expect_value);
        }
    }

    #[test]
    pub fn test_identifier() {
        let expect_value = "abc";
        let e = Expr::ID(String::from(expect_value));
        assert_eq!(&e.to_string(), "abc");
        if let Expr::ID(actual_value) = e {
            assert_eq!(actual_value, expect_value);
        }
    }

    #[test]
    pub fn test_unary_expr() {
        // (3)
        {
            let expect_value = 3;
            let e1 = Expr::Int(expect_value);
            let e = Expr::UnaryExpr(Unary::Plus(Box::new(e1)));
            assert_eq!(&e.to_string(), "(3)");
            if let Expr::UnaryExpr(ue) = e {
                if let Unary::Plus(e) = ue {
                    if let Expr::Int(actual_value) = *e {
                        assert_eq!(actual_value, expect_value);
                    }
                }
            }
        }
        // -(3)
        {
            let expect_value = 3;
            let e1 = Expr::Int(expect_value);
            let e = Expr::UnaryExpr(Unary::Minus(Box::new(e1)));
            assert_eq!(&e.to_string(), "-(3)");
            if let Expr::UnaryExpr(ue) = e {
                if let Unary::Minus(e) = ue {
                    if let Expr::Int(actual_value) = *e {
                        assert_eq!(actual_value, expect_value);
                    }
                }
            }
        }
        {
            let expect_value = false;
            let e1 = Expr::Bool(expect_value);
            let e = Expr::UnaryExpr(Unary::Bang(Box::new(e1)));
            assert_eq!(&e.to_string(), "!(false)");
            if let Expr::UnaryExpr(ue) = e {
                if let Unary::Bang(e) = ue {
                    if let Expr::Bool(actual_value) = *e {
                        assert_eq!(actual_value, expect_value);
                    }
                }
            }
        }
    }
}
