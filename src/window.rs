use winit::{
    dpi::LogicalSize, event_loop::{self, EventLoop}, window::{Window, WindowBuilder}
};

pub struct EngineWindow {
    pub window: Window,
    pub size: (u32, u32)
}

impl EngineWindow {
    pub fn new(title: &str, width: u32, height: u32) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width, height))
            .build(&event_loop)
            .unwrap();

        (Self {
            window,
            size: (width, height)
        }, event_loop)
    }
}
