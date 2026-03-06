use std::ops::{Add, Sub, Mul, Neg, AddAssign, MulAssign};
use std::fmt;

/// A 3-component floating-point vector used for positions, directions, and colors.
/// All ray tracer math (dot products, cross products, reflections, etc.) goes through this type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Creates a new vector with the given x, y, z components.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Returns the zero vector (0, 0, 0). Useful as a neutral starting point.
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Returns the vector (1, 1, 1). Used as a white color or for simple scaling.
    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    /// Euclidean length (magnitude) of the vector: sqrt(x² + y² + z²).
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Squared length (x² + y² + z²). Cheaper than `length()` when you only need
    /// to compare magnitudes (avoids the sqrt).
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Dot product: sum of component-wise products (self · other).
    /// Result is positive when vectors point in the same direction, negative when opposing,
    /// and zero when perpendicular. Essential for lighting and hit detection.
    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product: returns a vector perpendicular to both self and other.
    /// The order matters: self × other follows the right-hand rule.
    /// Used for building orthonormal bases (e.g., camera coordinate frames).
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Returns a unit vector (length = 1) pointing in the same direction.
    /// Don't call this on a zero vector — it would cause a division by zero.
    pub fn normalize(self) -> Vec3 {
        let len = self.length();
        self * (1.0 / len)
    }

    /// Returns true if all components are extremely close to zero (below 1e-8).
    /// Used to detect degenerate vectors that can cause NaN in normalization or reflection.
    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1e-8;
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }

    /// Reflects this vector off a surface with the given unit normal.
    /// Formula: v - 2(v·n)n — the component along the normal is flipped.
    /// Used for mirror-like (specular) reflections in materials.
    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - normal * 2.0 * self.dot(normal)
    }
}

/// Type alias: a Vec3 used as an RGB color (x=red, y=green, z=blue), each in [0, 1].
pub type Color = Vec3;
/// Type alias: a Vec3 used as a 3D point in world space.
pub type Point3 = Vec3;

// --- Operator overloads -------------------------------------------------------
// These let you write natural math expressions like `a + b`, `v * 3.0`, etc.

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

/// Scalar multiplication: Vec3 * f64 — scales every component by the same factor.
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}

/// Allows writing `3.0 * vec` in addition to `vec * 3.0` (commutativity).
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

/// Component-wise multiplication (Hadamard product): used for blending colors
/// with material albedo (e.g., light_color * surface_color).
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

/// Negation: flips the direction of a vector (or inverts a color).
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

/// Human-readable formatting: prints "(x.xxx, y.yyy, z.zzz)" rounded to 3 decimals.
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_add() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a + b, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_dot_product() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        // Two perpendicular unit vectors must have dot product = 0
        assert_eq!(a.dot(b), 0.0); 
    }

    #[test]
    fn test_normalize() {
        let v = Vec3::new(3.0, 0.0, 0.0);
        let n = v.normalize();
        // After normalization the length must be 1 (within floating-point tolerance)
        assert!((n.length() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_cross_product() {
        // X × Y should yield Z (standard right-hand rule)
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);
        let z = x.cross(y);
        assert_eq!(z, Vec3::new(0.0, 0.0, 1.0));
    }
}
