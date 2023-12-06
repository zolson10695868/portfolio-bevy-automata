use crate::cell::CellStatus;
use bevy::{prelude::Resource, reflect::Reflect};
use std::ops::Range;

#[derive(Resource, Clone, Reflect, Debug, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
pub enum Neighbors {
    Moore,
    Neumann,
}

mod parser {
    use std::str::FromStr;

    use super::{Neighbors, Rule};
    use chumsky::{
        prelude::Simple,
        primitive::{choice, just},
        text, Parser,
    };

    impl FromStr for Rule {
        type Err = Simple<char>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::parser()
                .parse(s)
                .map_err(|v| v.into_iter().next().unwrap())
        }
    }

    impl Rule {
        fn parser() -> impl Parser<char, Rule, Error = Simple<char>> {
            let single_num = text::int(10).map(|s: String| s.parse::<u8>().unwrap());
            let range = single_num
                .clone()
                .then(just("-").ignore_then(single_num).or_not())
                .map(|(low, high)| low..(high.unwrap_or(low) + 1));
            let ranges_rule = range.separated_by(just(','));
            let neighbor = choice((
                just('M').to(Neighbors::Moore),
                just('N').to(Neighbors::Neumann),
            ));
            ranges_rule
                .clone()
                .separated_by(just('/'))
                .exactly(2)
                .then_ignore(just('/'))
                .then(single_num)
                .then_ignore(just('/'))
                .then(neighbor)
                .map(|((ranges, states), neighbors)| {
                    let survival = ranges[0].clone();
                    let birth = ranges[1].clone();
                    Self {
                        survival,
                        birth,
                        states,
                        neighbors,
                    }
                })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_rule() {
            let input = "4/4/5/M";
            let rule = Rule::parser().parse(input).unwrap();
            assert_eq!(
                rule,
                Rule {
                    survival: vec![4..5],
                    birth: vec![4..5],
                    states: 5,
                    neighbors: Neighbors::Moore
                }
            );
            let input = "9-26/5-7,12-13,15/5/M";
            let rule = Rule::parser().parse(input).unwrap();
            assert_eq!(
                rule,
                Rule {
                    survival: vec![9..27],
                    birth: vec![5..8, 12..14, 15..16],
                    states: 5,
                    neighbors: Neighbors::Moore
                }
            );
        }
    }
}
