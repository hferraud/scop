mod error;
mod object;
mod parser;
mod application;

use std::io;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<(), io::Error> {
    // let obj = parser::parse("resources/42.obj")?;
    // dbg!(obj);
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut application = application::Application::new();
    event_loop.run_app(&mut application).unwrap();
    Ok(())
}
