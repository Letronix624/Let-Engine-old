extern crate image;
extern crate vulkano;
use crate::data::*;
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
    SwapchainCreationError, SwapchainPresentInfo, ColorSpace,
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