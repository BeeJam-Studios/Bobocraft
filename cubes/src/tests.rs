use crate::*;
use std::collections::HashMap;

#[test]
fn parse_cube_str() {
    for cube_str in ["t4_laser", "mortar", "t4_aeroflak", "holoflag_2019"] {
        let cube: &'static Cube = dbg!(cube_str).parse().unwrap();
        dbg!(cube);
    }
}

#[test]
fn validate_cube_stats() {
    for (i, cube) in CUBES.iter().enumerate() {
        if let Some((j, _)) = CUBES
            .iter()
            .enumerate()
            .find(|(j, c)| i != *j && c.id == cube.id)
        {
            panic!(
                "Cube with id '{}' on position '{}' found again at position '{}'",
                cube.id, i, j,
            );
        }
    }
}

#[test]
fn validate_cube_connections() {
    for cube in CUBES.iter() {
        let mut cache = HashMap::new();
        for (i, &connection) in cube.connections.iter().enumerate() {
            let sum = connection.x.abs() % 2 + connection.y.abs() % 2 + connection.z.abs() % 2;
            if sum != 1 {
                panic!("misaligned connection point on position {i} on cube {cube}");
            }
            if let Some(j) = cache.remove(&connection) {
                panic!(
                    "duplicate connection {connection:?} on cube {cube} on position {i} and {j}"
                );
            } else {
                cache.insert(connection, i);
            }
        }
    }
}

#[test]
fn validate_base_cubes() {
    for cube in CUBES.iter() {
        dbg!(SimilarCube::from(cube).base_cube());
    }
}
