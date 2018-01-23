extern crate log;
extern crate env_logger;
extern crate winit;

extern crate rusty_playground;

use winit::{EventsLoop, WindowEvent as window_event};
use winit::Event::WindowEvent;
use winit::WindowEvent::KeyboardInput;
 
use rusty_playground::renderer::RendererOptions;

pub fn get_version() -> &'static str { 
    return "1.0.0";
}
 
// TODO: Replace with the string from the compile time env variable: PROFILE
pub fn get_target() -> &'static str {
    return "-dev";
}

fn main() {
    let mut running: bool = true;
    let events_loop = EventsLoop::new();

    match rusty_playground::init(RendererOptions{
        events_loop: events_loop,
        title: format!("{} v{}{}", "Testing", get_version(), get_target()),
        dimensions: [1400, 900]
    }) {
        Ok(mut renderer) => {
            while running  {
                renderer.events_loop.poll_events(|event| {
                    if let WindowEvent { event, .. } = event {
                        match event {   
                            KeyboardInput {
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
                                println!("K  {:?}", input);
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
                        println!("D  {:?}", event);
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
