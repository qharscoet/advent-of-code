use core::num;

use crate::solution::Solution;

pub struct Day1;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

pub struct Rotation {
    dir:Direction,
    value : u32
}

impl std::str::FromStr for Rotation {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, value) = s.split_at(1);

        let v: u32;
        match value.trim().parse() {
            Err(_e) => return Err("value is not an int"),
            Ok(data) => v = data,
        }

        match direction {
            "L" => Ok(Rotation{dir:Direction::Left, value : v}),
            "R" => Ok(Rotation{dir:Direction::Right, value : v}),
            _ => Err("Rotation not valid"),
        }
    }
}

impl Solution for Day1 {
    type Input = Vec<Rotation>;
    type ReturnType = u32;
    const DAY : u32 = 1;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.flat_map(|line| line.parse()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        input.iter().fold((50,0), |(acc, count), rot| {
            let new_acc= match rot.dir {
                Direction::Right => (acc + rot.value)%100,
                Direction::Left => (acc + 100 - rot.value%100)%100, //+100 is to stay always positive as we're unsigned
            };

            (new_acc, count + (new_acc == 0) as u32)
        }).1
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        input.iter().fold((50,0), |(acc, count), rot| {
            let number_zero = rot.value/100;
            let v = rot.value%100;
            let (new_acc, overflow) = match rot.dir {
                Direction::Right => ((acc + v)%100, acc + v > 100),
                Direction::Left => if let Some(res) = acc.checked_sub(v) {(res, false)} else { (acc + 100 - v, acc != 0) }, //+100 is to stay always positive as we're unsigned, acc !0 is to not count double (is 0 + overflow)
            };

            let new_count = count + number_zero + (new_acc == 0) as u32 + overflow as u32;
            (new_acc, new_count)
        }).1
    }
}

#[cfg(test)]
mod tests {
    use super::Day1;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";


    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day1.parse_input(lines);
        assert_eq!(Day1.first_part(&input), 3)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day1.parse_input(lines);
        assert_eq!(Day1.second_part(&input), 6);
    }
}
