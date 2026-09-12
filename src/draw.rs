use crate::framebuffer::{Color, Framebuffer};
use crate::geometry::Vec2i;

pub struct Line {
    a: Vec2i,
    b: Vec2i,
}

impl Line {
    pub fn new(a: Vec2i, b: Vec2i) -> Self {
        Self { a, b }
    }

    pub fn draw(&self,framebuffer: &mut Framebuffer, color: Color) {
        let d = self.b - self.a;
        let steps = d.x.abs().max(d.y.abs());

        if steps == 0 {
            framebuffer.set(a.x, a.y, color);
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
