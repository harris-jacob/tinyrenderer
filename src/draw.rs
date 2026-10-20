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
            .for_each(|p| buffer.set(p.x, p.y, color));
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
