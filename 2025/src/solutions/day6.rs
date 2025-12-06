use crate::solution::Solution;

pub struct Day6;

#[derive(Debug)]
pub enum Operator {
    Invalid,Plus, Multiply
}

#[derive(Debug)]
pub struct Problem {
    numbers : Vec<u64>,
    op : Operator
}

impl Problem {
    fn compute(&self) -> u64 {
        match self.op {
            Operator::Invalid => 0,
            Operator::Plus => self.numbers.iter().sum(),
            Operator::Multiply => self.numbers.iter().product(),
        }
    }
}

impl Solution for Day6 {
    type Input = [Vec<Problem>;2];
    type ReturnType = u64;
    const DAY : u32 = 6;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {

        let lines_vec : Vec<String> = lines.collect();
        let mut problems : Vec<Problem> = vec![];
        {
            for line in &lines_vec {
                let elements : Vec<&str> = line.split_whitespace().collect();   
    
                if problems.len() < elements.len() { 
                    problems.resize_with(elements.len(), || Problem{numbers : vec![], op : Operator::Invalid});
                }
    
                let zipped = problems.iter_mut().zip(elements.iter());
                for (p, &e) in zipped {
                    if let Ok(v) = e.parse::<u64>() {
                        p.numbers.push(v);
                    } else {
                        p.op = match e {
                            "+" => Operator::Plus,
                            "*" => Operator::Multiply,
                            _ => Operator::Invalid
                        }
                    }
                }
            }
        }

        let mut problems_p2 : Vec<Problem> = vec![];
        {
            let mut transposed_lines : Vec<String> = (0..lines_vec[0].len())
                .map(|i| lines_vec.iter().flat_map(|row| row.chars().nth(i)).collect())
                .collect();

            for p in transposed_lines.split_mut(|s| s.trim().is_empty()) {
                let op = match p[0].chars().last().unwrap() {
                    '+' => Operator::Plus,
                    '*' => Operator::Multiply,
                    _ => Operator::Invalid
                };

                p[0].pop();

                let numbers : Vec<_>= p.iter().flat_map(|str| str.trim().parse::<u64>()).collect();
                problems_p2.push(Problem { numbers, op });
            }     
        }

        [problems, problems_p2]
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input[0].iter().map(|p| p.compute()).sum()
    }
    
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        input[1].iter().map(|p| p.compute()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day6;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";


    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day6.parse_input(lines);
        assert_eq!(Day6.first_part(&input), 4277556)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day6.parse_input(lines);
        assert_eq!(Day6.second_part(&input), 3263827);
    }
}
