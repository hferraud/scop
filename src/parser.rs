mod token;
mod statement;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
pub use token::Token;
use crate::object::Object;
use crate::parser::token::RemoveComment;

pub fn parse(path: &str) -> Result<Object, io::Error> {
    let mut object= Object::new();
    let lines = file_read_lines(path)?;

    for line in lines.flatten() {
        line_parse(line, &mut object)?;
    }
    Ok(object)
}

fn file_read_lines(path: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

fn line_parse(line: String, object: &mut Object) -> Result<(), io::Error> {
    let mut tokens = Token::lex(&line);

    tokens.remove_comment();
    statement::statement_router(tokens, object)?;
    Ok(())
}
