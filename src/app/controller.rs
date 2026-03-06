use crate::math::{Vec3, Point3};
use crate::render::Camera;
use std::f64::consts::PI;

/// Orbital camera controller.
/// The camera always looks at a fixed target (the glass sphere).
/// Mouse movement changes theta (horizontal) and phi (vertical) angles.
pub struct CameraController {
    /// Orbit target — the point the camera always looks at
    pub target: Point3,
    /// Distance from target to camera
    pub radius: f64,
    /// Horizontal angle in radians (0 = front)
    pub theta: f64,
    /// Vertical angle in radians (0 = horizon, PI/2 = top)
    pub phi: f64,
    /// Mouse sensitivity
    pub sensitivity: f64,
    /// Vertical FOV in degrees
    pub vfov: f64,
    /// Aperture for depth of field
    pub aperture: f64,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            target:      Point3::new(0.0, 1.0, 0.0),
            radius:      6.0,
            theta:       0.3,   // start slightly to the side
            phi:         0.4,   // start slightly above horizon
            sensitivity: 0.003,
            vfov:        45.0,
            aperture:    0.05,
        }
    }

    /// Apply mouse delta — called every time the mouse moves
    pub fn apply_mouse_delta(&mut self, dx: f64, dy: f64) {
        self.theta -= dx * self.sensitivity;
        // Clamp phi so camera doesn't flip over the top or below the floor
        self.phi = (self.phi - dy * self.sensitivity)
            .clamp(0.05, PI - 0.05);
    }

    /// Build a Camera from the current spherical coordinates
    pub fn build_camera(&self, aspect: f64) -> Camera {
        // Convert spherical → cartesian
        let look_from = Point3::new(
            self.target.x + self.radius * self.phi.sin() * self.theta.cos(),
            self.target.y + self.radius * self.phi.cos(),
            self.target.z + self.radius * self.phi.sin() * self.theta.sin(),
        );

        let focus_dist = (look_from - self.target).length();

        Camera::new(
            look_from,
            self.target,
            Vec3::new(0.0, 1.0, 0.0),
            self.vfov,
            aspect,
            self.aperture,
            focus_dist,
        )
    }
}