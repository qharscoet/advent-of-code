use crate::solution::Solution;
use std::collections::HashSet;

pub struct Day13;

type Diagram = HashSet<(u32, u32)>;

#[derive(Clone)]
enum Fold {
    Horizontal(u32),
    Vertical(u32)
}

#[derive(Clone)]
pub struct Paper {
    diagram : Diagram,
    folds : Vec<Fold>

}

impl Solution for Day13 {
    type Input = Paper;
    type ReturnType = u32;

    fn parse_input(&self, mut lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let diagram : Diagram = lines
            .by_ref()
            .take_while(|line| line != "")
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap_or_default();

                (
                    x.parse::<u32>().unwrap_or_default(),
                    y.parse::<u32>().unwrap_or_default(),
                )
            })
            .collect();

        let folds : Vec<Fold> = lines.flat_map( |line| {
            let (left, val) = line.split_once("=").unwrap_or_default();
            
            match left.chars().last().unwrap() {
                'x' => Some(Fold::Horizontal(val.parse::<u32>().unwrap_or_default())),
                'y' => Some(Fold::Vertical(val.parse::<u32>().unwrap_or_default())),
                _ => None
            }
        }).collect();

        Paper{diagram:diagram, folds:folds}
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        let mut paper = input.clone();
        let first_fold = &paper.folds[0];

        let positions : Vec<((u32,u32), (u32,u32))> = paper.diagram.iter().filter_map(|(x,y)| match first_fold {
            Fold::Horizontal(v) => {if x > v {Some( ((*x,*y), (v - (x - v), *y)) )} else {None}},
            Fold::Vertical(v) => { if y > v {Some( ((*x,*y), (*x, v - (y - v)))) } else {None}}
        }).collect();

        for (old_pos, new_pos) in positions {
            paper.diagram.remove(&old_pos);
            paper.diagram.insert(new_pos);
        }
        
        
        paper.diagram.len() as u32
    }

    fn second_part(&self, _input: &Self::Input) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;

    fn test_input_to_string_iter() -> Vec<String> {
        vec![
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn test_first_part() {
        let input = Day13.parse_input(test_input_to_string_iter().into_iter());
        assert_eq!(Day13.first_part(&input), 17)
    }

    #[test]
    fn test_second_part() {
        let input = Day13.parse_input(test_input_to_string_iter().into_iter());
        assert_eq!(Day13.second_part(&input), 195)
    }
}
