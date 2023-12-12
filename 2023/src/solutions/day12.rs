use crate::solution::Solution;

pub struct Day12;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SpringState {
    Broken,
    Operational,
    Unknown
}

#[derive(Debug)]
pub struct Row {
    springs : Vec<SpringState>,
    //springs : String,
    groups : Vec<u32>
}

impl std::str::FromStr for Row {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (states, groups_str) = s.split_once(' ').expect("Invalid state/groups");

        let springs = states.chars().map(|c| match c {
            '.' => Some(SpringState::Operational),
            '#' => Some(SpringState::Broken),
            '?' => Some(SpringState::Unknown),
            _ => None
        }.expect("Invalid state")).collect();

        let groups = groups_str.split(',').map(|v| v.parse().expect("Not a valid u32")).collect();
       
        Ok(Row{springs:springs, groups:groups})
    }

}

fn get_groups(springs : &Vec<SpringState>) -> Vec<u32> {
    //Could be implemented easily with group_by https://doc.rust-lang.org/std/primitive.slice.html#method.group_by

    let mut groups: Vec<Vec<SpringState>> = Vec::new();
    let mut curr_group : Vec<SpringState> = Vec::new();
    let mut curr_s = springs[0];
    for s in springs.clone() {
        if s == curr_s {
            curr_group.push(s);
        }
        else {
            groups.push(curr_group.clone());
            curr_group.clear();
            curr_s = s;
            curr_group.push(curr_s);
        }
    }
    groups.push(curr_group);
    groups.retain(|g| g[0] == SpringState::Broken);

    groups.iter().map(|g| g.len() as u32).collect()
}


impl Row {
    fn is_valid(&self) -> bool {

        let groups = get_groups(&self.springs);

        //let groups : Vec<_>= self.springs.split('.').filter(|g| !g.is_empty() && g.as_bytes()[0] != ('?' as u8)).map(|s| s.len() as u32).collect();

        groups == self.groups
    }

    fn is_solvable(&self) -> bool {
        let copy : Vec<_> = self.springs.iter().map(|s| match s {
            SpringState::Unknown => SpringState::Broken,
            _ => *s
        }).collect();

        let broken_max_count = copy.iter().filter(|&c| matches!(c,SpringState::Broken)).count() as u32;
        
        let enough_broken = broken_max_count >= self.groups.iter().sum();

        let groups  = get_groups(&copy);
        let group_ok =  (broken_max_count > self.groups.iter().sum())  || (broken_max_count == self.groups.iter().sum() && groups.len() == self.groups.len());

        let max_group_ok = groups.iter().max() >= self.groups.iter().max();

        let trail_groups = groups[0] >= self.groups[0] && groups.last().unwrap() >= self.groups.last().unwrap(); 
        
        // println!("{}, {}, {}, {}", enough_broken, group_ok, max_group_ok, trail_groups);
        enough_broken && group_ok && max_group_ok && trail_groups
    }

    fn get_valid_count(&self) -> u32 {
        let idx = self.springs.iter().position(|s| matches!(s, SpringState::Unknown));

        if let Some(i) = idx {
            let mut copy = self.springs.clone();
            copy[i] = SpringState::Operational;
            let r = Row {springs:copy.clone(), groups:self.groups.clone()};

            copy[i] = SpringState::Broken;
            let r2 = Row {springs:copy, groups:self.groups.clone()};

            r.get_valid_count() + r2.get_valid_count()
        } else {
            self.is_valid() as u32
        }
    }
}

impl Solution for Day12 {
    type Input = Vec<Row>;
    type ReturnType = u32;
    const DAY : u32 = 12;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|line| line.parse().unwrap()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.iter().map(|row| row.get_valid_count()).sum()
    }
    fn second_part(&self, _input: &Self::Input) -> Self::ReturnType {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::{Day12, Row};
    use crate::solution::Solution;

    static INPUT_TEST: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    static INPUT_TEST_2 : &str = "";

    #[test]
    fn test_row() {
        let r : Row = "#.#.### 1,1,3".parse().unwrap();
        assert!(r.is_valid())
    }

    #[test]
    fn test_row_count() {
        let mut r : Row = "???.### 1,1,3".parse().unwrap();
        assert_eq!(r.get_valid_count(), 1);

        r  = ".??..??...?##. 1,1,3".parse().unwrap();
        assert_eq!(r.get_valid_count(), 4)
    }

    #[test]
    fn test_solvable() {
        let mut  r : Row = ".??.### 1,1,3".parse().unwrap();
        assert!(!r.is_solvable());

        r = "???.### 1,1,3".parse().unwrap();
        assert!(r.is_solvable());

        r = ".??..??...?##. 1,1,3".parse().unwrap();
        assert!(r.is_solvable());
        r = ".??..??....##. 1,1,3".parse().unwrap();
        assert!(!r.is_solvable());

        r = "?#?#?#?#?#?#?#? 1,3,1,6".parse().unwrap();
        assert!(r.is_solvable());
        r = "?#?#?#?#?#?#.#? 1,3,1,6".parse().unwrap();
        assert!(!r.is_solvable());
    }

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day12.parse_input(lines);
        assert_eq!(Day12.first_part(&input), 21)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day12.parse_input(lines);
        assert_eq!(Day12.second_part(&input),525152)
    }
}
