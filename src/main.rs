//Day 1, puzzle 1

use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./input/day3/input.txt")?;
    let reader = io::BufReader::new(file);

    let num_of_bits = 12;
    //Format:
    //array index = position in bit string
    //first tuple position = 0 count
    //second tuple position = 1 count
    let mut bit_counts = [(0, 0); 12];
    let mut diagnostic_values = vec![];

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

        diagnostic_values.push(bits);
    }
    //011101001110
    let mut ox_values = diagnostic_values.clone();
    for i in 0..num_of_bits {
        ox_values = ox_values.into_iter().filter(|bits| {
            if bit_counts[i].1 >= bit_counts[i].0 {
                bits[i] == true
            } else  {
                bits[i] == false
            }
        }).collect();
        if ox_values.len() == 1 {
            break;
        }
    }

    let mut scrubber_ratings = diagnostic_values.clone();
    for i in 0..num_of_bits {
        scrubber_ratings = scrubber_ratings.into_iter().filter(|bits| {
            if bit_counts[i].1 >= bit_counts[i].0 {
                bits[i] == false
            } else  {
                bits[i] == true
            }
        }).collect();
        if scrubber_ratings.len() == 1 {
            break;
        }
    }
    
    let mut scrubber_rating = 0; //2230
    let mut ox_value = 0; //1871

    for i in 0..num_of_bits {
        scrubber_rating = scrubber_rating << 1;
        ox_value = ox_value << 1;
        
        if scrubber_ratings[0][i] {
            scrubber_rating = scrubber_rating | 1;
        } else {
            scrubber_rating = scrubber_rating | 0;
        }

        if ox_values[0][i] {
            ox_value = ox_value | 1;
        } else {
            ox_value = ox_value | 0;
        }
    }

    println!("ox rating: {}, CO2 rating: {}, final: {}", ox_value,scrubber_rating, ox_value * scrubber_rating);
    Ok(())
}

