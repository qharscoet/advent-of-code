use std::ops::RemAssign;

use crate::solution::Solution;

pub struct Day10;

#[derive(Debug)]
pub enum Instruction {
	Addx(i32),
	Noop
}

type Program = Vec<Instruction>;


pub struct CPU<'a> {
	register : i32,
	clock : u32,
	pc : usize, //Program Counter
	code : Option<&'a Program>,
	is_busy:bool,
	screen: [[bool;40];6],
}


impl std::str::FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair : Vec<_> = s.split_ascii_whitespace().collect();
		
        match pair[0] {
			"noop" => Ok(Instruction::Noop),
			"addx" => Ok(Instruction::Addx(pair[1].parse().unwrap_or_default())),
			_ => Err("Not a valid Instruction")
		}
    }
}

impl CPU<'_> {
	
	fn start_instruction(&mut self, instr : &Instruction) {
		match instr {
			Instruction::Noop =>  self.pc += 1,
			Instruction::Addx(_) => self.is_busy = true,
		}
	}
	
	fn end_instruction(&mut self, instr : &Instruction) {
		//println!("Processing {:?}", instr);
		match instr {
			Instruction::Noop => {},
			Instruction::Addx(v) => self.register += v,
		}
	}
	
	fn step_cycle(&mut self) {
		if let Some(code) = self.code {
			let fetched_instr = &code[self.pc];
			
			//Drawing the pixel
			{
				let (row, col) = ((self.clock / 40) as usize, (self.clock % 40) as usize);
				let reg = self.register as usize;
				if col >= reg.saturating_sub(1) && col <= reg +1 {
					self.screen[row][col] = true;
				}
				
			}
			
			if self.is_busy {
				self.end_instruction(fetched_instr);
				self.is_busy = false;
				self.pc += 1;
			} else {
				self.start_instruction(fetched_instr);
			}
			
			self.clock += 1;
		}
	}
	
	fn run_until_cycle(&mut self, n:u32) {
		while self.clock < n {
			self.step_cycle();
		}
	}
	
	
	fn print_screen(&self) {
		for i in 0..6 {
			for j in 0..40 {
				print!("{}", if self.screen[i][j] {"#"} else {"."});
			}
			
			print!("\n");
		}
	}
}

impl Default for CPU<'_> {
    fn default() -> Self {
        Self { register: 1, clock: 0, pc: 0, code: None, is_busy: false, screen: [[false;40];6] }
    }
}

impl Solution for Day10 {
    type Input = Program;
    type ReturnType = u32;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
       lines.flat_map(|l|l.parse()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
		let mut cpu : CPU = CPU::default();
		cpu.code = Some(input);
		
		(20..=220).step_by(40).fold(0, |acc, n| { 
			cpu.run_until_cycle(n - 1);
			acc + cpu.register as u32 * (cpu.clock  + 1)
		})
	}
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
		let mut cpu : CPU = CPU::default();
		cpu.code = Some(input);
		
		cpu.run_until_cycle(240);
		cpu.print_screen();
		0
	}
}

#[cfg(test)]
mod tests {
    use super::Day10;
    use crate::solution::Solution;

		static INPUT_TEST : &str =
"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day10.parse_input(lines);
		assert_eq!(Day10.first_part(&input),
        13140)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day10.parse_input(lines);
		assert_eq!(Day10.second_part(&input),
            42)
    }
}