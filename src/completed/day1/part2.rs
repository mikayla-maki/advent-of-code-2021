//Day 1, puzzle 2

use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./input/day1/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut sums = [0, 0, 0];
    let mut sum_indx = [0,-1,-2];
    let mut first_indx: usize = 99; //Index of the triple that completed last line
    let mut second_indx: usize = 99; //Index of the triple that completed this line

    let mut increases = 0;
    let mut decreases = 0;
    
    for line in reader.lines() {
        let val = line.unwrap().parse::<i64>().unwrap();
        //UPDATE STEP
        for i in 0..=2 {
            if sum_indx[i] >= 0 { //Should be true for all but initial values
                if sum_indx[i] < 3 {
                    sums[i] += val;
                } else {
                    first_indx = i;
                }
            }
            sum_indx[i] += 1;

            if sum_indx[i] == 3 {
                second_indx = i;
            }
        }
        //POST-UPDATE STEP
        if first_indx != 99 && second_indx != 99 { //Should be true for all but initial values
            if sums[first_indx] < sums[second_indx] {
                increases += 1;
            } else {
                decreases += 1;
            }
            sums[first_indx] = val; //Start up the next triple
            sum_indx[first_indx] = 1
        }
    }
    println!("There where {} increases and {} decreases", increases, decreases);

    Ok(())
}
/*               (0, -1, -2) CI: -1
199  A           AFTER: (1, 0, -1) CI: -1
200  A B         AFTER: (2, 1, 0) CI: -1
208  A B C       AFTER: (3, 2, 1) CI: -1
210    B C D     AFTER: (4, 3, 2) CI: 0
200  E   C D     ()
207  E F   D     ()
240  E F G       ()
269    F G H     ()
260      G H     ()
263        H     ()
*/
