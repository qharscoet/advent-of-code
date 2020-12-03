use std::fs::File;
use std::io::{BufRead, BufReader};


fn count_trees(grid: Vec<Vec<char>>) -> u32
{
    let mut curr_line = 0;
    let mut curr_col = 0;
    let mut count : u32 = 0;
    while curr_line < grid.len() {
        if grid[curr_line][curr_col] == '#'{
            count += 1;
        }

        curr_col = (curr_col + 3) % grid[curr_line].len();
        curr_line += 1;
    }

    return count;
}


//Main
fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let grid : Vec<Vec<char>> = buf_reader
        .lines()
        .map(|line| line.unwrap().trim().chars().collect())
        .collect();
    println!("Hello, world! Result is : \n{:?}", count_trees(grid));
}
