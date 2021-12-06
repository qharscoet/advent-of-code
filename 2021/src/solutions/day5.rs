use crate::solution::Solution;

use std::collections::HashMap;
use std::cmp;

pub struct Day5;

type Diagram = HashMap<(u32, u32), u8>;


fn parse_point((x,y) : (&str, &str)) -> (u32,u32) {
    ( x.parse().expect("Not a number"), y.parse().expect("Not a Number"))
}
fn parse_line(diagram: &mut Diagram, line:&str) {
    let (src, dst) = line.split_once(" -> ").unwrap_or_default();
    let (x1, y1) = parse_point(src.split_once(',').unwrap_or_default());
    let (x2, y2) = parse_point(dst.split_once(',').unwrap_or_default());

    /* !!! UNCOMMENT FOR PART 1  !!!*/
    // if x1 != x2 && y1 != y2 {return;}
    
    //points as i32 so that we can substract and get neg distance
    let (x1, y1) = (x1 as i32, y1 as i32);
    let (x2, y2) = (x2 as i32, y2 as i32);

    let d = cmp::max(i32::abs(x2 - x1), i32::abs(y2 - y1));
    let v = ((x2 - x1)/d, (y2 - y1)/d);

    //starting point
    let mut p = (x1,y1);

    while p != (x2,y2) {
        diagram.entry((p.0 as u32, p.1 as u32)).and_modify(|e| {*e += 1 }).or_insert(1);
        p.0 += v.0;
        p.1 += v.1;
    }

    //One last time because intervals are
    diagram.entry((p.0 as u32, p.1 as u32)).and_modify(|e| {*e += 1 }).or_insert(1);


}


impl Solution for Day5 {
    type Input = Diagram;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let mut diagram : Diagram = Diagram::new();
        for l in lines {
            parse_line(&mut diagram, &l)
        }

        diagram
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        input.iter().filter(|(_key, &val)| val >= 2).count() as u32
    }

    fn second_part(&self, input: &Self::Input) -> u32 {
        input.iter().filter(|(_key, &val)| val >= 2).count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;

    fn display(diagram: &Diagram) {
        let xmax = diagram.keys().max_by_key(|(x,_y)| x).unwrap().0;
        let ymax = diagram.keys().max_by_key(|(_x,y)| y).unwrap().1;
    
        println!("{:?}", diagram);
        println!("{:?}", xmax);
        println!("{:?}", ymax);
    
        for y in 0..=ymax {
            for x in 0..=xmax {
                print!("{}", if diagram.contains_key(&(x,y)) {(*diagram.get(&(x,y)).unwrap_or(&0) + 48u8) as char} else {'.'});
            }
    
            print!("\n");
            
        }
    }

    fn test_input_to_string_iter() -> Vec<String> {
        vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn test_first_part() {
        let input = Day5.parse_input(test_input_to_string_iter().into_iter());
        display(&input);
        assert_eq!(Day5.first_part(&input), 5)
    }

    #[test]
    fn test_second_part() {
        let input = Day5.parse_input(test_input_to_string_iter().into_iter());
        assert_eq!(Day5.second_part(&input), 12)
    }
}
