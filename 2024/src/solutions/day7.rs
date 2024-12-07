use crate::solution::Solution;

pub struct Day7;

type FnOperator = fn(u64, u64) -> u64;

fn concat(a : u64, b:u64) -> u64 {
    a * (10u64.pow(b.ilog10() +1)) +b
}

pub struct Equation {
    result : u64,
    values: Vec<u32>
}

impl std::str::FromStr for Equation {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':')
        {
            Some((result, values)) =>  Ok(Equation{ result: result.parse().unwrap_or_default(), values : values.split_ascii_whitespace().flat_map(|s| s.parse()).collect()}),
            None => return Err("no : to parse"),
        }
    }
}

impl Equation {

    fn valid(&self,idx :usize, acc: u64, operators : &Vec<FnOperator>) -> bool {

        if acc > self.result {return false;}
        if idx == self.values.len(){return acc == self.result;}

        let v = self.values[idx];

        operators.iter().any(|op| self.valid(idx +1, op(acc, v as u64), operators))
    }
    
    fn is_valid_p1(&self) -> bool {
        self.valid(1, self.values[0] as u64, &vec![|a,b| a + b, |a,b| a*b])
    }

    fn is_valid_p2(&self) -> bool {
        self.valid(1, self.values[0] as u64, &vec![|a,b| a + b, |a,b| a*b, |a,b| concat(a, b)])
    }

}

impl Solution for Day7 {
    type Input = Vec<Equation>;
    type ReturnType = u64;
    const DAY : u32 = 7;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.flat_map(|l|l.parse()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.iter().filter_map(|eq| if eq.is_valid_p1() {Some(eq.result)} else {None}).sum()
    }

    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {

        input.iter().filter_map(|eq| if eq.is_valid_p2() {Some(eq.result)} else {None}).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day7;
    use crate::{solution::Solution, solutions::day7::concat};

    static INPUT_TEST: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day7.parse_input(lines);
        assert_eq!(Day7.first_part(&input), 3749)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day7.parse_input(lines);
        assert_eq!(Day7.second_part(&input), 11387);
    }

    #[test]
    fn test_concat_operator(){
        assert_eq!(concat(12, 34), 1234);
        assert_eq!(concat(450, 9), 4509);
        assert_eq!(concat(123456789, 7777), 1234567897777);
    }
}
