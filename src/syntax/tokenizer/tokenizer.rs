use super::{attr::Attr, token::Token};

pub struct Tokenizer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    row: u32,
    col: u8,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            chars: input.chars().peekable(),
            row: 1,
            col: 0,
        }
    }

    fn with_attr(&self) -> Attr {
        Attr::new(self.row, self.col)
    }

    fn parse_number(&mut self, c: char) -> Token {
        let mut attr = self.with_attr();
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
        attr.set_end(self.row, self.col);
        Token::Int(attr, v as i32)
    }

    fn parse_identifier(&mut self, c: char) -> Token {
        let mut attr = self.with_attr();
        let mut v = String::new();
        v.push(c);
        while let Some(&c1) = self.chars.peek() {
            match c1 {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    self.chars.next();
                    self.col += 1;
                    v.push(c1.clone());
                }
                _ => {
                    break;
                }
            }
        }
        attr.set_end(self.row, self.col);

        match v.as_str() {
            "struct" => Token::Struct(attr),
            "enum" => Token::Enum(attr),
            "typedef" => Token::Typedef(attr),
            "if" => Token::If(attr),
            "else" => Token::Else(attr),
            "while" => Token::While(attr),
            "return" => Token::Return(attr),
            _ => {
                if v.starts_with('#') {
                    return Token::Directive(attr, v.to_string());
                }
                return Token::String(attr, v);
            }
        }
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
        while let Some(c) = self.chars.next() {
            if self.is_newline(c) {
                self.row += 1;
                self.col = 0;
                continue;
            }
            self.col += 1;
            if !c.is_whitespace() {
                return Some(match c {
                    '0'..='9' => self.parse_number(c),
                    'a'..='z' | 'A'..='Z' | '#' => self.parse_identifier(c),
                    '+' => {
                        if let Some(c1) = self.chars.peek() {
                            match c1 {
                                '+' => {
                                    self.chars.next();
                                    Token::Incr(self.with_attr())
                                }
                                _ => Token::Plus(self.with_attr()),
                            }
                        } else {
                            Token::Plus(self.with_attr())
                        }
                    }
                    '<' => {
                        if let Some(c1) = self.chars.peek() {
                            match c1 {
                                '=' => {
                                    self.chars.next();
                                    Token::Lte(self.with_attr())
                                }
                                '<' => {
                                    self.chars.next();
                                    Token::LShift(self.with_attr())
                                }
                                _ => Token::Lt(self.with_attr()),
                            }
                        } else {
                            Token::Lt(self.with_attr())
                        }
                    }
                    '>' => {
                        if let Some(c1) = self.chars.peek() {
                            match c1 {
                                '=' => {
                                    self.chars.next();
                                    Token::Gte(self.with_attr())
                                }
                                '>' => {
                                    self.chars.next();
                                    Token::RShift(self.with_attr())
                                }
                                _ => Token::Gt(self.with_attr()),
                            }
                        } else {
                            Token::Lt(self.with_attr())
                        }
                    }
                    '-' => Token::Minus(self.with_attr()),
                    '*' => Token::Mul(self.with_attr()),
                    '/' => Token::Div(self.with_attr()),
                    '(' => Token::LParen(self.with_attr()),
                    ')' => Token::RParen(self.with_attr()),
                    '{' => Token::LBrace(self.with_attr()),
                    '}' => Token::RBrace(self.with_attr()),
                    '[' => Token::LBraket(self.with_attr()),
                    ']' => Token::RBraket(self.with_attr()),
                    '"' => Token::Quote(self.with_attr()),
                    '\'' => Token::SingleQuote(self.with_attr()),
                    '.' => Token::Dot(self.with_attr()),
                    _ => panic!("not supported yet"),
                });
            }
        }

        Some(Token::EOF)
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::tokenizer::{attr::Attr, token::Token};

    use super::Tokenizer;

    #[test]
    pub fn test_tokenizer() {
        {
            let mut tokenizer = Tokenizer::new(
                "+ - * \n/ ( ) [ ] \n{ } 123 \" ' abc def . if else #include < <= > >= typedef struct enum",
            );
            assert_eq!(tokenizer.next(), Some(Token::Plus(Attr::new(1, 1))));
            assert_eq!(tokenizer.next(), Some(Token::Minus(Attr::new(1, 3))));
            assert_eq!(tokenizer.next(), Some(Token::Mul(Attr::new(1, 5))));
            assert_eq!(tokenizer.next(), Some(Token::Div(Attr::new(2, 1))));
            assert_eq!(tokenizer.next(), Some(Token::LParen(Attr::new(2, 3))));
            assert_eq!(tokenizer.next(), Some(Token::RParen(Attr::new(2, 5))));
            assert_eq!(tokenizer.next(), Some(Token::LBraket(Attr::new(2, 7))));
            assert_eq!(tokenizer.next(), Some(Token::RBraket(Attr::new(2, 9))));
            assert_eq!(tokenizer.next(), Some(Token::LBrace(Attr::new(3, 1))));
            assert_eq!(tokenizer.next(), Some(Token::RBrace(Attr::new(3, 3))));
            assert_eq!(tokenizer.next(), Some(Token::Int(Attr::new(3, 5), 123)));
            assert_eq!(tokenizer.next(), Some(Token::Quote(Attr::new(3, 7))));
            assert_eq!(tokenizer.next(), Some(Token::SingleQuote(Attr::new(3, 9))));
            assert_eq!(
                tokenizer.next(),
                Some(Token::String(
                    Attr::range(3, 11, 3, 13,),
                    String::from("abc")
                ))
            );
            assert_eq!(
                tokenizer.next(),
                Some(Token::String(
                    Attr::range(3, 15, 3, 17,),
                    String::from("def")
                ))
            );
            assert_eq!(tokenizer.next(), Some(Token::Dot(Attr::new(3, 19))));
            assert_eq!(tokenizer.next(), Some(Token::If(Attr::range(3, 21, 3, 22))));
            assert_eq!(
                tokenizer.next(),
                Some(Token::Else(Attr::range(3, 24, 3, 27)))
            );
            assert_eq!(
                tokenizer.next(),
                Some(Token::Directive(
                    Attr::range(3, 29, 3, 36),
                    String::from("#include")
                ))
            );
            //     assert_eq!(tokenizer.next(), Some(Token::Lt));
            //     assert_eq!(tokenizer.next(), Some(Token::Lte));
            //     assert_eq!(tokenizer.next(), Some(Token::Gt));
            //     assert_eq!(tokenizer.next(), Some(Token::Gte));
            //     assert_eq!(tokenizer.next(), Some(Token::Typedef));
            //     assert_eq!(tokenizer.next(), Some(Token::Struct));
            //     assert_eq!(tokenizer.next(), Some(Token::Enum));
            //     assert_eq!(tokenizer.next(), Some(Token::EOF));
        }
        // {
        //     let mut tokenizer = Tokenizer::new("+ ++");
        //     assert_eq!(tokenizer.next(), Some(Token::Plus));
        //     assert_eq!(tokenizer.next(), Some(Token::Incr));
        //     assert_eq!(tokenizer.next(), Some(Token::EOF));
        // }
    }
}
