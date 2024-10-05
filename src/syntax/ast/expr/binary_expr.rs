use super::expr::Expr;

#[derive(PartialEq, Debug)]
pub enum Binary {
    /// a + b
    Plus(Box<Expr>, Box<Expr>),
    /// a - b
    Minus(Box<Expr>, Box<Expr>),
    /// a * b
    Mul(Box<Expr>, Box<Expr>),
    /// a / b
    Div(Box<Expr>, Box<Expr>),
    /// a % b
    Mod(Box<Expr>, Box<Expr>),
    /// a == b
    Eq(Box<Expr>, Box<Expr>),
    /// a != b
    Neq(Box<Expr>, Box<Expr>),
    /// a > b
    Gt(Box<Expr>, Box<Expr>),
    /// a < b
    Lt(Box<Expr>, Box<Expr>),
    /// a >= b
    Gte(Box<Expr>, Box<Expr>),
    /// a <= b
    Lte(Box<Expr>, Box<Expr>),
    /// a && b
    And(Box<Expr>, Box<Expr>),
    /// a || b
    Or(Box<Expr>, Box<Expr>),
    /// a & b
    BitAnd(Box<Expr>, Box<Expr>),
    /// a | b
    BitOr(Box<Expr>, Box<Expr>),
    /// a ^ b
    BitXor(Box<Expr>, Box<Expr>),
    /// a << b
    LShift(Box<Expr>, Box<Expr>),
    /// a >> b
    RShift(Box<Expr>, Box<Expr>),
    /// foo(p1, p2, ...)
    Call(Box<Expr>, Vec<Expr>),
    /// foo.bar
    Member(Box<Expr>, Box<Expr>),
}
