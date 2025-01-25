use crate::idents::ParseIdentError;
use crate::{Category, Connection, Ident, Stats, CUBES};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use thiserror::Error as ThisError;

pub struct Cube {
    pub id: u32,
    pub idents: &'static [Ident],
    pub stats: Stats,
    pub connections: &'static [Connection],
}

impl TryFrom<u32> for &'static Cube {
    type Error = ParseCubeError;

    fn try_from(id: u32) -> Result<Self, Self::Error> {
        CUBES
            .iter()
            .find(|cube| cube.id == id)
            .ok_or_else(|| ParseCubeError::NotFound(format!("cube with id {}", id)))
    }
}

#[derive(ThisError, Debug)]
pub enum ParseCubeError {
    #[error(transparent)]
    Ident(#[from] ParseIdentError),
    #[error("unable to determine the exact cube for \"{0}\", multiple possibilities found {1:?}")]
    Multiple(String, Vec<String>),
    #[error("cube \"{0}\" not found")]
    NotFound(String),
}

impl FromStr for &'static Cube {
    type Err = ParseCubeError;

    fn from_str(id_str: &str) -> Result<Self, Self::Err> {
        let mut samples = id_str.split('_');
        let tier = id_str.chars().nth(1).and_then(|c| c.to_digit(10));
        if id_str.starts_with('t') && tier.is_some() {
            samples.next();
        }
        let mut idents = Vec::new();
        for sample in samples {
            idents.push(sample.parse()?);
        }
        if let Some(cube) = CUBES
            .iter()
            .find(|cube| cube.idents == idents && cube.display_tier() == tier)
        {
            Ok(cube)
        } else {
            let matching: Vec<&'static Cube> = if let Some(tier) = tier {
                CUBES
                    .iter()
                    .filter(|cube| {
                        if cube.stats.tier != tier {
                            return false;
                        }
                        for (i, ident) in idents.iter().enumerate() {
                            if cube.idents.get(i) != Some(ident) {
                                return false;
                            }
                        }
                        true
                    })
                    .collect()
            } else {
                CUBES
                    .iter()
                    .filter(|cube| {
                        for (i, ident) in idents.iter().enumerate() {
                            if cube.idents.get(i) != Some(ident) {
                                return false;
                            }
                        }
                        true
                    })
                    .collect()
            };
            match matching.len() {
                0 => Err(ParseCubeError::NotFound(id_str.to_string())),
                1 => Ok(matching[0]), // unreachable
                _ => Err(ParseCubeError::Multiple(
                    id_str.to_string(),
                    matching.into_iter().map(|c| format!("{:?}", c)).collect(),
                )),
            }
        }
    }
}

impl Debug for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(tier) = self.display_tier() {
            write!(f, "t{}_", tier)?;
        }
        for (i, ident) in self.idents.iter().enumerate() {
            let ident = ident.as_str().to_lowercase();
            if i + 1 != self.idents.len() {
                write!(f, "{}_", ident)?;
            } else {
                write!(f, "{}", ident)?;
            }
        }
        Ok(())
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(tier) = self.display_tier() {
            write!(f, "T{} ", tier)?;
        }
        for (i, ident) in self.idents.iter().enumerate() {
            let ident = ident.as_str();
            if i + 1 != self.idents.len() {
                write!(f, "{} ", ident)?;
            } else {
                write!(f, "{}", ident)?;
            }
        }
        Ok(())
    }
}

impl Eq for Cube {}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Cube {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.category().cmp(&other.category()) {
            Ordering::Equal => {}
            cmp => return cmp,
        }
        match self.stats.tier.cmp(&other.stats.tier) {
            Ordering::Equal => {}
            Ordering::Less => return Ordering::Greater,
            Ordering::Greater => return Ordering::Less,
        }
        self.idents.cmp(other.idents)
    }
}

impl PartialOrd for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Cube {
    pub fn category(&self) -> Category {
        let cube_type = self.idents[0] as usize;
        if (0..=Ident::Mortar as usize).contains(&cube_type) {
            Category::Weapon
        } else if self.idents[0] == Ident::Module {
            Category::Module
        } else if (Ident::Wheel as usize..=Ident::Propeller as usize).contains(&cube_type) {
            Category::Movement
        } else if (Ident::Armored as usize..=Ident::ElectroShield as usize).contains(&cube_type) {
            Category::Armor
        } else {
            Category::Cosmetic
        }
    }

    pub(crate) fn display_tier(&self) -> Option<u32> {
        let cube_type = self.idents[0] as usize;
        if (0..Ident::Mortar as usize).contains(&cube_type)
            || (Ident::Wheel as usize..=Ident::Propeller as usize).contains(&cube_type)
            || self.idents[0] == Ident::ElectroShield
        {
            Some(self.stats.tier)
        } else {
            None
        }
    }
}
