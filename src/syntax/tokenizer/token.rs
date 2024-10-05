use super::attr::Attr;

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,

    // 'a'
    Char(Attr, char),

    // true, false
    Bool(Attr, bool),

    // 3
    Int8(Attr, i8),
    Int16(Attr, i16),
    Int32(Attr, i32),
    Int64(Attr, i64),

    // 3.1
    Float(Attr, f32),
    Float64(Attr, f64),

    // "abc"
    String(Attr, String),

    // a
    ID(Attr, String),

    LParen(Attr),       // (
    RParen(Attr),       // )
    LBrace(Attr),       // {
    RBrace(Attr),       // }
    LBraket(Attr),      // [
    RBraket(Attr),      // ]
    QuestionMark(Attr), // ?

    Hash(Attr),  // #
    Dot(Attr),   // .
    Arrow(Attr), // ->

    Incr(Attr),   // ++
    Plus(Attr),   // +
    Minus(Attr),  // -
    Mul(Attr),    // *
    Div(Attr),    // /
    Lt(Attr),     // <
    Gt(Attr),     // >
    Lte(Attr),    // <=
    LShift(Attr), // <<
    Gte(Attr),    // >=
    RShift(Attr), // >>
    Not(Attr),    // !

    Typedef(Attr), // typedef
    Struct(Attr),  // struct
    Enum(Attr),    // enum
    If(Attr),      // if
    Else(Attr),    // else
    While(Attr),   // while
    Return(Attr),  // return

    EOF,
}
