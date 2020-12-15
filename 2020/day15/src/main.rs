use std::collections::HashMap;

fn main() {
    const STARTING_NUMBERS : [u32; 7] =  [5, 2, 8, 16, 18, 0, 1];
    const TURN_CNT: usize = 30000000;

    let mut age : HashMap<u32,u32> = HashMap::new();
    let mut last_number = STARTING_NUMBERS[0];

    for i in 1..TURN_CNT {
        let number;
        if i < STARTING_NUMBERS.len() {
            number = STARTING_NUMBERS[i];
        } else {
            number = match age.get(&last_number) {
                Some(last_spoken) => (i -1) as u32 - last_spoken,
                None => 0,
            };
        }

        age.insert(last_number, (i-1) as u32);
        last_number = number;
    }

    println!("Hello, world! {}", last_number);
}
