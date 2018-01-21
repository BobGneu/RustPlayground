extern crate rusty_playground;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate winit;

use winit::{EventsLoop, WindowEvent as window_event, ControlFlow};
use winit::Event::WindowEvent;
use winit::WindowEvent::{KeyboardInput};

fn main() {
    let mut running: bool = true;
    env_logger::init();

    let mut events_loop = EventsLoop::new();

    match rusty_playground::init(&"Testing", events_loop) {
        Ok(mut renderer) => {
            while running  {
                renderer.events_loop.poll_events(|event| {
                    if let WindowEvent { event, .. } = event {
                        match event {   
                            window_event::KeyboardInput {
                                input: winit::KeyboardInput {
                                    virtual_keycode: Some(winit::VirtualKeyCode::Escape),
                                .. 
                                },
                                ..
                            } | window_event::Closed => running = false,
                            winit::WindowEvent::KeyboardInput {
                                input,
                                ..
                            } => {
                                println!("K  {:?}", event);
                            },
                            winit::WindowEvent::MouseInput {
                                ..
                            } => {
                                println!("M  {:?}", event);
                            },
                            winit::WindowEvent::MouseWheel {
                                ..
                            } => {
                                println!("MW {:?}", event);
                            },
                            winit::WindowEvent::CursorMoved {       // Sent every tick as the mouse moves within the context of this window. Only fired when the window is focused. 
                                ..
                            } => {}, 
                            winit::WindowEvent::ReceivedCharacter { // Sent along with the KeyboardInput - Pressed event
                                ..
                            } => {}, 
                            _ => {
                                println!("#  {:?}", event);
                            }
                        }
                    } else if let winit::Event::DeviceEvent { event, .. } = event {
                        // TODO: DeviceEvent handled here
                        debug!("");
                    } else {
                        println!("?  {:?}", event);
                    }
                });
            }
        },
        Err(_error) => {

        }
    }
}