use winit::{EventsLoop, Window, WindowBuilder};

pub struct Renderer {
    pub window: Window,
    pub events_loop: EventsLoop,
}

pub struct RendererOptions {
    pub title: String,
    pub dimensions: [u32; 2],
    pub events_loop: EventsLoop,
}

impl Renderer {
    pub fn new(options: RendererOptions) -> Self {
        let builder = WindowBuilder::new()
            .with_title(options.title.to_string())
            .with_dimensions(options.dimensions[0], options.dimensions[1]);

        let _window = builder.build(&options.events_loop).unwrap();

        let _instance = return Renderer {
            window: _window,
            events_loop: options.events_loop,
        };
    }
}
