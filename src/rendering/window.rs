use super::backend::{backend, BACKEND_NAME};
use super::renderer::Renderer;

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

pub const DIMS: window::Extent2D = window::Extent2D {
    width: 1280,
    height: 720,
};

pub struct Window {
    winit_window: winit::window::Window,
    event_loop: EventLoop<()>,
    renderer: Renderer<backend::Backend>,
}

impl Window {
    pub fn new(name: &str) -> Window {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_min_inner_size(Size::Logical(LogicalSize::new(240.0, 135.0)))
            .with_inner_size(Size::Physical(PhysicalSize::new(DIMS.width, DIMS.height)))
            .with_title(format!("{} - {}", name, BACKEND_NAME))
            .build(&event_loop)
            .unwrap();

        #[cfg(not(feature = "gl"))]
        let (window, instance, mut adapters, surface) = {
            let instance =
                backend::Instance::create("kepler game", 1).expect("Failed to create instance");
            let surface = unsafe {
                instance
                    .create_surface(&window)
                    .expect("Failed to create surface")
            };
            let mut adapters = instance.enumerate_adapters();
            (window, Some(instance), adapters, surface)
        };

        #[cfg(feature = "gl")]
        let (window, instance, mut adapters, surface) = {
            let builder = backend::config_context(
                backend::glutin::ContextBuilder::new(),
                ColorFormat::SELF,
                None,
            )
            .with_vsync(true);
            let windowed_context = builder.build_windowed(window_builder, &event_loop).unwrap();
            let (context, window) = unsafe {
                windowed_context
                    .make_current()
                    .expect("Unable to make context current")
                    .split()
            };
            let surface = backend::Surface::from_context(context);
            let adapters = surface.enumerate_adapters();
            (window, None, adapters, surface)
        };

        for adapter in &adapters {
            println!("{:?}", adapter.info);
        }

        let adapter = adapters.remove(0);

        let renderer = Renderer::new(instance, surface, adapter);

        Window {
            winit_window: window,
            event_loop,
            renderer,
        }
    }

    pub fn start_event_loop<F: 'static>(self, on_render: F) -> !
    where
        F: Fn(),
    {
        let winit_window = self.winit_window;
        let event_loop = self.event_loop;
        let mut renderer = self.renderer;
        renderer.render();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit
                    }
                    winit::event::WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = winit::event_loop::ControlFlow::Exit,
                    winit::event::WindowEvent::Resized(dims) => {
                        println!("resized to {:?}", dims);
                        #[cfg(all(feature = "gl", not(target_arch = "wasm32")))]
                        {
                            let context = renderer.surface.context();
                            context.resize(dims);
                        }
                        renderer.dimensions = window::Extent2D {
                            width: dims.width,
                            height: dims.height,
                        };
                        renderer.recreate_swapchain();
                    }
                    _ => {}
                },
                Event::RedrawEventsCleared => {
                    on_render();
                    renderer.render();
                }
                _ => (),
            }
        });
    }
}
