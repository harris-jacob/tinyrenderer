use crate::framebuffer::{Color, Framebuffer};
use crate::geometry::{Vec2i, vec2i};
use crate::{BLUE, RED, YELLOW};

pub trait Drawable {
    fn draw(&self, buffer: &mut Framebuffer, color: Color);
}

pub struct Line {
    a: Vec2i,
    b: Vec2i,
}

pub struct Triangle {
    a: Vec2i,
    b: Vec2i,
    c: Vec2i,
}

impl Triangle {
    pub fn new(a: Vec2i, b: Vec2i, c: Vec2i) -> Option<Self> {
        let triangle = Self { a, b, c };

        if triangle.area() == 0.0 {
            None
        } else {
            Some(triangle)
        }
    }

    pub fn area(&self) -> f32 {
        let ab = self.b - self.a;
        let ac = self.c - self.a;

        ab.cross(ac).abs() as f32 / 2.0
    }
}

impl Line {
    pub fn new(a: Vec2i, b: Vec2i) -> Self {
        Self { a, b }
    }
}

impl Drawable for Triangle {
    fn draw(&self, buffer: &mut Framebuffer, color: Color) {
        let mut vertices = [self.a, self.b, self.c];

        vertices.sort_by_key(|v| v.y);

        // a -> c long axis, b is hinge vertex
        let [a, b, c] = vertices;

        // Scanline
        for y in a.y..=c.y {
            // always sample AC
            let x_1 = a.x + (y - a.y) * (c.x - a.x) / (c.y - a.y);

            let x_2 = if y < b.y {
                // sample AB
                a.x + (y - a.y) * (b.x - a.x) / (b.y - a.y)
            } else if b.y == c.y {
                // flat bottom: BC lies on this row, so span to b to cover the edge
                b.x
            } else {
                // sample BC
                b.x + (y - b.y) * (c.x - b.x) / (c.y - b.y)
            };

            let scanline = Line::new(vec2i(x_1, y), vec2i(x_2, y));

            scanline.draw(buffer, color);
        }

        // Draw lines for debugging
        let ab = Line::new(a, b);
        let bc = Line::new(b, c);
        let ac = Line::new(a, c);

        ab.draw(buffer, RED);
        bc.draw(buffer, BLUE);
        ac.draw(buffer, YELLOW);
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
