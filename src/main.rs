mod error;
mod object;
mod parser;
mod engine;

use std::io;

fn main() -> Result<(), io::Error> {
    // let obj = parser::parse("resources/42.obj")?;
    // dbg!(obj);
    let engine = engine::Engine::new();
    engine.run();
    Ok(())
}
