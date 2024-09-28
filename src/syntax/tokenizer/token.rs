use super::attr::Attr;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,

    Int(Attr, i32),
    String(Attr, String),

    Directive(Attr, String),

    LParen(Attr),       // (
    RParen(Attr),       // )
    LBrace(Attr),       // {
    RBrace(Attr),       // }
    LBraket(Attr),      // [
    RBraket(Attr),      // ]
    QuestionMark(Attr), // ?

    Quote(Attr),       // "
    SingleQuote(Attr), // '
    Dot(Attr),         // .
    Arrow(Attr),       // ->

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

    Typedef(Attr), // typedef
    Struct(Attr),  // struct
    Enum(Attr),    // enum
    If(Attr),      // if
    Else(Attr),    // else
    While(Attr),   // while
    Return(Attr),  // return

    EOF,
}

#[cfg(test)]
mod tests {
    use crate::syntax::tokenizer::attr::Attr;

    use super::Token;

    #[test]
    pub fn test_tokens() {
        let token = Token::Int(Attr::new(1, 1), 3);
        if let Token::Int(_, value) = token {
            assert_eq!(value, 3);
        }
    }
}
