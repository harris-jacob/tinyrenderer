use crate::framebuffer::{Color, Framebuffer};
use crate::geometry::{Line, Triangle};

pub trait Drawable {
    fn draw(&self, buffer: &mut Framebuffer, color: Color);
}

impl Drawable for Triangle {
    fn draw(&self, buffer: &mut Framebuffer, color: Color) {
        self.bounding_box()
            .points()
            .filter(|&p| self.barycentric(p).is_inside())
            .for_each(|p| {
                let depth = self.barycentric(p).interpolate(self.depth);
                // buffer.set(p.x, p.y, color);
                buffer.set_if_closer(p.x, p.y, depth, color)
            });
    }
}

impl Drawable for Line {
    fn draw(&self, framebuffer: &mut Framebuffer, color: Color) {
        let d = self.b - self.a;
        let steps = d.x.abs().max(d.y.abs());

        if steps == 0 {
            framebuffer.set(self.a.x, self.a.y, color);
            return;
        }

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let x = self.a.x as f32 + t * d.x as f32;
            let y = self.a.y as f32 + t * d.y as f32;
            framebuffer.set(x.round() as i32, y.round() as i32, color);
        }
    }
}

/// Draws a triangle's edges instead of filling it.
pub struct Wireframe(pub Triangle);

impl Drawable for Wireframe {
    fn draw(&self, buffer: &mut Framebuffer, color: Color) {
        self.0
            .edges()
            .into_iter()
            .for_each(|edge| edge.draw(buffer, color));
    }
}
