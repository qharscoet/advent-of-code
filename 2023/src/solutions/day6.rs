use crate::solution::Solution;

pub struct Day6;

pub struct Race {
    time : u32,
    distance: u32
}

impl Solution for Day6 {
    type Input = Vec<Race>;
    type ReturnType = u32;
    const DAY: u32 = 6;

    fn parse_input(&self, _lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        vec![
            Race{time:62, distance:553},
            Race{time:64, distance:1010},
            Race{time:91, distance:1473},
            Race{time:90, distance:1074}
        ]
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        input
            .iter()
            .map(|race| {
                (0..=race.time).filter(|t| t * (race.time - t) > race.distance).count() as u32
            })
            .product()
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day6;
    use crate::{solution::Solution, solutions::day6::Race};

    #[test]
    fn test_first_part() {
        let input = vec![
            Race{time:7, distance:9},
            Race{time:15, distance:40},
            Race{time:30, distance:200}
        ];
        assert_eq!(Day6.first_part(&input), 288)
    }

    #[test]
    fn test_second_part() {
        let input = vec![
            Race{time:7, distance:9},
            Race{time:15, distance:30},
            Race{time:30, distance:200}
        ];
        assert_eq!(Day6.second_part(&input), u32::MAX)
    }
}
