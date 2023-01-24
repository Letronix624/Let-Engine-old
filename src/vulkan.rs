extern crate image;
extern crate vulkano;
use super::data::*;
use crate::consts::*;
use crate::game::Object;
use image::DynamicImage;
use std::collections::HashMap;
use std::default;
use std::io::Cursor;
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuBufferPool};
use vulkano::command_buffer::{
    allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
    PrimaryCommandBufferAbstract, RenderPassBeginInfo, SubpassContents,
};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{
    physical::PhysicalDeviceType, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo,
};
use vulkano::device::{Device, Queue};
use vulkano::image::{view::ImageView, ImageAccess, ImageUsage, SwapchainImage};
use vulkano::image::{ImageDimensions, ImmutableImage, MipmapsCount};
use vulkano::instance::{debug::*, Instance, InstanceCreateInfo, InstanceExtensions};
use vulkano::memory::allocator::{MemoryUsage, StandardMemoryAllocator};
use vulkano::pipeline::graphics::color_blend::ColorBlendState;
use vulkano::pipeline::graphics::{
    input_assembly::InputAssemblyState, vertex_input::BuffersDefinition, viewport::Viewport,
    viewport::ViewportState,
};
use vulkano::pipeline::{GraphicsPipeline, Pipeline};
use vulkano::render_pass::RenderPass;
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, Subpass};
use vulkano::sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo};
use vulkano::shader::ShaderModule;
use vulkano::swapchain::{
    acquire_next_image, AcquireError, PresentMode, Surface, Swapchain, SwapchainCreateInfo,
    SwapchainCreationError, SwapchainPresentInfo,
};
use vulkano::sync::{self};
use vulkano::sync::{FlushError, GpuFuture};
use vulkano::{library::VulkanLibrary, Version};
use vulkano_win::VkSurfaceBuild;
use winit::dpi::LogicalSize;
use winit::{
    event_loop::EventLoop,
    window::{Fullscreen, Window, WindowBuilder},
};
mod vertexshader {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/vert.glsl",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        }
    }
}

mod fragmentshader {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/frag.glsl"
    }
}

#[allow(unused)]
pub struct App {
    instance: Arc<Instance>,
    debugmessenger: Option<DebugUtilsMessenger>,
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
    descriptors: [Arc<PersistentDescriptorSet>; 2],
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    pub objects: HashMap<String, Object>,
    pub render_order: Vec<String>,
    vertex_buffer: CpuBufferPool<Vertex>,
    object_buffer: CpuBufferPool<vertexshader::ty::Object>,
    index_buffer: CpuBufferPool<u16>,
    memoryallocator: Arc<StandardMemoryAllocator>,
    commandbufferallocator: StandardCommandBufferAllocator,
    descriptor_set_allocator: StandardDescriptorSetAllocator,
}

