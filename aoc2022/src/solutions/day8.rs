use crate::solution::Solution;

pub struct Day8;

type Tree = u8;
type Forest = Vec<Vec<Tree>>; // 2DGrid

fn is_tree_visible(forest : &Forest, tree:&(usize,usize)) -> bool{

    let slice_left= forest[tree.0][..tree.1].iter().all(|t| t < &forest[tree.0][tree.1] );
    let slice_right = forest[tree.0][(tree.1 +1) ..].iter().all(|t| t < &forest[tree.0][tree.1] );
    let slice_top  = forest[..tree.0].iter().map(|row| &row[tree.1]).all(|t| t < &forest[tree.0][tree.1] );
    let slice_bottom = forest[(tree.0 +1)..].iter().map(|row| &row[tree.1]).all(|t| t < &forest[tree.0][tree.1] );
    
    [slice_top, slice_bottom, slice_left, slice_right].iter().any(|b| *b)
}

fn scenic_score(forest : &Forest, tree:&(usize,usize)) -> usize {
    let viewdist_left= forest[tree.0][..tree.1].iter().rev().position(|t|t >= &forest[tree.0][tree.1]).and_then(|p| Some(p+1)).unwrap_or(tree.1);
    let viewdist_right = forest[tree.0][(tree.1 +1) ..].iter().position(|t|t >=  &forest[tree.0][tree.1]).and_then(|p| Some(p+1)).unwrap_or(forest[0].len() - (tree.1 +1 ));
    let viewdist_top  = forest[..tree.0].iter().map(|row| &row[tree.1]).rev().position(|t|t >=  &forest[tree.0][tree.1]).and_then(|p| Some(p+1)).unwrap_or(tree.0);
    let viewdist_bottom = forest[(tree.0 +1)..].iter().map(|row| &row[tree.1]).position(|t|t >=  &forest[tree.0][tree.1]).and_then(|p| Some(p+1)).unwrap_or(forest.len() - (tree.0 + 1));

    
    viewdist_left * viewdist_right * viewdist_top * viewdist_bottom
}

impl Solution for Day8 {
    type Input = Forest;
    type ReturnType = usize;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines
			.map(|line| {
				line.chars()
					.map(|c| (c as u8 - '0' as u8))
					.collect()
			})
			.collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        let len_x = input.len();
        let len_y = input[0].len();
        let edge_count = len_x * 2 + len_y * 2 - 4;
        let v  = (1..(len_x -1)).map(|x| (1..(len_y -1)).map(|y| (x,y)).collect::<Vec<(usize,usize)>>()).flatten();


        v.filter(|pos| is_tree_visible(&input, pos)).count() + edge_count
        
    }
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        let len_x = input.len();
        let len_y = input[0].len();
        let v  = (1..(len_x -1)).map(|x| (1..(len_y -1)).map(|y| (x,y)).collect::<Vec<(usize,usize)>>()).flatten();
        v.map(|pos| scenic_score(&input, &pos)).max().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::Day8;
    use crate::solution::Solution;

	static INPUT_TEST : &str =
"30373
25512
65332
33549
35390";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day8.parse_input(lines);
		assert_eq!(Day8.first_part(&input),
        21)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day8.parse_input(lines);
		assert_eq!(Day8.second_part(&input),
        8)
    }
}