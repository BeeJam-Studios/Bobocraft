use crate::connections::{ConnectionsBuilder, ToVecBTreeConnections, VecBTreeConnections};
use crate::cubes::history::{Cube, Status, VecBTreeCubes};
use crate::position::CubePosition;
use crate::rng::Rng;
use bobocraft_cubes::Cube as CubeStats;
use bobocraft_format::Rotation;
use std::fmt::{self, Display};
use std::ops::Range;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("cube does not exist")]
    CubeNotFound,
    #[error("cube already destroyed")]
    CubeDestroyed,
    #[error("bug bot detected, {0} found twice at the same position")]
    DuplicatedCube(&'static CubeStats),
}

#[derive(Debug)]
pub struct DamageReport {
    pub damage: u32,
    pub crit_damage: u32,
    pub destroyed: bool,
    pub remaining: f32,
}

#[derive(Debug, Clone)]
pub struct Graph {
    connections: VecBTreeConnections,
    cubes: VecBTreeCubes,
    next_cubes: Vec<usize>,
    next_connections: Vec<usize>,
    next_commit: u32,
    damage: u32,
    rng: Rng,
    initial_hit: bool,
}

impl Graph {
    pub fn dead_ends(&self) -> Vec<(&Cube, (u8, u8, u8))> {
        let mut dead = Vec::new();
        for (index, (position, connected)) in self.connections.iter().enumerate() {
            let cube = &self.cubes[index];
            if cube.info.connections.len() > 1 && connected.len() == 1 {
                dead.push((cube, (position.x, position.y, position.z)));
            }
        }
        dead
    }

    #[inline]
    pub fn hard_reset(&mut self) {
        self.cubes.reset();
        self.next_cubes.clear();
        self.next_connections.clear();
        self.damage = 0;
        self.next_commit = 1;
    }

    fn apply_crits(&mut self) -> DamageReport {
        let mut segment_id = 1;
        let mut alive_segment_id = 0;
        let mut cpu_max = 0;
        let mut effective_cpu_max: f32 = 0.0;
        self.next_cubes.clear();

        for cube_index in 0..self.cubes.len() {
            if self.cubes[cube_index].unassigned_segment() {
                let mut cpu = 0;
                let mut effective_cpu: f32 = 0.0;
                self.next_cubes.push(cube_index);
                while let Some(cube_index) = self.next_cubes.pop() {
                    let cube = &mut self.cubes[cube_index];
                    if cube.assign_segment(segment_id) {
                        cpu += cube.info.stats.cpu;
                        effective_cpu += ((cube.info.stats.cpu * cube.health()) as f32)
                            / (cube.info.stats.health as f32);
                        self.next_cubes.extend(&self.connections[cube_index]);
                    }
                }

                //FIXME: 2 parts with same amount of cpu...add rng here?...maybe even use `effective_cpu`?
                if cpu > cpu_max {
                    alive_segment_id = segment_id;
                    cpu_max = cpu;
                    effective_cpu_max = effective_cpu;
                }

                segment_id += 1;
            }
        }
        let mut remaining = effective_cpu_max / (self.cubes.total_cpu as f32);

        let destroyed = remaining <= 0.2;
        if destroyed {
            alive_segment_id = segment_id;
            remaining = 0.0;
        }

        let mut crit_damage = 0;
        for cube in self.cubes.values_mut() {
            crit_damage += cube.crit_dead_segments(self.next_commit, alive_segment_id);
        }

        self.next_commit += 1;
        DamageReport {
            damage: self.damage,
            crit_damage,
            destroyed,
            remaining,
        }
    }

