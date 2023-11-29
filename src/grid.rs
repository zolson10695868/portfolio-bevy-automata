use crate::cell::CellStatus;
use enum_map::{enum_map, Enum, EnumMap};
use itertools::iproduct;
use std::{iter, ops::Range};
use strum::{EnumIter, IntoEnumIterator};

struct Grid(Vec<Vec<Vec<CellStatus>>>);

impl Grid {
    fn new(size: usize) -> Self {
        Self(vec![vec![vec![CellStatus::Dead; size]; size]; size])
    }
}

struct Rule {
    survival: Vec<Range<u8>>,
    birth: Vec<Range<u8>>,
    states: u8,
    neighbors: Neighbors,
}

#[derive(Debug, Clone, Copy)]
enum Neighbors {
    Moore,
    Neumann,
}

macro_rules! point {
    ($x:expr, $y:expr, $z:expr) => {
        Point(enum_map! {
            Dim::X => $x,
            Dim::Y => $y,
            Dim::Z => $z
        })
    };
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
