use crate::solution::Solution;

pub struct Day10;

fn transpose(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}

fn get_neighbours(input : &Vec<Vec<char>>, pos : (usize,usize)) -> Vec<(usize,usize)>{

    let up = if pos.1 > 0 && matches!(input[pos.0][pos.1 -1], '|'|'7'|'F') { Some((pos.0, pos.1 -1)) }else { None };
    let right = if pos.0 < input[0].len() && matches!(input[pos.0 +1][pos.1], '-'|'7'|'J') { Some((pos.0 + 1, pos.1)) }else { None };
    let down = if pos.1 < input.len() && matches!(input[pos.0][pos.1 +1], '|'|'L'|'J') { Some((pos.0, pos.1 + 1)) }else { None };
    let left = if pos.0 > 0  && matches!(input[pos.0 -1][pos.1], '-'|'L'|'F') { Some((pos.0 -1, pos.1)) }else { None };
    [up,right,down,left].iter().flatten().cloned().collect()
}

impl Solution for Day10 {
    type Input = Vec<Vec<char>>;
    type ReturnType = u32;
    const DAY: u32 = 10;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let map = lines.map(|line| line.chars().collect()).collect();
        transpose(&map)
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {

        let (pos, _) = input
            .iter()
            .enumerate()
            .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, c)| ((i, j), c)))
            .find(|(_, &c)| c == 'S')
            .unwrap();

        
        let neighbours = get_neighbours(&input, pos);
        let init_pos = pos;

        let mut prev_pos = init_pos;
        let mut curr_pos = neighbours[0];
        let mut count = 1;
        while curr_pos != init_pos {
            let up = if curr_pos.1 > 0  { Some((curr_pos.0, curr_pos.1 -1)) }else { None };
            let right = if curr_pos.0 < input[0].len()  { Some((curr_pos.0 + 1, curr_pos.1)) }else { None };
            let down = if curr_pos.1 < input.len()  { Some((curr_pos.0, curr_pos.1 + 1)) }else { None };
            let left = if curr_pos.0 > 0   { Some((curr_pos.0 -1, curr_pos.1)) }else { None };

            let vals : Vec<_>= match input[curr_pos.0][curr_pos.1] {
            '|' => vec![up,down],
            '-' => vec![left, right],
            'L' => vec![up, right],
            'J' => vec![up, left],
            '7' => vec![left, down],
            'F' => vec![down, right],
            _ => vec![]
            }.iter().flatten().cloned().collect();

            
            let new_pos = *vals.iter().find(|&&pos| pos != prev_pos ).unwrap();
            prev_pos = curr_pos;
            curr_pos = new_pos;

            // println!("{:?}", curr_pos);
            count += 1;
        }
        count /2
    }
    fn second_part(&self, _input: &Self::Input) -> Self::ReturnType {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day10;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    static INPUT_TEST_2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn test_first_part_1() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day10.parse_input(lines);
        assert_eq!(Day10.first_part(&input), 4)
    }

    #[test]
    fn test_first_part_2() {
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day10.parse_input(lines);
        assert_eq!(Day10.first_part(&input), 8)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day10.parse_input(lines);
        assert_eq!(Day10.second_part(&input), u32::MAX)
    }
}
