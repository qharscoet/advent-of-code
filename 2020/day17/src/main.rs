use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = (i32, i32, i32);
struct Universe {
    grid: HashSet<Coord>,
    min_dim: (i32, i32, i32),
    max_dim: (i32, i32, i32),
}

impl Universe {
    fn compute_min_max(&mut self) {
        self.min_dim = (
            self.grid.iter().min_by_key(|c| c.0).unwrap().0,
            self.grid.iter().min_by_key(|c| c.1).unwrap().1,
            self.grid.iter().min_by_key(|c| c.2).unwrap().2,
        );
        self.max_dim = (
            self.grid.iter().max_by_key(|c| c.0).unwrap().0,
            self.grid.iter().max_by_key(|c| c.1).unwrap().1,
            self.grid.iter().max_by_key(|c| c.2).unwrap().2,
        );
    }

    fn count_active_neighbours(&self, cell: (i32, i32, i32)) -> u32 {
        let mut count = 0;
        for i in cell.0 - 1..=cell.0 + 1 {
            for j in cell.1 - 1..=cell.1 + 1 {
                for k in cell.2 - 1..=cell.2 + 1 {
                    if (i, j, k) != cell && self.grid.contains(&(i, j, k)) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn new(grid: HashSet<Coord>) -> Universe {
        let mut universe = Universe {
            grid: grid,
            min_dim: (0, 0, 0),
            max_dim: (0, 0, 0),
        };

        universe.compute_min_max();
        universe
    }

    fn step(&mut self) {
        let mut new_grid: HashSet<Coord> = HashSet::new();

       for i in self.min_dim.0-1..=self.max_dim.0+1 {
            for j in self.min_dim.1-1..=self.max_dim.1+1  {
                for k in self.min_dim.2-1..=self.max_dim.2+1  {
                    let active_neigbours = self.count_active_neighbours((i,j,k));
                    if (self.grid.contains(&(i,j,k)) && (active_neigbours == 2 || active_neigbours == 3) ) ||
                        ( !self.grid.contains(&(i,j,k)) && active_neigbours == 3  ){
                        new_grid.insert((i,j,k));
                    }
                }
            }
        }

        self.grid = new_grid.clone();
        self.compute_min_max();
    }


    fn display(&self) {
        for i in self.min_dim.2..=self.max_dim.2 {
            println!("z = {}", i);

            for j in self.min_dim.0..=self.max_dim.0  {
                for k in self.min_dim.1..=self.max_dim.1  {
                    print!("{}", if self.grid.contains(&(j, k, i)) { '#' } else { '.' })
                }
                print!("\n");
            }
            println!();
        }
    }
}




fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let grid: HashSet<Coord> =
        buf_reader
            .lines()
            .enumerate()
            .fold(HashSet::new(), |acc, (idx_l, line)| {
                let line_set = line
                    .unwrap()
                    .trim()
                    .chars()
                    .enumerate()
                    .filter_map(|(idx_c, c)| {
                        if c == '#' {
                            Some((idx_l as i32, idx_c as i32, 0i32))
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<Coord>>();
                acc.union(&line_set).cloned().collect()
            });

    let mut universe = Universe::new(grid);
    for _ in 0..6 {
        universe.step();
    }
    universe.display();

    println!("Hello, world! Answer fort part 1 is : {}", universe.grid.len());

}
