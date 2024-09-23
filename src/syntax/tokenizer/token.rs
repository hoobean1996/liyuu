#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,

    IntLiteral(i64),

    Plus,

    EOF,
}

#[cfg(test)]
mod tests {
    use super::Token;

    #[test]
    pub fn test_tokens() {
        let token = Token::IntLiteral(3);
        if let Token::IntLiteral(value) = token {
            assert_eq!(value, 3);
        }
    }
}
