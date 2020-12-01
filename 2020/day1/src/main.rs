use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;

// values need to be sorted, this is O(n) and implement
fn find_pair_sorted(values: &[i32], target_value: i32) -> Option<i32> {

    let mut l = 0;
    let mut r = values.len() - 1;

    while l < r {
        match target_value.cmp(&(values[l] + values[r]))
        {
            Ordering::Less => r-=1,
            Ordering::Greater => l += 1,
            Ordering::Equal => return Some(values[l] * values[r]),
        }
    }
    None
}

// values need to be sorted, this is O(n²)
fn find_triplet_sorted(values: &[i32], target_value: i32) -> Option<i32> {
    for (idx,val) in values.iter().enumerate() {
        if let Some(ret) = find_pair_sorted(&values[idx..], target_value - val) {
            return Some(val * ret);
        }
    }
    None
}

fn find_pair(values: &Vec<i32>, target_value: i32) -> Option<i32> {
    let mut values_copy =  values.to_vec();
    values_copy.sort();
    return find_pair_sorted(&values_copy, target_value);
}

fn find_triplet(values: &Vec<i32>, target_value: i32) -> Option<i32> {

    let mut values_copy =  values.to_vec();
    values_copy.sort();
    return find_triplet_sorted(&values_copy, target_value);
}


// Naive solutions

fn find_values(values: &Vec<i32>, target_value: i32) -> Option<i32> {
    for v in values {
        for v2 in values {
            if v + v2 == target_value {
                return Some(v * v2);
            }
        }
    }
    None
}

fn find_3_values(values: &Vec<i32>, target_value: i32) -> Option<i32> {
    for v in values {
        if let Some(ret) = find_values(values, target_value - v) {
            return Some(v * ret);
        }
    }
    None
}

//Main
fn main() {
    let file = File::open("./src/input.txt").expect("Error opening the input");
    let buf_reader = BufReader::new(file);
    let values: Vec<i32> = buf_reader
        .lines()
        .map(|line| line.unwrap().trim().parse().expect("Not a number"))
        .collect();
    println!("Hello, world! Contents is : \n{:?}", values);
    println!(
        "Result : {}",
        find_pair(&values, 2020).unwrap_or_default()
    );
    println!(
        "Result : {}",
        find_triplet(&values, 2020).unwrap_or_default()
    );
}
