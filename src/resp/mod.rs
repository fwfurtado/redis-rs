use std::fmt::{Display, Formatter};

pub mod read;
pub mod write;

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    NullArray,
    String(String),
    Error(String),
    Integer(i64),
    Array(Vec<Value>),
    Bulk(String),
}


#[derive(Debug)]
pub enum Error {
    InvalidInput(&'static str),
    InvalidData(&'static str),
    UnexpectedEof,
    Io(&'static str),
    BufReadError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
            Error::InvalidData(msg) => write!(f, "Invalid Data: {}", msg),
            Error::UnexpectedEof => write!(f, "Unexpected EOF"),
            Error::Io(msg) => write!(f, "IO Error: {}", msg),
            Error::BufReadError => write!(f, "BufRead Error"),
        }
    }
}
