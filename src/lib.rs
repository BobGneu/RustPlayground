#[macro_use]
extern crate log;
extern crate env_logger;

extern crate winit;
extern crate cgmath;
extern crate gfx_hal as hal;
#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;

mod util;
mod renderer;

use std::env;
use winit::{Window, WindowBuilder, EventsLoop, CreationError};
use util::OS;
pub use renderer::{Renderer, RendererOptions};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn get_version() -> &'static str { 
    return "1.0.0";
}

pub fn init (options : RendererOptions) -> Result<Renderer, CreationError> {
    env_logger::init();
    debug!("{}", OS::name());

    Ok(Renderer::new(options))
}
