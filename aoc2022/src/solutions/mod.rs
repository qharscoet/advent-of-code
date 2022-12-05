pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

use crate::solution::Solution;

pub fn run(day:u32) {
    match day {
        1 => { day1::Day1.solve(1); }
        2 => { day2::Day2.solve(2); }
        3 => { day3::Day3.solve(3); }
        4 => { day4::Day4.solve(4); }
        5 => { day5::Day5.solve(5); }
        _ => {}
    };
}