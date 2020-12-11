use std::fs::File;
use std::io::{BufRead, BufReader};

fn step(grid: &mut Vec<Vec<char>>) -> bool
{
    let mut new_grid :Vec<Vec<char>> = grid.clone();
    let h = grid.len();
    let w = grid[0].len();
    let mut changed = false;

    for i in 0..h {
        for j in 0..w {
            let y_range = i.saturating_sub(1)..=std::cmp::min(i+1, h -1);
            let x_range = j.saturating_sub(1)..=std::cmp::min(j+1, w -1);
            let occupied_neigbours =  &grid[y_range].iter().flat_map(|row| &row[x_range.clone()]).filter(|&&c| c == '#').count();

            new_grid[i][j] = match grid[i][j] {
                'L' => if *occupied_neigbours == 0 {changed = true; '#'} else {'L'},
                // -1 because range includes the seat itself so we sub if it's occupied
                '#' => if *occupied_neigbours - 1 >= 4 {changed = true; 'L'} else {'#'},
                _ => {'.'}
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            grid[i][j] = new_grid[i][j];
        }
    }
    changed
}

fn display(grid: &[Vec<char>])
{
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = buf_reader
        .lines()
        .map(|line| line.unwrap().trim().chars().collect())
        .collect();



    while step(&mut grid) {}
    display(&grid);
    println!("Hello, world! Answer is : {:?}", grid.iter().flatten().filter(|&&c| c == '#').count());
}