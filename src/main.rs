//Day 7, puzzle 1

use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./input/day6/input.txt")?;
    let reader = io::BufReader::new(file);
    

    for text_line in reader.lines() {
        
    }

    Ok(())
}