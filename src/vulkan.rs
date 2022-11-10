extern crate image;
extern crate vulkano;
use super::data::*;
use image::DynamicImage;
use vulkano::memory::allocator::{StandardMemoryAllocator};
use std::{sync::Arc, time::*};
use vulkano::buffer::CpuBufferPool;
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassContents, allocator::StandardCommandBufferAllocator,
};
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{
    physical::PhysicalDeviceType, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo,
};
use vulkano::device::{Device, Queue};
use vulkano::image::{view::ImageView, ImageAccess, ImageUsage, SwapchainImage};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::pipeline::graphics::{
    input_assembly::InputAssemblyState, vertex_input::BuffersDefinition, viewport::Viewport,
    viewport::ViewportState,
};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::RenderPass;
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, Subpass};
use vulkano::shader::ShaderModule;
use vulkano::swapchain::{
    acquire_next_image, AcquireError, PresentMode, Surface, Swapchain,
    SwapchainCreateInfo, SwapchainCreationError, SwapchainPresentInfo,
};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};
use vulkano::{Version, library::VulkanLibrary};
use vulkano_win::VkSurfaceBuild;
use winit::dpi::LogicalSize;
use winit::{
    event_loop::EventLoop,
    window::{Fullscreen, Window, WindowBuilder},
};
mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/vert.glsl"
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/frag.glsl"
    }
}

const WIDTH: u16 = 1000;
const HEIGHT: u16 = 1000;
const TITLE: &str = "Let - Vulkan";

#[allow(unused)]
pub struct App {
    instance: Arc<Instance>,
    pub surface: Arc<Surface>,
    device_extensions: DeviceExtensions,
    physical_device: Arc<PhysicalDevice>,
    queue_family_index: u32,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    render_pass: Arc<RenderPass>,
    pipeline: Arc<GraphicsPipeline>,
    viewport: Viewport,
    framebuffers: Vec<Arc<Framebuffer>>,
    pub recreate_swapchain: bool,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    pub vertices: Vec<Vertex>,
    vertex_buffer: CpuBufferPool<Vertex>,
    pub dt1: f64,
    memoryallocator: Arc<StandardMemoryAllocator>,
    commandbufferallocator: StandardCommandBufferAllocator,
}

