use bevy::prelude::{Color, Vec4};

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

    pub fn color(&self) -> Color {
        match self {
            Self::Dead => Color::rgba(0., 0., 0., 0.),
            Self::Alive => Color::WHITE,
            _ => Color::GRAY,
        }
    }

    pub fn color_grad(&self, states: &u8) -> Color {
        const C1: Color = Color::hsl(359.9, 1., 0.5);
        const C2: Color = Color::hsl(300., 1., 0.);
        match self {
            Self::Dead => C2,
            Self::Alive => C1,
            Self::Dying { health } => {
                let h1 = C1.h();
                let h2 = C2.h();
                let weight = *health as f32 / *states as f32;
                let l = weight / 2.;
                let h = (1. - weight) * h1 + weight * h2;
                Color::hsl(h, 1., l)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::Neighbors;

    use super::*;

    #[test]
    fn cell_next() {
        // 4-5/4/6/N
        let rule = Rule {
            survival: vec![4..6],
            birth: vec![4..5],
            states: 6,
            neighbors: Neighbors::Neumann,
        };
        let c = CellStatus::Alive;
        // stays alive at 4 or 5
        let c = c.next_state(&rule, 4);
        assert_eq!(c, CellStatus::Alive);
        let c = c.next_state(&rule, 5);
        assert_eq!(c, CellStatus::Alive);
        {
            // starts dying at 3
            let c = c.next_state(&rule, 3);
            assert_eq!(c, CellStatus::Dying { health: 4 });
        }
        // ... or at 6
        let c = c.next_state(&rule, 6);
        assert_eq!(c, CellStatus::Dying { health: 4 });
        // once it starts dying, it won't stop
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
        // comes back at 4
        let c = c.next_state(&rule, 4);
        assert_eq!(c, CellStatus::Alive);
    }
}
