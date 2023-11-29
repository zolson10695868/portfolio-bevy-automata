use crate::rule::Rule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellStatus {
    Alive,
    Dying { health: u8 },
    Dead,
}

impl CellStatus {
    pub fn next_state(&self, rule: &Rule, neighbor_count: usize) -> Self {
        match self {
            Self::Alive if rule.passes_survive(neighbor_count) => Self::Alive,
            Self::Alive => rule.kill_cell(),
            Self::Dying { health: 1 } => Self::Dead,
            Self::Dying { health } => Self::Dying { health: health - 1 },
            Self::Dead if rule.passes_birth(neighbor_count) => Self::Alive,
            Self::Dead => Self::Dead,
        }
    }

    pub fn is_live(&self) -> bool {
        matches!(self, Self::Alive | Self::Dying { .. })
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::Neighbors;

    use super::*;

    #[test]
    fn cell_next() {
        let rule = Rule {
            survival: vec![4..6],
            states: 6,
            birth: vec![4..5],
            neighbors: Neighbors::Neumann,
        };
        let c = CellStatus::Alive;
        let c = c.next_state(&rule, 4);
        assert_eq!(c, CellStatus::Alive);
        let c = c.next_state(&rule, 5);
        assert_eq!(c, CellStatus::Alive);
        {
            let c = c.next_state(&rule, 3);
            assert_eq!(c, CellStatus::Dying { health: 4 });
        }
        let c = c.next_state(&rule, 6);
        assert_eq!(c, CellStatus::Dying { health: 4 });
        let c = c.next_state(&rule, 4);
        assert_eq!(c, CellStatus::Dying { health: 3 });
        let c = c.next_state(&rule, 4);
        assert_eq!(c, CellStatus::Dying { health: 2 });
        let c = c.next_state(&rule, 2);
        assert_eq!(c, CellStatus::Dying { health: 1 });
        let c = c.next_state(&rule, 4);
        assert_eq!(c, CellStatus::Dead);
        let c = c.next_state(&rule, 3);
        assert_eq!(c, CellStatus::Dead);
        let c = c.next_state(&rule, 5);
        assert_eq!(c, CellStatus::Dead);
        let c = c.next_state(&rule, 4);
        assert_eq!(c, CellStatus::Alive);
    }
}
