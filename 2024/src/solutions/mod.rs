pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;


use crate::solution::Solution;

pub fn run(day:u32) {
    match day {
        1 => { day1::Day1.solve(); }
        2 => { day2::Day2.solve(); }
        3 => { day3::Day3.solve(); }
        4 => { day4::Day4.solve(); }
        _ => {}
    };
}