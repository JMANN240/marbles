use std::{error::Error, ops::RangeInclusive, path::Path, process::ExitStatus};

use glam::DVec2;
#[cfg(feature = "image")]
use image::Rgba;
#[cfg(feature = "macroquad")]
use macroquad::color::Color;
use palette::Srgba;
use rand::Rng;
use serde::Deserialize;
use tracing::info;

use crate::{
    Config,
    posting::{
        cloudinary::Cloudinary,
        instagram::{InstagramPoster, MediaPublishResponse},
    },
    scene::Scene,
    scenes::{scene_1, scene_2, scene_3, scene_4, scene_5, scene_6, scene_7, scene_8, scene_9, scene_10},
};

#[cfg(feature = "macroquad")]
pub fn srgba_to_color(srgba: Srgba) -> Color {
    Color {
        r: srgba.red,
        g: srgba.green,
        b: srgba.blue,
        a: srgba.alpha,
    }
}

#[cfg(feature = "image")]
pub fn srgba_to_rgba8(color: Srgba) -> Rgba<u8> {
    let red = (color.red * 255.0).round().clamp(0.0, 255.0) as u8;
    let green = (color.green * 255.0).round().clamp(0.0, 255.0) as u8;
    let blue = (color.blue * 255.0).round().clamp(0.0, 255.0) as u8;
    let alpha = (color.alpha * 255.0).round().clamp(0.0, 255.0) as u8;
    Rgba([red, green, blue, alpha])
}

pub fn lerp_f32(start: f32, end: f32, t: f32) -> f32 {
    start * (1.0 - t) + end * t
}

pub fn lerp_color(start: Srgba, end: Srgba, t: f32) -> Srgba {
    Srgba::new(
        lerp_f32(start.red, end.red, t),
        lerp_f32(start.green, end.green, t),
        lerp_f32(start.blue, end.blue, t),
        lerp_f32(start.alpha, end.alpha, t),
    )
}

pub fn space_evenly(n: usize, start: DVec2, end: DVec2) -> Vec<DVec2> {
    (0..n)
        .map(|i| start + ((end - start) / n as f64) * (i as f64 + 0.5))
        .collect()
}

pub fn render_video<V: AsRef<Path>, T: AsRef<Path>, A: AsRef<Path>>(
    video_path: V,
    frame_template: T,
    audio_path: A,
) -> std::io::Result<ExitStatus> {
    std::process::Command::new("ffmpeg")
        .args([
            "-framerate",
            "60",
            "-i",
            frame_template.as_ref().to_str().unwrap(),
            "-i",
            audio_path.as_ref().to_str().unwrap(),
            "-c:v",
            "libx264",
            "-crf",
            "18",
            "-preset",
            "slow",
            "-pix_fmt",
            "yuv420p",
            "-c:a",
            "aac",
            "-shortest",
            video_path.as_ref().to_str().unwrap(),
        ])
        .status()
}

pub fn upload_to_youtube<'a, V: AsRef<Path>, T: AsRef<[&'a str]>>(
    video_path: V,
    title: &str,
    description: &str,
    tags: T,
) -> std::io::Result<ExitStatus> {
    std::process::Command::new("python3")
        .args([
            "youtube_uploader.py",
            "--path",
            video_path.as_ref().to_str().unwrap(),
            "--title",
            title,
            "--description",
            description,
            "--tags",
            &tags.as_ref().join(","),
        ])
        .status()
}

pub fn upload_to_instagram<V: AsRef<Path>>(
    cloudinary: Cloudinary,
    instagram: InstagramPoster,
    video_path: V,
    caption: &str,
) -> Option<Result<MediaPublishResponse, Box<dyn Error>>> {
    let cloudinary_response_result = cloudinary.post(video_path);

    match cloudinary_response_result {
        Ok(cloudinary_response) => {
            let maybe_media_publish_response = instagram.post(
                caption,
                &cloudinary_response.secure_url,
                (cloudinary_response.duration * 0.25 * 1000.0).floor(),
            );

            if let Err(e) = cloudinary.delete(&cloudinary_response.public_id) {
                return Some(Err(e));
            }

            maybe_media_publish_response
        }
        Err(e) => Some(Err(e)),
    }
}

pub fn prepare_images_path<P: AsRef<Path>>(images_path: P) -> std::io::Result<()> {
    if images_path.as_ref().exists() {
        info!("Clearing previous frames!");
        std::fs::remove_dir_all(&images_path)?;
    }

    std::fs::create_dir(images_path)?;

    Ok(())
}

pub fn prepare_videos_path<P: AsRef<Path>>(videos_path: P) -> std::io::Result<()> {
    if !videos_path.as_ref().exists() {
        std::fs::create_dir(videos_path)?;
    }

    Ok(())
}

fn format_frame_name(content: &str) -> String {
    format!("frame_{content}.png")
}

pub fn get_formatted_frame_name(padding: usize, frame_number: usize) -> String {
    format_frame_name(&format!("{frame_number:0padding$}"))
}

pub fn get_frame_template(padding: usize) -> String {
    format_frame_name(&format!("%0{padding}d"))
}

pub fn get_scene(
    rng: &mut impl Rng,
    scene_number: usize,
    config: &Config,
    width: f64,
    height: f64,
) -> Scene {
    let scenes = [
        scene_1(rng, config.get_balls().clone(), width, height),
        scene_2(rng, config.get_balls().clone(), width, height),
        scene_3(rng, config.get_balls().clone(), width, height),
        scene_4(rng, config.get_balls().clone(), width, height),
        scene_5(rng, config.get_balls().clone(), width, height),
        scene_6(rng, config.get_balls().clone(), width, height),
        scene_7(width, height),
        scene_8(rng, config.get_balls().clone(), width, height),
        scene_9(rng, config.get_balls().clone(), width, height),
        scene_10(rng, config.get_balls().clone(), width, height),
    ];

    scenes[scene_number - 1].clone()
}

#[derive(Deserialize)]
pub struct Message {
    pub message: String,
    pub user: String,
}

#[derive(Deserialize)]
pub struct MaybeMessage {
    pub message: Option<Message>,
}

#[derive(Clone)]
pub struct ValueOverTime<T> {
    base_value: T,
    modifiers: Vec<(RangeInclusive<f64>, T)>,
}

impl<T> ValueOverTime<T> {
    pub fn new(base_value: T) -> Self {
        Self { base_value, modifiers: Vec::new() }
    }

    pub fn set_value(&mut self, new_value: T) {
        self.base_value = new_value;
    }

    pub fn get_value(&self, time: f64) -> &T {
        let mut value = &self.base_value;

        for (time_range, modified_value) in &self.modifiers {
            if time_range.contains(&time) {
                value = modified_value;
            }
        }

        value
    }

    pub fn add_modifier(&mut self, time_range: RangeInclusive<f64>, modified_value: T) {
        self.modifiers.push((time_range, modified_value));
    }
}
