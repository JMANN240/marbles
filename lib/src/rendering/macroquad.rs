use macroquad::prelude::*;
use palette::Srgba;

use crate::{
    rendering::{HorizontalTextAnchor, Renderer, TextAnchor2D, VerticalTextAnchor},
    util::srgba_to_color,
    wall::straight_wall::Line,
};

pub struct MacroquadRenderer {
    font: Font,
}

impl MacroquadRenderer {
    pub async fn new(font_path: &str) -> Self {
        Self {
            font: load_ttf_font(font_path).await.unwrap(),
        }
    }
}

impl Renderer for MacroquadRenderer {
    fn render_line(&mut self, line: &Line, thickness: f64, color: Srgba) {
        draw_line(
            line.get_start().x as f32,
            line.get_start().y as f32,
            line.get_end().x as f32,
            line.get_end().y as f32,
            thickness as f32,
            srgba_to_color(color),
        );
    }

    fn render_circle(&mut self, position: ::glam::DVec2, radius: f64, color: Srgba) {
        draw_circle(
            position.x as f32,
            position.y as f32,
            radius as f32,
            srgba_to_color(color),
        );
    }

    fn render_circle_lines(
        &mut self,
        position: ::glam::DVec2,
        radius: f64,
        thickness: f64,
        color: Srgba,
    ) {
        draw_circle_lines(
            position.x as f32,
            position.y as f32,
            radius as f32,
            thickness as f32,
            srgba_to_color(color),
        );
    }

    fn render_arc(
        &mut self,
        position: ::glam::DVec2,
        radius: f64,
        rotation: f64,
        arc: f64,
        thickness: f64,
        color: Srgba,
    ) {
        draw_arc(
            position.x as f32,
            position.y as f32,
            64,
            radius as f32,
            rotation as f32,
            thickness as f32,
            arc as f32,
            srgba_to_color(color),
        );
    }

    fn render_text(
        &mut self,
        text: &str,
        position: ::glam::DVec2,
        anchor: TextAnchor2D,
        size: f64,
        color: Srgba,
    ) {
        let measurement = measure_text(text, Some(&self.font), size as u16, 1.0);

        let x = match anchor.horizontal {
            HorizontalTextAnchor::Left => position.x,
            HorizontalTextAnchor::Center => position.x - measurement.width as f64 / 2.0,
            HorizontalTextAnchor::Right => position.x - measurement.width as f64,
        };

        let y = match anchor.vertical {
            VerticalTextAnchor::Bottom => position.y,
            VerticalTextAnchor::Center => position.y + measurement.offset_y as f64 / 2.0,
            VerticalTextAnchor::Top => position.y + measurement.offset_y as f64,
        };

        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    draw_text_ex(
                        text,
                        x as f32 - i as f32,
                        y as f32 - j as f32,
                        TextParams {
                            font: Some(&self.font),
                            font_size: size as u16,
                            color: BLACK,
                            ..TextParams::default()
                        },
                    );
                }
            }
        }

        draw_text_ex(
            text,
            x as f32,
            y as f32,
            TextParams {
                font: Some(&self.font),
                font_size: size as u16,
                color: srgba_to_color(color),
                ..TextParams::default()
            },
        );
    }

    fn render_rectangle(
        &mut self,
        position: ::glam::DVec2,
        width: f64,
        height: f64,
        offset: ::glam::DVec2,
        rotation: f64,
        color: Srgba,
    ) {
        draw_rectangle_ex(
            position.x as f32,
            position.y as f32,
            width as f32,
            height as f32,
            DrawRectangleParams {
                offset: vec2(offset.x as f32, offset.y as f32),
                rotation: rotation as f32,
                color: srgba_to_color(color),
            },
        );
    }

    fn render_rectangle_lines(
        &mut self,
        position: ::glam::DVec2,
        width: f64,
        height: f64,
        offset: ::glam::DVec2,
        rotation: f64,
        thickness: f64,
        color: Srgba,
    ) {
        draw_rectangle_lines_ex(
            position.x as f32,
            position.y as f32,
            width as f32,
            height as f32,
            thickness as f32,
            DrawRectangleParams {
                offset: vec2(offset.x as f32, offset.y as f32),
                rotation: rotation as f32,
                color: srgba_to_color(color),
            },
        );
    }
}
