use crate::position::CubePosition;
use bobocraft_cubes::Cube;
use std::collections::HashMap;
use std::ops::Deref;
use vec_btree_map::VecBTreeMap;

pub trait Map {
    fn with_capacity(capacity: usize) -> Self;
    fn insert(&mut self, k: CubePosition, v: &'static Cube) -> Option<&'static Cube>;
}

impl Map for VecBTreeMap<CubePosition, &'static Cube> {
    fn with_capacity(capacity: usize) -> Self {
        VecBTreeMap::with_capacity(capacity)
    }

    fn insert(&mut self, k: CubePosition, v: &'static Cube) -> Option<&'static Cube> {
        self.insert(k, v)
    }
}

impl Map for HashMap<CubePosition, &'static Cube> {
    fn with_capacity(capacity: usize) -> Self {
        HashMap::with_capacity(capacity)
    }

    fn insert(&mut self, k: CubePosition, v: &'static Cube) -> Option<&'static Cube> {
        self.insert(k, v)
    }
}

#[derive(Debug, Clone)]
pub struct Cubes<M> {
    base: M,
    pub min_health: u32,
    pub max_health: u32,
}

impl<M> Deref for Cubes<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<M: Map> Cubes<M> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            base: M::with_capacity(capacity),
            max_health: 0,
            min_health: u32::MAX,
        }
    }

    pub fn push(
        &mut self,
        cube: &'static Cube,
        position: CubePosition,
    ) -> Result<(), &'static Cube> {
        if self.base.insert(position, cube).is_some() {
            return Err(cube);
        }

        self.max_health = u32::max(self.max_health, cube.stats.health);
        self.min_health = u32::min(self.min_health, cube.stats.health);

        Ok(())
    }
}

pub type HashCubes = Cubes<HashMap<CubePosition, &'static Cube>>;
pub type VecBTreeCubes = Cubes<VecBTreeMap<CubePosition, &'static Cube>>;
