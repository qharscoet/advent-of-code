use std::fs::File;
use std::io::{BufRead, BufReader};

type Grid = Vec<Vec<char>>;
type FnCount = fn(&Grid, (usize, usize)) -> usize;

fn first_seat_occupied_dir(grid: &Grid, mut pos: (usize, usize), dir: (i32, i32)) -> bool {
    loop {
        pos.0 = ((pos.0 as i32) + dir.0) as usize;
        pos.1 = ((pos.1 as i32) + dir.1) as usize;

        if !(0..grid.len()).contains(&pos.0) || !(0..grid[0].len()).contains(&pos.1) {
            return false;
        } else if grid[pos.0][pos.1] != '.' {
            return grid[pos.0][pos.1] == '#';
        }
    }
}

fn count_occupied_in_sight(grid: &Grid, pos: (usize, usize)) -> usize {
    let (i, j) = pos;
    let dirs = vec![(-1, -1),(-1, 0),(-1, 1),(0, -1),(0, 1),(1, -1),(1, 0),(1, 1),];
    dirs.iter()
        .map(|d| first_seat_occupied_dir(&grid, (i, j), *d))
        .filter(|b| *b)
        .count()
}

fn count_adjacent_occupied(grid: &Grid, pos: (usize, usize)) -> usize {
    let (i, j) = pos;
    let y_range = i.saturating_sub(1)..=std::cmp::min(i + 1, grid.len() - 1);
    let x_range = j.saturating_sub(1)..=std::cmp::min(j + 1, grid[0].len() - 1);
    grid[y_range]
        .iter()
        .flat_map(|row| &row[x_range.clone()])
        .filter(|&&c| c == '#')
        .count()
}


fn step(grid: &mut Grid, count_fn: FnCount, seat_tolerance : usize) -> bool
{
    let mut new_grid :Grid = grid.clone();
    let h = grid.len();
    let w = grid[0].len();
    let mut changed = false;

    for i in 0..h {
        for j in 0..w {
            let occupied_neigbours = count_fn(grid, (i,j));

            new_grid[i][j] = match grid[i][j] {
                'L' => if occupied_neigbours == 0 {changed = true; '#'} else {'L'},
                '#' => if occupied_neigbours >= seat_tolerance {changed = true; 'L'} else {'#'},
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


fn step_part2(grid: &mut Grid) -> bool {
    step(grid, count_occupied_in_sight, 5)
}
fn step_part1(grid: &mut Grid) -> bool {
    step(grid, count_adjacent_occupied, 5)
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
    let grid: Grid = buf_reader
        .lines()
        .map(|line| line.unwrap().trim().chars().collect())
        .collect();

    let mut new_grid = grid.clone();
    while step_part1(&mut new_grid) {}
    println!("Hello, world! Answer fort part 1 is : {}", new_grid.iter().flatten().filter(|&&c| c == '#').count());

    new_grid = grid.clone();
    while step_part2(&mut new_grid) {}
    // display(&grid);
    println!("Hello, world! Answer for Part 2 is : {}", new_grid.iter().flatten().filter(|&&c| c == '#').count());
}