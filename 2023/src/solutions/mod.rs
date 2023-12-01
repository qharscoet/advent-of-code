pub mod day1;

use crate::solution::Solution;

pub fn run(day:u32) {
    match day {
        1 => { day1::Day1.solve(1); }
        _ => {}
    };
}