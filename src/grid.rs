use crate::{
    cell::CellStatus,
    rule::{Neighbors, Rule},
};
use enum_map::{enum_map, Enum, EnumMap};
use itertools::iproduct;
use std::{iter, ops::Range};
use strum::{EnumIter, IntoEnumIterator};

macro_rules! point {
    ($x:expr, $y:expr, $z:expr) => {
        Point(enum_map! {
            Dim::X => $x,
            Dim::Y => $y,
            Dim::Z => $z
        })
    };
}

struct Grid(Vec<Vec<Vec<CellStatus>>>);

impl Grid {
    pub fn new(size: usize) -> Self {
        Self(vec![vec![vec![CellStatus::Dead; size]; size]; size])
    }

    fn next_as_point(&self, p: &Point, rule: &Rule) -> CellStatus {
        let count = p
            .neighbors(&rule.neighbors)
            .into_iter()
            .filter(|p| self.get(p).is_some_and(|c| c.is_live()))
            .count();
        self.get(p).unwrap().next_state(rule, count)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn points(&self) -> impl Iterator<Item = Point> {
        let l = self.len();
        iproduct!(0..l, 0..l, 0..l).map(|(x, y, z)| point!(x, y, z))
    }

    fn get(&self, p: &Point) -> Option<&CellStatus> {
        self.0.get(p.0[Dim::X])?.get(p.0[Dim::Y])?.get(p.0[Dim::Z])
    }

    fn get_mut(&mut self, p: &Point) -> Option<&mut CellStatus> {
        self.0
            .get_mut(p.0[Dim::X])?
            .get_mut(p.0[Dim::Y])?
            .get_mut(p.0[Dim::Z])
    }

    fn count_live(&self, it: impl IntoIterator<Item = Point>) -> usize {
        it.into_iter()
            .filter(|p| match self.get(&p).unwrap() {
                CellStatus::Alive | CellStatus::Dying { .. } => true,
                _ => false,
            })
            .count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point(EnumMap<Dim, usize>);

impl Point {
    fn neighbors(&self, n: &Neighbors) -> Vec<Self> {
        match n {
            Neighbors::Moore => iproduct!(-1isize..=1, -1isize..=1, -1isize..=1)
                .filter(|o| *o != (0, 0, 0))
                .zip(iter::repeat(self.clone()))
                .map(|((x, y, z), mut p)| {
                    Dim::iter()
                        .zip([x, y, z])
                        .for_each(|(d, o)| p.0[d] = p.0[d].wrapping_add_signed(o));
                    p
                })
                .collect(),
            Neighbors::Neumann => iproduct!(Dim::iter(), [-1, 1])
                .zip(iter::repeat(self.clone()))
                .map(|((d, o), mut p)| {
                    p.0[d] = p.0[d].wrapping_add_signed(o);
                    p
                })
                .collect(),
        }
    }
}

#[derive(Debug, Enum, EnumIter, Clone, Copy)]
enum Dim {
    X,
    Y,
    Z,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_neumann() {
        let p = point!(1, 1, 1);
        assert_eq!(
            p.neighbors(&Neighbors::Neumann),
            vec![
                point!(0, 1, 1),
                point!(2, 1, 1),
                point!(1, 0, 1),
                point!(1, 2, 1),
                point!(1, 1, 0),
                point!(1, 1, 2)
            ]
        );
    }

    #[test]
    fn neighbors_moore() {
        let p = point!(1, 1, 1);
        let ne = p.neighbors(&Neighbors::Moore);
        assert_eq!(ne.len(), 26);
        assert_eq!(
            ne,
            vec![
                point!(0, 0, 0),
                point!(0, 0, 1),
                point!(0, 0, 2),
                point!(0, 1, 0),
                point!(0, 1, 1),
                point!(0, 1, 2),
                point!(0, 2, 0),
                point!(0, 2, 1),
                point!(0, 2, 2),
                point!(1, 0, 0),
                point!(1, 0, 1),
                point!(1, 0, 2),
                point!(1, 1, 0),
                //point!(1, 1, 1), // original point is ignored
                point!(1, 1, 2),
                point!(1, 2, 0),
                point!(1, 2, 1),
                point!(1, 2, 2),
                point!(2, 0, 0),
                point!(2, 0, 1),
                point!(2, 0, 2),
                point!(2, 1, 0),
                point!(2, 1, 1),
                point!(2, 1, 2),
                point!(2, 2, 0),
                point!(2, 2, 1),
                point!(2, 2, 2),
            ]
        );
    }

    #[test]
    fn neighbors_wrapping() {
        let p = point!(0, usize::MAX, 0);
        assert_eq!(
            p.neighbors(&Neighbors::Neumann),
            vec![
                point!(usize::MAX, usize::MAX, 0),
                point!(1, usize::MAX, 0),
                point!(0, usize::MAX - 1, 0),
                point!(0, 0, 0),
                point!(0, usize::MAX, usize::MAX),
                point!(0, usize::MAX, 1)
            ]
        )
    }
}
