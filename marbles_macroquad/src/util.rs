use macroquad::prelude::*;

pub fn draw_text_outline(
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    text_color: Color,
    outline_color: Color,
) {
    let pixel_size = (font_size / 16.0).ceil();

    for i in -1..=1 {
        for j in -1..=1 {
            if i != 0 || j != 0 {
                draw_text(
                    text,
                    x + i as f32 * pixel_size,
                    y + j as f32 * pixel_size,
                    font_size,
                    outline_color,
                );
            }
        }
    }

    draw_text(text, x, y, font_size, text_color);
}
