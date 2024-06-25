mod engine;

use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use engine::Engine;

pub struct Application {
    window: Option<Arc<Window>>,
    engine: Option<Engine>,
}

impl Application {
    pub fn new() -> Self {
        Self {
            window: None,
            engine: None,
        }
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop
                .create_window(Window::default_attributes().with_title("Scop"))
                .expect("Couldn't create window");
            self.window = Some(Arc::new(window));
        }
        if self.engine.is_none() {
            if let Some(window) = self.window.as_ref() {
                let engine = Engine::new(event_loop, window);
                self.engine = Some(engine)
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("WINDOW_EVENT: CloseRequested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}
