use bobocraft_cubes::{Cube, CUBES};
use std::path::PathBuf;

const GLTF_ASSET_PATH: &str = "assets/gltf";

#[test]
fn verify_gltf_asset_names() {
    let path = PathBuf::from(GLTF_ASSET_PATH);
    for cube in &CUBES {
        if path.join(format!("{:?}.glb", cube)).exists() {
            println!("found '{cube}'")
        } else {
            panic!("cube '{cube}' is missing");
        }
    }
}

#[test]
fn reverse_verify_gltf_asset_names() {
    for entry in std::fs::read_dir(GLTF_ASSET_PATH).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_dir() {
            let input = path.file_stem().unwrap().to_str().unwrap().to_lowercase();
            match input.parse::<&'static Cube>() {
                Ok(cube) => println!("found '{cube}'"),
                Err(err) => panic!("parsing of '{input}' failed with {err:?}"),
            }
        }
    }
}
