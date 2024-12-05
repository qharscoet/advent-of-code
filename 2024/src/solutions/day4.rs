use crate::solution::Solution;
pub struct Day4;


impl Solution for Day4 {
    type Input = Vec<Vec<char>>;
    type ReturnType = u32;
    const DAY : u32 = 4;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|l| l.chars().collect()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> u32 {

        let mut xmas_count : usize = 0;
        
        //Lines
        xmas_count += input.iter().map(|s| s.iter().collect::<String>().matches("XMAS").count()).sum::<usize>();
        xmas_count += input.iter().map(|s| s.iter().collect::<String>().matches("SAMX").count()).sum::<usize>();

        //Columns
        let size_x = input.len();
        let size_y = input[0].len();
        for i in 0..size_y {
            let col = input.iter().map(|s| s[i]).collect::<String>();
            xmas_count += col.matches("XMAS").count();
            xmas_count += col.matches("SAMX").count();

            //Diagonals
            let diag = input.iter().enumerate().filter_map(|(idx,s)| if i + idx < size_y {Some(s[i + idx])} else {None}).collect::<String>();
            xmas_count += diag.matches("XMAS").count();
            xmas_count += diag.matches("SAMX").count();

            
            let diag2 = input.iter().enumerate().filter_map(|(idx,s)|  if i >= idx  {Some(s[i - idx])} else {None}).collect::<String>();
            xmas_count += diag2.matches("XMAS").count();
            xmas_count += diag2.matches("SAMX").count();

            if i > 0 {
                let diag3 = input.iter().enumerate().filter_map(|(idx,s)| if i + (size_x - 1 - idx) < size_y {Some(s[i + (size_x - 1 - idx)])} else {None}).collect::<String>();
                xmas_count += diag3.matches("XMAS").count();
                xmas_count += diag3.matches("SAMX").count();
            }

            if i < size_y -1 {
                let diag4 = input.iter().enumerate().filter_map(|(idx,s)| if i >= (size_x - 1 - idx) {Some(s[i - (size_x - 1 - idx)])} else {None}).collect::<String>();
                xmas_count += diag4.matches("XMAS").count();
                xmas_count += diag4.matches("SAMX").count();
            }

        }


        xmas_count as u32
    }
    fn second_part(&self, input: &Self::Input) -> u32 {
        let size_x = input.len();
        let size_y = input[0].len();

        let mut xmas_count = 0;
        for i in 1..size_x-1 {
            for j in 1..size_y-1 {
                if input[i][j] == 'A' {
                    let cross = (input[i-1][j-1], input[i+1][j+1], input[i-1][j+1], input[i+1][j-1]);
                    xmas_count += match cross {
                        ('M', 'S', 'M', 'S') | ('M', 'S', 'S', 'M') | ('S', 'M', 'M', 'S') | ('S', 'M', 'S', 'M') => 1,
                        _ => 0
                    };
                }
            }
        }
        xmas_count
    }
}

#[cfg(test)]
mod tests {
    use super::Day4;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day4.parse_input(lines);
        assert_eq!(Day4.first_part(&input), 18)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day4.parse_input(lines);
        assert_eq!(Day4.second_part(&input), 9);
    }
}
