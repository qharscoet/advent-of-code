use crate::solution::Solution;

pub struct Day13;

#[derive(Debug)]
pub struct Claw {
    a:(i64,i64),
    b:(i64,i64),
    prize:(i64,i64),
}


impl Claw {
    fn claw_from_str_vec(lines:&Vec<String>) -> Claw {
    
        fn get_button(button_s:&str) -> (i64,i64) {
            let (_, values_str) = button_s.split_once(':').unwrap_or_default();
            let (x_s,y_s) = values_str.split_once(',').unwrap_or_default();
            let x = x_s.split_once(['+','=']).unwrap_or_default().1.parse::<i64>().unwrap_or_default();
            let y = y_s.split_once(['+','=']).unwrap_or_default().1.parse::<i64>().unwrap_or_default();
            (x,y)
        }
        let a = get_button(&lines[0]);
        let b = get_button(&lines[1]);
        let prize = get_button(&lines[2]);
    
        Claw{a, b, prize}
    }


    fn token_cost(&self, offset : i64) -> u64 {
    
        let total = (self.prize.0 + offset, self.prize.1 + offset);
        let a = (total.1 * self.b.0 -  total.0 * self.b.1 )/(self.b.0 * self.a.1 - self.b.1 * self.a.0);
        let b = (total.0 * self.a.1 - total.1 * self.a.0) / (self.b.0 * self.a.1 - self.b.1 * self.a.0);

        println!("A : {}, B : {}", a,b);

        if a * self.a.0 + b * self.b.0 == total.0 && a * self.a.1 + b * self.b.1 == total.1 {
            (3 * a +  b) as u64
        } else {
            0
        }
    }
}

impl Solution for Day13 {
    type Input = Vec<Claw>;
    type ReturnType = u64;
    const DAY: u32 = 13;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let mut peekable = lines.peekable();
        let mut claws: Vec<Claw> = Vec::new();

        while peekable.peek().is_some() {
            let s=  peekable
                    .by_ref()
                    .take_while(|line| !line.is_empty())
                    .collect::<Vec<_>>();

            claws.push(Claw::claw_from_str_vec(&s));
            
        }
        claws
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        println!("{:?}", input);
        
        input.iter().map(|claw| claw.token_cost(0)).sum()
    }

    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.iter().map(|claw| claw.token_cost(10000000000000)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day13;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";


    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day13.parse_input(lines);
        assert_eq!(Day13.first_part(&input), 480);
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day13.parse_input(lines);
        assert_eq!(Day13.second_part(&input), 0);
    }
}
