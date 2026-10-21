use crate::{
    draw::{Drawable, Wireframe},
    framebuffer::{Color, Framebuffer},
    geometry::Triangle,
    model::Model,
};

/// How each face of a model is drawn.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Style {
    #[allow(dead_code)]
    Wireframe,
    Filled,
}

/// Which faces are skipped before drawing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cull {
    /// Draw every face.
    #[allow(dead_code)]
    None,
    /// Skip faces pointing away from the viewer.
    Back,
}

impl Cull {
    fn keeps(self, triangle: &Triangle) -> bool {
        match self {
            Cull::None => true,
            Cull::Back => triangle.is_front_facing(),
        }
    }
}

/// Projects every face of `model` onto `buffer` and draws the ones `cull`
/// keeps in `style`, with a random colour per face. 
pub fn render(model: &Model, buffer: &mut Framebuffer, style: Style, cull: Cull) {
    let viewport = buffer.viewport();

    model
        .faces()
        .filter_map(|face| viewport.project(face))
        .filter(|triangle| cull.keeps(triangle))
        .for_each(|triangle| {
            let color = Color::random();
            match style {
                Style::Wireframe => Wireframe(triangle).draw(buffer, color),
                Style::Filled => triangle.draw(buffer, color),
            }
        });
}
