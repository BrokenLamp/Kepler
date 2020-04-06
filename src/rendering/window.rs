use gfx_hal::window;

use winit::{
    dpi::{LogicalSize, PhysicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const DIMS: window::Extent2D = window::Extent2D {
    width: 1280,
    height: 720,
};

pub struct Window {
    winit_window: winit::window::Window,
    event_loop: EventLoop<()>,
}

impl Window {
    pub fn new(name: &str) -> Window {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_min_inner_size(Size::Logical(LogicalSize::new(64.0, 64.0)))
            .with_inner_size(Size::Physical(PhysicalSize::new(DIMS.width, DIMS.height)))
            .with_title(format!("{} - Vulkan", name))
            .build(&event_loop)
            .unwrap();

        Window {
            winit_window: window,
            event_loop,
        }
    }

    pub fn poll_events(&mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == self.winit_window.id() => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }
}
