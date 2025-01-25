use anyhow::format_err;
use bobocraft_cubes::Cube;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

mod color;
pub mod json;
pub mod postcard;
mod rotation;

pub use color::Color;
pub use rotation::Rotation;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct RawPlacement {
    pub cube_id: u32,
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub orientation: u8,
    pub color: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Translation {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Placement {
    pub translation: Translation,
    pub rotation: Rotation,
    pub color: Color,
    pub cube: &'static Cube,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonPlacement {
    pub translation: Translation,
    pub rotation: Rotation,
    pub color: Color,
    pub cube_name: String,
}

pub fn from_buf(ext: Option<&str>, buf: &Vec<u8>) -> Result<Vec<Placement>, anyhow::Error> {
    Ok(match ext {
        Some("json") => json::from_slice(&buf)?,
        Some("bobo") => postcard::from_slice(&buf)?,
        Some(ext) => anyhow::bail!("unknown file extension {}", ext),
        None => anyhow::bail!("file extension is missing, unable to determine bobo format"),
    })
}

pub fn from_file(path: impl AsRef<Path>) -> Result<(String, Vec<Placement>), anyhow::Error> {
    let path = path.as_ref();
    let file_name = path
        .file_name()
        .ok_or_else(|| format_err!("unable to get file name for {}", path.display()))?;
    let buf = fs::read(path)?;
    Ok((
        file_name.to_string_lossy().to_string(),
        from_buf(path.extension().and_then(|e| e.to_str()), &buf)?,
    ))
}
