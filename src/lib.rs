extern crate env_logger;
#[macro_use]
extern crate log;

#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(feature = "gl")]
extern crate gfx_backend_gl as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;
extern crate gfx_hal as hal;

extern crate image;
extern crate winit;

use hal::{buffer, command, device as d, format as f, image as i, memory as m, pass, pool, pso};
use hal::{Device, Instance, PhysicalDevice, Surface, Swapchain};
use hal::{Backbuffer, DescriptorPool, FrameSync, Primitive, SwapchainConfig};
use hal::format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle};
use hal::pass::Subpass;
use hal::pso::{PipelineStage, ShaderStageFlags, Specialization};
use hal::queue::Submission;

pub mod renderer;

use winit::CreationError;
use renderer::{Renderer, RendererOptions};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn get_version() -> &'static str {
    return VERSION;
}

pub fn init(options: RendererOptions) -> Result<Renderer, CreationError> {
    env_logger::init();
    info!("Intializing Rusty v{}", get_version());

    Ok(Renderer::new(options))
}
