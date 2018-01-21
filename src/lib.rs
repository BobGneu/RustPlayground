extern crate winit;

use winit::{Window, WindowBuilder, EventsLoop, CreationError};

// TODO: Replace this method with a version that returns the actual OS info. 
// Should be a function that returns something like Window 10 x64 - v155551
pub fn get_os_name() -> &'static str {
    if cfg!(windows) {
        return &"Windows";
    } else if cfg!(unix) {
        return &"Unix";
    } else if cfg!(macos) {
        return &"MacOS";
    }

    return &"Unknown";
}

pub fn get_os_version() -> &'static str {
    // TODO: Find OS Version
    return "0.0.0";
}

pub fn get_version() -> &'static str { 
    return "1.0.0";
}

pub fn init (game_name: &str, events_loop: &EventsLoop) -> Result<Window, CreationError> {
    let win_builder = WindowBuilder::new();

    let window = win_builder
        .with_min_dimensions(800, 600)

        // TODO: Add x64,x86 etc.
        .with_title(format!("{} v.{} :: {}", game_name, get_version(), get_os_name()))
        .build(&events_loop)
        .unwrap();

    Ok(window)
}
