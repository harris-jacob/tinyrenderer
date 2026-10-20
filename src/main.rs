mod draw;
mod framebuffer;
mod geometry;
mod window;

use framebuffer::{Color, Framebuffer};
use geometry::Vec2i;

use crate::{draw::Drawable, geometry::Triangle};

const WHITE: Color = Color::rgb(255, 255, 255);
const GREEN: Color = Color::rgb(0, 255, 0);
const RED: Color = Color::rgb(255, 0, 0);

fn main() -> Result<(), minifb::Error> {
    const WIDTH: usize = 150;
    const HEIGHT: usize = 150;
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    [
        (
            Triangle::new(Vec2i::new(40, 50), Vec2i::new(35, 70), Vec2i::new(45, 70)),
            RED,
        ),
        (
            Triangle::new(Vec2i::new(120, 35), Vec2i::new(90, 5), Vec2i::new(45, 110)),
            WHITE,
        ),
        (
            Triangle::new(Vec2i::new(115, 83), Vec2i::new(80, 90), Vec2i::new(85, 120)),
            GREEN,
        ),
    ]
    .into_iter()
    .filter_map(|(triangle, color)| triangle.map(|t| (t, color)))
    .for_each(|(triangle, color)| triangle.draw(&mut framebuffer, color));

    window::show(&framebuffer, "tinyrenderer")
}
