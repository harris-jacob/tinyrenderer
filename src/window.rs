use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

use crate::framebuffer::Framebuffer;

/// Open a window showing `image`. Blocks until the window is closed
/// (or Escape / Q is pressed).
pub fn show(image: &Framebuffer, title: &str) -> Result<(), minifb::Error> {
    let options = WindowOptions {
        resize: true,
        scale: Scale::X1,
        scale_mode: ScaleMode::AspectRatioStretch,
        ..WindowOptions::default()
    };
    // minifb sizes in usize; the framebuffer's public API is signed because
    // it addresses coordinates, so the cast happens here at the boundary.
    let (width, height) = (image.width() as usize, image.height() as usize);
    let mut window = Window::new(title, width, height, options)?;
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window.update_with_buffer(image.pixels(), width, height)?;
    }
    Ok(())
}
