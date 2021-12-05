//Day 1, puzzle 1

use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./input/day1/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut prev = 0;
    let mut increases = -1; //-1 because 0 < the first value in the data
    let mut decreases = 0;
    for line in reader.lines() {
        let cur = line.unwrap().parse::<i64>().unwrap();
        if prev < cur {
            increases += 1;
        } else {
            decreases += 1;
        }
        prev = cur;
    }
    println!("There where {} increases and {} decreases", increases, decreases);

    Ok(())
}
