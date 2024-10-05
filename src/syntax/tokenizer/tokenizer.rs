use core::panic;

use super::{attr::Attr, cursor::Cursor, token::Token};

pub struct Tokenizer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    cursor: Cursor,
    attr: Attr,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let tokenizer = Tokenizer {
            chars: input.chars().peekable(),
            cursor: Cursor::new(),
            attr: Attr::new(),
        };
        tokenizer
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next();
        if c.is_some() {
            self.cursor.incr_col();
        }
        c
    }

    fn peek(&mut self) -> Option<&char> {
        let c = self.chars.peek();
        c
    }

    fn is_keyword(&self, s: &str) -> bool {
        match s {
            "typedef" | "struct" | "enum" | "if" | "else" | "while" | "return" => true,
            _ => false,
        }
    }

    fn mark_start(&mut self) {
        self.attr.set_start_pos(self.cursor.cur_pos());
    }

    fn mark_end(&mut self) {
        self.attr.set_end_pos(self.cursor.cur_pos());
    }

    fn read_number(&mut self, c: char) -> i64 {
        let mut v = c.to_digit(10).unwrap();
        while let Some(&c) = self.chars.peek() {
            if c.is_digit(10) {
                self.chars.next();
                let digit = c.to_digit(10).unwrap() as u32;
                v = v * 10 + digit;
            } else {
                break;
            }
        }
        v as i64
    }

    fn parse_number(&mut self, c: char) -> Token {
        self.mark_start();
        let v = self.read_number(c);
        self.mark_end();
        Token::Int64(self.attr.clone(), v)
    }

    fn is_newline(&mut self, c: char) -> bool {
        match c {
            '\n' => true,
            '\r' => true,
            _ => false,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        while let Some(c) = self.advance() {
            if self.is_newline(c) {
                self.cursor.incr_row();
                continue;
            }
            if c.is_whitespace() {
                continue;
            }
            return Some(match c {
                '+' => {
                    self.mark_start();
                    let c1 = self.peek();
                    match c1 {
                        Some('+') => {
                            self.advance();
                            self.mark_end();
                            Token::Incr(self.attr.clone())
                        }
                        _ => {
                            self.mark_end();
                            Token::Plus(self.attr.clone())
                        }
                    }
                }
                '-' => {
                    self.mark_start();
                    let c1 = self.peek();
                    match c1 {
                        Some('>') => {
                            self.advance();
                            self.mark_end();
                            Token::Arrow(self.attr.clone())
                        }
                        _ => {
                            self.mark_end();
                            Token::Minus(self.attr.clone())
                        }
                    }
                }
                '/' => {
                    self.mark_start();
                    self.mark_end();
                    Token::Div(self.attr.clone())
                }
                '*' => {
                    self.mark_start();
                    self.mark_end();
                    Token::Mul(self.attr.clone())
                }
                '!' => {
                    self.mark_start();
                    self.mark_end();
                    Token::Not(self.attr.clone())
                }
                '<' => {
                    self.mark_start();
                    let c1 = self.peek();
                    match c1 {
                        Some('<') => {
                            self.advance();
                            self.mark_end();
                            Token::LShift(self.attr.clone())
                        }
                        Some('=') => {
                            self.advance();
                            self.mark_end();
                            Token::Lte(self.attr.clone())
                        }
                        _ => {
                            self.mark_end();
                            Token::Lt(self.attr.clone())
                        }
                    }
                }
                '>' => {
                    self.mark_start();
                    let c1 = self.peek();
                    match c1 {
                        Some('>') => {
                            self.advance();
                            self.mark_end();
                            Token::RShift(self.attr.clone())
                        }
                        Some('=') => {
                            self.advance();
                            self.mark_end();
                            Token::Gte(self.attr.clone())
                        }
                        _ => {
                            self.mark_end();
                            Token::Gt(self.attr.clone())
                        }
                    }
                }
                '.' => {
                    self.mark_start();
                    self.mark_end();
                    Token::Dot(self.attr.clone())
                }
                '(' => {
                    self.mark_start();
                    self.mark_end();
                    Token::LParen(self.attr.clone())
                }
                ')' => {
                    self.mark_start();
                    self.mark_end();
                    Token::RParen(self.attr.clone())
                }
                '[' => {
                    self.mark_start();
                    self.mark_end();
                    Token::LBraket(self.attr.clone())
                }
                ']' => {
                    self.mark_start();
                    self.mark_end();
                    Token::RBraket(self.attr.clone())
                }
                '{' => {
                    self.mark_start();
                    self.mark_end();
                    Token::LBrace(self.attr.clone())
                }
                '}' => {
                    self.mark_start();
                    self.mark_end();
                    Token::RBrace(self.attr.clone())
                }
                '?' => {
                    self.mark_start();
                    self.mark_end();
                    Token::QuestionMark(self.attr.clone())
                }
                '#' => {
                    self.mark_start();
                    self.mark_end();
                    Token::Hash(self.attr.clone())
                }
                // char
                '\'' => {
                    self.mark_start();
                    if let Some(c1) = self.advance() {
                        self.advance();
                        self.mark_end();
                        let attr = self.attr.clone();
                        Token::Char(attr, c1)
                    } else {
                        panic!("the ' is not enclosed")
                    }
                }
                // string
                '"' => {
                    self.mark_start();
                    let mut s = String::new();
                    loop {
                        let c1 = self.advance();
                        match c1 {
                            Some('"') => {
                                self.mark_end();
                                return Some(Token::String(self.attr.clone(), s));
                            }
                            Some(c2) => {
                                s.push(c2);
                            }
                            None => panic!("\" is not enclosed"),
                        }
                    }
                }
                // number
                '0'..='9' => self.parse_number(c),
                // id
                'a'..='z' | 'A'..='Z' => {
                    self.mark_start();
                    let mut s = String::new();
                    s.push(c);
                    loop {
                        if let Some(c1) = self.advance() {
                            match c1 {
                                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                                    s.push(c1);
                                }
                                _ => {
                                    self.mark_end();
                                    if self.is_keyword(&s) {
                                        return Some(Token::ID(self.attr.clone(), s));
                                    }
                                    return Some(Token::String(self.attr.clone(), s));
                                }
                            }
                        } else {
                            self.mark_end();
                            if self.is_keyword(&s) {
                                return Some(Token::ID(self.attr.clone(), s));
                            }
                            return Some(Token::String(self.attr.clone(), s));
                        }
                    }
                }
                _ => panic!("{} is not supported yet", c),
            });
        }

        Some(Token::EOF)
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::tokenizer::{attr::Attr, token::Token};

    use super::Tokenizer;

    #[test]
    pub fn test_char() {
        let mut tokenizer = Tokenizer::new("'a'");
        assert_eq!(
            tokenizer.next(),
            Some(Token::Char(Attr::range(1, 1, 1, 3), 'a'))
        )
    }

    #[test]
    pub fn test_i64() {
        let testcases = vec!["1", "12", "123", "1234"];
        let testcase_results: Vec<i64> = vec![1, 12, 123, 1234];
        for (i, testcase) in testcases.iter().enumerate() {
            let mut tokenizer = Tokenizer::new(&testcase);
            assert_eq!(
                tokenizer.next(),
                Some(Token::Int64(Attr::point(1, 1), testcase_results[i]))
            )
        }
    }

    #[test]
    pub fn test_string() {
        let testcases = vec!["\"a\"", "\"ab\"", "\"abc\"", "\"abcd\""];
        let testcase_results: Vec<String> = vec![
            String::from("a"),
            String::from("ab"),
            String::from("abc"),
            String::from("abcd"),
        ];
        for (i, testcase) in testcases.iter().enumerate() {
            let mut tokenizer = Tokenizer::new(&testcase);
            assert_eq!(
                tokenizer.next(),
                Some(Token::String(
                    Attr::range(1, 1, 1, (2 + (i + 1)) as u8),
                    testcase_results[i].clone(),
                ))
            );
        }
    }

    #[test]
    pub fn test_id() {
        let mut tokenizer = Tokenizer::new("abc");
        assert_eq!(
            tokenizer.next(),
            Some(Token::String(Attr::range(1, 1, 1, 3), String::from("abc")))
        );
    }

    #[test]
    pub fn test_symbols() {
        {
            let mut tokenizer = Tokenizer::new("+");
            assert_eq!(tokenizer.next(), Some(Token::Plus(Attr::point(1, 1))));
        }
        {
            let mut tokenizer = Tokenizer::new("++");
            assert_eq!(tokenizer.next(), Some(Token::Incr(Attr::range(1, 1, 1, 2))));
        }
        {
            let mut tokenizer = Tokenizer::new("-");
            assert_eq!(tokenizer.next(), Some(Token::Minus(Attr::point(1, 1))));
        }
        {
            let mut tokenizer = Tokenizer::new("->");
            assert_eq!(
                tokenizer.next(),
                Some(Token::Arrow(Attr::range(1, 1, 1, 2)))
            );
        }
        {
            let mut tokenizer = Tokenizer::new("/");
            assert_eq!(tokenizer.next(), Some(Token::Div(Attr::point(1, 1))));
        }
        {
            let mut tokenizer = Tokenizer::new("*");
            assert_eq!(tokenizer.next(), Some(Token::Mul(Attr::point(1, 1))));
        }
        {
            let mut tokenizer = Tokenizer::new("<");
            assert_eq!(tokenizer.next(), Some(Token::Lt(Attr::point(1, 1))));
        }
        {
            let mut tokenizer = Tokenizer::new("<<");
            assert_eq!(
                tokenizer.next(),
                Some(Token::LShift(Attr::range(1, 1, 1, 2)))
            );
        }
        {
            let mut tokenizer = Tokenizer::new("<=");
            assert_eq!(tokenizer.next(), Some(Token::Lte(Attr::range(1, 1, 1, 2))));
        }
        {
            let mut tokenizer = Tokenizer::new(">");
            assert_eq!(tokenizer.next(), Some(Token::Gt(Attr::point(1, 1))));
        }
        {
            let mut tokenizer = Tokenizer::new(">>");
            assert_eq!(
                tokenizer.next(),
                Some(Token::RShift(Attr::range(1, 1, 1, 2)))
            );
        }
        {
            let mut tokenizer = Tokenizer::new(">=");
            assert_eq!(tokenizer.next(), Some(Token::Gte(Attr::range(1, 1, 1, 2))));
        }
        {
            let mut tokenizer = Tokenizer::new("!");
            assert_eq!(tokenizer.next(), Some(Token::Not(Attr::point(1, 1))));
        }
        {
            let mut tokenizer = Tokenizer::new("(");
            assert_eq!(tokenizer.next(), Some(Token::LParen(Attr::point(1, 1))));
        }
        {
            let mut tokenizer = Tokenizer::new(")");
            assert_eq!(tokenizer.next(), Some(Token::RParen(Attr::point(1, 1))));
        }
        {
            let mut tokenizer = Tokenizer::new("[");
            assert_eq!(tokenizer.next(), Some(Token::LBraket(Attr::point(1, 1))));
        }
        {
            let mut tokenizer = Tokenizer::new("]");
            assert_eq!(tokenizer.next(), Some(Token::RBraket(Attr::point(1, 1))));
        }
        {
            let mut tokenizer = Tokenizer::new("{");
            assert_eq!(tokenizer.next(), Some(Token::LBrace(Attr::point(1, 1))));
        }
        {
            let mut tokenizer = Tokenizer::new("}");
            assert_eq!(tokenizer.next(), Some(Token::RBrace(Attr::point(1, 1))));
        }
        {
            let mut tokenizer = Tokenizer::new("?");
            assert_eq!(
                tokenizer.next(),
                Some(Token::QuestionMark(Attr::point(1, 1)))
            );
        }
        {
            let mut tokenizer = Tokenizer::new("?");
            assert_eq!(
                tokenizer.next(),
                Some(Token::QuestionMark(Attr::point(1, 1)))
            );
        }
        {
            let mut tokenizer = Tokenizer::new("#");
            assert_eq!(tokenizer.next(), Some(Token::Hash(Attr::point(1, 1))));
        }
    }

    #[test]
    pub fn test_keywords() {
        let keywords = vec!["typedef", "struct", "enum", "if", "else", "while", "return"];
        for (i, keyword) in keywords.iter().enumerate() {
            let mut tokenizer = Tokenizer::new(&keyword);
            assert_eq!(
                tokenizer.next(),
                Some(Token::ID(
                    Attr::range(1, 1, 1, keyword.len() as u8),
                    String::from(keywords[i])
                ))
            );
        }
    }
}
