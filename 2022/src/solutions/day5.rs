use crate::solution::Solution;

pub struct Day5;

type Crate = char;
type Stack = Vec<Crate>;
type Storage = Vec<Stack>;

#[derive(Debug, Clone, Copy)]
struct Rule {
    count : u32,
    from_stack : usize,
    to_stack: usize,
}

impl std::str::FromStr for Rule {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words : Vec<&str> = s.split_ascii_whitespace().collect();

        Ok(Rule {
            count : words[1].parse::<u32>().expect("Not a number"),
            from_stack : (words[3].parse::<u32>().expect("Not a number") -1) as usize,
            to_stack : (words[5].parse::<u32>().expect("Not a number") -1) as usize,
        })
    }
}


#[derive(Clone)]
pub struct Docks {
    storage: Storage,
    rules: Vec<Rule>
}


impl Docks {
    fn apply_rules(&mut self) {
        for r in &self.rules {
            for _ in 0..r.count{
                let c = self.storage[r.from_stack].pop().unwrap_or_default();
                self.storage[r.to_stack].push(c);
            }
        }
    }

    fn apply_rules_cratemover9001(&mut self) {
        for r in &self.rules {
            let len = self.storage[r.from_stack].len();
            let count = r.count as usize;

            //Could use extend to go from the drain iter to the other vec without collecting, but borrow_checker is a bitch
            let mut removed = self.storage[r.from_stack].drain(len-count..).collect();
            self.storage[r.to_stack].append(&mut removed);
        }
    }
}

impl Solution for Day5 {
    type Input = Docks;
    type ReturnType = String;

    fn parse_input(&self, mut lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let crate_lines : Vec<String> = lines.by_ref().take_while(|l| !l.is_empty()).collect();
        let mut rev_crate_lines = crate_lines.iter().rev();
        let count = rev_crate_lines.next().map(|s| s.split("   ").last().expect("Can't split").trim().parse::<u32>().expect("is not a number")).unwrap_or_default() as usize;

        /* Classic more imperative style */
        
        //getting our n empty stacks of crates
        let mut storage : Storage = Vec::new();
        storage.resize(count, Vec::new());

        // Stacking them up, chunk is either 4 spaces or '[N] '
        for l in rev_crate_lines {
            for (idx, chunk) in l.chars().collect::<Vec<char>>().chunks(4).enumerate(){
                if chunk[0] != ' '{
                    storage[idx].push(chunk[1]);
                }
            }
        }
        
        /* Functional style for reference*/
        // let storage = rev_crate_lines.fold(vec![vec![];count], |mut acc, l| {
        //     l.chars().collect::<Vec<char>>().chunks(4) // Getting chunks of 4 chars
        //         .enumerate().filter(|(_,chunk)| chunk[0] != ' ') // filtering the not empty ones
        //         .for_each(|(idx,chunk)| acc[idx].push(chunk[1])); //Add value to stack
        //     acc
        // });

        let rules : Vec<Rule> = lines.flat_map(|l| l.parse()).collect();
        Docks {
            storage : storage,
            rules : rules,
        }
    }

    fn first_part(&self, input: &Self::Input) -> String {
        let mut input_copy = input.clone();
        input_copy.apply_rules();

        input_copy.storage.iter().flat_map(|stack| stack.last()).collect()
    }
    fn second_part(&self, input: &Self::Input) -> String {
        let mut input_copy = input.clone();
        input_copy.apply_rules_cratemover9001();

        input_copy.storage.iter().flat_map(|stack| stack.last()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Day5;
    use crate::solution::Solution;

	static INPUT_TEST : &str =
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day5.parse_input(lines);
		assert_eq!(Day5.first_part(&input),
        "CMZ")
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day5.parse_input(lines);
		assert_eq!(Day5.second_part(&input),
            "MCD")
    }
}