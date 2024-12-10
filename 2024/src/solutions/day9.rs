use crate::solution::Solution;

pub struct Day9;

impl Solution for Day9 {
    type Input = String;
    type ReturnType = u64;
    const DAY: u32 = 9;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        let mut i = 0;
        let mut j = if (input.len() & 1) == 0 { input.len() - 2 } else {input.len() - 1};

        let chars = input.as_bytes();
        let mut is_free_space = false;
        let mut checksum = 0;
        let mut curr_id = 0;
        let mut curr_rid = (input.len() - 1)/2;

        let mut remaining_i = chars[i] -('0' as u8);
        let mut remaining_j = chars[j] -('0' as u8);
        let mut curr_pos = 0;
        while i < j {
            if !is_free_space {
                if remaining_i == 0 {
                    i+=1;
                    remaining_i = chars[i]  -('0' as u8);
                    is_free_space = !is_free_space;
                    curr_id += 1;
                } else {
                    // println!("block {}, remaining {}", curr_id, remaining_i);
                    checksum += curr_pos*curr_id;
                    remaining_i -= 1;
                    curr_pos+=1;
                        
                }
            } else {

                if remaining_i > 0 && remaining_j > 0 {
                    // println!("free space filled by {}, remaining {}, remaining j {} i = {}, j = {}", curr_rid, remaining_i, remaining_j,i,j);
                    checksum += curr_pos * curr_rid;
                    remaining_i -= 1;
                    remaining_j -= 1;
                    curr_pos+=1;
                } else {
                    if remaining_i == 0 {
                        i+=1;
                        remaining_i = chars[i] -('0' as u8);
                        is_free_space = !is_free_space;
                    }
    
                    if remaining_j == 0 {
                        j -= 2;
                        remaining_j = chars[j] -('0' as u8) ;
                        curr_rid -= 1;
                    }
                }

            }

        }

        // println!("same block, curr_id {}, curr_rid {}, remaining_i {}, remaining_j {}", curr_id, curr_rid, remaining_i, remaining_j);
        while remaining_j > 0 {
            checksum += curr_pos * curr_rid;
            remaining_j-=1;

            curr_pos+=1;
        }
        checksum as u64
    }

    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {

        #[derive(Debug)]
        struct Block {
            start : usize,
            size : usize,
        }

        let (mut files, mut free_space,_) = input.bytes().enumerate().fold((Vec::new(), Vec::new(), 0), |(mut files, mut free_blocks, total_pos),(idx,c)| {
            let value = (c - ('0' as u8)) as usize;
            if (idx & 1) == 0 {
                files.push(Block{start:total_pos, size: value });
            }  else {
                free_blocks.push(Block{start:total_pos, size: value});
            }

            (files,free_blocks, total_pos + value)
        });

        for f in files.iter_mut().rev() {
            if let Some(dst) = free_space.iter_mut().find(|block| block.size >= f.size && block.start < f.start) {
                f.start = dst.start;
                dst.start += f.size;
                dst.size -= f.size;
            }
        }

        let checksum : usize = files.iter().enumerate().map(|(id,f)| {
            //(f.start..(f.start + f.size)).map(|i| i * f.id).sum::<usize>()
            id * ( f.start* f.size + (((f.size -1) * (f.size))/2))
        }).sum();

       checksum as u64
    }
}

#[cfg(test)]
mod tests {
    use super::Day9;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "2333133121414131402";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day9.parse_input(lines);
        assert_eq!(Day9.first_part(&input), 1928)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day9.parse_input(lines);
        assert_eq!(Day9.second_part(&input), 2858);
    }
}
