use std::ops::Range;

use crate::cell::CellStatus;

pub struct Rule {
    pub survival: Vec<Range<u8>>,
    pub birth: Vec<Range<u8>>,
    pub states: u8,
    pub neighbors: Neighbors,
}

impl Rule {
    pub fn passes_survive(&self, count: usize) -> bool {
        let count = count as u8;
        rule_contains(count, &self.survival)
    }

    pub fn first_dying_state(&self) -> u8 {
        self.states - 2
    }

    pub fn passes_birth(&self, count: usize) -> bool {
        let count = count as u8;
        rule_contains(count, &self.birth)
    }

    pub fn kill_cell(&self) -> CellStatus {
        match self.states {
            0 | 1 => panic!(),
            2 => CellStatus::Dead,
            x => CellStatus::Dying { health: x - 2 },
        }
    }
}

fn rule_contains(n: u8, range: &[Range<u8>]) -> bool {
    range.iter().any(|r| r.contains(&n))
}

#[derive(Debug, Clone, Copy)]
pub enum Neighbors {
    Moore,
    Neumann,
}
