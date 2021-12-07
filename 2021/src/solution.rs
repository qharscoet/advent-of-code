use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fmt::Display;

fn input_file(day: i32) -> String {
    format!("./src/inputs/day{}.txt", day)
}

pub trait Solution {
    type Input;
    type ReturnType: Display;

    fn parse_input(&self, r : impl Iterator<Item=std::string::String>) -> Self::Input;
    fn first_part(&self, input: &Self::Input) -> Self::ReturnType;
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType;

    fn load_input<P: AsRef<Path>>(&self, p: P) -> io::Result<Self::Input> {
        let file = File::open(p)?;
        let buf_reader = BufReader::new(file);
        Ok(self.parse_input(buf_reader.lines().flatten()))
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
