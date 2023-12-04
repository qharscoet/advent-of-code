use crate::solution::Solution;

pub struct Day2;


#[derive(Debug)]
struct Set {
    red : u32,
    green : u32,
    blue : u32
}

impl std::str::FromStr for Set {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for color_str in s.split(',') 
        {
            if let Some((n, color)) = color_str.trim().split_once(' ') {
                match color.trim() {
                    "red" => red = if let Ok(r) = n.parse::<u32>() { r} else { return Err("Error parsing")},
                    "green" => green = if let Ok(g) = n.parse::<u32>() { g} else { return Err("Error parsing")},
                    "blue" => blue = if let Ok(b) = n.parse::<u32>() { b} else { return Err("Error parsing")},
                    _ => return Err("not a valid color !")
                }
            }
        }

        Ok(Set { red: red, green: green, blue: blue })
    }
}

#[derive(Debug)]
pub struct Game
{
    id : u32,
    sets : Vec<Set>
}

impl std::str::FromStr for Game {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let Some((game, sets)) = s.split_once(':') else { return Err("String not in the right format !")};

        let Some((_,str_id)) = game.split_once(' ') else { return Err("String not in the right format !")} ;
        let Ok(id) = str_id.parse::<u32>() else { return Err("Id not a number")};

        let sets_vec : Vec<Set> = sets.split(';').map(|s| s.parse().unwrap()).collect();



        Ok(Game{id:id, sets: sets_vec})
    }
}

impl Solution for Day2 {
    type Input = Vec<Game>;
    type ReturnType = u32;
    const DAY : u32 = 2;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.flat_map(|line| line.parse()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {
        input.iter().filter(|game| !game.sets.iter().any(|set| set.red > 12 || set.green > 13 || set.blue > 14)).map(|game| game.id).sum()
        
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day2;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    static INPUT_TEST_2 : &str = "";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day2.parse_input(lines);
        assert_eq!(Day2.first_part(&input), 8)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day2.parse_input(lines);
        assert_eq!(Day2.second_part(&input), 0)
    }
}
