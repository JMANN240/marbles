use std::f64::consts::PI;

use ab_glyph::FontRef;
use glam::{dvec2, DVec2};
use image::{Rgba, RgbaImage, imageops::overlay};
use imageproc::{
    drawing::{
        draw_antialiased_line_segment_mut, draw_antialiased_polygon_mut, draw_filled_circle_mut, draw_filled_rect_mut, draw_hollow_circle_mut, draw_line_segment_mut, draw_text_mut, text_size
    }, pixelops::interpolate, point::Point, rect::Rect
};
use palette::Srgba;

use crate::{
    rendering::{HorizontalTextAnchor, Renderer, TextAnchor2D, VerticalTextAnchor},
    util::srgba_to_rgba8,
    wall::straight_wall::Line,
};

pub struct ImageRenderer {
    width: u32,
    height: u32,
    image: RgbaImage,
    scale: f64,
}

impl ImageRenderer {
    pub fn new(width: u32, height: u32, scale: f64) -> Self {
        Self {
            width,
            height,
            image: RgbaImage::new(width, height),
            scale,
        }
    }

    fn map_x(&self, x: f64) -> f64 {
        let half_width = self.width as f64 / 2.0;
        (x - half_width) * self.scale + half_width
    }

    fn map_y(&self, y: f64) -> f64 {
        let half_height = self.height as f64 / 2.0;
        (y - half_height) * self.scale + half_height
    }

    fn map_dvec2(&self, v: DVec2) -> DVec2 {
        dvec2(self.map_x(v.x), self.map_y(v.y))
    }

    pub fn reset(&mut self) {
        self.image = self.transparent();
    }

    pub fn get_image(&self) -> RgbaImage {
        let mut image = self.black();

        overlay(&mut image, &self.image, 0, 0);

        image
    }

    fn transparent(&self) -> RgbaImage {
        RgbaImage::new(self.width, self.height)
    }

    fn black(&self) -> RgbaImage {
        RgbaImage::from_par_fn(self.width, self.height, |_, _| Rgba([0, 0, 0, 255]))
    }
}

impl Renderer for ImageRenderer {
    fn render_line(&mut self, line: &Line, thickness: f64, color: Srgba) {
        let offset= (thickness / 2.0).floor();

        let normal = DVec2::from_angle((line.get_end() - line.get_start()).to_angle() + PI / 2.0);

        let mapped_start = self.map_dvec2(line.get_start());
        let mapped_end = self.map_dvec2(line.get_end());

        let p1 = mapped_start + normal * offset;
        let p2 = mapped_start - normal * offset;
        let p3 = mapped_end - normal * offset;
        let p4 = mapped_end + normal * offset;

        let mut points = Vec::new();

        points.push(Point::new(p1.x as i32, p1.y as i32));
        points.push(Point::new(p2.x as i32, p2.y as i32));
        points.push(Point::new(p3.x as i32, p3.y as i32));
        points.push(Point::new(p4.x as i32, p4.y as i32));

        draw_antialiased_polygon_mut(
            &mut self.image,
            &points,
            srgba_to_rgba8(color),
            interpolate,
        );
    }

    fn render_circle(&mut self, position: ::glam::DVec2, radius: f64, color: Srgba) {
        let position = self.map_dvec2(position);
        let radius = radius * self.scale;

        let mut circle_image = RgbaImage::new(((radius + 1.0) * 2.0) as u32 + 1, ((radius + 1.0) * 2.0) as u32 + 1);

        draw_filled_circle_mut(
            &mut circle_image,
            (radius as i32 + 1, radius as i32 + 1),
            radius as i32 + 1,
            srgba_to_rgba8(Srgba::new(color.red, color.green, color.blue, color.alpha / 2.0)),
        );

        draw_filled_circle_mut(
            &mut circle_image,
            (radius as i32 + 1, radius as i32 + 1),
            radius as i32,
            srgba_to_rgba8(color),
        );

        overlay(
            &mut self.image,
            &circle_image,
            (position.x - radius) as i64,
            (position.y - radius) as i64,
        );
    }

