use std::io;

use crate::parser::Token;

pub fn custom(e: impl ToString) -> io::Error {
    io::Error::new(
        io::ErrorKind::Other,
        e.to_string(),
    )
}

pub fn invalid_token(token: &Token) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("Invalid token '{:?}'", token),
    )
}

pub fn unexpected_token(token: &Token) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, format!("Found unexpected token '{:?}'", token))
}

pub fn invalid_statement() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, "Invalid statement")
}

pub fn non_value_token() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, "Non value token")
}

pub fn invalid_argument_length(expected: impl std::fmt::Display, received: usize)  -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("Expected {expected}, received {received}")
    )
}

pub fn index_out_of_bound(index: usize, len: usize) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("Trying to access an index of {index} when array is of len {len}")
    )
}