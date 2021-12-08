//Day 8, puzzle 1 & 2

use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./input/day8/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut total = 0;
    for text_line in reader.lines() {
        let z = text_line.unwrap();
        let x = z.split(" | ").collect::<Vec<&str>>();
        let y: Vec<&str> = x[1].split(" ").filter(|str| {
            str.len() == 2 || str.len() == 4 || str.len() == 3 || str.len() == 7
        }).collect();
        total += y.len();
        
    }

    println!("{}", total);

    Ok(())
}

