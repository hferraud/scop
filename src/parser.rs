use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
// use crate::object::{Object, Vertex, Face};

pub fn parser(path: &str) -> Result<(), io::Error> {
    let lines = read_lines(path)?;

    for line in lines.flatten() {
        parse_line(line)?
    }
    Ok(())
}

pub fn read_lines(path: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

pub fn parse_line(line: String) -> Result<(), io::Error>{
    let mut tokens = line.split(' ');
    if let Some(id) = tokens.next() {
        println!("token: {}", id);
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid line"))
    }
    Ok(())
}