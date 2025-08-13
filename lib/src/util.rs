use std::process::ExitStatus;

use glam::DVec2;
#[cfg(feature = "image")]
use image::Rgba;
#[cfg(feature = "macroquad")]
use macroquad::color::Color;
use palette::Srgba;

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

pub fn render_video(
    video_filename: &str,
    frame_template: &str,
    audio_path: &str,
) -> std::io::Result<ExitStatus> {
    std::process::Command::new("ffmpeg")
        .args([
            "-framerate",
            "60",
            "-i",
            frame_template,
            "-i",
            audio_path,
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
            video_filename,
        ])
        .status()
}

pub fn upload_to_youtube(
    video_filename: &str,
    title: &str,
    description: &str,
    tags: &str,
) -> std::io::Result<ExitStatus> {
    std::process::Command::new("python3")
        .args([
            "youtube_uploader.py",
            "--path",
            video_filename,
            "--title",
            title,
            "--description",
            description,
            "--tags",
            tags,
        ])
        .status()
}
