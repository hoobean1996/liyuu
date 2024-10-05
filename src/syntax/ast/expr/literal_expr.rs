#[derive(PartialEq, Debug)]
pub enum Literal {
    Bool(bool),
    Int(i64),
    Char(char),
    String(String),
    ID(String),
}
