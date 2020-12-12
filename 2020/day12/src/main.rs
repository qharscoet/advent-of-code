use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Action{
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(u32),
    Right(u32),
    Forward(i32),
}

impl std::str::FromStr for Action {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, value) = s.split_at(1);


        let v: i32;
        match value.trim().parse(){
            Err(_e) => return Err("v is not an int"),
            Ok(data) => v = data,
        }

        match action {
            "N" => Ok(Action::North(v)),
            "S" => Ok(Action::South(v)),
            "E" => Ok(Action::East(v)),
            "W" => Ok(Action::West(v)),
            "L" => Ok(Action::Left(v as u32)),
            "R" => Ok(Action::Right(v as u32)),
            "F" => Ok(Action::Forward(v)),
            _ => Err("Action not valid")
        }
    }
}

struct Ship{
    pos:(i32,i32), // N/S, E/W
    dir: u32, // [0-360]
}

impl Ship {
    const COS_LUT: [i32;4] = [1, 0, -1, 0];
    const SIN_LUT: [i32;4] = [0, 1, 0, -1];

    fn new() -> Ship {
        Ship { pos: (0,0), dir:0}
    }

    fn process_action(&mut self, a: &Action){
        match a {
            Action::North(v) => self.pos.0 += v,
            Action::South(v) => self.pos.0 -= v,
            Action::East(v) => self.pos.1 += v,
            Action::West(v) => self.pos.1 -= v,
            Action::Left(v) => {self.dir += v; self.dir %= 360;}
            Action::Right(v) => { if v > &self.dir {self.dir += 360 - v;} else {self.dir -= v;}}
            Action::Forward(v) => {
                let idx = (self.dir/90) as usize;
                self.pos = (self.pos.0 + Ship::SIN_LUT[idx] * v, self.pos.1 + Ship::COS_LUT[idx] * v);
            }
        }
    }

    fn manathan(&self) -> i32 {
        self.pos.0.abs() + self.pos.1.abs()
    }
}



fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let actions: Vec<Action> = buf_reader
        .lines()
        .flat_map(|line| line.unwrap().trim().parse())
        .collect();

    let mut ship = Ship::new();

    actions.iter().for_each(|a| ship.process_action(a));

    println!("Hello, world! {:?}", ship.manathan());
}
