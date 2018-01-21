extern crate rusty_playground;

extern crate log;
extern crate env_logger;
extern crate winit;

use winit::{EventsLoop, WindowEvent as window_event, ControlFlow};
use winit::Event::WindowEvent;

fn main() {
    let mut events_loop = EventsLoop::new();

    match rusty_playground::init(&"Testing", &events_loop) {
        Ok(_window) => {
            events_loop.run_forever(|event| {
                match event {
                    WindowEvent { event: window_event::Closed, .. } => {
                        ControlFlow::Break
                    },
                    _ => ControlFlow::Continue,
                }
            });
        },
        Err(_error) => {

        }
    }
}