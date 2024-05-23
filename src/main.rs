mod error;
mod object;
mod parser;

use std::io;

fn main() -> Result<(), io::Error> {
    let obj = parser::parse("resources/42.obj")?;
    dbg!(obj);
    Ok(())
}