    fn apply_damage(&mut self, mut damage: u32) {
        debug_assert!(self.next_connections.is_empty());
        loop {
            if damage == 0 {
                return;
            }
            let layer_hp = self
                .next_cubes
                .iter()
                .fold(0, |hp, cube_index| hp + self.cubes[*cube_index].health());
            for &cube_index in self.next_cubes.iter() {
                let cube = &mut self.cubes[cube_index];
                if cube.is_destroyed() {
                    continue;
                }
                let cube_damage = if damage >= layer_hp {
                    cube.health()
                } else {
                    ((cube.health() as u64 * damage as u64) / layer_hp as u64) as u32
                };
                cube.damage(cube_damage, self.next_commit, self.initial_hit);
                self.damage += cube_damage;
                self.initial_hit = false;
                //add connecting cubes to next layer
                if damage >= layer_hp {
                    let connections = &self.connections[cube_index];
                    self.next_connections.extend(connections);
                }
            }
            // if layer did not die or on overkill
            if self.next_connections.is_empty() {
                return;
            }
            damage -= layer_hp;
            self.next_cubes.clear();
            for connection in self.next_connections.drain(..) {
                self.next_cubes.push(connection);
            }
        }
    }

    #[inline]
    fn apply_target(&mut self, index: usize) -> Result<(), Error> {
        if self.cubes[index].is_destroyed() {
            return Err(Error::CubeDestroyed);
        }
        self.next_cubes.clear();
        self.next_cubes.push(index);
        self.next_connections.clear();
        self.cubes.commit_seek();
        self.initial_hit = true;
        Ok(())
    }

    #[inline]
    pub fn target(&mut self, x: u8, y: u8, z: u8) -> Result<&mut Self, Error> {
        let index = self.cubes.find_index(x, y, z).ok_or(Error::CubeNotFound)?;
        self.apply_target(index)?;
        Ok(self)
    }

    #[inline]
    pub fn damage(&mut self, damage: u32) -> &mut Self {
        self.apply_damage(damage);
        self
    }

    #[inline]
    pub fn commit(&mut self) -> DamageReport {
        let report = self.apply_crits();
        self.damage = 0;
        report
    }

    #[inline]
    pub fn cube(&self, x: u8, y: u8, z: u8) -> Result<&Cube, Error> {
        let index = self.cubes.find_index(x, y, z).ok_or(Error::CubeNotFound)?;
        Ok(&self.cubes[index])
    }

    #[inline]
    pub fn prev(&mut self) -> bool {
        let end = self.next_commit == 1;
        if !end {
            self.next_commit -= 1;
            self.cubes.seek(self.next_commit - 1);
        }
        !end
    }

    #[inline]
    pub fn next(&mut self) -> bool {
        let max_commit = self.cubes.seek(self.next_commit);
        if max_commit == self.next_commit {
            self.next_commit += 1;
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.next_commit = 1;
        self.cubes.seek(self.next_commit - 1);
    }

    #[inline]
    pub fn damage_test(&mut self, damage: u32, rng_redundancy: u32) -> DamageTest<'_> {
        DamageTest {
            graph: self,
            rng_redundancy,
            damage_per_shot: damage,
            shots: 0,
            remaining: 1.0,
        }
    }

    #[inline]
    pub fn initial_hit(&self, cube: &Cube, range: Range<i8>) -> bool {
        let commit_id = (self.next_commit - 1) as i64;
        let start = i64::max(commit_id + range.start as i64, 0) as u32;
        let end = i64::max(commit_id + range.end as i64, 0) as u32;
        for stats in cube.history.iter() {
            if stats.commit_id >= start && stats.commit_id < end && stats.initial_hit {
                return true;
            }
        }
        false
    }

    #[inline]
    pub fn damage_dealt(&self) -> DamageReport {
        let mut damage = 0;
        let mut crit_damage = 0;
        let mut destroyed = true;
        let mut remaining_cpu = 0.0;
        let mut cpu = 0;

        for cube_index in 0..self.cubes.len() {
            let cube = &self.cubes[cube_index];
            let stats = cube.history.current();
            if stats.commit_id == self.next_commit - 1 {
                match stats.status {
                    Status::RemainingHealth(remaining) => {
                        destroyed &= remaining != 0;
                        damage += cube.info.stats.health - remaining;
                        remaining_cpu += ((cube.info.stats.cpu * remaining) as f32)
                            / (cube.info.stats.health as f32);
                        cpu += cube.info.stats.cpu;
                    }
                    Status::CritDamage(damage) => crit_damage += damage,
                }
            }
        }

        let remaining = remaining_cpu / (cpu as f32);

        DamageReport {
            damage,
            crit_damage,
            destroyed,
            remaining,
        }
    }
}

