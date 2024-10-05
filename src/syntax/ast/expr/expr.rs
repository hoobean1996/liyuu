use std::fmt;

use super::{
    binary_expr::Binary, group_expr::GroupExpr, literal_expr::Literal, trinary_expr::Trinary,
    unary_expr::Unary,
};

#[derive(PartialEq, Debug)]
pub enum Expr {
    LiteralExpr(Literal),

    UnaryExpr(Unary),
    BinaryExpr(Binary),

    TrinaryExpr(Trinary),
    GroupExpr(GroupExpr),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::LiteralExpr(Literal::Bool(b)) => {
                write!(f, "{}", b)
            }
            Expr::LiteralExpr(Literal::Int(i)) => {
                write!(f, "{}", i)
            }
            Expr::LiteralExpr(Literal::Char(c)) => {
                write!(f, "'{}'", c)
            }
            Expr::LiteralExpr(Literal::String(s)) => {
                write!(f, "\"{}\"", s)
            }
            Expr::LiteralExpr(Literal::ID(s)) => {
                write!(f, "{}", s)
            }
            Expr::UnaryExpr(ue) => match ue {
                Unary::Plus(e) => {
                    let se = e.to_string();
                    write!(f, "{}", se)
                }
                Unary::Minus(e) => {
                    let se = e.to_string();
                    write!(f, "-{}", se)
                }
                Unary::Bang(e) => {
                    let se = e.to_string();
                    write!(f, "!{}", se)
                }
                Unary::BitNot(e) => {
                    let se = e.to_string();
                    write!(f, "~{}", se)
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
                Binary::Mod(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} % {}", &se1, &se2)
                }
                Binary::Eq(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} == {}", &se1, &se2)
                }
                Binary::Neq(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} != {}", &se1, &se2)
                }
                Binary::Gt(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} > {}", &se1, &se2)
                }
                Binary::Gte(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} >= {}", &se1, &se2)
                }
                Binary::Lt(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} < {}", &se1, &se2)
                }
                Binary::Lte(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} <= {}", &se1, &se2)
                }
                Binary::And(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} && {}", &se1, &se2)
                }
                Binary::Or(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} || {}", &se1, &se2)
                }
                Binary::BitAnd(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} & {}", &se1, &se2)
                }
                Binary::BitOr(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} | {}", &se1, &se2)
                }
                Binary::BitXor(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} ^ {}", &se1, &se2)
                }
                Binary::LShift(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} << {}", &se1, &se2)
                }
                Binary::RShift(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{} >> {}", &se1, &se2)
                }
                Binary::Call(e1, e2s) => {
                    let se1 = e1.to_string();
                    let se2 = e2s
                        .iter()
                        .map(|expr| expr.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    write!(f, "{}({})", &se1, &se2)
                }
                Binary::Member(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{}.{}", &se1, &se2)
                }
                Binary::PtrMember(e1, e2) => {
                    let se1 = e1.to_string();
                    let se2 = e2.to_string();
                    write!(f, "{}->{}", &se1, &se2)
                }
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
    use crate::syntax::ast::expr::{
        binary_expr::Binary, literal_expr::Literal, trinary_expr::Trinary, unary_expr::Unary,
    };

    use super::Expr;

    #[test]
    pub fn test_bool() {
        let e = Expr::LiteralExpr(Literal::Bool(false));
        assert_eq!(&e.to_string(), "false");
    }

    #[test]
    pub fn test_int_literal_expr() {
        let e = Expr::LiteralExpr(Literal::Int(3));
        assert_eq!(&e.to_string(), "3");
    }

    #[test]
    pub fn test_char() {
        let e = Expr::LiteralExpr(Literal::Char('a'));
        assert_eq!(&e.to_string(), "'a'");
    }

    #[test]
    pub fn test_string() {
        let e = Expr::LiteralExpr(Literal::String(String::from("abc")));
        assert_eq!(&e.to_string(), "\"abc\"");
    }

    #[test]
    pub fn test_identifier() {
        let e = Expr::LiteralExpr(Literal::ID(String::from("abc")));
        assert_eq!(&e.to_string(), "abc");
    }

    #[test]
    pub fn test_unary_expr() {
        {
            let e1 = Expr::LiteralExpr(Literal::Int(3));
            let e = Expr::UnaryExpr(Unary::Plus(Box::new(e1)));
            assert_eq!(&e.to_string(), "3");
        }
        {
            let e1 = Expr::LiteralExpr(Literal::Int(3));
            let e = Expr::UnaryExpr(Unary::Minus(Box::new(e1)));
            assert_eq!(&e.to_string(), "-3");
        }
        {
            let e1 = Expr::LiteralExpr(Literal::Bool(false));
            let e = Expr::UnaryExpr(Unary::Bang(Box::new(e1)));
            assert_eq!(&e.to_string(), "!false");
        }
        {
            let e1 = Expr::LiteralExpr(Literal::Int(3));
            let e = Expr::UnaryExpr(Unary::BitNot(Box::new(e1)));
            assert_eq!(&e.to_string(), "~3");
        }
    }

    #[test]
    pub fn test_binary_expr() {
        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::Plus(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 + 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::Minus(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 - 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::Mul(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 * 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::Div(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 / 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::Mod(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 % 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::Eq(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 == 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::Neq(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 != 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::Gt(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 > 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::Gte(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 >= 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::Lt(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 < 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::Lte(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 <= 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::And(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 && 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::Or(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 || 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::BitAnd(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 & 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::BitOr(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 | 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::BitXor(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 ^ 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::LShift(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 << 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::Int(1));
            let e2 = Expr::LiteralExpr(Literal::Int(2));
            let e = Expr::BinaryExpr(Binary::RShift(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "1 >> 2");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::ID(String::from("sum")));
            let e = Expr::BinaryExpr(Binary::Call(
                Box::new(e1),
                vec![
                    Expr::LiteralExpr(Literal::Int(1)),
                    Expr::LiteralExpr(Literal::ID(String::from("a"))),
                    Expr::LiteralExpr(Literal::Int(3)),
                ],
            ));
            assert_eq!(&e.to_string(), "sum(1, a, 3)");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::ID(String::from("sum")));
            let e = Expr::BinaryExpr(Binary::Call(
                Box::new(e1),
                vec![
                    Expr::LiteralExpr(Literal::Int(1)),
                    Expr::LiteralExpr(Literal::String(String::from("a"))),
                    Expr::LiteralExpr(Literal::Int(3)),
                ],
            ));
            assert_eq!(&e.to_string(), "sum(1, \"a\", 3)");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::ID(String::from("user")));
            let e2 = Expr::LiteralExpr(Literal::ID(String::from("name")));
            let e = Expr::BinaryExpr(Binary::Member(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "user.name");
        }

        {
            let e1 = Expr::LiteralExpr(Literal::ID(String::from("user")));
            let e2 = Expr::LiteralExpr(Literal::ID(String::from("name")));
            let e = Expr::BinaryExpr(Binary::PtrMember(Box::new(e1), Box::new(e2)));
            assert_eq!(&e.to_string(), "user->name");
        }
    }

    #[test]
    pub fn test_trinary_expr() {
        let cond = Expr::BinaryExpr(Binary::Gt(
            Box::new(Expr::LiteralExpr(Literal::Int(1))),
            Box::new(Expr::LiteralExpr(Literal::Int(5))),
        ));
        let e1 = Expr::LiteralExpr(Literal::Int(1));
        let e2 = Expr::LiteralExpr(Literal::Int(5));
        let e = Expr::TrinaryExpr(Trinary::TrinaryExpr(
            Box::new(cond),
            Box::new(e1),
            Box::new(e2),
        ));
        assert_eq!(&e.to_string(), "1 > 5 ? 1 : 5");
    }
}
