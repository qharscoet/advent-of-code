mod solutions;

use chrono::{Datelike, Utc};

mod solution;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day: u32 = match args.len() {
        2 => args[1].trim().parse().expect("Not a number"),
        _ => Utc::now().day()
    };

    solutions::run(day);
}
