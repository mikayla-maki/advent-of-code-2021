//Day 7, puzzle 1 & 2

use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./input/day7/input.txt")?;
    let reader = io::BufReader::new(file);
    

    for text_line in reader.lines() {
        let crab_pos: Vec<i32> = text_line.unwrap().split(",").map(|crab| crab.parse().unwrap()).collect();

        let mut best_fuel = 999999999;
        for test_pos in 0..2000 {
            let cost:i32 = crab_pos.iter().map(|pos| {
                let n = (pos - test_pos).abs();
                (n * (n + 1)) / 2 //Remove for part 1
            }).sum();
            if cost < best_fuel {
                best_fuel = cost;
            }
        }

        println!("Least cost: {}", best_fuel);
    }

    Ok(())
}