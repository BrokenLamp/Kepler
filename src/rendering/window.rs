#![cfg_attr(
    not(any(
        feature = "vulkan",
        feature = "dx11",
        feature = "dx12",
        feature = "metal",
        feature = "gl",
        feature = "wgl"
    )),
    allow(dead_code, unused_extern_crates, unused_imports)
)]

#[cfg(feature = "dx11")]
extern crate gfx_backend_dx11 as back;
#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(any(feature = "gl", feature = "wgl"))]
extern crate gfx_backend_gl as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;

#[cfg(not(any(
    feature = "vulkan",
    feature = "dx11",
    feature = "dx12",
    feature = "metal",
    feature = "gl",
    feature = "wgl"
)))]
extern crate gfx_backend_empty as back;

use gfx_hal::{
    buffer, command, format as f,
    format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle},
    image as i, memory as m, pass,
    pass::Subpass,
    pool,
    prelude::*,
    pso,
    pso::{PipelineStage, ShaderStageFlags, VertexInputRate},
    queue::{QueueGroup, Submission},
    window,
};

use winit::{
    dpi::{LogicalSize, PhysicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::{
    borrow::Borrow,
    io::Cursor,
    iter,
    mem::{self, ManuallyDrop},
    ptr,
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
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_min_inner_size(Size::Logical(LogicalSize::new(240.0, 135.0)))
            .with_inner_size(Size::Physical(PhysicalSize::new(DIMS.width, DIMS.height)))
            .with_title(format!("{} - Vulkan", name))
            .build(&event_loop)
            .unwrap();

        let instance = back::Instance::create("kepler game", 1).expect("Failed to create instance");
        let surface = unsafe {
            instance
                .create_surface(&window)
                .expect("Failed to create surface")
        };
        let adapters = instance.enumerate_adapters();

        Window {
            winit_window: window,
            event_loop,
        }
    }

    pub fn start_event_loop<F: 'static>(self, on_render: F) -> !
    where
        F: Fn(),
    {
        let winit_window = self.winit_window;
        let event_loop = self.event_loop;
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == winit_window.id() => *control_flow = ControlFlow::Exit,
                Event::RedrawEventsCleared => {
                    on_render();
                }
                _ => (),
            }
        });
    }
}
