use image::{ImageResult, Luma};

use crate::solution::Solution;

pub struct Day14;

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    p : (isize,isize),
    v : (isize,isize),
}

impl std::str::FromStr for Robot {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p_str, v_str) = s.split_once(' ').ok_or("Invalid format")?;


        fn get_values(button_s:&str) -> Result<(isize,isize), &'static str> {
            let (_, values_str) = button_s.split_once('=').ok_or("Invalid format")?;
            let (x_s,y_s) = values_str.split_once(',').ok_or("Invalid format")?;
            let x = x_s.parse::<isize>().or(Err("Invalid value"))?;
            let y = y_s.parse::<isize>().or(Err("Invalid value"))?;

            Ok((x,y))
        }

        Ok(Robot{p : get_values(p_str)?, v:get_values(v_str)?})
    }
}

impl Robot {

    fn move_n_step(&mut self, n: u32, total_size : (isize,isize)) {
        self.p = ((self.p.0 + (n as isize) * self.v.0).rem_euclid(total_size.0), (self.p.1 + (n as isize)*self.v.1).rem_euclid(total_size.1))
    }

}

fn draw_map(robots:&Vec<Robot>, size : (isize, isize)) {

    for i in 0..size.1 {
        for j in 0..size.0 {
            print!("{}", match robots.iter().filter(|r| r.p.0 == j && r.p.1 == i).count() {
                0 => '.',
                c => char::from_digit(c as u32, 10).unwrap()
            });
        }
        print!("\n");
    }
}

fn render_img(robots:&Vec<Robot>, size : (u32, u32), time:u32) -> ImageResult<()> {

    let mut img = image::ImageBuffer::new(size.0, size.1);

    for i in 0..size.0 {
        for j in 0..size.1 {
            let c = robots.iter().filter(|r| r.p.0 == i as isize && r.p.1 == j as isize).count();
            img.put_pixel(i, j, Luma([(c  * 127) as u8]));
        }
    }

    img.save(format!(".\\images\\{}.png", time))
}


impl Solution for Day14 {
    type Input = Vec<Robot>;
    type ReturnType = u32;
    const DAY: u32 = 14;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.flat_map(|l| l.parse()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
    
        let total_size : (isize,isize) = if cfg!(test) { (11,7) } else { (101,103)};
        let mut robots = input.clone();

        for r in robots.iter_mut() {
            r.move_n_step(100, total_size);
        }

        (0..4).map(|quad| {
            let x_quad = (quad&1 as isize) * (total_size.0/2 +1);
            let y_quad = ((quad&2 != 0) as isize) * (total_size.1/2 +1);

            let is_in_quad =  |p:(isize,isize)|  {p.0 >= x_quad && p.0 < x_quad + total_size.0/2 && p.1 >= y_quad && p.1 < y_quad + total_size.1/2};

            let c = robots.iter().filter(|r| is_in_quad(r.p) ).count();
            println!("Quad {} : [{}, {}], [{},{}]count {}", quad, x_quad, x_quad+total_size.0/2, y_quad, y_quad + total_size.1/2, c);
            c
        }).product::<usize>() as u32
    }

    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
        let total_size : (u32,u32) = if cfg!(test) { (11,7) } else { (101,103)};
        let isize = (total_size.0 as isize, total_size.1 as isize);
        
        let mut robots = input.clone();
        draw_map(&robots, isize);
        render_img(&robots, total_size, 0).unwrap();

        
        for i in 1..10000 {
            for r in robots.iter_mut() {
                r.move_n_step(1, isize);
            }
            render_img(&robots, total_size, i).unwrap();

            // draw_map(&robots, total_size);
            println!("{} seconds", i);
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day14;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";


    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day14.parse_input(lines);
        assert_eq!(Day14.first_part(&input), 12);
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day14.parse_input(lines);
        assert_eq!(Day14.second_part(&input), 10);
    }
}
