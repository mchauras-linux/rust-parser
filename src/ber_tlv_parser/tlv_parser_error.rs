use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum TlvParserError {}

impl Display for TlvParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for TlvParserError {}