#[derive(Debug)]
pub struct DamageTest<'a> {
    graph: &'a mut Graph,
    rng_redundancy: u32,
    damage_per_shot: u32,
    shots: u32,
    remaining: f32,
}

impl<'a> Iterator for DamageTest<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        #[derive(Debug, Clone, Copy)]
        struct MaxState {
            eff_damage_per_shot: f32,
            shots_at_target: u32,
            index: usize,
            rng: Rng,
        }

        if self.remaining == 0.0 {
            return None;
        }

        let mut equal_maxima = 1;
        let mut state: Option<MaxState> = None;

        for cube_index in 0..self.graph.cubes.len() {
            if self.graph.apply_target(cube_index).is_err() {
                continue;
            }
            let shots_at_target =
                (self.graph.cubes[cube_index].health() - 1) / self.damage_per_shot + 1;
            for _ in 0..self.rng_redundancy {
                let rng = self.graph.rng;
                self.graph
                    .apply_damage(shots_at_target * self.damage_per_shot);
                let report = self.graph.commit();
                self.graph.prev();
                let damage_eff = self.remaining - report.remaining;
                let eff_damage_per_shot = damage_eff / (shots_at_target as f32);
                let max_damage_per_shot = state.map(|s| s.eff_damage_per_shot).unwrap_or(0.0);
                let new_max = if eff_damage_per_shot == max_damage_per_shot {
                    equal_maxima += 1;
                    self.graph.rng.new() % equal_maxima == 0
                } else {
                    eff_damage_per_shot > max_damage_per_shot
                };
                if new_max {
                    if eff_damage_per_shot > max_damage_per_shot {
                        equal_maxima = 1;
                    }
                    state = Some(MaxState {
                        eff_damage_per_shot,
                        shots_at_target,
                        index: cube_index,
                        rng,
                    });
                }
            }
        }

        let max = state.unwrap();
        self.graph.apply_target(max.index).unwrap();
        self.graph.rng = max.rng;
        self.graph
            .apply_damage(max.shots_at_target * self.damage_per_shot);
        let report = self.graph.commit();
        self.remaining = report.remaining;
        self.shots += max.shots_at_target;
        Some((1.0 - self.remaining) / 0.8)
    }
}

impl<'a> DamageTest<'a> {
    pub fn get_shots(&self) -> u32 {
        self.shots
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

        for (index, (position, connection_index)) in self.connections.iter().enumerate() {
            visited[index] = Some(position);

            let cube = &self.cubes[index];

            let div = cube.info.stats.health as f32 / diff as f32;

            let initial_hit = if cube.initial_hit() { "*" } else { "" };
            let height = (10.0 * div) + 1.0;
            let width = (15.0 * div) + 1.0;
            let font_size = (60.0 * div) + 10.0;

            writeln!(
                f,
                "    \"{position}\" [ label = \"{initial_hit}{cube}\" width={width} height={height} fontsize=\"{font_size}pt\" tooltip=\"{position:?}\" ]",
            )?;

            for &index in connection_index {
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
            cubes: VecBTreeCubes::with_capacity(capacity),
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
    ) -> Result<(), Error> {
        let position = CubePosition { x, y, z };

        self.connections
            .push(cube, position, rotation)
            .map_err(|_| Error::DuplicatedCube(cube))?;
        self.cubes
            .push(cube, position)
            .map_err(|_| Error::DuplicatedCube(cube))?;

        Ok(())
    }

    #[inline]
    pub fn build_with_seed(self, seed: u32) -> Graph {
        Graph {
            connections: self.connections.build().vec_btree(),
            cubes: self.cubes,
            next_cubes: Vec::new(),
            next_connections: Vec::new(),
            next_commit: 1,
            damage: 0,
            rng: Rng::with_seed(seed),
            initial_hit: false,
        }
    }

    #[inline]
    pub fn build(self) -> Graph {
        self.build_with_seed(1)
    }
}
