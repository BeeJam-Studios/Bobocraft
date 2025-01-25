use crate::{Color, Cube, Placement, RawPlacement, Rotation, Translation};
use chrono::NaiveDateTime;
use serde::Deserialize;

pub fn to_vec(placements: impl AsRef<[RawPlacement]>) -> Result<Vec<u8>, anyhow::Error> {
    for placement in placements.as_ref() {
        <&'static Cube>::try_from(placement.cube_id)?;
        Rotation::try_from(placement.orientation)?;
        placement.color.map(Color::try_from).transpose()?;
    }
    Ok(postcard::to_allocvec(placements.as_ref())?)
}

#[derive(Deserialize, Debug)]
pub struct Robot {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub thumbnail_url: String,
    pub creator_name: String,
    pub creator_nick_name: String,
    pub uploaded_at: NaiveDateTime,
    pub expired_at: NaiveDateTime,
    pub cpu: u16,
    pub robot_ranking: Option<u32>,
    pub buyable: bool,
    pub buy_count: u32,
    pub battle_count: u32,
    pub combat_rating: Option<f32>,
    pub cosmetic_rating: Option<f32>,
    pub featured: bool,
    pub banner_message: Option<String>,
    pub x_offset: u8,
    pub y_offset: u8,
    pub z_offset: u8,
    pub placements: Vec<RawPlacement>,
}

pub fn from_slice(buf: &[u8]) -> Result<Vec<Placement>, anyhow::Error> {
    let postcard_placements: Vec<RawPlacement> = if let Ok(placements) = postcard::from_bytes(buf) {
        placements
    } else {
        let robot: Robot = postcard::from_bytes(buf)?;
        robot.placements
    };
    let mut placements = Vec::with_capacity(postcard_placements.len());
    for placement in postcard_placements {
        let cube = <&'static Cube>::try_from(placement.cube_id)?;
        let rotation = Rotation::try_from(placement.orientation)?;
        let color = Color::try_from(placement.color.unwrap_or(0))?;
        let translation = Translation {
            x: placement.x,
            y: placement.y,
            z: placement.z,
        };
        placements.push(Placement {
            translation,
            rotation,
            color,
            cube,
        })
    }
    Ok(placements)
}
