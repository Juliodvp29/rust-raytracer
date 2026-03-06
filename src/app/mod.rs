pub mod accumulator;
pub mod controller;

use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent, DeviceEvent, ElementState, MouseButton},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::LogicalSize,
};
use pixels::{Pixels, SurfaceTexture};

use crate::geometry::{Hittable, Bounded};
use crate::render::Renderer;
use crate::scene::World;
use accumulator::Accumulator;
use controller::CameraController;

pub const WIDTH:  u32 = 1280;
pub const HEIGHT: u32 = 720;
const MAX_DEPTH:  u32 = 8;   // lower than offline for speed

pub fn run(objects: Vec<Arc<dyn Bounded>>) {
    // Build BVH once
    let bvh: Arc<dyn Hittable> = Arc::new(World::build_bvh(objects));

    let event_loop = EventLoop::new().expect("Failed to create event loop");
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

    let renderer   = Renderer::new(WIDTH, HEIGHT);
    let mut accum  = Accumulator::new(WIDTH, HEIGHT);
    let mut ctrl   = CameraController::new();
    let aspect     = WIDTH as f64 / HEIGHT as f64;

    // Mouse capture state
    let mut mouse_captured = false;

    eprintln!("Controls:");
    eprintln!("  Left click  → capture mouse / start orbit");
    eprintln!("  Escape      → release mouse");
    eprintln!("  Move mouse  → orbit camera");

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            // ── Window close ─────────────────────────────────────────────
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                elwt.exit();
            }

            // ── Keyboard: Escape releases mouse ──────────────────────────
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { event: ref key_event, .. }, ..
            } => {
                use winit::keyboard::{Key, NamedKey};
                if key_event.state == ElementState::Pressed {
                    if key_event.logical_key == Key::Named(NamedKey::Escape) {
                        mouse_captured = false;
                        window.set_cursor_visible(true);
                    }
                }
            }

            // ── Mouse click: capture mouse ────────────────────────────────
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. }, ..
            } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    mouse_captured = true;
                    window.set_cursor_visible(false);
                }
            }

            // ── Mouse movement: orbit camera ──────────────────────────────
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta: (dx, dy) }, ..
            } => {
                if mouse_captured {
                    ctrl.apply_mouse_delta(dx, dy);
                    accum.reset(); // camera moved → start fresh
                }
            }

            // ── Render next frame ─────────────────────────────────────────
            Event::AboutToWait => {
                // Skip if we have enough samples (scene is converged)
                if accum.sample_count < 512 {
                    let camera = ctrl.build_camera(aspect);
                    let sample = renderer.render_sample(&camera, bvh.as_ref(), MAX_DEPTH);
                    accum.add_sample(&sample);

                    // Write to pixel buffer
                    accum.to_rgba(pixels.frame_mut());

                    if let Err(e) = pixels.render() {
                        eprintln!("pixels render error: {}", e);
                        elwt.exit();
                    }

                    // Show sample count in title
                    window.set_title(&format!(
                        "Rust Ray Tracer  |  {} spp  |  {}",
                        accum.sample_count,
                        if mouse_captured { "click ESC to release mouse" }
                        else { "click to orbit" }
                    ));
                }
            }

            _ => {}
        }
    }).expect("Event loop error");
}