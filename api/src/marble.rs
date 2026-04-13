use std::path::PathBuf;

use palette::Srgba;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Marble {
    pub id: i64,
    pub name: String,
    #[serde(with = "palette::serde::as_array")]
    pub color: Srgba,
    pub radius: f64,
    pub density: f64,
    pub elasticity: f64,
    pub sound_path: PathBuf,
    pub maybe_image_path: Option<PathBuf>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WriteMarble {
    pub name: String,
    #[serde(with = "palette::serde::as_array")]
    pub color: Srgba,
    pub radius: f64,
    pub density: f64,
    pub elasticity: f64,
    pub sound_path: PathBuf,
    pub maybe_image_path: Option<PathBuf>,
    pub active: bool,
}