impl App {
    pub fn initialize() -> (Self, EventLoop<()>) {
        let instance = Self::create_instance();
        let debugmessenger = Self::setup_debug(&instance);
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

        let memoryallocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        let vertex_buffer: CpuBufferPool<Vertex> =
            CpuBufferPool::vertex_buffer(memoryallocator.clone().into());

        let object_buffer: CpuBufferPool<vertexshader::ty::Object> = CpuBufferPool::new(
            memoryallocator.clone().into(),
            BufferUsage {
                uniform_buffer: true,
                ..Default::default()
            },
            MemoryUsage::Upload,
        );

        let index_buffer: CpuBufferPool<u16> = CpuBufferPool::new(
            memoryallocator.clone().into(),
            BufferUsage {
                index_buffer: true,
                ..Default::default()
            },
            vulkano::memory::allocator::MemoryUsage::Upload,
        );

        let vs = vertexshader::load(device.clone()).unwrap();
        let fs = fragmentshader::load(device.clone()).unwrap();

        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        let render_pass = Self::create_render_pass(&device, &swapchain);
        let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());
        let commandbufferallocator =
            StandardCommandBufferAllocator::new(device.clone(), Default::default());
        let mut uploads = AutoCommandBufferBuilder::primary(
            &commandbufferallocator,
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        let texture = {
            let png_bytes = include_bytes!("../assets/textures/shidkitty69.png").to_vec();
            let cursor = Cursor::new(png_bytes);
            let decoder = png::Decoder::new(cursor);
            let mut reader = decoder.read_info().unwrap();
            let info = reader.info();
            let dimensions = ImageDimensions::Dim2d {
                width: info.width,
                height: info.height,
                array_layers: 1,
            };
            let mut image_data = Vec::new();
            image_data.resize((info.width * info.height * 4) as usize, 0);
            reader.next_frame(&mut image_data).unwrap();

            let image = ImmutableImage::from_iter(
                &memoryallocator,
                image_data,
                dimensions,
                MipmapsCount::One,
                vulkano::format::Format::R8G8B8A8_SRGB,
                &mut uploads,
            )
            .unwrap();
            ImageView::new_default(image).unwrap()
        };

        let sampler = Sampler::new(
            device.clone(),
            SamplerCreateInfo {
                mag_filter: Filter::Nearest,
                min_filter: Filter::Linear,
                address_mode: [
                    SamplerAddressMode::ClampToBorder,
                    SamplerAddressMode::ClampToBorder,
                    SamplerAddressMode::Repeat,
                ],
                ..Default::default()
            },
        )
        .unwrap();

        let subpass = Subpass::from(render_pass.clone(), 0).unwrap();
        let pipeline: Arc<GraphicsPipeline> = Self::create_pipeline(&device, &vs, &fs, subpass);

        //Use storage buffer for object position. UGAA!

        //CpuAccessibleBuffer::from_data(
        //     &memoryallocator,
        //     BufferUsage {
        //         uniform_buffer: true,
        //         ..BufferUsage::empty()
        //     },
        //     false,
        //     Object {
        //         position: [0.0, 0.0],
        //         size: [1.0, 1.7],
        //         rotation: 0.0
        //     }
        // )
        // .unwrap();
        let descriptors = [
            PersistentDescriptorSet::new(
                &descriptor_set_allocator,
                pipeline.layout().set_layouts().get(0).unwrap().clone(),
                [WriteDescriptorSet::image_view_sampler(
                    0,
                    texture.clone(),
                    sampler.clone(),
                )],
            )
            .unwrap(),
            PersistentDescriptorSet::new(
                &descriptor_set_allocator,
                pipeline.layout().set_layouts().get(1).unwrap().clone(),
                [WriteDescriptorSet::buffer(
                    0,
                    object_buffer
                        .from_data(vertexshader::ty::Object {
                            color: [0.0, 0.0, 0.0, 0.0],
                            position: [0.0, 0.0],
                            size: [1.0, 1.0],
                            rotation: 0.0,
                        })
                        .unwrap(),
                )],
            )
            .unwrap(),
        ];

        let mut viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [0.0, 0.0],
            depth_range: 0.0..1.0,
        };
        let framebuffers =
            Self::window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);

        let recreate_swapchain = false;

        let previous_frame_end = Some(
            uploads
                .build()
                .unwrap()
                .execute(queue.clone())
                .unwrap()
                .boxed(),
        );

        let objects = HashMap::new();
        let render_order = Vec::new();

        surface
            .object()
            .unwrap()
            .downcast_ref::<Window>()
            .unwrap()
            .set_visible(true);
        (
            Self {
                instance,
                debugmessenger,
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
                descriptors,
                previous_frame_end,
                objects,
                render_order,
                memoryallocator,
                commandbufferallocator,
                vertex_buffer,
                object_buffer,
                index_buffer,
                descriptor_set_allocator,
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
        let extensions = InstanceExtensions {
            ext_debug_utils: true,
            ..required_extensions
        };

        let layers = vec![
            "VK_LAYER_KHRONOS_validation".to_owned(),
            //"VK_LAYER_VALVE_steam_overlay_64".to_owned(),
        ];

        let gameinfo = InstanceCreateInfo {
            enabled_layers: layers,
            application_name: Some(APPNAME.into()),
            application_version: Version {
                major: (0),
                minor: (0),
                patch: (0),
            },
            enabled_extensions: extensions,
            engine_name: Some("Let Engine".into()),
            engine_version: Version {
                major: (0),
                minor: (0),
                patch: (1),
            },
            ..Default::default()
        };
        Instance::new(library, gameinfo).expect("Couldn't start Vulkan.")
    }
    fn setup_debug(instance: &Arc<Instance>) -> Option<DebugUtilsMessenger> {
        unsafe {
            DebugUtilsMessenger::new(
                instance.clone(),
                DebugUtilsMessengerCreateInfo {
                    message_severity: DebugUtilsMessageSeverity {
                        error: true,
                        warning: true,
                        information: true,
                        verbose: true,
                        ..DebugUtilsMessageSeverity::empty()
                    },
                    message_type: DebugUtilsMessageType {
                        general: true,
                        validation: true,
                        performance: true,
                        ..DebugUtilsMessageType::empty()
                    },
                    ..DebugUtilsMessengerCreateInfo::user_callback(Arc::new(|msg| {
                        let severity = if msg.severity.error {
                            "error"
                        } else if msg.severity.warning {
                            "warning"
                        } else if msg.severity.information {
                            "information"
                        } else if msg.severity.verbose {
                            "verbose"
                        } else {
                            panic!("no-impl");
                        };

                        let ty = if msg.ty.general {
                            "general"
                        } else if msg.ty.validation {
                            "validation"
                        } else if msg.ty.performance {
                            "performance"
                        } else {
                            panic!("no-impl");
                        };
                        if severity != "verbose" {
                            println!(
                                "{} {} {}: {}",
                                msg.layer_prefix.unwrap_or("unknown"),
                                ty,
                                severity,
                                msg.description
                            );
                        }
                    }))
                },
            )
            .ok()
        }
    }

    fn create_window(instance: &Arc<Instance>) -> (EventLoop<()>, Arc<Surface>) {
        let icon: DynamicImage =
            image::load_from_memory(include_bytes!("../assets/handsomesquidward.bmp")).unwrap();
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
            .with_visible(false)
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
        let innersize = surface
            .object()
            .unwrap()
            .downcast_ref::<Window>()
            .unwrap()
            .inner_size()
            .into();
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
    fn create_render_pass(device: &Arc<Device>, swapchain: &Arc<Swapchain>) -> Arc<RenderPass> {
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
        vs: &Arc<ShaderModule>,
        fs: &Arc<ShaderModule>,
        subpass: Subpass,
    ) -> Arc<GraphicsPipeline> {
        GraphicsPipeline::start()
            .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
            .input_assembly_state(InputAssemblyState::new())
            .vertex_shader(vs.entry_point("main").unwrap(), ())
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(fs.entry_point("main").unwrap(), ())
            .color_blend_state(ColorBlendState::new(subpass.num_color_attachments()).blend_alpha())
            .render_pass(subpass)
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
            window.set_fullscreen(Some(Fullscreen::Borderless(window.current_monitor())));
        //borderless
        //surface.window().set_fullscreen(Some(Fullscreen::Exclusive(MonitorHandle::video_modes(&surface.window().current_monitor().unwrap()).next().unwrap()))); //exclusive
        } else {
            window.set_fullscreen(None)
        }
    }
    pub fn redrawevent(&mut self) {
        //windowevents
        let window = self
            .surface
            .object()
            .unwrap()
            .downcast_ref::<Window>()
            .unwrap();
        let dimensions = window.inner_size();

        let (image_num, suboptimal, acquire_future) =
            match acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return;
                }
                Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };

        let mut builder = AutoCommandBufferBuilder::primary(
            &self.commandbufferallocator,
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();
        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.0, 0.0, 0.0, 0.0].into())],
                    ..RenderPassBeginInfo::framebuffer(
                        self.framebuffers[image_num as usize].clone(),
                    )
                },
                SubpassContents::Inline,
            )
            .unwrap()
            .set_viewport(0, [self.viewport.clone()])
            .bind_pipeline_graphics(self.pipeline.clone());

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

        if suboptimal {
            self.recreate_swapchain = true;
        }

        //buffer updates

        let push_constants = vertexshader::ty::PushConstant {
            resolution: [dimensions.width as f32, dimensions.height as f32],
            camera: [0.0, 0.0],
        };
        if dimensions.width == 0 || dimensions.height == 0 {
            return;
        }

        for obj in self
            .render_order
            .iter()
            .map(|x| self.objects.get(x).unwrap())
        {
            self.descriptors[1] = PersistentDescriptorSet::new(
                &self.descriptor_set_allocator,
                self.pipeline.layout().set_layouts().get(1).unwrap().clone(),
                [WriteDescriptorSet::buffer(
                    0,
                    self.object_buffer
                        .from_data(vertexshader::ty::Object {
                            color: obj.color,
                            position: obj.position,
                            size: obj.size,
                            rotation: obj.rotation,
                        })
                        .unwrap(),
                )],
            )
            .unwrap();

            let index_sub_buffer = self.index_buffer.from_iter(obj.indices.clone()).unwrap();
            let vertex_sub_buffer = self.vertex_buffer.from_iter(obj.data.clone()).unwrap();
            builder
                .bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Graphics,
                    self.pipeline.layout().clone(),
                    0,
                    self.descriptors.to_vec(),
                )
                .bind_vertex_buffers(0, vertex_sub_buffer.clone())
                .bind_index_buffer(index_sub_buffer.clone())
                .push_constants(self.pipeline.layout().clone(), 0, push_constants)
                .draw(obj.data.len() as u32, 1, 0, 0)
                .unwrap();
        }

        builder.end_render_pass().unwrap();
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
