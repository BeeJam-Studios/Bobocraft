use crate::position::CubePosition;
use crate::{Cube, Rotation};
use bobocraft_cubes::Connection;
use glam::{Affine3A, EulerRot, IVec3, Mat3A, Quat, Vec3, Vec3A};
use std::collections::HashMap;
use vec_btree_map::VecBTreeMap;

const SCALE: Affine3A = Affine3A {
    matrix3: Mat3A::from_diagonal(Vec3::splat(2.0)),
    translation: Vec3A::ZERO,
};

#[derive(Debug, Clone)]
pub(crate) struct ConnectionsBuilder {
    base: HashMap<CubePosition, Vec<CubePosition>>,
    cache: HashMap<IVec3, CubePosition>,
}

pub type VecBTreeConnections = VecBTreeMap<CubePosition, Vec<usize>>;
pub type HashConnections = HashMap<CubePosition, Vec<CubePosition>>;

pub trait ToVecBTreeConnections {
    fn vec_btree(&self) -> VecBTreeConnections;
}

impl ToVecBTreeConnections for HashConnections {
    fn vec_btree(&self) -> VecBTreeConnections {
        let mut connections = VecBTreeMap::with_capacity(self.len());

        for (position, values) in self.iter() {
            connections.insert(*position, Vec::with_capacity(values.len()));
        }

        for (position, values) in self.iter() {
            let from_index = connections.binary_search(position).unwrap();
            for position in values {
                let to_index = connections.binary_search(position).unwrap();
                connections[from_index].push(to_index);
            }
        }

        connections
    }
}

impl ConnectionsBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            base: HashMap::with_capacity(capacity),
            cache: HashMap::with_capacity(capacity),
        }
    }

    pub fn push(
        &mut self,
        cube: &'static Cube,
        position: CubePosition,
        rotation: Rotation,
    ) -> Result<(), &'static Cube> {
        let pt = SCALE.transform_point3(Vec3::new(
            position.x as f32,
            position.y as f32,
            position.z as f32,
        ));

        let rotation = Affine3A::from_quat(Quat::from_euler(
            EulerRot::ZYX,
            (rotation.z as f32).to_radians(),
            (rotation.y as f32).to_radians(),
            (rotation.x as f32).to_radians(),
        ));

        let translation = Affine3A::from_translation(pt);
        if self.base.insert(position, Vec::with_capacity(1)).is_some() {
            return Err(cube);
        }

        for &Connection { x, y, z } in cube.connections {
            let connection = translation
                .transform_point3(
                    rotation.transform_point3(Vec3::new(x as f32, y as f32, z as f32)),
                )
                .round()
                .as_ivec3();
            if let Some(destination) = self.cache.remove(&connection) {
                self.base.get_mut(&destination).unwrap().push(position);
                self.base.get_mut(&position).unwrap().push(destination);
            } else {
                self.cache.insert(connection, position);
            }
        }

        Ok(())
    }

    pub fn build(self) -> HashConnections {
        self.base
    }
}
