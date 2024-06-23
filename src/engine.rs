use std::sync::Arc;
use winit::{
    window::{
        Window,
        WindowId,
    },
    event::WindowEvent,
    event_loop::{
        EventLoop,
        ActiveEventLoop,
    },
    application::ApplicationHandler,
};

use vulkano::{
    instance::{
        Instance,
        InstanceCreateInfo,
        InstanceExtensions,
        debug::{
            DebugUtilsMessenger,
            DebugUtilsMessengerCallback,
            DebugUtilsMessengerCreateInfo,
        }
    },
    device::{
        Device,
        Queue,
        QueueFlags,
        physical::PhysicalDevice,
    },
    swapchain::Surface,
    Version,
    VulkanLibrary
};
use vulkano::device::{DeviceCreateInfo, DeviceExtensions, Features, QueueCreateInfo};

const VALIDATION_LAYERS: &[&str] = &[
    "VK_LAYER_KHRONOS_validation",
];

#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

struct QueueFamilyIndices {
    graphics_family: Option<u32>
}

impl QueueFamilyIndices {
    fn new() -> Self {
        Self {
            graphics_family: None,
        }
    }

    fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }
}

pub struct Engine {
    instance: Arc<Instance>,
    surface: Arc<Surface>,
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    window: Option<Window>,
    debug_messenger: Option<DebugUtilsMessenger>,
}

impl Engine {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let instance = Self::create_instance(&event_loop);
        let debug_messenger = unsafe {
            Self::debug_messenger(Arc::clone(&instance))
        };
        let physical_device = Self::pick_physical_device(&instance);
        let (device, graphics_queue) = Self::create_logical_device(&instance, Arc::clone(&physical_device));
        Self {
            instance,
            physical_device,
            device,
            graphics_queue,
            window: None,
            debug_messenger,
        }
    }

    fn create_instance(event_loop: &EventLoop<()>) -> Arc<Instance> {
        let library = VulkanLibrary::new()
            .unwrap_or_else(|e| panic!("Couldn't load Vulkan library: {:?}", e));
        if ENABLE_VALIDATION_LAYERS && !Self::check_validation_layers(&library) {
            panic!("Validation layers unavailable:\n{:?}", VALIDATION_LAYERS)
        }
        let extensions = Self::required_extensions(event_loop);
        let create_info = InstanceCreateInfo{
            application_name: Some("Scop".into()),
            application_version: Version { major: 1, minor: 0, patch: 0 },
            engine_name: Some("No Engine".into()),
            engine_version: Version { major: 1, minor: 0, patch: 0 },
            enabled_extensions: extensions,
            enabled_layers: if ENABLE_VALIDATION_LAYERS {
                VALIDATION_LAYERS.iter().cloned().map(String::from).collect()
            } else {
                Default::default()
            },
            ..Default::default()
        };
        Instance::new(library, create_info)
            .expect("Failed to create Vulkan instance")
    }

    fn pick_physical_device(instance: &Arc<Instance>) -> Arc<PhysicalDevice> {
        instance.enumerate_physical_devices()
            .expect("Failed to enumerate physical devices")
            .find(|device| Self::check_device(device))
            .expect("Failed to pick a physical device")
    }

    fn check_device(device: &PhysicalDevice) -> bool {
        Self::find_queue_families(device).is_complete()
    }

    fn find_queue_families(device: &PhysicalDevice) -> QueueFamilyIndices {
        let mut indices = QueueFamilyIndices::new();
        let queue_families = device.queue_family_properties();
        for (i, queue_family) in queue_families.iter().enumerate() {
            if queue_family.queue_flags.contains(QueueFlags::GRAPHICS) {
                indices.graphics_family = Some(i as u32);
            }
            if indices.is_complete() {
                break;
            }
        }
        indices
    }

    fn create_logical_device(
        instance: &Arc<Instance>, physical_device: Arc<PhysicalDevice>
    ) -> (Arc<Device>, Arc<Queue>) {
        let indices = Self::find_queue_families(&physical_device);
        let device_create_info = DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index: indices.graphics_family.unwrap(),
                ..Default::default()
            }],
            enabled_extensions: DeviceExtensions::empty(),
            enabled_features: Features::empty(),
            ..Default::default()
        };
        let (device, mut queues) = Device::new(physical_device, device_create_info)
            .expect("Failed to create logical device");
        let graphics_queue = queues.next()
            .expect("Failed to create graphics queue");
        (device, graphics_queue)
    }

    fn check_validation_layers(library: &Arc<VulkanLibrary>) -> bool {
        let layer_properties = library.layer_properties()
            .expect("Couldn't retrieve layers properties");
        let available_layers: Vec<_> = layer_properties
            .map(|layer| layer.name().to_owned())
            .collect();
        VALIDATION_LAYERS.iter()
            .all(|layer| available_layers.contains(&layer.to_string()))
    }

    fn required_extensions(event_loop: &EventLoop<()>) -> InstanceExtensions {
        let mut extensions = Surface::required_extensions(&event_loop);
        if ENABLE_VALIDATION_LAYERS {
            extensions.ext_debug_utils = true;
        }
        extensions
    }

    unsafe fn debug_messenger(instance: Arc<Instance>) -> Option<DebugUtilsMessenger> {
        if !ENABLE_VALIDATION_LAYERS {
            return None;
        }
        DebugUtilsMessenger::new(
            instance,
            DebugUtilsMessengerCreateInfo::user_callback(
                DebugUtilsMessengerCallback::new(|severity, message_type, data| {
                    println!("Debug callback: {:?}", data.message);
                }),
            ),
        ).ok()
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .expect("Couldn't create window");
        self.window = Some(window);
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