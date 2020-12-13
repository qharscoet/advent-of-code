use std::fs::File;
use std::io::{BufRead, BufReader};


fn euclid(a: u64, b:u64) -> (u64, i64,i64) {
    fn euclid_in(ruv_i: (u64,i64,i64), ruv_i2: (u64,i64,i64) ) -> (u64, i64,i64){
        if ruv_i.0 == 0 {
            return ruv_i2;
        } else {
            let q = ruv_i2.0/ruv_i.0;
            let new_ruv = (ruv_i2.0 - q*ruv_i.0, ruv_i2.1 - (q as i64)*ruv_i.1, ruv_i2.2 - (q as i64)*ruv_i.2);
            return euclid_in(new_ruv, ruv_i);
        }
    }

    euclid_in((b as u64, 0, 1), (a as u64,1,0))
}

fn inverse(a:u64, b:u64) -> u64 {
    let (_r,_u,v) = euclid(a, b);
    if v < 0 { (v + a as i64) as u64 } else {v as u64}
}

fn chinese(v: Vec<(u32,u32)>) -> u64 {
    let n : u64= v.iter().map(|v| v.1 as u64).product();

    v.iter().map(|(a,b)| {let n2 = n/(*b as u64); *a as u64 *  n2 as u64 * inverse(*b as u64, n2) as u64 }).sum::<u64>() % n
}


fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines().flatten();
    let arrival_time : u32 = lines.next().expect("Not line").trim().parse().expect("Not a number");
    let buses : Vec<(u32,u32)> = lines.next().unwrap().split(',').enumerate()
    .map(|(idx, id)| (idx as u32, id.trim().parse::<u32>()))
    .filter(|(_idx, r)| r.is_ok())
    .map(|(idx, id)| (idx, id.unwrap()) )
    .collect();
    // let buses_id : Vec<u32> = lines.next().unwrap().split(',').filter_map(|s| s.parse().ok()).collect();

    let buses_id : Vec<u32> = buses.iter().map(|(_idx, id)| *id).collect();
    let equations: Vec<(u32, u32)> = buses.iter().map(|(idx, id)|  (((-(*idx as i32) % *id as i32 + *id as i32) as u32) % id, *id) ).collect();



    let result = buses_id.iter().map(|id| (id, ((arrival_time/id) + 1) * id - arrival_time)).min_by_key(|(_id, time)| time.clone()).unwrap();


    println!("Hello, world! Content for Part 1: {:?}", result.0 * result.1);
    println!("Hello, world! Contents for part 2 is : \n{:?}", chinese( equations ));
}
