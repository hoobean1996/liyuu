use super::token::Token;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TokenStream {
    tokens: Vec<Token>,
}
#[cfg(test)]
mod tests {
    use crate::syntax::tokenizer::token::Token;

    use super::TokenStream;

    #[test]
    pub fn test_default() {
        let ts = TokenStream::default();
        let ts2: Vec<Token> = vec![];
        assert_eq!(ts.tokens, ts2);
    }
}