impl App {
    pub fn initialize() -> (Self, EventLoop<()>) {
        let instance = Self::create_instance();
        let (event_loop, surface) = Self::create_window(&instance);
        let device_extensions = Self::create_device_extensions();
        let (physical_device, queue_family_index) =
            Self::create_physical_and_queue(&instance, device_extensions, &surface);
        let (device, queue) = Self::create_device_and_queues(
            &physical_device,
            &device_extensions,
            queue_family_index,
        );
        let (swapchain, images) = Self::create_swapchain_and_images(&device, &surface);

        let vs = vs::load(device.clone()).unwrap();
        let fs = fs::load(device.clone()).unwrap();

        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        let render_pass = Self::create_render_pass(&device, &swapchain);
        let pipeline = Self::create_pipeline(&device, &render_pass, &vs, &fs);

        let mut viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [0.0, 0.0],
            depth_range: 0.0..1.0,
        };
        let framebuffers =
            Self::window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);
        let recreate_swapchain = false;
        let previous_frame_end = Some(sync::now(device.clone()).boxed());

        let vertices = vec![];
        let memoryallocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let commandbufferallocator = StandardCommandBufferAllocator::new(device.clone(), Default::default());
        let vertex_buffer = CpuBufferPool::vertex_buffer(memoryallocator.clone().into());
        let dt1 = unix_timestamp();
        (
            Self {
                instance,
                surface,
                device_extensions,
                physical_device,
                queue_family_index,
                device,
                queue,
                swapchain,
                images,
                vs,
                fs,
                render_pass,
                pipeline,
                viewport,
                framebuffers,
                recreate_swapchain,
                previous_frame_end,
                vertices,
                memoryallocator,
                commandbufferallocator,
                vertex_buffer,
                dt1,
            },
            event_loop,
        )
    }

    fn create_instance() -> Arc<Instance> {

        let library = match VulkanLibrary::new() {
            Err(_) => {
                println!("This PC does not support Vulkan.\nProgram can not be started.");
                std::process::exit(0);
            }
            Ok(a) => a,
        };
        let required_extensions = vulkano_win::required_extensions(&library);
        let gameinfo = InstanceCreateInfo {
            application_name: Some("Let Vulkan Test".into()),
            application_version: Version {
                major: (0),
                minor: (0),
                patch: (0),
            },
            enabled_extensions: required_extensions,
            engine_name: Some("LetsTestVulkanEngine".into()),
            engine_version: Version {
                major: (0),
                minor: (0),
                patch: (0),
            },
            ..Default::default()
        };
        Instance::new(library, gameinfo).expect("Couldn't start Vulkan.")
    }

    fn create_window(instance: &Arc<Instance>) -> (EventLoop<()>, Arc<Surface>) {
        let icon: DynamicImage =
            image::load_from_memory(include_bytes!("../handsomesquidward.bmp")).unwrap();
        let icondimension = (icon.height(), icon.width());
        let iconbytes: Vec<u8> = icon.into_rgba8().into_raw();
        let event_loop = winit::event_loop::EventLoopBuilder::new().build();
        let surface = WindowBuilder::new()
            .with_resizable(true)
            .with_title(TITLE)
            .with_min_inner_size(LogicalSize::new(200, 200))
            .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
            .with_window_icon(Some(
                winit::window::Icon::from_rgba(iconbytes, icondimension.1, icondimension.0)
                    .unwrap(),
            ))
            .with_always_on_top(true)
            .with_decorations(true)
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();
        //surface.window().set_fullscreen(Some(Fullscreen::Exclusive(MonitorHandle::video_modes(&surface.window().current_monitor().unwrap()).next().unwrap())));
        //surface.window().set_cursor_grab(winit::window::CursorGrabMode::Confined).unwrap();
        //surface.window().set_fullscreen(Some(Fullscreen::Borderless(surface.window().current_monitor())));
        //surface.window().set_cursor_visible(false);
        (event_loop, surface)
    }

    fn create_device_extensions() -> DeviceExtensions {
        DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        }
    }
    fn create_physical_and_queue(
        instance: &Arc<Instance>,
        device_extensions: DeviceExtensions,
        surface: &Arc<Surface>,
    ) -> (Arc<PhysicalDevice>, u32) {
        instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.graphics
                            && p.surface_support(i as u32, surface).unwrap_or(false)
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .expect("No suitable physical device found")
    }
    fn create_device_and_queues(
        physical_device: &Arc<PhysicalDevice>,
        device_extensions: &DeviceExtensions,
        queue_family_index: u32,
    ) -> (Arc<Device>, Arc<Queue>) {
        let (device, mut queues) = Device::new(
            // Which physical device to connect to.
            physical_device.clone(),
            DeviceCreateInfo {
                enabled_extensions: device_extensions.clone(),
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],

                ..Default::default()
            },
        )
        .unwrap();
        (device, queues.next().unwrap())
    }
    fn create_swapchain_and_images(
        device: &Arc<Device>,
        surface: &Arc<Surface>,
    ) -> (Arc<Swapchain>, Vec<Arc<SwapchainImage>>) {
        let surface_capabilities = device
            .physical_device()
            .surface_capabilities(&surface, Default::default())
            .unwrap();
        println!("{:?}", surface_capabilities.supported_composite_alpha);
        let image_format = Some(
            device
                .physical_device()
                .surface_formats(&surface, Default::default())
                .unwrap()[0]
                .0,
        );
        let image_usage = ImageUsage {
            color_attachment: true,
            ..ImageUsage::empty()
        };
        let innersize = surface.object().unwrap().downcast_ref::<Window>().unwrap().inner_size().into();
        let create_info = SwapchainCreateInfo {
            min_image_count: surface_capabilities.min_image_count,
            image_format,
            image_extent: innersize,
            image_usage,
            present_mode: PresentMode::Mailbox,
            composite_alpha: surface_capabilities
                .supported_composite_alpha
                .iter()
                .next()
                .unwrap(),

            ..Default::default()
        };

        let swapchain = match Swapchain::new(device.clone(), surface.clone(), create_info) {
            Ok(t) => t,
            Err(e) => {
                if e == SwapchainCreationError::PresentModeNotSupported {
                    let create_info = SwapchainCreateInfo {
                        min_image_count: surface_capabilities.min_image_count,
                        image_format,
                        image_extent: innersize,
                        image_usage,
                        present_mode: PresentMode::Immediate,
                        composite_alpha: surface_capabilities
                            .supported_composite_alpha
                            .iter()
                            .next()
                            .unwrap(),

                        ..Default::default()
                    };
                    match Swapchain::new(device.clone(), surface.clone(), create_info) {
                        Ok(t) => t,
                        Err(e) => {
                            if e == SwapchainCreationError::PresentModeNotSupported {
                                let create_info = SwapchainCreateInfo {
                                    min_image_count: surface_capabilities.min_image_count,
                                    image_format,
                                    image_extent: innersize,
                                    image_usage,
                                    present_mode: PresentMode::Fifo,
                                    composite_alpha: surface_capabilities
                                        .supported_composite_alpha
                                        .iter()
                                        .next()
                                        .unwrap(),

                                    ..Default::default()
                                };
                                Swapchain::new(device.clone(), surface.clone(), create_info)
                                    .unwrap()
                            } else {
                                panic!("{e}")
                            }
                        }
                    }
                } else {
                    panic!("{e}")
                }
            }
        };
        swapchain
    }
    fn create_render_pass(
        device: &Arc<Device>,
        swapchain: &Arc<Swapchain>,
    ) -> Arc<RenderPass> {
        vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.image_format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap()
    }
    fn create_pipeline(
        device: &Arc<Device>,
        render_pass: &Arc<RenderPass>,
        vs: &Arc<ShaderModule>,
        fs: &Arc<ShaderModule>,
    ) -> Arc<GraphicsPipeline> {
        GraphicsPipeline::start()
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
            .input_assembly_state(InputAssemblyState::new())
            .vertex_shader(vs.entry_point("main").unwrap(), ())
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(fs.entry_point("main").unwrap(), ())
            .build(device.clone())
            .unwrap()
    }
    fn window_size_dependent_setup(
        images: &[Arc<SwapchainImage>],
        render_pass: Arc<RenderPass>,
        viewport: &mut Viewport,
    ) -> Vec<Arc<Framebuffer>> {
        let dimensions = images[0].dimensions().width_height();
        viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

        images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>()
    }
    pub fn fullscreen(surface: Arc<Surface>) {
        let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();
        if window.fullscreen() == None {
            window.set_fullscreen(Some(Fullscreen::Borderless(
                window.current_monitor(),
            ))); //borderless
                 //surface.window().set_fullscreen(Some(Fullscreen::Exclusive(MonitorHandle::video_modes(&surface.window().current_monitor().unwrap()).next().unwrap()))); //exclusive
        } else {
            window.set_fullscreen(None)
        }
    }
    pub fn redrawevent(&mut self) {
        let window = self.surface.object().unwrap().downcast_ref::<Window>().unwrap();
        self.dt1 = unix_timestamp();
        let sub_buffer = self.vertex_buffer.from_iter(self.vertices.clone()).unwrap();
        let dimensions = window.inner_size();
        if dimensions.width == 0 || dimensions.height == 0 {
            return;
        }
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();
        if self.recreate_swapchain {
            let (new_swapchain, new_images) = match self.swapchain.recreate(SwapchainCreateInfo {
                image_extent: dimensions.into(),
                ..self.swapchain.create_info()
            }) {
                Ok(r) => r,
                Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
                Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
            };

            self.swapchain = new_swapchain;
            self.framebuffers = Self::window_size_dependent_setup(
                &new_images,
                self.render_pass.clone(),
                &mut self.viewport,
            );
            self.recreate_swapchain = false;
        }
        let (image_num, suboptimal, acquire_future) =
            match acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return;
                }
                Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };
        if suboptimal {
            self.recreate_swapchain = true;
        }
        let mut builder = AutoCommandBufferBuilder::primary(
            &self.commandbufferallocator,
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();
        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.0, 0.0, 0.0, 1.0].into())],
                    ..RenderPassBeginInfo::framebuffer(self.framebuffers[image_num as usize].clone())
                },
                SubpassContents::Inline,
            )
            .unwrap()
            .set_viewport(0, [self.viewport.clone()])
            .bind_pipeline_graphics(self.pipeline.clone())
            .bind_vertex_buffers(0, sub_buffer.clone())
            .draw(self.vertices.len() as u32, 1, 0, 0)
            .unwrap()
            .end_render_pass()
            .unwrap();
        let command_buffer = builder.build().unwrap();

        let future = self
            .previous_frame_end
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(
                self.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_num),
            )
            .then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                self.previous_frame_end = Some(future.boxed());
            }
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
            Err(e) => {
                println!("Failed to flush future: {:?}", e);
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
        }
    }
}

fn unix_timestamp() -> f64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
}
