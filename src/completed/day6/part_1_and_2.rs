//Day 6, puzzle 1 & 2
use std::fs::File;
use std::io::{self, BufRead};
fn main() -> io::Result<()> {
    let file = File::open("./input/day6/input.txt")?;
    let reader = io::BufReader::new(file);
    
    //index = age, value = count
    let mut age_counts:[u128; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for text_line in reader.lines() {
        let data: Vec<usize> = text_line.unwrap().split(",").map(|t| t.parse::<usize>().unwrap()).collect();
        for i in 0..data.len() {
            age_counts[data[i]] += 1;
        }
        for i in 0..256 {
            println!("on gen: {}", i);
            let spawn_count = age_counts[0];
            age_counts[0] = 0;
            for i in 1..9 {
                age_counts[i - 1] = age_counts[i];
            }
            age_counts[6] += spawn_count;
            age_counts[8] = spawn_count;
        }

        let mut fishies:u128 = 0;
        for i in 0..9 {
            fishies += age_counts[i]
        }
        println!("Fishies: {}", fishies);
    }
    Ok(())
}