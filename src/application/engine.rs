extern crate vulkano;

use ahash::HashSet;
use std::sync::Arc;
use winit::{
    window::{
        Window,
    },
    event_loop::{
        ActiveEventLoop,
    },
};

use vulkano::{instance::{
    Instance,
    InstanceCreateInfo,
    InstanceExtensions,
    debug::{
        DebugUtilsMessenger,
        DebugUtilsMessengerCallback,
        DebugUtilsMessengerCreateInfo,
    }
}, device::{
    Device,
    DeviceCreateInfo,
    DeviceExtensions,
    Features,
    Queue,
    QueueCreateInfo,
    QueueFlags,
    physical::PhysicalDevice,
}, swapchain::Surface, Version, VulkanLibrary};
use vulkano::format::Format;
use vulkano::image::{Image, ImageLayout, ImageUsage};
use vulkano::image::view::ImageView;
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::vertex_input::VertexInputState;
use vulkano::pipeline::graphics::viewport::{Scissor, Viewport, ViewportState};
use vulkano::pipeline::{DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::pipeline::graphics::color_blend::{ColorBlendAttachmentState, ColorBlendState};
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::{CullMode, FrontFace, RasterizationState};
use vulkano::pipeline::layout::{PipelineDescriptorSetLayoutCreateInfo};
use vulkano::render_pass::{AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp, Framebuffer, FramebufferCreateInfo, RenderPass, RenderPassCreateInfo, Subpass, SubpassDescription};
use vulkano::swapchain::{ColorSpace, CompositeAlpha, PresentMode, SurfaceCapabilities, Swapchain, SwapchainCreateInfo};
use vulkano::sync::Sharing;
use vulkano_shaders;
use crate::application::{HEIGHT, WIDTH};

const VALIDATION_LAYERS: &[&str] = &[
    "VK_LAYER_KHRONOS_validation",
];

const DEVICE_EXTENSIONS: DeviceExtensions = DeviceExtensions {
    khr_swapchain: true,
    ..DeviceExtensions::empty()
};

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

struct SwapChainSupport {
    capabilities: SurfaceCapabilities,
    formats: Vec<(Format, ColorSpace)>,
    present_modes: Vec<PresentMode>,
}

impl SwapChainSupport {
    fn new(physical_device: &PhysicalDevice, surface: &Surface) -> Self {
        let capabilities = physical_device.surface_capabilities(
            surface,
            Default::default(),
        ).expect("Failed to query physical device surface capabilities");
        let formats = physical_device.surface_formats(
            surface,
            Default::default(),
        ).expect("Failed to query physical device surface formats");
        let present_modes = physical_device.surface_present_modes(
            surface,
            Default::default(),
        ).expect("Failed to query physical device surface present modes").collect();
        Self {
            capabilities,
            formats,
            present_modes,
        }
    }
}

pub struct Engine {
    instance: Arc<Instance>,
    debug_messenger: Option<DebugUtilsMessenger>,
    surface: Arc<Surface>,
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    present_queue: Arc<Queue>,
    swap_chain: Arc<Swapchain>,
    swap_chain_images: Vec<Arc<Image>>,
    render_pass: Arc<RenderPass>,
    graphics_pipeline: Arc<GraphicsPipeline>,
    framebuffers: Vec<Arc<Framebuffer>>
}

impl Engine {
    pub fn new(event_loop: &ActiveEventLoop, window: &Arc<Window>) -> Self {
        let instance = Self::create_instance(&event_loop);
        let debug_messenger = Self::debug_messenger(instance.clone());
        let surface = Self::create_surface(&instance, window);
        let physical_device = Self::pick_physical_device(&instance, &surface);
        let (
            device,
            graphics_queue,
            present_queue,
        ) = Self::create_logical_device(&physical_device, &surface);
        let (swap_chain, swap_chain_images) = Self::create_swap_chain(
            &physical_device,
            &surface,
            &device,
        );
        let render_pass = Self::create_render_pass(&device, &swap_chain);
        let graphics_pipeline = Self::create_graphics_pipeline(
            &device,
            swap_chain.image_extent(),
            &render_pass,
        );
        let framebuffers = Self::create_framebuffers(
            &render_pass,
            &swap_chain_images,
            swap_chain.image_extent(),
        );
        Self {
            instance,
            debug_messenger,
            surface,
            physical_device,
            device,
            graphics_queue,
            present_queue,
            swap_chain,
            swap_chain_images,
            render_pass,
            graphics_pipeline,
            framebuffers,
        }
    }

    // INSTANCE CREATION

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

    fn required_extensions(event_loop: &ActiveEventLoop) -> InstanceExtensions {
        let mut extensions = Surface::required_extensions(&event_loop);
        if ENABLE_VALIDATION_LAYERS {
            extensions.ext_debug_utils = true;
        }
        extensions
    }

    // SURFACE CREATION

    fn create_surface(instance: &Arc<Instance>, window: &Arc<Window>) -> Arc<Surface> {
        Surface::from_window(Arc::clone(&instance), Arc::clone(&window))
            .expect("Failed to create surface from window")
    }

    // PHYSICAL DEVICE SELECTION

    fn pick_physical_device(instance: &Arc<Instance>, surface: &Arc<Surface>) -> Arc<PhysicalDevice> {
        instance.enumerate_physical_devices()
            .expect("Failed to enumerate physical devices")
            .find(|device| Self::check_device(device, surface))
            .expect("Failed to pick a physical device")
    }

    fn check_device(device: &PhysicalDevice, surface: &Arc<Surface>) -> bool {
        let indices = Self::find_queue_families(device, surface);
        let extension_supported = Self::check_device_extensions(device);
        if !extension_supported {
            return false;
        }
        let swap_chain_supported = Self::check_swap_chain_support(device, surface);
        indices.is_complete() && extension_supported && swap_chain_supported
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

    fn check_device_extensions(device: &PhysicalDevice) -> bool {
        let supported_extension = device.supported_extensions();
        supported_extension.contains(&DEVICE_EXTENSIONS)
    }

    fn check_swap_chain_support(device: &PhysicalDevice, surface: &Surface) -> bool {
        let swap_chain_support= SwapChainSupport::new(device, surface);
        !swap_chain_support.formats.is_empty() && !swap_chain_support.present_modes.is_empty()
    }

    // LOGICAL DEVICE CREATION

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
            enabled_extensions: DEVICE_EXTENSIONS,
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

    // SWAP CHAIN CREATION

    fn create_swap_chain(
        physical_device: &PhysicalDevice,
        surface: &Arc<Surface>,
        device: &Arc<Device>,
    ) -> (Arc<Swapchain>, Vec<Arc<Image>>){
        let swap_chain_support = SwapChainSupport::new(physical_device, surface);
        let surface_format = Self::select_swap_surface_format(&swap_chain_support.formats);
        let present_mode = Self::select_swap_present_mode(&swap_chain_support.present_modes);
        let swap_extent = Self::select_swap_extent(&swap_chain_support.capabilities);
        let min_image_count = match swap_chain_support.capabilities.max_image_count {
            None => 2.max(swap_chain_support.capabilities.min_image_count),
            Some(limit) => 2.clamp(swap_chain_support.capabilities.min_image_count, limit)
        };
        let indices = Self::find_queue_families(physical_device, surface);
        let sharing = if indices.graphics_family != indices.present_family {
            panic!("idk");
            //TODO: fix
        } else {
            Sharing::Exclusive
        };
        let create_info = SwapchainCreateInfo {
            min_image_count,
            image_format: surface_format.0,
            image_color_space: surface_format.1,
            image_extent: swap_extent,
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            image_sharing: sharing,
            pre_transform: swap_chain_support.capabilities.current_transform,
            composite_alpha: CompositeAlpha::Opaque,
            present_mode,
            clipped: true,
            ..Default::default()
        };
        Swapchain::new(
            device.clone(),
            surface.clone(),
            create_info,
        ).expect("Failed to create swap chain")
    }

    fn select_swap_surface_format(formats: &Vec<(Format, ColorSpace)>) -> (Format, ColorSpace) {
        *formats.iter().find(|(format, color_space)| {
            *format == Format::B8G8R8A8_SRGB && *color_space == ColorSpace::SrgbNonLinear
        }).unwrap_or_else(|| &formats[0])
    }

    fn select_swap_present_mode(present_modes: &Vec<PresentMode>) -> PresentMode {
        if present_modes.contains(&PresentMode::Mailbox) {
            PresentMode::Mailbox
        } else {
            PresentMode::Fifo
        }
    }

    fn select_swap_extent(capabilities: &SurfaceCapabilities) -> [u32; 2] {
        if let Some(current_extent) = capabilities.current_extent {
            current_extent
        } else {
            let mut actual_extent = [WIDTH, HEIGHT];
            actual_extent[0] = actual_extent[0].clamp(
                capabilities.min_image_extent[0],
                capabilities.max_image_extent[0],
            );
            actual_extent[1] = actual_extent[1].clamp(
                capabilities.min_image_extent[1],
                capabilities.max_image_extent[1],
            );
            actual_extent
        }
    }

    // RENDER PASS

    fn create_render_pass(device: &Arc<Device>, swap_chain: &Arc<Swapchain>) -> Arc<RenderPass>{
        let color_attachments = vec![
            AttachmentDescription {
                format: swap_chain.image_format(),
                load_op: AttachmentLoadOp::Clear,
                store_op: AttachmentStoreOp::Store,
                final_layout: ImageLayout::PresentSrc,
                ..Default::default()
            }
        ];
        let color_attachments_ref = AttachmentReference {
            attachment: 0,
            layout: ImageLayout::ColorAttachmentOptimal,
            ..Default::default()
        };
        let subpasses = vec![
            SubpassDescription {
                color_attachments: vec![Some(color_attachments_ref)],
                ..Default::default()
            }
        ];
        let create_info = RenderPassCreateInfo {
            attachments: color_attachments,
            subpasses,
            ..Default::default()
        };
        RenderPass::new(
            device.clone(),
            create_info,
        ).expect("Failed to create render pass")
    }

    // GRAPHIC PIPELINE

    fn create_graphics_pipeline(
        device: &Arc<Device>,
        image_extent: [u32; 2],
        render_pass: &Arc<RenderPass>,
    ) -> Arc<GraphicsPipeline> {
        mod vertex_shader {
            vulkano_shaders::shader! {
                ty: "vertex",
                path: "src/shaders/shader.vert"
            }
        }

        mod fragment_shader {
            vulkano_shaders::shader! {
                ty: "fragment",
                path: "src/shaders/shader.frag"
            }
        }

        let vert_shader_module = vertex_shader::load(device.clone())
            .expect("Failed to create vertex shader module")
            .entry_point("main")
            .expect("Failed to set vertex shader entrypoint");
        let frag_shader_module = fragment_shader::load(device.clone())
            .expect("Failed to create fragment shader module")
            .entry_point("main")
            .expect("Failed to set fragment shader entrypoint");

        let stages = [
            PipelineShaderStageCreateInfo::new(vert_shader_module),
            PipelineShaderStageCreateInfo::new(frag_shader_module),
        ];
        let viewport_state = Some(ViewportState {
            viewports: [Viewport {
                extent: [image_extent[0] as f32, image_extent[1] as f32],
                ..Default::default()
            }].into(),
            scissors: [Scissor {
                extent: image_extent,
                ..Default::default()
            }].into(), //TODO: verify that this cannot be set to default
            ..Default::default()
        });
        let mut dynamic_state: HashSet<DynamicState> = Default::default();
        dynamic_state.insert(DynamicState::Viewport);
        dynamic_state.insert(DynamicState::Scissor);
        let rasterization_state = Some(RasterizationState {
            cull_mode: CullMode::Back,
            front_face: FrontFace::Clockwise,
            ..Default::default()
        });
        let multisample_state = Some(MultisampleState::default());
        let color_blend_state = Some(ColorBlendState {
            attachments: vec![ColorBlendAttachmentState::default()],
            ..Default::default()
        });
        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone())
                .expect("Failed to create a pipeline layout from stages")
        ).expect("Failed to create a pipeline layout");
        let subpass = Subpass::from(render_pass.clone(), 0)
            .expect("Failed to get subpass from render pass");
        let create_info = GraphicsPipelineCreateInfo {
            stages: stages.into_iter().collect(),
            vertex_input_state: Some(VertexInputState::new()),
            input_assembly_state: Some(InputAssemblyState::default()),
            tessellation_state: None,
            viewport_state,
            rasterization_state,
            multisample_state,
            depth_stencil_state: None,
            color_blend_state,
            dynamic_state,
            subpass: Some(subpass.into()),
            ..GraphicsPipelineCreateInfo::layout(layout)
        };
        GraphicsPipeline::new(
            device.clone(),
            None,
            create_info,
        ).expect("Failed to create graphic pipeline")
    }

    // FRAMEBUFFER

    fn create_framebuffers(
        render_pass: &Arc<RenderPass>,
        swap_chain_images: &Vec<Arc<Image>>,
        image_extent: [u32; 2],
    ) -> Vec<Arc<Framebuffer>> {
        swap_chain_images.iter().map(|image| {
            let view = ImageView::new_default(image.clone())
                .expect("Failed to create image view");
            let create_info = FramebufferCreateInfo {
                attachments: vec![view],
                extent: image_extent,
                layers: 1,
                ..Default::default()
            };
            Framebuffer::new(
                render_pass.clone(),
                create_info,
            ).expect("Failed to create framebuffer")
        }).collect::<Vec<_>>()
    }

    // VALIDATION LAYERS

    fn check_validation_layers(library: &Arc<VulkanLibrary>) -> bool {
        let layer_properties = library.layer_properties()
            .expect("Couldn't retrieve layers properties");
        let available_layers: Vec<_> = layer_properties
            .map(|layer| layer.name().to_owned())
            .collect();
        VALIDATION_LAYERS.iter()
            .all(|layer| available_layers.contains(&layer.to_string()))
    }

    fn debug_messenger(instance: Arc<Instance>) -> Option<DebugUtilsMessenger> {
        if !ENABLE_VALIDATION_LAYERS {
            return None;
        }
        let create_info = DebugUtilsMessengerCreateInfo::user_callback(
            unsafe {
                DebugUtilsMessengerCallback::new(
                    |severity, _message_type, data| {
                        println!("{:?}:\n{:?}", severity, data.message);
                    }
                )
            }
        );
        DebugUtilsMessenger::new(instance, create_info).ok()
    }
}
