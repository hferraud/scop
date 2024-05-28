mod window_handler;

use ash::{vk, Entry};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::Window;
use crate::engine::window_handler::WindowHandler;

pub struct Engine {
    entry: Entry,
    instance: ash::Instance,
}

impl Engine {
    pub fn new() -> Self {
        let entry = Entry::linked();
        let instance = Self::create_instance(&entry);

        Engine {
            entry,
            instance,
        }
    }

    fn create_instance(entry: &Entry) -> ash::Instance {
        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
        };
        unsafe {
            entry.create_instance(&create_info, None).unwrap()
        }
    }

    fn init_window(event_loop: &ActiveEventLoop) -> Window {
        event_loop
            .create_window(Window::default_attributes())
            .expect("Couldn't create window")
    }

    pub fn run(&self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        let mut window = WindowHandler::default();
        event_loop.run_app(&mut window).unwrap();
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None)
        }
    }
}