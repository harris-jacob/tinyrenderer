mod draw;
mod framebuffer;
mod geometry;
mod model;
mod render;
mod viewport;
mod window;

use framebuffer::Framebuffer;
use model::Model;
use render::{Style, render};

const HEAD_MODEL: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/obj/african_head.obj");
const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = Model::from_obj(HEAD_MODEL)?;
    println!(
        "loaded {} vertices, {} faces",
        model.vertices().len(),
        model.faces().len()
    );

    let mut buffer = Framebuffer::new(WIDTH, HEIGHT);
    render(&model, &mut buffer, Style::Filled);
    render(&model, &mut buffer, Style::Wireframe);

    window::show(&buffer, "tinyrenderer")?;
    Ok(())
}
