use std::f64::consts::PI;

use ab_glyph::FontRef;
use anchor2d::{Anchor2D, HorizontalAnchor, VerticalAnchorContext, VerticalAnchorValue};
use glam::{DVec2, dvec2};
use image::{
    Rgba, RgbaImage,
    imageops::{FilterType, overlay, resize},
};
use imageproc::{
    drawing::{
        draw_filled_circle_mut, draw_filled_rect_mut, draw_polygon_mut, draw_text_mut, text_size,
    },
    point::Point,
    rect::Rect,
};
use palette::Srgba;

use crate::{rendering::Renderer, util::srgba_to_rgba8, wall::straight_wall::Line};

pub struct ImageRenderer {
    width: u32,
    height: u32,
    image: RgbaImage,
    scale: f64,
    supersampling: u32,
}

impl ImageRenderer {
    pub fn new(width: u32, height: u32, scale: f64, supersampling: u32) -> Self {
        Self {
            width,
            height,
            image: RgbaImage::new(width * supersampling, height * supersampling),
            scale,
            supersampling,
        }
    }

    fn get_supersampled_width(&self) -> u32 {
        self.width * self.supersampling
    }

    fn get_supersampled_height(&self) -> u32 {
        self.height * self.supersampling
    }

    fn map_x(&self, x: f64) -> f64 {
        let half_width = self.get_supersampled_width() as f64 / 2.0;
        (x * self.supersampling as f64 - half_width) * self.scale + half_width
    }

    fn map_y(&self, y: f64) -> f64 {
        let half_height = self.get_supersampled_height() as f64 / 2.0;
        (y * self.supersampling as f64 - half_height) * self.scale + half_height
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

        resize(&image, self.width, self.height, FilterType::Lanczos3)
    }

    fn transparent(&self) -> RgbaImage {
        RgbaImage::new(
            self.get_supersampled_width(),
            self.get_supersampled_height(),
        )
    }

    fn black(&self) -> RgbaImage {
        RgbaImage::from_par_fn(
            self.get_supersampled_width(),
            self.get_supersampled_height(),
            |_, _| Rgba([0, 0, 0, 255]),
        )
    }
}

impl Renderer for ImageRenderer {
    fn render_line(&mut self, line: &Line, thickness: f64, color: Srgba) {
        let thickness = thickness * self.scale * self.supersampling as f64;

        let offset = (thickness / 2.0).round();

        let normal = DVec2::from_angle((line.get_end() - line.get_start()).to_angle() + PI / 2.0);

        let mapped_start = self.map_dvec2(line.get_start());
        let mapped_end = self.map_dvec2(line.get_end());

        let p1 = mapped_start + normal * offset;
        let p2 = mapped_start - normal * offset;
        let p3 = mapped_end - normal * offset;
        let p4 = mapped_end + normal * offset;

        let points = vec![
            Point::new(p1.x.round() as i32, p1.y.round() as i32),
            Point::new(p2.x.round() as i32, p2.y.round() as i32),
            Point::new(p3.x.round() as i32, p3.y.round() as i32),
            Point::new(p4.x.round() as i32, p4.y.round() as i32),
        ];

        draw_polygon_mut(&mut self.image, &points, srgba_to_rgba8(color));
    }

    fn render_circle(&mut self, position: ::glam::DVec2, radius: f64, color: Srgba) {
        let position = self.map_dvec2(position).round().as_ivec2();
        let radius = (radius * self.scale * self.supersampling as f64).round() as u32;

        draw_filled_circle_mut(
            &mut self.image,
            position.into(),
            radius as i32,
            srgba_to_rgba8(color),
        );
    }

