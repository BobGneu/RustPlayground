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

pub mod renderer;
pub mod position;
pub mod vertex;
pub mod window;

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
