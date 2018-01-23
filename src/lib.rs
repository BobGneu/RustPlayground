#[macro_use]
extern crate log;
extern crate env_logger;

extern crate gfx_hal as GFX_HAL;
#[cfg(all(windows, feature = "dx12"))]
extern crate gfx_backend_dx12 as GFX_Backend;
#[cfg(all (windows, feature = "vulkan"))]
extern crate gfx_backend_vulkan as GFX_Backend;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as GFX_Backend;
#[cfg(feature = "gl")]
extern crate gfx_backend_gl as GFX_Backend;

extern crate winit;

pub mod renderer;

use winit::{CreationError};
use renderer::{Renderer, RendererOptions};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn get_version() -> &'static str { 
    return VERSION;
}

pub fn init (options : RendererOptions) -> Result<Renderer, CreationError> {
    env_logger::init();
    info!("Intializing Rusty v{}", get_version());

    Ok(Renderer::new(options))
}