    fn render_circle_lines(
        &mut self,
        position: ::glam::DVec2,
        radius: f64,
        thickness: f64,
        color: Srgba,
    ) {
        let position = self.map_dvec2(position).round().as_ivec2();
        let radius = (radius * self.scale * self.supersampling as f64).round() as u32;
        let thickness = (thickness * self.scale * self.supersampling as f64).round() as u32;

        let mut circle_image = RgbaImage::new(2 * radius + 1, 2 * radius + 1);

        draw_filled_circle_mut(
            &mut circle_image,
            (radius as i32, radius as i32),
            radius as i32,
            srgba_to_rgba8(color),
        );

        draw_filled_circle_mut(
            &mut circle_image,
            (radius as i32, radius as i32),
            radius as i32 - thickness as i32,
            Rgba([0, 0, 0, 0]),
        );

        overlay(
            &mut self.image,
            &circle_image,
            (position.x - radius as i32) as i64,
            (position.y - radius as i32) as i64,
        );
    }

    fn render_arc(
        &mut self,
        position: ::glam::DVec2,
        radius: f64,
        _rotation: f64,
        _arc: f64,
        thickness: f64,
        color: Srgba,
    ) {
        self.render_circle_lines(position, radius, thickness, color);
    }

    fn render_text(
        &mut self,
        text: &str,
        position: ::glam::DVec2,
        anchor: Anchor2D,
        size: f64,
        color: Srgba,
    ) {
        let position = self.map_dvec2(position);
        let size = size * self.scale * self.supersampling as f64;

        let font = FontRef::try_from_slice(include_bytes!("../../../roboto.ttf")).unwrap();

        let (text_width, _) = text_size(size as f32, &font, text);

        let x = match anchor.get_horizontal() {
            HorizontalAnchor::Left => position.x,
            HorizontalAnchor::Center => position.x - text_width as f64 / 2.0,
            HorizontalAnchor::Right => position.x - text_width as f64,
        };

        let vertical_anchor = anchor.get_vertical();

        let y = match (vertical_anchor.get_context(), vertical_anchor.get_value()) {
            (VerticalAnchorContext::Graphics, VerticalAnchorValue::Bottom) => {
                position.y - size / 1.25
            }
            (VerticalAnchorContext::Math, VerticalAnchorValue::Bottom) => position.y,
            (_, VerticalAnchorValue::Center) => position.y - size / 1.25 / 2.0,
            (VerticalAnchorContext::Graphics, VerticalAnchorValue::Top) => position.y,
            (VerticalAnchorContext::Math, VerticalAnchorValue::Top) => position.y - size / 1.25,
        };

        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    draw_text_mut(
                        &mut self.image,
                        Rgba([0, 0, 0, 1]),
                        x as i32
                            - (i as f64 * self.scale * self.supersampling as f64).round() as i32,
                        y as i32
                            - (j as f64 * self.scale * self.supersampling as f64).round() as i32,
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
        let width = width * self.scale * self.supersampling as f64;
        let height = height * self.scale * self.supersampling as f64;

        // TODO: Handle rotation
        draw_filled_rect_mut(
            &mut self.image,
            Rect::at(position.x as i32, position.y as i32)
                .of_size((width as u32).max(1), (height as u32).max(1)),
            srgba_to_rgba8(color),
        );
    }

    fn render_rectangle_lines(
        &mut self,
        position: ::glam::DVec2,
        width: f64,
        height: f64,
        _offset: ::glam::DVec2,
        _rotation: f64,
        thickness: f64,
        color: Srgba,
    ) {
        let position = self.map_dvec2(position);
        let width = (width * self.scale * self.supersampling as f64).round();
        let height = (height * self.scale * self.supersampling as f64).round();
        let thickness = (thickness * self.scale * self.supersampling as f64).round() as u32;

        let mut rectangle_image = RgbaImage::new(width as u32, height as u32);

        // TODO: Handle rotation
        draw_filled_rect_mut(
            &mut rectangle_image,
            Rect::at(0, 0).of_size((width as u32).max(1), (height as u32).max(1)),
            srgba_to_rgba8(color),
        );

        draw_filled_rect_mut(
            &mut rectangle_image,
            Rect::at(thickness as i32, thickness as i32).of_size(
                (width as u32 - 2 * thickness).max(1),
                (height as u32 - 2 * thickness).max(1),
            ),
            Rgba([0, 0, 0, 0]),
        );

        overlay(
            &mut self.image,
            &rectangle_image,
            position.x as i64,
            position.y as i64,
        );
    }
}
