extern crate log;
extern crate env_logger;
extern crate winit;

use winit::{EventsLoop, WindowBuilder, WindowEvent as window_event, ControlFlow};
use winit::Event::WindowEvent;

fn get_os_name() -> &'static str {
    if cfg!(windows) {
        return &"Windows";
    } else if cfg!(unix) {
        return &"Unix";
    } else if cfg!(macos) {
        return &"MacOS";
    }

    return &"Unknown";
}

fn get_os_version() -> &'static str {
    // TODO: Find OS Version
    return "0.0.0";
}

fn get_version() -> &'static str { 
    return "1.0.0";
}

fn main() {
    env_logger::init();
    
    let mut events_loop = EventsLoop::new();
    let win_builder = WindowBuilder::new();
    let _window = win_builder
        .with_min_dimensions(800, 600)
        .with_title(format!("{} - v.{} {}::{}", "Rust Playground", get_version(), get_os_name(), get_os_version()))
        .build(&events_loop)
        .unwrap();

    events_loop.run_forever(|event| {
        match event {
            WindowEvent { event: window_event::Closed, .. } => {
                ControlFlow::Break
            },
            _ => ControlFlow::Continue,
        }
    });
}
