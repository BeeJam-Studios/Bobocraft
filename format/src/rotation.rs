use anyhow::{bail, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Rotation {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl TryFrom<u8> for Rotation {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let (x, y, z) = match value {
            0 => (0, 0, 0),
            1 => (0, -90, 0),
            2 => (180, 0, 180),
            3 => (0, 90, 0),
            4 => (0, 0, -90),
            5 => (0, -90, -90),
            6 => (90, 0, 90),
            7 => (0, 90, -90),
            8 => (0, -90, 90),
            9 => (0, 90, 90),
            10 => (-90, 0, 90),
            11 => (-90, 0, -90),
            12 => (90, 0, -90),
            13 => (180, 0, 90),
            14 => (0, 0, 180),
            15 => (0, -90, 180),
            16 => (180, 0, 0),
            17 => (0, 90, 180),
            18 => (0, 0, 90),
            19 => (180, 0, -90),
            20 => (-90, 0, 0),
            21 => (-90, 0, 180),
            22 => (90, 0, 0),
            23 => (90, 0, 180),
            _ => bail!("unable to parse orientation '{}'", value),
        };
        Ok(Self { x, y, z })
    }
}
