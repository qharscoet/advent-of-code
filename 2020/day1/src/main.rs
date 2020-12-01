use std::fs::File;
use std::io::{BufReader, BufRead};

fn find_values(values: &Vec<i32>, target_value: i32) -> i32 {
    for v in values{
        for v2 in values {
            if  v + v2 == target_value {
                return v * v2
            }
        }
    }
    0
}

fn find_3_values(values: &Vec<i32>, target_value: i32) -> i32 {
    for v in values{
            let ret = find_values(values, target_value - v);
            if ret != 0 {
                return v * ret;
            }
        }
    0
}


fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let values : Vec<i32> = buf_reader.lines().map(|line| line.unwrap().trim().parse().expect("Not a number") ).collect();
    println!("Hello, world! Contents is : \n{:?}", values);
    println!("Result : {}", find_values(&values, 2020));
    println!("Result : {}", find_3_values(&values, 2020));
}
