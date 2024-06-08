mod error;
mod object;
mod parser;
mod engine;

use std::io;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<(), io::Error> {
    // let obj = parser::parse("resources/42.obj")?;
    // dbg!(obj);
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut engine = engine::Engine::new();
    event_loop.run_app(&mut engine).unwrap();
    Ok(())
}
