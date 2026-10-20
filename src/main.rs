mod draw;
mod framebuffer;
mod geometry;
mod model;
mod viewport;
mod window;

use framebuffer::{Color, Framebuffer};

use crate::{draw::Drawable, model::Model};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = Model::from_obj(concat!(env!("CARGO_MANIFEST_DIR"), "/obj/african_head.obj"))?;
    println!(
        "loaded {} vertices, {} faces",
        model.vertices().len(),
        model.faces().count()
    );

    let mut buffer = Framebuffer::new(WIDTH, HEIGHT);
    let viewport = buffer.viewport();

    model
        .faces()
        .filter_map(|f| viewport.project(f))
        .for_each(|f| {
            let color = Color::random();

            f.edges().iter().for_each(|e| e.draw(&mut buffer, color))
        });

    window::show(&buffer, "tinyrenderer")?;
    Ok(())
}
