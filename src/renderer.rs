
use winit::{Window, WindowBuilder, EventsLoop, CreationError};

pub struct Renderer {
    pub window: Window,
    pub events_loop: EventsLoop
}

pub struct RendererOptions {
    pub title: String,
    pub events_loop: EventsLoop
}

impl Renderer {
    pub fn new(options: RendererOptions) -> Self {
        let builder = WindowBuilder::new()
            .with_title(options.title);

        return Renderer{
            window: builder.build(&options.events_loop).unwrap(),
            events_loop: options.events_loop
        };
    }
}