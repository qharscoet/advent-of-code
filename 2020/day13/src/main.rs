use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines().flatten();
    let arrival_time : u32 = lines.next().expect("Not line").trim().parse().expect("Not a number");
    let buses_id : Vec<u32> = lines.next().unwrap().split(',').filter_map(|s| s.parse().ok()).collect();

    let result = buses_id.iter().map(|id| (((arrival_time/id) + 1) * id - arrival_time, id)).min().unwrap();
    println!("Hello, world! Contents is : \n{:?}", result.0 * result.1);
}
