//Day 2, puzzle 2

use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./input/day2/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;

    for line in reader.lines() {        
        if let [command, val] = &line.unwrap().split(" ").collect::<Vec<&str>>()[..] {
            let val = val.parse::<usize>().unwrap();
            match &command[..] {
                "forward" => { 
                    position += val;
                    depth += aim * val;
                },
                "down" => aim += val,
                "up" => aim -= val,
                &_ => println!("this should be impossible")
            }
       }
        
    }
    println!("Depth {}, position {}, aim, {}, multiplied {}", depth, position, aim, depth * position);

    Ok(())
}

