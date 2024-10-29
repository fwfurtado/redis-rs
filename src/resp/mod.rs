use std::fmt::{Display, Formatter};

pub mod read;
pub mod write;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    NullArray,
    String(String),
    Error(String),
    Integer(i64),
    Array(Vec<Value>),
    Bulk(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::NullArray => write!(f, "NullArray"),
            Value::String(v) => write!(f, "String({})", v),
            Value::Error(e) => write!(f, "Error({})", e),
            Value::Integer(n) => write!(f, "Integer({})", n),
            Value::Array(xs) => write!(f, "Array({:?})", xs),
            Value::Bulk(s) => write!(f, "Bulk({})", s),
        }
    }
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
