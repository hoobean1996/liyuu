use crate::syntax::{
    ast::stmt::stmt::Stmt,
    tokenizer::{token::Token, tokenizer::Tokenizer},
};

pub struct Parser<'a> {
    tokens: std::iter::Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let tokenizer = Tokenizer::new(input);
        Parser {
            tokens: tokenizer.peekable(),
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Stmt;

    fn next(&mut self) -> Option<Stmt> {}
}
