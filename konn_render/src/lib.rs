pub mod renderer;

use tiny_skia::*;

pub fn sample_texture(width: u32, height: u32) -> Pixmap {
    let paint = Paint {
        anti_alias: true,
        shader: Shader::SolidColor(Color::from_rgba8(255, 0, 0, 255)),
        ..Default::default()
    };

    let mut pixmap = Pixmap::new(width, height).unwrap();
    pixmap.fill(Color::from_rgba8(255, 255, 255, 255));
    pixmap.fill_rect(
        Rect::from_xywh(300.0, 300.0, 500.0, 500.0).unwrap(),
        &paint,
        Transform::identity(),
        None,
    );

    pixmap
}
