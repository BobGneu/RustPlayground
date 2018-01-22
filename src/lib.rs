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
use renderer::{Renderer, RendererOptions};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn get_version() -> &'static str { 
    return VERSION;
}

// TODO: Replace with the string from the compile time env variable: PROFILE
pub fn get_target() -> &'static str {
    return "-dev";
}

pub fn init (game_name: &str, events_loop: EventsLoop) -> Result<Renderer, CreationError> {
    Ok(Renderer::new(RendererOptions{
        events_loop: events_loop,
        title: format!("{} v{}{} :: {}", game_name, get_version(), get_target(), OS::name())
    }))
}
