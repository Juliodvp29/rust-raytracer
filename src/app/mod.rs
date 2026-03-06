pub mod accumulator;
pub mod controller;

use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::{
    dpi::LogicalSize,
    event::{
        DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::geometry::{Bounded, Hittable};
use crate::render::Renderer;
use crate::scene::World;
use accumulator::Accumulator;
use controller::CameraController;

pub const WIDTH: u32 = 1600;
pub const HEIGHT: u32 = 900;
const MAX_DEPTH: u32 = 50;

pub fn run(objects: Vec<Arc<dyn Bounded>>) {
    let bvh: Arc<dyn Hittable> = Arc::new(World::build_bvh(objects));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Ray Tracer")
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .with_resizable(false)
        .build(&event_loop)
        .expect("Failed to create window");

    let mut pixels = {
        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface).expect("Failed to create pixels")
    };

    let renderer = Renderer::new(WIDTH, HEIGHT);
    let mut accum = Accumulator::new(WIDTH, HEIGHT);
    let mut ctrl = CameraController::new();
    let aspect = WIDTH as f64 / HEIGHT as f64;
    let mut mouse_captured = false;
    let mut pending_reset = false; // reset diferido — evita frame negro

    eprintln!("Controls:");
    eprintln!("  Left click → capture mouse / orbit");
    eprintln!("  Escape     → release mouse");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }

            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if mouse_captured {
                    mouse_captured = false;
                    window.set_cursor_visible(true);
                    let _ = window.set_cursor_grab(winit::window::CursorGrabMode::None);
                }
            }

            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                        ..
                    },
                ..
            } => {
                if !mouse_captured {
                    mouse_captured = true;
                    window.set_cursor_visible(false);
                    let _ = window.set_cursor_grab(winit::window::CursorGrabMode::Confined);
                }
            }

            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta: (dx, dy) },
                ..
            } => {
                if mouse_captured {
                    ctrl.apply_mouse_delta(dx, dy);
                    pending_reset = true; // marcar reset, no hacerlo todavía
                }
            }

            Event::MainEventsCleared => {
                // Renderiza el nuevo sample PRIMERO, luego resetea si hace falta
                let camera = ctrl.build_camera(aspect);
                let sample = renderer.render_sample(&camera, bvh.as_ref(), MAX_DEPTH);

                if pending_reset {
                    accum.reset();
                    pending_reset = false;
                }

                accum.add_sample(&sample);
                accum.to_rgba(pixels.frame_mut());

                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                window.set_title(&format!(
                    "Rust Ray Tracer  |  {} spp  |  {}",
                    accum.sample_count,
                    if mouse_captured {
                        "ESC = soltar mouse"
                    } else {
                        "click = orbitar"
                    }
                ));
            }

            _ => {}
        }
    });
}
