mod draw;
mod framebuffer;
// Vector helpers land ahead of the code that needs them.
#[allow(dead_code)]
mod geometry;
mod window;

use framebuffer::{Color, Framebuffer};
use geometry::vec2i;

use crate::draw::Line;

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
    const WIDTH: usize = 64;
    const HEIGHT: usize = 64;
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let a = vec2i(7, 3);
    let b = vec2i(12, 37);
    let c = vec2i(62, 53);
    
    let ab = Line::new(a, b);
    let bc = Line::new(b, c);
    let ac = Line::new(a, c);
    
    ab.draw(&mut framebuffer, RED);
    bc.draw(&mut framebuffer, BLUE);
    ac.draw(&mut framebuffer, GREEN);

    for v in [a, b, c] {
        framebuffer.set(v.x, v.y, WHITE);
    }

    window::show(&framebuffer, "tinyrenderer")
}
