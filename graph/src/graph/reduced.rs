use crate::connections::{ConnectionsBuilder, HashConnections};
use crate::cubes::simple::HashCubes;
use crate::position::CubePosition;
use bobocraft_cubes::{Category, Cube as CubeStats};
use bobocraft_format::Rotation;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct Graph {
    connections: HashConnections,
    ticks: HashMap<(CubePosition, CubePosition), usize>,
    cubes: HashCubes,
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

        let mut visited = HashSet::new();

        writeln!(f, "graph {{")?;
        writeln!(f, "    overlap = \"false\"")?;
        writeln!(f, "    layout = \"sfdp\"")?;

        for (position, connected) in self.connections.iter() {
            visited.insert(position);

            let cube = self.cubes[position];

            let div = cube.stats.health as f32 / diff as f32;

            let height = (10.0 * div) + 1.0;
            let width = (15.0 * div) + 1.0;
            let font_size = (60.0 * div) + 10.0;

            writeln!(
                f,
                "    \"{position}\" [ label = \"{cube}\" width={width} height={height} fontsize=\"{font_size}pt\" tooltip=\"{position:?}\" ]",
            )?;

            for to in connected {
                if !visited.contains(to) {
                    if let Some(&ticks) = self.ticks.get(&(*position, *to)) {
                        let s = if ticks == 1 { "" } else { "s" };
                        writeln!(
                            f,
                            "    \"{}\" -- \"{}\" [ label = \"{ticks} tick{s}\" ]",
                            position, to
                        )?;
                    } else {
                        writeln!(f, "    \"{}\" -- \"{}\"", position, to)?;
                    }
                }
            }
        }

        writeln!(f, "}}")?;
        Ok(())
    }
}

pub struct GraphBuilder {
    connections: ConnectionsBuilder,
    cubes: HashCubes,
}

impl GraphBuilder {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            connections: ConnectionsBuilder::with_capacity(capacity),
            cubes: HashCubes::with_capacity(capacity),
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
        let mut reduced = HashMap::new();
        let mut connections = self.connections.build();

        for (&position, connected) in connections.iter() {
            let ([left, right], Category::Armor | Category::Cosmetic) = (
                connected.as_slice(),
                self.cubes.get(&position).unwrap().category(),
            ) else {
                continue;
            };
            reduced.insert(position, (*left, *right));
        }

        let search = |mut position: CubePosition, direction: bool| -> (CubePosition, usize) {
            let mut last = None;
            let mut ticks = 0_usize;
            loop {
                if let Some(&(left, right)) = reduced.get(&position) {
                    ticks += 1;
                    let n = if direction {
                        if Some(left) == last {
                            right
                        } else {
                            left
                        }
                    } else if Some(right) == last {
                        left
                    } else {
                        right
                    };
                    last = Some(position);
                    position = n;
                } else {
                    break (position, ticks);
                }
            }
        };

        connections.retain(|position, _| !reduced.contains_key(position));

        let mut tick_map = HashMap::new();

        for (&from, connected) in connections.iter_mut() {
            for to in connected.iter_mut() {
                let (new_to, ticks) = search(*to, true);
                if new_to != from {
                    *to = new_to;
                    if ticks != 0 {
                        tick_map.insert((from, new_to), ticks);
                    }
                    continue;
                }
                let (new_to, ticks) = search(*to, false);
                if new_to != from {
                    *to = new_to;
                    if ticks != 0 {
                        tick_map.insert((from, new_to), ticks);
                    }
                    continue;
                }
            }
        }

        Graph {
            connections,
            ticks: tick_map,
            cubes: self.cubes,
        }
    }
}
