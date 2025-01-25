use crate::Fallible;
use anyhow::{format_err, Context};
use bevy::prelude::*;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs::{read, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};

const CONFIG_FILE_NAME: &str = "config.json";

mod default {
    use super::*;

    macro_rules! default {
        ($name:tt, $l:expr, $t:ty) => {
            pub const fn $name() -> $t {
                $l
            }
        };
    }

    default!(up, KeyCode::Space, KeyCode);
    default!(down, KeyCode::ShiftLeft, KeyCode);
    default!(forward, KeyCode::KeyW, KeyCode);
    default!(backward, KeyCode::KeyS, KeyCode);
    default!(left, KeyCode::KeyA, KeyCode);
    default!(right, KeyCode::KeyD, KeyCode);
    default!(speed, 30.0, f32);
    default!(sensitivity, 0.0001, f32);
    default!(toggle_background, KeyCode::KeyB, KeyCode);
    default!(undo, KeyCode::KeyZ, KeyCode);
    default!(redo, KeyCode::KeyY, KeyCode);
    default!(reset, KeyCode::KeyR, KeyCode);
    default!(test, KeyCode::KeyT, KeyCode);
    default!(toggle_mode, KeyCode::KeyM, KeyCode);
    default!(delete_bobo, KeyCode::Backspace, KeyCode);
    default!(toggle_cursor_grab, KeyCode::Escape, KeyCode);

    pub fn movement() -> Movement {
        serde_json::from_str("{}").unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Movement {
    #[serde(default = "default::up")]
    pub up: KeyCode,
    #[serde(default = "default::down")]
    pub down: KeyCode,
    #[serde(default = "default::forward")]
    pub forward: KeyCode,
    #[serde(default = "default::backward")]
    pub backward: KeyCode,
    #[serde(default = "default::left")]
    pub left: KeyCode,
    #[serde(default = "default::right")]
    pub right: KeyCode,
    #[serde(default = "default::speed")]
    pub speed: f32,
    #[serde(default = "default::sensitivity")]
    pub sensitivity: f32,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default::movement")]
    pub movement: Movement,
    #[serde(default = "default::toggle_background")]
    pub toggle_background: KeyCode,
    #[serde(default = "default::undo")]
    pub undo: KeyCode,
    #[serde(default = "default::redo")]
    pub redo: KeyCode,
    #[serde(default = "default::reset")]
    pub reset: KeyCode,
    #[serde(default = "default::test")]
    pub test: KeyCode,
    #[serde(default = "default::toggle_mode")]
    pub toggle_mode: KeyCode,
    #[serde(default = "default::delete_bobo")]
    pub delete_bobo: KeyCode,
    #[serde(default = "default::toggle_cursor_grab")]
    pub toggle_cursor_grab: KeyCode,
    #[serde(default)]
    pub gpu_backend: Option<String>,
}

fn read_to(path: impl AsRef<Path>) -> Option<Result<Config, serde_json::Error>> {
    let buf = read(path).ok()?;
    Some(serde_json::from_slice(&buf))
}

fn get_config_path() -> Fallible<PathBuf> {
    let path = BaseDirs::new()
        .ok_or_else(|| format_err!("unable to get base dirs"))?
        .config_dir()
        .join(env!("CARGO_PKG_NAME"));
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

fn try_read_config() -> Option<Result<Config, serde_json::Error>> {
    match read_to(get_config_path().ok()?.join(CONFIG_FILE_NAME)) {
        Some(config) => Some(config),
        None => read_to(CONFIG_FILE_NAME),
    }
}

fn write_config(config: &Config) -> Fallible {
    let buf = serde_json::to_vec_pretty(config)?;
    let path = get_config_path()?;
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    let mut file = File::create(path.join(CONFIG_FILE_NAME))?;
    file.write_all(&buf)?;
    Ok(())
}

pub fn get() -> Fallible<Config> {
    Ok(
        if let Some(config) = try_read_config()
            .transpose()
            .context("reading config failed")?
        {
            if let Some(ref gpu_backend) = config.gpu_backend {
                env::set_var("WGPU_BACKEND", gpu_backend);
            }
            write_config(&config).context("writing config failed")?;
            config
        } else {
            let config: Config = serde_json::from_str("{}")?;
            write_config(&config).context("writing config failed")?;
            config
        },
    )
}
