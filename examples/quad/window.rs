use winit::{Window, WindowBuilder,  EventsLoop};

#[cfg(any(feature = "vulkan", feature = "dx12", feature = "metal"))]
pub fn create_window(events_loop: &EventsLoop) -> Window {
    let wb = WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("quad".to_string());

    let window = wb
        .build(&events_loop)
        .unwrap();
        
    (window)
}

#[cfg(feature = "gl")]
pub fn create_window(events_loop: &EventsLoop) -> Window {
    let wb = WindowBuilder::new() 
        .with_dimensions(1024, 768)
        .with_title("quad".to_string());

    let window = {
        let builder = GFXBackend::config_context(
            GFXBackend::glutin::ContextBuilder::new(),
            ColorFormat::SELF,
            None,
        ).with_vsync(true);

        GFXBackend::glutin::GlWindow::new(wb, builder, &events_loop).unwrap()
    };

    (window)
}