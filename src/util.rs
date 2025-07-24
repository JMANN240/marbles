use macroquad::prelude::*;

pub fn draw_text_outline(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let pixel_size = (font_size / 16.0).ceil();

    for i in -1..=1 {
        for j in -1..=1 {
            if i != 0 || j != 0 {
                draw_text(
                    text,
                    x + i as f32 * pixel_size,
                    y + j as f32 * pixel_size,
                    font_size,
                    BLACK,
                );
            }
        }
    }

    draw_text(text, x, y, font_size, color);
}

pub fn lerp_f32(start: f32, end: f32, t: f32) -> f32 {
    start * (1.0 - t) + end * t
}

pub fn lerp_color(start: Color, end: Color, t: f32) -> Color {
    Color {
        r: lerp_f32(start.r, end.r, t),
        g: lerp_f32(start.g, end.g, t),
        b: lerp_f32(start.b, end.b, t),
        a: lerp_f32(start.a, end.a, t),
    }
}

pub fn space_evenly(n: usize, start: DVec2, end: DVec2) -> Vec<DVec2> {
    (0..n)
        .map(|i| start + ((end - start) / n as f64) * (i as f64 + 0.5))
        .collect()
}
