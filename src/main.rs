extern crate log;
extern crate env_logger;
extern crate winit;

fn main() {
    env_logger::init();

    let mut events_loop = winit::EventsLoop::new();
    let win_builder = winit::WindowBuilder::new();
    let _window = win_builder.with_title("Rust Playground")
        .build(&events_loop)
        .unwrap();

    events_loop.run_forever(|event| {
        match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => {
                winit::ControlFlow::Break
            },
            _ => winit::ControlFlow::Continue,
        }
    });
}
