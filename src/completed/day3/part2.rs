//Day 3, puzzle 2

use std::fs::File;
use std::io::{self, BufRead};

const BITS: usize = 12;


fn main() -> io::Result<()> {
    let file = File::open("./input/day3/input.txt")?;
    let reader = io::BufReader::new(file);

    let num_of_bits = BITS;
    //Format:
    //array index = position in bit string
    //first tuple position = 0 count
    //second tuple position = 1 count
    let mut bit_counts = [(0, 0); BITS];
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

    let ox_value_vec = get_diagnostic_value(|(bit_counts, i)| {
        Box::new(move |bits| { 
            if bit_counts[i].1 >= bit_counts[i].0 {
                bits[i] == true
            } else  {
                bits[i] == false
            }
         })
    }, 0, diagnostic_values.clone(), bit_counts);

    let scrubber_rating_vec = get_diagnostic_value(|(bit_counts, i)| {
        Box::new(move |bits| { 
            if bit_counts[i].1 >= bit_counts[i].0 {
                bits[i] == false
            } else  {
                bits[i] == true
            }
         })
    }, 0, diagnostic_values.clone(), bit_counts);

    let scrubber_rating = convert_vec_to_int(scrubber_rating_vec);
    let ox_value = convert_vec_to_int(ox_value_vec);

    println!("ox rating: {}, CO2 rating: {}, final: {}", ox_value,scrubber_rating, ox_value * scrubber_rating);
    Ok(())
}

fn get_diagnostic_value<F>(
        bit_criteria: F,
        bit_index: usize,
        mut diagnostic_values: Vec<Vec<bool>>,
        bit_counts: [(i32, i32); BITS]
    ) -> Vec<bool> 
    where F: Fn((
            [(i32, i32); BITS],
            usize
        )) -> Box<dyn Fn(&Vec<bool>) -> bool>
    {
    let mut next_bit_counts = [(0,0); BITS];

    diagnostic_values = diagnostic_values.into_iter()
        .filter(bit_criteria((bit_counts, bit_index)))
        .inspect(|bits| {
            for i in 0..BITS {
                match bits[i] {
                    false => next_bit_counts[i].0 += 1,
                    true => next_bit_counts[i].1 += 1,
                }
            }
        })
        .collect();
    
    if diagnostic_values.len() == 1 {
        return diagnostic_values[0].clone();
    } else {
       return get_diagnostic_value(bit_criteria, bit_index + 1, diagnostic_values, next_bit_counts);
     }
}

fn convert_vec_to_int(bits: Vec<bool>) -> i32 {
    let mut tmp = 0;
    for bit in bits {
        tmp = tmp << 1;
        match bit {
            true => tmp = tmp | 1,
            false => tmp = tmp | 0,
        }
    }
    tmp
}