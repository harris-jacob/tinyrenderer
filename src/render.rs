use crate::{
    draw::{Drawable, Wireframe},
    framebuffer::{Color, Framebuffer},
    model::Model,
};

/// How each face of a model is drawn.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Style {
    Wireframe,
    Filled,
}

/// Projects every face of `model` onto `buffer` and draws it in `style`, with
/// a random colour per face. Faces that collapse on screen are skipped.
pub fn render(model: &Model, buffer: &mut Framebuffer, style: Style) {
    let viewport = buffer.viewport();

    model
        .faces()
        .filter_map(|face| viewport.project(face))
        .for_each(|triangle| {
            let color = Color::random();
            match style {
                Style::Wireframe => Wireframe(triangle).draw(buffer, color),
                Style::Filled => triangle.draw(buffer, color),
            }
        });
}
