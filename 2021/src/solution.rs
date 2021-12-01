use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

fn input_file(day: i32) -> String {
    format!("./inputs/day{}.txt", day)
}

pub trait Solution {
    type Input;
    fn parse_input(&self, r: BufReader<File>) -> Self::Input;
    fn first_part(&self, input: &Self::Input) -> i32;
    fn second_part(&self, input: &Self::Input) -> i32;

    fn  load_input<P: AsRef<Path>>(&self, p: P) -> io::Result<Self::Input> {
        let file = File::open(p)?;
        let buf_reader = BufReader::new(file);
        Ok(self.parse_input(buf_reader))
    }

    fn solve(&self, day: i32) {
        let input_file = input_file(day);
        let input = self
            .load_input(input_file)
            .expect("unable to open input file");
        let s1 = self.first_part(&input);
        let s2 = self.second_part(&input);
        println!("Solution 1: {}", s1);
        println!("Solution 2: {}", s2);
    }
}