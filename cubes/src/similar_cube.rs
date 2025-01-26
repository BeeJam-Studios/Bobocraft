use super::CUBES;
use crate::{Cube, Ident};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

#[derive(Eq)]
pub struct SimilarCube<'a> {
    pub(crate) cube: &'a Cube,
}

impl SimilarCube<'_> {
    pub fn base_cube(&self) -> &'static Cube {
        fn first_two_eq(idents: &[Ident], first_two: (Ident, Ident)) -> bool {
            idents.first() == Some(&first_two.0) && idents.get(1) == Some(&first_two.1)
        }

        let i0 = self.cube.idents[0];
        let opt_i1 = if self.cube.idents[0] == Ident::ElectroShield {
            Some(self.cube.idents[1])
        } else if (Ident::Medium as usize..=Ident::Compact as usize)
            .contains(&(self.cube.idents[0] as usize))
        {
            Some(Ident::Cube)
        } else {
            None
        };

        if let Some(tier) = self.cube.display_tier() {
            if let Some(i1) = opt_i1 {
                CUBES
                    .iter()
                    .find(|cube| cube.stats.tier == tier && first_two_eq(cube.idents, (i0, i1)))
            } else {
                CUBES
                    .iter()
                    .find(|cube| cube.stats.tier == tier && cube.idents.first() == Some(&i0))
            }
        } else if let Some(i1) = opt_i1 {
            CUBES
                .iter()
                .find(|cube| first_two_eq(cube.idents, (i0, i1)))
        } else {
            CUBES.iter().find(|cube| cube.idents.first() == Some(&i0))
        }
        .unwrap()
    }
}

impl<'a> Deref for SimilarCube<'a> {
    type Target = Cube;

    fn deref(&self) -> &'a Self::Target {
        self.cube
    }
}

impl Debug for SimilarCube<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(tier) = self.cube.display_tier() {
            write!(f, "t{}_", tier)?;
        }
        write!(f, "{}", self.cube.idents[0].as_str())?;
        if self.cube.idents[0] == Ident::ElectroShield {
            write!(f, "_{}", self.cube.idents[1].as_str())?;
        } else if (Ident::Medium as usize..=Ident::Compact as usize)
            .contains(&(self.cube.idents[0] as usize))
        {
            write!(f, "_Cube")?;
        }
        Ok(())
    }
}

impl Display for SimilarCube<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(tier) = self.cube.display_tier() {
            write!(f, "T{} ", tier)?;
        }
        write!(f, "{}", self.cube.idents[0].as_str())?;
        if self.cube.idents[0] == Ident::ElectroShield {
            write!(f, " {}", self.cube.idents[1].as_str())?;
        } else if (Ident::Medium as usize..=Ident::Compact as usize)
            .contains(&(self.cube.idents[0] as usize))
        {
            write!(f, " Cube")?;
        }
        Ok(())
    }
}

impl<'a> From<&'a Cube> for SimilarCube<'a> {
    fn from(cube: &'a Cube) -> Self {
        Self { cube }
    }
}

impl PartialEq for SimilarCube<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for SimilarCube<'_> {
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
        if self.cube.idents[0] == Ident::ElectroShield {
            self.cube
                .idents
                .iter()
                .take(2)
                .cmp(other.cube.idents.iter().take(2))
        } else {
            self.cube.idents.first().cmp(&other.cube.idents.first())
        }
    }
}

impl PartialOrd for SimilarCube<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
