#[macro_use]
extern crate log;
extern crate env_logger;
extern crate winit;

mod util;
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
