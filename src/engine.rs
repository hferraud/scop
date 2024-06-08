use std::collections::HashSet;
use std::ffi::{c_char, CStr, CString};
use std::ptr;
use ash::{vk, Entry};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

const APPLICATION_NAME: &'static str = "Scop";
const APPLICATION_VERSION: u32 = vk::make_api_version(0, 1, 0, 0);
const ENGINE_NAME: &'static str = "Vulkan Engine";
const ENGINE_VERSION: u32 = vk::make_api_version(0, 1, 0, 0);
const API_VERSION: u32 = vk::make_api_version(0, 1, 0, 0);

const VALIDATION_LAYERS: &[&str] = &[
    "VK_LAYER_KHRONOS_validation",
];

pub struct Engine {
    window: Option<Window>,
    entry: Entry,
    instance: ash::Instance,
}

impl Engine {
    pub fn new() -> Self {
        let entry = Entry::linked();
        let instance = Self::create_instance(&entry);

        Engine {
            window: None,
            entry,
            instance,
        }
    }

    // INSTANCE CREATION
    fn create_instance(entry: &Entry) -> ash::Instance {
        if cfg!(debug_assertions) && !Self::check_validation_layer_support(entry) {
            panic!("Validation layers requested but not available");
        }
        let app_name = CString::new(APPLICATION_NAME).unwrap();
        let engine_name = CString::new(APPLICATION_NAME).unwrap();
        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: APPLICATION_VERSION,
            p_engine_name: engine_name.as_ptr(),
            engine_version: ENGINE_VERSION,
            api_version: API_VERSION,
            _marker: Default::default(),
        };
        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            ..Default::default()
        };
        unsafe {
            entry.create_instance(&create_info, None).unwrap()
        }
    }

    //VALIDATION LAYERS
    fn vk_to_string(raw_string: &[c_char]) -> String {
        let cstr = unsafe {
            CStr::from_ptr(raw_string.as_ptr())
        };
        cstr.to_str().expect("Failed to convert vulkan raw string").to_owned()
    }

    fn check_validation_layer_support(entry: &Entry) -> bool {
        let layer_properties = unsafe {
            entry.
            enumerate_instance_layer_properties().
            expect("Failed to enumerate instance layers properties")
        };
        let available_layers: HashSet<String> = layer_properties
            .iter()
            .map(|layer| Self::vk_to_string(&layer.layer_name))
            .collect();
        println!("Instance available layers:\n{:#?}", available_layers);
        for layer in VALIDATION_LAYERS {
            if !available_layers.contains(&String::from(*layer)) {
                return false
            }
        }
        true
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop
            .create_window(Window::default_attributes())
            .expect("Couldn't create window"));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("WINDOW_EVENT: CloseRequested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            _ => (),
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None)
        }
    }
}