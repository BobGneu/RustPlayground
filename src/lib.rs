extern crate winit;

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
    /*let win_builder = WindowBuilder::new();

    let _window = win_builder
        .with_min_dimensions(800, 600)
        .with_title()
        .build(&events_loop)
        .unwrap();

    Ok(Renderer {
        window: _window 
    })*/

    Ok(Renderer::new(RendererOptions{
        events_loop: events_loop,
        title: format!("{} v{}{} :: {}", game_name, get_version(), get_target(), OS::name())
    }))
}
