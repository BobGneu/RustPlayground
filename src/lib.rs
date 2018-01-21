extern crate winit;

mod util;

use std::env;
use winit::{Window, WindowBuilder, EventsLoop, CreationError};
use util::OS;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn get_version() -> &'static str { 
    return VERSION;
}

// TODO: Replace with the string from the compile time env variable: PROFILE
pub fn get_target() -> &'static str {
    return "-dev";
}

pub fn init (game_name: &str, events_loop: &EventsLoop) -> Result<Window, CreationError> {
    let win_builder = WindowBuilder::new();

    let _window = win_builder
        .with_min_dimensions(800, 600)
        .with_title(format!("{} v{}{} :: {}", game_name, get_version(), get_target(), OS::name()))
        .build(&events_loop)
        .unwrap();

    Ok(_window)
}
