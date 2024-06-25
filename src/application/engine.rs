use std::collections::HashSet;
use std::sync::Arc;
use winit::{
    window::{
        Window,
    },
    event_loop::{
        ActiveEventLoop,
    },
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
        DeviceCreateInfo,
        DeviceExtensions,
        Features,
        Queue,
        QueueCreateInfo,
        QueueFlags,
        physical::PhysicalDevice,
    },
    swapchain::Surface,
    Version,
    VulkanLibrary
};

const VALIDATION_LAYERS: &[&str] = &[
    "VK_LAYER_KHRONOS_validation",
];

#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

impl QueueFamilyIndices {
    fn new() -> Self {
        Self {
            graphics_family: None,
            present_family: None
        }
    }

    fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }

    fn set(&self) -> Option<HashSet<u32>> {
        if !self.is_complete() {
            return None;
        }
        let indices = [
            self.graphics_family.unwrap(),
            self.present_family.unwrap(),
        ];
        let set = HashSet::from_iter(indices.into_iter());
        Some(set)
    }
}

pub struct Engine {
    instance: Arc<Instance>,
    surface: Arc<Surface>,
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    present_queue: Arc<Queue>,
    debug_messenger: Option<DebugUtilsMessenger>,
}

impl Engine {
    pub fn new(event_loop: &ActiveEventLoop, window: &Arc<Window>) -> Self {
        let instance = Self::create_instance(&event_loop);
        let debug_messenger = Self::debug_messenger(Arc::clone(&instance));
        let surface = Self::create_surface(&instance, window);
        let physical_device = Self::pick_physical_device(&instance, &surface);
        let (
            device,
            graphics_queue,
            present_queue,
        ) = Self::create_logical_device(&physical_device, &surface);
        Self {
            instance,
            surface,
            physical_device,
            device,
            graphics_queue,
            present_queue,
            debug_messenger,
        }
    }

    fn create_instance(event_loop: &ActiveEventLoop) -> Arc<Instance> {
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

    fn pick_physical_device(instance: &Arc<Instance>, surface: &Arc<Surface>) -> Arc<PhysicalDevice> {
        instance.enumerate_physical_devices()
            .expect("Failed to enumerate physical devices")
            .find(|device| Self::check_device(device, surface))
            .expect("Failed to pick a physical device")
    }

    fn check_device(device: &PhysicalDevice, surface: &Arc<Surface>) -> bool {
        Self::find_queue_families(device, surface).is_complete()
    }

    fn find_queue_families(device: &PhysicalDevice, surface: &Arc<Surface>) -> QueueFamilyIndices {
        let mut indices = QueueFamilyIndices::new();
        let queue_families = device.queue_family_properties();
        for (i, queue_family) in queue_families.iter().enumerate() {
            if queue_family.queue_flags.contains(QueueFlags::GRAPHICS) {
                indices.graphics_family = Some(i as u32);
            }
            if device.surface_support(i as u32, surface).unwrap() {
                indices.present_family = Some(i as u32);
            }
            if indices.is_complete() {
                break;
            }
        }
        indices
    }

    fn create_logical_device(
        physical_device: &Arc<PhysicalDevice>,
        surface: &Arc<Surface>
    ) -> (Arc<Device>, Arc<Queue>, Arc<Queue>) {
        let mut indices = Self::find_queue_families(&physical_device, surface);
        let unique_indices = indices.set()
            .expect("Queue families indices are incomplete at logical device creation");
        let queue_create_infos = unique_indices.iter().map(|i| {
           QueueCreateInfo {
               queue_family_index: *i,
               ..Default::default()
           }
        }).collect();
        let device_create_info = DeviceCreateInfo {
            queue_create_infos,
            enabled_extensions: DeviceExtensions::empty(),
            enabled_features: Features::empty(),
            ..Default::default()
        };
        let (device, mut queues) = Device::new(
            Arc::clone(physical_device),
            device_create_info,
        )
            .expect("Failed to create logical device");
        let graphics_queue = queues.next()
            .expect("Failed to create graphics queue");
        let present_queue = queues.next()
            .unwrap_or_else(|| graphics_queue.clone());
        (device, graphics_queue, present_queue)
    }

    fn create_surface(instance: &Arc<Instance>, window: &Arc<Window>) -> Arc<Surface> {
        Surface::from_window(Arc::clone(&instance), Arc::clone(&window))
            .expect("Failed to create surface from window")
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

    fn required_extensions(event_loop: &ActiveEventLoop) -> InstanceExtensions {
        let mut extensions = Surface::required_extensions(&event_loop);
        if ENABLE_VALIDATION_LAYERS {
            extensions.ext_debug_utils = true;
        }
        extensions
    }

    fn debug_messenger(instance: Arc<Instance>) -> Option<DebugUtilsMessenger> {
        if !ENABLE_VALIDATION_LAYERS {
            return None;
        }
        let create_info = DebugUtilsMessengerCreateInfo::user_callback(
            unsafe {
                DebugUtilsMessengerCallback::new(
                    |severity, message_type, data| {
                        println!("{:?}:\n{:?}", severity, data.message);
                    }
                )
            }
        );
        DebugUtilsMessenger::new(instance, create_info).ok()
    }
}
