use crate::{Color, Cube, JsonPlacement, Placement, RawPlacement, Rotation, Translation};

pub fn to_vec(placements: impl AsRef<[RawPlacement]>) -> Result<Vec<u8>, anyhow::Error> {
    let mut model = Vec::new();
    for placement in placements.as_ref() {
        let cube = <&'static Cube>::try_from(placement.cube_id)?;
        let rotation = Rotation::try_from(placement.orientation)?;
        let color = Color::try_from(placement.color.unwrap_or(0))?;
        let placement = JsonPlacement {
            translation: Translation {
                x: placement.x,
                y: placement.y,
                z: placement.z,
            },
            rotation,
            color,
            cube_name: format!("{:?}", cube),
        };
        model.push(placement);
    }
    Ok(serde_json::to_vec_pretty(&model)?)
}

pub fn from_slice(buf: &[u8]) -> Result<Vec<Placement>, anyhow::Error> {
    let json_placements: Vec<JsonPlacement> = serde_json::from_slice(buf)?;
    let mut placements = Vec::with_capacity(json_placements.len());
    for JsonPlacement {
        translation,
        rotation,
        color,
        cube_name,
    } in json_placements
    {
        placements.push(Placement {
            translation,
            rotation,
            color,
            cube: cube_name.parse()?,
        })
    }
    Ok(placements)
}
