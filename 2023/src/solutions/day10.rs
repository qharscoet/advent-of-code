use std::collections::{VecDeque, HashSet};

use crate::solution::Solution;

pub struct Day10;

fn get_neighbours(input : &Vec<Vec<char>>, pos : (usize,usize)) -> (Vec<(usize,usize)>, char){

    let left = if pos.1 > 0 && matches!(input[pos.0][pos.1 -1], '-'|'L'|'F') { Some((pos.0, pos.1 -1)) }else { None };
    let down = if pos.0 < input.len() && matches!(input[pos.0 +1][pos.1], '|'|'L'|'J') { Some((pos.0 + 1, pos.1)) }else { None };
    let right = if pos.1 < input[0].len() && matches!(input[pos.0][pos.1 +1], '-'|'7'|'J') { Some((pos.0, pos.1 + 1)) }else { None };
    let up = if pos.0 > 0  && matches!(input[pos.0 -1][pos.1], '|'|'7'|'F') { Some((pos.0 -1, pos.1)) }else { None };

    let mut possible_s : HashSet<char> = HashSet::from(['|','-','L','J','7','F']);
    if right.is_some(){
        println!("R{:?}", possible_s);
        possible_s = possible_s.intersection(&HashSet::from(['-', 'L', 'F'])).cloned().collect();
    }
    if up.is_some(){
        println!("U{:?}", possible_s);
        possible_s = possible_s.intersection(&HashSet::from(['|', 'L', 'J'])).cloned().collect();
    }
    if left.is_some(){
        println!("L{:?}", possible_s);
        possible_s = possible_s.intersection(&HashSet::from(['-', 'J', '7'])).cloned().collect();
    }
    if down.is_some(){
        println!("D{:?}", possible_s);
        possible_s = possible_s.intersection(&HashSet::from(['|', '7', 'F'])).cloned().collect();
    }

    println!("{:?}", possible_s);
    ([up,right,down,left].iter().flatten().cloned().collect(), *possible_s.iter().next().unwrap())
}


fn get_main_loop(input : &mut Vec<Vec<char>>) -> Vec<(usize,usize)> {
    let (pos, _) = input
            .iter()
            .enumerate()
            .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, c)| ((i, j), c)))
            .find(|(_, &c)| c == 'S')
            .unwrap();

    
    let (neighbours,s) = get_neighbours(&input, pos);
    let init_pos = pos;
    input[pos.0][pos.1] = s;

    let mut prev_pos = init_pos;
    let mut curr_pos = neighbours[0];
    let mut main_loop = vec![init_pos];
    while curr_pos != init_pos {
        let left = if curr_pos.1 > 0  { Some((curr_pos.0, curr_pos.1 -1)) }else { None };
        let down = if curr_pos.0 < input.len()  { Some((curr_pos.0 + 1, curr_pos.1)) }else { None };
        let right = if curr_pos.1 < input[0].len()  { Some((curr_pos.0, curr_pos.1 + 1)) }else { None };
        let up = if curr_pos.0 > 0   { Some((curr_pos.0 -1, curr_pos.1)) }else { None };

        let vals : Vec<_>= match input[curr_pos.0][curr_pos.1] {
        '|' => vec![up,down],
        '-' => vec![left, right],
        'L' => vec![up, right],
        'J' => vec![up, left],
        '7' => vec![left, down],
        'F' => vec![down, right],
        _ => vec![]
        }.iter().flatten().cloned().collect();

        main_loop.push(curr_pos);
        
        let new_pos = *vals.iter().find(|&&pos| pos != prev_pos ).unwrap();
        prev_pos = curr_pos;
        curr_pos = new_pos;

    }

    main_loop
}

fn display(vec : &Vec<Vec<[[u8;3];3]>>) {

    let untiled : Vec<_>= vec.iter().flat_map(|line| {
        line.iter().fold(vec![vec![],vec![],vec![]],|mut acc,arr| {
            acc[0].extend_from_slice(&arr[0]);
            acc[1].extend_from_slice(&arr[1]);
            acc[2].extend_from_slice(&arr[2]);

            acc
        })
    }
    ).collect();



    for x in untiled {
        println!("{}", x.iter().map(|c| match c {
            0 => '.',
            1 => '#',
            2 => '~',
            _ => '0',
        }).collect::<String>());
    }
}

impl Solution for Day10 {
    type Input = Vec<Vec<char>>;
    type ReturnType = u32;
    const DAY: u32 = 10;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|line| line.chars().collect()).collect()
        // transpose(&map)
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        let mut  input = input.clone();
        let main_loop = get_main_loop(&mut input);
        (main_loop.iter().count()/2) as u32
    }
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        let mut input = input.clone();
        let main_loop = get_main_loop(&mut input);
        println!("{:?}", main_loop.iter().count()/2);
        println!("{}*{}", input.len(), input[0].len());

        let mut zoomed : Vec<Vec<[[u8;3];3]>>   = vec![ vec![[[0u8;3];3]; input[0].len()]; input.len()];

        for (x, y) in main_loop {
            zoomed[x][y] = match input[x][y] {
                '|' => [[0, 1, 0], [0, 1, 0], [0, 1, 0]],
                '-' => [[0, 0, 0], [1, 1, 1], [0, 0, 0]],
                'L' => [[0, 1, 0], [0, 1, 1], [0, 0, 0]],
                'J' => [[0, 1, 0], [1, 1, 0], [0, 0, 0]],
                '7' => [[0, 0, 0], [1, 1, 0], [0, 1, 0]],
                'F' => [[0, 0, 0], [0, 1, 1], [0, 1, 0]],
                '.' => [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
                _ => zoomed[x][y],
            }
        }

        let mut queue : VecDeque<(usize,usize)>  = VecDeque::new();
        queue.push_back((0,0));

        let mut filled_tiles : HashSet<(usize,usize)> = HashSet::new();

        while let Some((r,c)) = queue.pop_front() {
            if zoomed[r/3][c/3][r%3][c%3] == 0 {
                filled_tiles.insert((r/3, c/3));

                zoomed[r/3][c/3][r%3][c%3] = 2;
                let left = if c > 0  { Some((r, c -1)) }else { None };
                let down = if r < (zoomed.len() * 3) -1  { Some((r + 1, c)) }else { None };
                let right = if c < (zoomed[0].len() * 3) -1 { Some((r, c + 1)) }else { None };
                let up = if r > 0   { Some((r -1, c)) }else { None };

                for p in  [up,right,down,left].iter().flatten() {
                    queue.push_back(*p);
                }
            }
        }
        display(&zoomed);
        

        ((input.len() * input[0].len()) - filled_tiles.len()) as u32

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

    static INPUT_TEST_3 : &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    static INPUT_TEST_4 : &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    static INPUT_TEST_5 : &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

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
    fn test_second_part_1() {
        let lines = INPUT_TEST_3.split('\n').map(|s| s.to_string());
        let input = Day10.parse_input(lines);
        assert_eq!(Day10.second_part(&input), 4)
    }

    #[test]
    fn test_second_part_2() {
        let lines = INPUT_TEST_4.split('\n').map(|s| s.to_string());
        let input = Day10.parse_input(lines);
        assert_eq!(Day10.second_part(&input), 8)
    }

    #[test]
    fn test_second_part_3() {
        let lines = INPUT_TEST_5.split('\n').map(|s| s.to_string());
        let input = Day10.parse_input(lines);
        assert_eq!(Day10.second_part(&input), 10)
    }
}