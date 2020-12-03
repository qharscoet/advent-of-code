use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_trees(grid: &Vec<Vec<char>>, slope: (i32, i32)) -> u32 {
    let (right, down) = slope;

    let mut curr_line = 0;
    let mut curr_col = 0;
    let mut count = 0;
    while curr_line < grid.len() {
        if grid[curr_line][curr_col] == '#' {
            count += 1;
        }

        curr_col = (curr_col + (right as usize)) % grid[curr_line].len();
        curr_line += down as usize;
    }

    return count;
}

//Main
fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let grid: Vec<Vec<char>> = buf_reader
        .lines()
        .map(|line| line.unwrap().trim().chars().collect())
        .collect();

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    println!(
        "Hello, world! Result is : \n{:?}",
        slopes.iter().fold(1, |acc, &s| acc * count_trees(&grid, s))
    );
}
