pub mod environment;
pub mod read;

#[derive(Debug, PartialEq, Clone)]
pub enum MalDataType {
    Nil,
    Boolean(bool),
    Int(usize),
    String(String),
    Keyword(String),
    Vector(Vec<MalToken>),
    List(Vec<MalToken>),
    Symbol(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum MalToken {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Data(MalDataType),
}