    fn render_circle_lines(
        &mut self,
        position: ::glam::DVec2,
        radius: f64,
        thickness: f64,
        color: Srgba,
    ) {
        let position = self.map_dvec2(position);
        let radius = radius * self.scale;

        let mut circle_image = RgbaImage::new(((radius + 1.0) * 2.0) as u32 + 1, ((radius + 1.0) * 2.0) as u32 + 1);

        draw_filled_circle_mut(
            &mut circle_image,
            (radius as i32 + 1, radius as i32 + 1),
            radius as i32 + 1,
            srgba_to_rgba8(Srgba::new(color.red, color.green, color.blue, color.alpha / 2.0)),
        );

        draw_filled_circle_mut(
            &mut circle_image,
            (radius as i32 + 1, radius as i32 + 1),
            radius as i32,
            srgba_to_rgba8(color),
        );

        draw_filled_circle_mut(
            &mut circle_image,
            (radius as i32 + 1, radius as i32 + 1),
            (radius - thickness) as i32,
            Rgba([0, 0, 0, 0]),
        );

        draw_filled_circle_mut(
            &mut circle_image,
            (radius as i32 + 1, radius as i32 + 1),
            (radius - thickness) as i32,
            srgba_to_rgba8(Srgba::new(color.red, color.green, color.blue, color.alpha / 2.0)),
        );

        draw_filled_circle_mut(
            &mut circle_image,
            (radius as i32 + 1, radius as i32 + 1),
            (radius - thickness) as i32 - 1,
            Rgba([0, 0, 0, 0]),
        );

        overlay(
            &mut self.image,
            &circle_image,
            (position.x - radius) as i64,
            (position.y - radius) as i64,
        );
    }

    fn render_arc(
        &mut self,
        _position: ::glam::DVec2,
        _radius: f64,
        _rotation: f64,
        _arc: f64,
        _thickness: f64,
        _color: Srgba,
    ) {
        //TODO: Darn it
    }

    fn render_text(
        &mut self,
        text: &str,
        position: ::glam::DVec2,
        anchor: TextAnchor2D,
        size: f64,
        color: Srgba,
    ) {
        let position = self.map_dvec2(position);
        let size = size * self.scale;

        let font = FontRef::try_from_slice(include_bytes!("../../../roboto.ttf")).unwrap();

        let (text_width, _) = text_size(size as f32, &font, text);

        let x = match anchor.horizontal {
            HorizontalTextAnchor::Left => position.x,
            HorizontalTextAnchor::Center => position.x - text_width as f64 / 2.0,
            HorizontalTextAnchor::Right => position.x - text_width as f64,
        };

        let y = match anchor.vertical {
            VerticalTextAnchor::Bottom => position.y - size / 1.25,
            VerticalTextAnchor::Center => position.y - size / 1.25 / 2.0,
            VerticalTextAnchor::Top => position.y,
        };



        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    draw_text_mut(
                        &mut self.image,
                        Rgba([0, 0, 0, 1]),
                        x as i32 - i,
                        y as i32 - j,
                        size as f32,
                        &font,
                        text,
                    );
                }
            }
        }

        draw_text_mut(
            &mut self.image,
            srgba_to_rgba8(color),
            x as i32,
            y as i32,
            size as f32,
            &font,
            text,
        );
    }

    fn render_rectangle(
        &mut self,
        position: ::glam::DVec2,
        width: f64,
        height: f64,
        _offset: ::glam::DVec2,
        _rotation: f64,
        color: Srgba,
    ) {
        let position = self.map_dvec2(position);
        let width = width * self.scale;
        let height = height * self.scale;

        // TODO: Handle rotation
        draw_filled_rect_mut(
            &mut self.image,
            Rect::at(position.x as i32, position.y as i32)
                .of_size((width as u32).max(1), (height as u32).max(1)),
            srgba_to_rgba8(color),
        );
    }
}
