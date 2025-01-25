use crate::connections::{ConnectionsBuilder, ToVecBTreeConnections, VecBTreeConnections};
use crate::cubes::simple::{Cubes, VecBTreeCubes};
use crate::position::CubePosition;
use bobocraft_cubes::Cube as CubeStats;
use bobocraft_format::Rotation;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct Graph {
    connections: VecBTreeConnections,
    cubes: VecBTreeCubes,
}

impl Graph {
    pub fn dead_ends(&self) -> Vec<(&'static CubeStats, (u8, u8, u8))> {
        let mut dead = Vec::new();
        for (index, (position, connected)) in self.connections.iter().enumerate() {
            let cube = self.cubes[index];
            if cube.connections.len() > 1 && connected.len() == 1 {
                dead.push((cube, (position.x, position.y, position.z)));
            }
        }
        dead
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let diff = {
            let diff = self.cubes.max_health - self.cubes.min_health;
            if diff > 0 {
                diff
            } else {
                self.cubes.max_health * 100
            }
        };

        let mut visited = vec![None; self.connections.len()];

        writeln!(f, "graph {{")?;
        writeln!(f, "    overlap = \"false\"")?;
        writeln!(f, "    layout = \"sfdp\"")?;

        for (index, (position, connected)) in self.connections.iter().enumerate() {
            visited[index] = Some(position);

            let cube = self.cubes[index];

            let div = cube.stats.health as f32 / diff as f32;

            let height = (10.0 * div) + 1.0;
            let width = (15.0 * div) + 1.0;
            let font_size = (60.0 * div) + 10.0;

            writeln!(
                f,
                "    \"{position}\" [ label = \"{cube}\" width={width} height={height} fontsize=\"{font_size}pt\" tooltip=\"{position:?}\" ]",
            )?;

            for &index in connected {
                if let Some(to) = visited[index] {
                    writeln!(f, "    \"{}\" -- \"{}\"", position, to)?;
                }
            }
        }

        writeln!(f, "}}")?;
        Ok(())
    }
}

pub struct GraphBuilder {
    connections: ConnectionsBuilder,
    cubes: VecBTreeCubes,
}

impl GraphBuilder {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            connections: ConnectionsBuilder::with_capacity(capacity),
            cubes: Cubes::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn push(
        &mut self,
        cube: &'static CubeStats,
        x: u8,
        y: u8,
        z: u8,
        rotation: Rotation,
    ) -> Result<(), &'static CubeStats> {
        let position = CubePosition { x, y, z };

        self.connections.push(cube, position, rotation)?;
        self.cubes.push(cube, position)?;

        Ok(())
    }

    #[inline]
    pub fn build(self) -> Graph {
        Graph {
            connections: self.connections.build().vec_btree(),
            cubes: self.cubes,
        }
    }
}
