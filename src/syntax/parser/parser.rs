use crate::syntax::{
    ast::{expr::expr::Expr, stmt::stmt::Stmt},
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

    fn next(&mut self) -> Option<Stmt> {
        while let Some(t) = self.tokens.next() {
            match t {
                Token::Int64(_, i) => {
                    return Some(Stmt::Expr(Box::new(Expr::Int(i))));
                }
                Token::String(_, s) => {
                    return Some(Stmt::Expr(Box::new(Expr::String(s))));
                }
                Token::Char(_, c) => {
                    return Some(Stmt::Expr(Box::new(Expr::Char(c))));
                }
                _ => panic!("Not supported yet"),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::ast::{expr::expr::Expr, stmt::stmt::Stmt};

    use super::Parser;

    #[test]
    pub fn test_parser_int_expr() {
        let mut parser = Parser::new("1");
        assert_eq!(parser.next(), Some(Stmt::Expr(Box::new(Expr::Int(1)))));
    }

    #[test]
    pub fn test_parser_string_expr() {
        let mut parser = Parser::new("\"abc\"");
        assert_eq!(
            parser.next(),
            Some(Stmt::Expr(Box::new(Expr::String(String::from("abc")))))
        );
    }

    #[test]
    pub fn test_parser_char_expr() {
        let mut parser = Parser::new("'a'");
        assert_eq!(parser.next(), Some(Stmt::Expr(Box::new(Expr::Char('a')))))
    }
}
