use crate::framebuffer::{Color, Framebuffer};
use crate::geometry::Vec2i;

/// Draw the segment `a`–`b`.
pub fn line(a: Vec2i, b: Vec2i, framebuffer: &mut Framebuffer, color: Color) {
    let d = b - a;
    let steps = d.x.abs().max(d.y.abs());

    if steps == 0 {
        framebuffer.set(a.x, a.y, color);
        return;
    }

    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let x = a.x as f32 + t * d.x as f32;
        let y = a.y as f32 + t * d.y as f32;
        framebuffer.set(x.round() as i32, y.round() as i32, color);
    }
}
