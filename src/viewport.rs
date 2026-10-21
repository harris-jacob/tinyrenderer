use crate::geometry::{Triangle, Vec3f};

/// Maps model space, [−1, 1] on each axis, onto a framebuffer's pixels.
pub struct Viewport {
    width: f32,
    height: f32,
}

impl Viewport {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width as f32,
            height: height as f32,
        }
    }

    /// Maps each vertex onto the screen. None when the face collapses to
    /// collinear pixels, e.g. seen edge-on.
    pub fn project(&self, face: [Vec3f; 3]) -> Option<Triangle> {
        Triangle::new(face.map(|v| self.to_screen(v)))
    }

    /// Shifts [−1, 1] to [0, 2], then scales to the pixel range. z passes
    /// through unchanged.
    fn to_screen(&self, v: Vec3f) -> Vec3f {
        Vec3f::new(
            (v.x + 1.0) * self.width / 2.0,
            (v.y + 1.0) * self.height / 2.0,
            v.z,
        )
    }
}
