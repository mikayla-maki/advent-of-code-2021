//Day 3, puzzle 1

use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    //Lets see if I remember how to mess with bits....
    let mut x = 000;
    x = x | 1;
    x = x << 1;
    println!("x: {}", x);
    x = x | 0;
    x = x << 1;
    println!("x: {}", x);
    x = x | 1;
    // x = x << 1;
    println!("x: {}", x);

    let file = File::open("./input/day3/input.txt")?;
    let reader = io::BufReader::new(file);

    let num_of_bits = 12;
    //Format:
    //array index = position in bit string
    //first tuple position = 0 count
    //second tuple position = 1 count
    let mut bit_counts = [(0, 0); 12];

    for line in reader.lines() {        
        //parse bits
        let bits = line.unwrap()
        .chars()
        .map(|c| {
            match c {
                '1' => true,
                '0' => false,
                _ => {println!("This should be impossible"); false }
            }
        })
        .collect::<Vec<bool>>();

        for i in 0..num_of_bits {
            match bits[i] {
                false => bit_counts[i].0 += 1,
                true => bit_counts[i].1 += 1,
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..num_of_bits {
        gamma = gamma << 1; 
        epsilon = epsilon << 1;

        if bit_counts[i].0 > bit_counts[i].1 {
            gamma = gamma | 0;
            epsilon = epsilon | 1;
        } else {
            gamma = gamma | 1;
            epsilon = epsilon | 0;
        }
    }

    println!("Power consumption: {}", gamma * epsilon);
    Ok(())
}

