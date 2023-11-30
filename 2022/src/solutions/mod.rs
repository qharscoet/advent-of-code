pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;

use crate::solution::Solution;

pub fn run(day:u32) {
    match day {
        1 => { day1::Day1.solve(1); }
        2 => { day2::Day2.solve(2); }
        3 => { day3::Day3.solve(3); }
        4 => { day4::Day4.solve(4); }
        5 => { day5::Day5.solve(5); }
        6 => { day6::Day6.solve(6); }
        7 => { day7::Day7.solve(7); }
        8 => { day8::Day8.solve(8); }
        9 => { day9::Day9.solve(9); }
        10 => { day10::Day10.solve(10); }
        11 => { day11::Day11.solve(11); }
        12 => { day12::Day12.solve(12); }
        13 => { day13::Day13.solve(13); }
        14 => { day14::Day14.solve(14); }
        _ => {}
    };
}