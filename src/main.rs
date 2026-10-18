mod draw;
mod framebuffer;
// Vector helpers land ahead of the code that needs them.
#[allow(dead_code)]
mod geometry;
mod window;

use framebuffer::{Color, Framebuffer};
use geometry::vec2i;

use crate::draw::{Drawable, Triangle};

const WHITE: Color = Color::rgb(255, 255, 255);
#[allow(dead_code)]
const GREEN: Color = Color::rgb(0, 255, 0);
#[allow(dead_code)]
const RED: Color = Color::rgb(255, 0, 0);
#[allow(dead_code)]
const BLUE: Color = Color::rgb(64, 128, 255);
#[allow(dead_code)]
const YELLOW: Color = Color::rgb(255, 200, 0);

fn main() -> Result<(), minifb::Error> {
    const WIDTH: usize = 150;
    const HEIGHT: usize = 150;
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    [
        (
            Triangle::new(vec2i(40, 50), vec2i(35, 70), vec2i(45, 70)),
            RED,
        ),
        (
            Triangle::new(vec2i(120, 35), vec2i(90, 5), vec2i(45, 110)),
            WHITE,
        ),
        (
            Triangle::new(vec2i(115, 83), vec2i(80, 90), vec2i(85, 120)),
            GREEN,
        ),
    ]
    .into_iter()
    .filter_map(|(triangle, color)| triangle.map(|t| (t, color)))
    .for_each(|(triangle, color)| triangle.draw(&mut framebuffer, color));

    window::show(&framebuffer, "tinyrenderer")
}
