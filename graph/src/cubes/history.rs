use crate::position::CubePosition;
use bobocraft_cubes::Cube as CubeInfo;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use std::ops::DerefMut;
use vec_btree_map::VecBTreeMap;

#[derive(Debug, Clone)]
pub enum Status {
    RemainingHealth(u32),
    CritDamage(u32),
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub status: Status,
    pub initial_hit: bool,
    pub commit_id: u32,
}

impl Stats {
    const fn remaining_health(health: u32, commit_id: u32, initial_hit: bool) -> Self {
        Self {
            status: Status::RemainingHealth(health),
            commit_id,
            initial_hit,
        }
    }

    const fn crit_damage(damage: u32, commit_id: u32) -> Self {
        Self {
            status: Status::CritDamage(damage),
            commit_id,
            initial_hit: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct History {
    base: Vec<Stats>,
    position: usize,
}

impl Deref for History {
    type Target = Vec<Stats>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl History {
    fn push(&mut self, value: Stats) {
        self.position += 1;
        self.base.truncate(self.position);
        self.base.push(value);
    }

    pub fn current(&self) -> &Stats {
        self.base.get(self.position).unwrap()
    }

    fn commit_seek(&mut self) {
        self.base.truncate(self.position + 1);
    }

    fn reset(&mut self) {
        self.base.truncate(1);
        self.position = 0;
    }

    fn seek(&mut self, commit_id: u32) -> u32 {
        let mut max_commit_id = 0;
        for (i, item) in self.base.iter().enumerate() {
            if item.commit_id <= commit_id {
                self.position = i;
                max_commit_id = item.commit_id;
            }
        }
        max_commit_id
    }
}

impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.position > 1 {
            if let Some(change) = self.base.get(1) {
                write!(f, "{}", change.commit_id)?;
            }
        }

        for change in self.base.iter().skip(2).take(self.position - 1) {
            write!(f, " - {}", change.commit_id)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Cube {
    pub(crate) info: &'static CubeInfo,
    pub(crate) history: History,
    segment_id: u32,
}

impl fmt::Debug for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let was_crit = self.was_crit();
        let health = if was_crit {
            self.damage_received()
        } else {
            self.health()
        };
        let full_health = self.info.stats.health;
        match (was_crit, full_health == health) {
            (true, true) => write!(f, "CRIT/{health}"),
            (true, false) => write!(f, "CRIT {health}/{full_health}"),
            (false, true) => write!(f, "{full_health}"),
            (false, false) => write!(f, "{health}/{full_health}"),
        }?;
        Ok(())
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let info = self.info;
        let was_crit = self.was_crit();
        let health = if was_crit {
            self.damage_received()
        } else {
            self.health()
        };
        let full_health = self.info.stats.health;
        match (was_crit, full_health == health) {
            (true, true) => write!(f, "{info}\\nCRIT/{health}"),
            (true, false) => write!(f, "{info}\\nCRIT\\n{health}/{full_health}"),
            (false, true) => write!(f, "{info}"),
            (false, false) => write!(f, "{info}\\n{health}/{full_health}"),
        }?;
        Ok(())
    }
}

impl Cube {
    pub fn new(info: &'static CubeInfo) -> Self {
        Self {
            info,
            segment_id: 0,
            history: History {
                base: vec![Stats::remaining_health(info.stats.health, 0, false)],
                position: 0,
            },
        }
    }
}

impl Cube {
    pub fn health(&self) -> u32 {
        match self.history.current().status {
            Status::RemainingHealth(remaining) => remaining,
            Status::CritDamage(_) => 0,
        }
    }

    pub fn is_destroyed(&self) -> bool {
        self.health() == 0
    }

    pub fn damage_received(&self) -> u32 {
        match self.history.current().status {
            Status::RemainingHealth(remaining) => self.info.stats.health - remaining,
            Status::CritDamage(damage) => damage,
        }
    }

    pub fn damage(&mut self, damage: u32, commit_id: u32, initial_hit: bool) -> u32 {
        let health = self.health();
        if health == 0 {
            damage
        } else {
            let remaining_damage = damage.saturating_sub(health);
            let remaining_health = health.saturating_sub(damage);
            self.history.push(Stats::remaining_health(
                remaining_health,
                commit_id,
                initial_hit,
            ));
            remaining_damage
        }
    }

    pub fn unassigned_segment(&self) -> bool {
        self.segment_id == 0 && self.health() != 0
    }

    pub fn assign_segment(&mut self, segment_id: u32) -> bool {
        let unassigned = self.unassigned_segment();
        if unassigned {
            self.segment_id = segment_id;
        }
        unassigned
    }

    pub fn crit_dead_segments(&mut self, commit_id: u32, alive_segment_id: u32) -> u32 {
        let crit_damage = if self.segment_id != alive_segment_id {
            let health = self.health();
            if health != 0 {
                self.history.push(Stats::crit_damage(health, commit_id));
            }
            health
        } else {
            0
        };
        self.segment_id = 0;
        crit_damage
    }

    pub fn was_crit(&self) -> bool {
        match self.history.current().status {
            Status::RemainingHealth(_) => false,
            Status::CritDamage(_) => true,
        }
    }

    pub fn initial_hit(&self) -> bool {
        self.history.current().initial_hit
    }
}

#[derive(Debug, Clone)]
pub(crate) struct VecBTreeCubes {
    base: VecBTreeMap<CubePosition, Cube>,
    pub min_health: u32,
    pub max_health: u32,
    pub total_cpu: u32,
    seeked: bool,
}

impl Deref for VecBTreeCubes {
    type Target = VecBTreeMap<CubePosition, Cube>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for VecBTreeCubes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl VecBTreeCubes {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            base: VecBTreeMap::with_capacity(capacity),
            max_health: 0,
            min_health: u32::MAX,
            total_cpu: 0,
            seeked: false,
        }
    }

    pub fn push(
        &mut self,
        cube: &'static CubeInfo,
        position: CubePosition,
    ) -> Result<(), &'static CubeInfo> {
        // TODO: maybe use let Some(other_cube) here and return proper error containing both cubes
        if self.base.insert(position, Cube::new(cube)).is_some() {
            return Err(cube);
        }

        self.max_health = u32::max(self.max_health, cube.stats.health);
        self.min_health = u32::min(self.min_health, cube.stats.health);

        self.total_cpu += cube.stats.cpu;
        Ok(())
    }

    pub fn find_index(&self, x: u8, y: u8, z: u8) -> Option<usize> {
        self.base.binary_search(&CubePosition { x, y, z }).ok()
    }

    pub fn commit_seek(&mut self) {
        if self.seeked {
            for cube in self.base.values_mut() {
                cube.history.commit_seek();
            }
            self.seeked = false;
        }
    }

    pub fn reset(&mut self) {
        for cube in self.base.values_mut() {
            cube.history.reset();
        }
    }

    pub fn seek(&mut self, commit_id: u32) -> u32 {
        let mut reverted_to_commit_id = 0;
        for cube in self.base.values_mut() {
            reverted_to_commit_id = u32::max(reverted_to_commit_id, cube.history.seek(commit_id));
        }
        self.seeked = true;
        reverted_to_commit_id
    }
}
