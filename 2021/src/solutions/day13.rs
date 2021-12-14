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

fn display(diagram: &Diagram) {
    let xmax = diagram.iter().max_by_key(|(x,_y)| x).unwrap().0;
    let ymax = diagram.iter().max_by_key(|(_x,y)| y).unwrap().1;

    println!("{:?}", diagram);
    println!("{:?}", xmax);
    println!("{:?}", ymax);

    for y in 0..=ymax {
        for x in 0..=xmax {
            print!("{}", if diagram.contains(&(x,y)) {'#'} else {' '});
        }
        print!("\n");
    }
}

fn fold_diagram( diagram : &mut Diagram, fold : &Fold) {
    let positions : Vec<((u32,u32), (u32,u32))> = diagram.iter().filter_map(|(x,y)| match fold {
        Fold::Horizontal(v) => {if x > v {Some( ((*x,*y), (v - (x - v), *y)) )} else {None}},
        Fold::Vertical(v) => { if y > v {Some( ((*x,*y), (*x, v - (y - v)))) } else {None}}
    }).collect();

    for (old_pos, new_pos) in positions {
        diagram.remove(&old_pos);
        diagram.insert(new_pos);
    }
    
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

        fold_diagram(&mut paper.diagram, &first_fold);
        
        paper.diagram.len() as u32
    }

    fn second_part(&self, input: &Self::Input) -> u32 {
        let mut paper = input.clone();
        for fold in &input.folds {
            fold_diagram(&mut paper.diagram, fold);
        }

        display(&paper.diagram);
        paper.diagram.len() as u32
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
