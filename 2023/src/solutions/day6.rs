use crate::solution::Solution;

pub struct Day6;

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Race {
    time : u64,
    distance: u64
}

impl Solution for Day6 {
    type Input = Vec<Race>;
    type ReturnType = u64;
    const DAY: u32 = 6;

    fn parse_input(&self, _lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        vec![
            Race{time:62, distance:553},
            Race{time:64, distance:1010},
            Race{time:91, distance:1473},
            Race{time:90, distance:1074}
        ]
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input
            .iter()
            .map(|race| {
                (0..=race.time).filter(|t| t * (race.time - t) > race.distance).count() as u64
            })
            .product()
    }
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        let race = input
            .iter()
            .copied()
            .reduce(|acc, r| Race {
                time: acc.time * 10u64.pow(r.time.ilog10() + 1) + r.time,
                distance: acc.distance * 10u64.pow(r.distance.ilog10() + 1) + r.distance,
            })
            .unwrap();

        println!("{:?}", race);
        (0..=race.time).filter(|t| t * (race.time - t) > race.distance).count() as u64
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
            Race{time:15, distance:40},
            Race{time:30, distance:200}
        ];
        assert_eq!(Day6.second_part(&input), 71503)
    }
}
