
use crate::solution::Solution;

pub struct Day9;

type Tile = (u64, u64);
type Shape = Vec<Tile>;
type Line = (Tile,Tile);
type Lines = Vec<Line>;

fn area(a: Tile, b: Tile) -> u64 {
    let width = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 } + 1;
    let height = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 } +1;

    //println!("area : {:?}, {:?}, {}", a, b, width * height);
    width * height
}

//https://wrfranklin.org/Research/Short_Notes/pnpoly.html
/*
int pnpoly(int nvert, float *vertx, float *verty, float testx, float testy)
{
  int i, j, c = 0;
  for (i = 0, j = nvert-1; i < nvert; j = i++) {
    if ( ((verty[i]>testy) != (verty[j]>testy)) &&
	 (testx < (vertx[j]-vertx[i]) * (testy-verty[i]) / (verty[j]-verty[i]) + vertx[i]) )
       c = !c;
  }
  return c;
}
*/
fn pnpoly(vert : &Shape, t : Tile) -> bool
{
    if vert.contains(&t) {
        return true;
    }

    let epsilon = 0.0000001f32;
    let nvert = vert.len();
    
    let mut i = 0;
    let mut j = nvert - 1;
    let mut c = false;

  while i < nvert {

    let vertx_i = vert[i].0 as f32;
    let verty_i = vert[i].1 as f32;
    let vertx_j = vert[j].0 as f32;
    let verty_j = vert[j].1 as f32;
    let testx = t.0 as f32;
    let testy = t.1 as f32;

    if (verty_i - verty_j).abs() <= epsilon && (verty_j - testy).abs() <= epsilon && (vertx_i >= testx) != (vertx_j >= testx)
    {
        return true;
    }


    let v = (vertx_j - vertx_i) * (testy - verty_i) / (verty_j-verty_i) + vertx_i;
    if (verty_i > testy) != (verty_j > testy) {
        if (testx -v).abs() < epsilon  {
            return true;
        }
        if testx < v {
            c = !c;
        }
    }
    j = i;
    i += 1;
  }

  //println!("pnpoly point {:?}, inside {},, count {}", t, c, cout);
  return c;
}

// first try before stealing the above pnpoly
#[allow(dead_code)]
fn is_inside(lines: &Lines, point: Tile) -> bool {
    let x_lines = lines.iter().filter(|(t0,t1)| t0.0 == t1.0 && t0.0 < point.0 && t0.1 <= point.1 && t1.1 >= point.1);
    let y_lines = lines.iter().filter(|(t0,t1)| t0.1 == t1.1 && t0.1 < point.1 && t0.0 <= point.0 && t1.0 >= point.0);

    let is_on_x = lines.iter().any(|(t0,t1)| t0.0 == t1.0 && t0.0 == point.0 && t0.1 <= point.1 && t1.1 >= point.1);
    let is_on_y = lines.iter().any(|(t0,t1)| t0.1 == t1.1 && t0.1 == point.1 && t0.0 <= point.0 && t1.0 >= point.0);

    let vx : Vec<_> = x_lines.collect();
    let x_count = vx.len();
    let y_count = y_lines.count();

    let mut inside = x_count%2 != 0 && y_count%2 != 0;
    inside |= is_on_x;
    inside |= is_on_y;

    println!("{:?}", vx);
    //println!("point {:?} x_count {}, y_count {}, is_x {}, is_y {}, insides {:?}", point, x_count, y_count, is_on_x, is_on_y, inside);

    inside
}

fn compute_lines(shape: &Shape) -> Vec<(Tile, Tile)>
{
    let mut lines : Vec<_> = shape.windows(2).map(|w| (w[0].min(w[1]), w[0].max(w[1]))).collect();
    let first = shape.first().unwrap_or(&(0,0));
    let last = shape.last().unwrap_or(&(0,0));
    lines.push((*first.min(last), *first.max(last)));

    lines
}


fn is_square_inside(shape: &Shape, _lines : &Lines, a:Tile, b:Tile) -> bool {

    let t0 = (b.0, a.1);
    let t1 = (a.0, b.1);

    let vert = [a,b,t0,t1];
    if vert.iter().any(|v| !pnpoly(shape, *v)) {
        return false;
    }

    let sides = [(a, t0), (t0, b), (b, t1), (t1, a)];
    let points : Vec<Tile> = sides.iter().flat_map(|(a,b)| {
        let min = a.min(b);
        let max = a.max(b);

        let v : Vec<Tile> =  if min.0 == max.0 {
            (min.1..=max.1).map(|y| (min.0, y)).collect()
        } else {
            (min.0..=max.0).map(|x| (x, min.1)).collect()
        };

        v
    }).collect();

    let inside = points.iter().all(|p| pnpoly(shape, *p));
    //println!("square {:?},{:?} inside {}",a,b, inside);
    inside
}

fn area_if_inside(shape: &Shape, lines : &Lines, a:Tile, b:Tile) -> u64 {
    if is_square_inside(shape, lines, a, b) { area(a,b)} else {0}
}

impl Solution for Day9 {
    type Input = Vec<Tile>;
    type ReturnType = u64;
    const DAY : u32 = 9;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines
            .map(|l| {
                let mut it = l.split(',').flat_map(|s| s.trim().parse::<u64>());
                (
                    it.next().unwrap_or_default(),
                    it.next().unwrap_or_default(),
                )
            })
            .collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        input.iter().flat_map(|&t| input.iter().map(move |&t2| area(t,t2))).max().unwrap_or_default()
        
    }
    
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
       let lines = compute_lines(input);

       input.iter().flat_map(|&t| input.iter().map(move |&t2| (t,t2))).map(|(t,t2)| area_if_inside(input, &lines,t,t2)).max().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::Day9;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";


    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day9.parse_input(lines);
        assert_eq!(Day9.first_part(&input), 50)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day9.parse_input(lines);
        assert_eq!(Day9.second_part(&input), 24);
    }
}
