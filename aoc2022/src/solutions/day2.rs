use crate::solution::Solution;

pub struct Day2;

pub enum Action {
    Rock,
    Paper,
    Scissors
}

enum Outcome {
    Win,
    Draw,
    Loose
}

impl Action {
    pub fn from_char(c : char) -> Option<Self> {
        match c {
            'A' | 'X' => Some(Action::Rock),
            'B' | 'Y' => Some(Action::Paper),
            'C' | 'Z' => Some(Action::Scissors),
            _ => None
        }
    }
}
// Defining a struct because can't impl FromStr on a type alias on tuple :(
pub struct Round {
    opponent : Action,
    me : Action
}

impl std::str::FromStr for Round {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, me) = s.split_once(" ").unwrap_or_default();

        let round_res = (Action::from_char(opponent.chars().next().unwrap_or_default()), Action::from_char(me.chars().next().unwrap_or_default()));

        match round_res {
            (Some(a), Some(b)) => Ok(Round { opponent : a, me: b}),
            _ => Err("The round  is invalid !"),
        }

    }
}

impl Round {
    fn compute_score(&self) -> u32 {
        let s1 = match self.me {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3
        };

        let s2 = match (&self.me, &self.opponent) {
            (Action::Rock, Action::Scissors) | (Action::Paper, Action::Rock) | (Action::Scissors, Action::Paper) => 6,
            (Action::Rock, Action::Rock) | (Action::Paper, Action::Paper) | (Action::Scissors, Action::Scissors) => 3,
            (Action::Rock, Action::Paper) | (Action::Paper, Action::Scissors) | (Action::Scissors, Action::Rock) => 0
        };

        s1 + s2
    }


    fn compute_score_p2(&self) -> u32 {
        //LOL enum names don't have the same meaning anymore :') 
        let outcome = match &self.me {
            Action::Rock => Outcome::Loose,
            Action::Paper => Outcome::Draw,
            Action::Scissors => Outcome::Win
        };

        let s1 = match (&self.opponent, &outcome)  {
            (Action::Scissors, Outcome::Win) | (Action::Rock, Outcome::Draw) | (Action::Paper, Outcome::Loose) => 1, //Playing Rock
            (Action::Rock, Outcome::Win) | (Action::Paper, Outcome::Draw) | (Action::Scissors, Outcome::Loose) => 2, //Playing Paper
            (Action::Paper, Outcome::Win) | (Action::Scissors, Outcome::Draw) | (Action::Rock, Outcome::Loose) => 3, //Playing Scissors
        };

        let s2 = match &outcome {
            Outcome::Loose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6
        };

        s1 + s2
    }
}

impl Solution for Day2 {
    type Input = Vec<Round>;
    type ReturnType = u32;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.flat_map(|line| line.parse()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        input.iter().map(|r| r.compute_score()).sum()
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        input.iter().map(|r| r.compute_score_p2()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Day2;
    use crate::solution::Solution;

    #[test]
    fn test_first_part() {

        let input_strs = vec!["A Y",
        "B X",
        "C Z"];
        let input = Day2.parse_input(input_strs.iter().map(|s| s.to_string()));

        assert_eq!(
            Day2.first_part(&input),
            15
        );
    }

    #[test]
    fn test_second_part() {
        let input_strs = vec!["A Y",
        "B X",
        "C Z"];
        let input = Day2.parse_input(input_strs.iter().map(|s| s.to_string()));

        assert_eq!(
            Day2.second_part(&input),
            12
        );
    }
}
