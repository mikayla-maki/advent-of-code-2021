//Day 8, puzzle 1 & 2

use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./input/day8/testinput2.txt")?;
    let reader = io::BufReader::new(file);
    


    let mut total = 0;
    for text_line in reader.lines() {
        let line = text_line.unwrap();
        let inputAndOutput = line.split(" | ").collect::<Vec<&str>>();
        let mut input = inputAndOutput[0].split(" ").collect::<Vec<&str>>();
        let output = inputAndOutput[1];

        /*We can always find digits 1 4 7 and 8.
        Comparing digits 1, 4, 7, and 8 gives us the identities of all other 
        COMPARE 1, 4 => segments c & f
        COMPARE 4, 8 => segments b & d (after subtract c & f)
        COMPARE 4, 7 => segment a (after subtract c & f)

        EXAMPLE: 
        acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
           8                        7                 4          1
   
        8 => acedgfb
        7 => dab
        4 => eafb
        1 => ab

        COMPARE 1, 4 => segments a & b MAP TO c & f, e & f MAP TO b & d
        COMPARE 4, 7 => segment d MAP TO a
        
        NEW c & f => 0, 3, 9
        NEW b & d => 5, 6, 9

        c & f + b & d => 9 

        COMPARE 9 and what we've found so far, gives us a hard mapping to g
        What we know so far: c&f, b&d, a, g 

        NEW a + g => 6, 5, 3, 2, 0
        a + g + c&f => 0, 3
        compare lengths of 0 & 3 to disambiguate b & d 
        */
        /*
        let pool = LIST OF UNKNOWN NUMBERS
        let mappings = character => location in real segment
        ALGORITHM:
        Using the unique length of the text string GAIN 1, 4, 7, 8
        Find the matching characters between 1 & 4, map to *c&f
        Find the different characters between 1 & 4, map to *b&d
        find the matching characters between 4 & 7, subtract the characters in c&f, map result to *a
        Find the text string containing *c&f + *b&d, GAIN 9
        Subtract *c&f, *b&d, and *a to 9, map result to *g
        find the 2 strings containing *a, *g, *c&f, test length, length six GAIN 0, length five GAIN 3
        0 contains b & not d, 3 contains d & b, use to disambiguate *b, *d
        Whichever character is left in 0 after removing *a, *b, *c&f, *g, map to *e
        Take the strings of length 
        //Strings of length 5 left in unknown pool: 2, 5, 

        //At this point: 0,1,3,4,7,8,9 => 2,5,6
        // *a, *b, *d, *e, *g, *c&f => cf

        //GOAL: given the input, generate a mapping from letters to segment index
        */
        let one = remove(&mut input, |str| str.len() == 2);
        let four = remove(&mut input, |str| str.len() == 4);
        let seven = remove(&mut input, |str| str.len() == 3);
        let eight = remove(&mut input, |str| str.len() == 7);

        let cAndf = matching(one, four);
        let bAndd = subtract(four, cAndf);
        let a = subtract(matching(four, seven), cAndf)[0];

        let nine = remove(&mut input, |str| str.contains(cAndf)[0] && str.contains(cAndf)[1] 
                                         && str.contains(bAndd)[0] && str.contains(bAndd)[1] );

        let g = subtract(subtract(subtract(nine, cAndf), bAndd), a);
        let zeroAnd3 = ( remove(&mut input, |str| str.contains(cAndf)[0] &&  str.contains(cAndf)[1] && str.contains(a) && str.contains(g)),
                         remove(&mut input, |str| str.contains(cAndf)[0] &&  str.contains(cAndf)[1] && str.contains(a) && str.contains(g)));

        let zero;
        let three;
        if zeroAnd3.0.len() == 6 { //Disambiguate
            zero = zeroAnd3.0;
            three = zeroAnd3.1; 
        } else {
            zero = zeroAnd3.1;
            three = zeroAnd3.0; 
        }

        let b;
        let d;
        if zero.contains(bAndd[0]) {
            b = bAndd[0];
            d = bAndd[1]
        } else {
            b = bAndd[1];
            d = bAndd[0];
        }

        let e = subtract(subtract(subtract(subtract(zero, a), b), cAndf), g);
        let six = remove(&mut input, |str| str.len() == 6);

        //Just 2, 5 cAndF, left.


        println!("1: {}, 4: {}, 7: {}, 8: {}, list: {:?}", one, four, seven, eight, input);
        panic!();    
    }

    println!("{}", total);

    Ok(())
}

fn remove<T, F>( list: &mut Vec<T>, pred: F) -> T
    where F: FnMut(&T) -> bool  {
    let index = list.iter().position(pred).unwrap();
    let item = list.swap_remove(index);
    return item;
}

//Based on the mapping shown at the beginning of the challenge
fn match_data_to_num(x: (usize,usize,usize,usize,usize,usize,usize)) -> usize{
    match x {
    //   a b c d e f g
        (1,1,1,0,1,1,1) => 0,
        (0,0,1,0,0,1,0) => 1,
        (1,0,1,1,1,0,1) => 2,
        (1,0,1,1,0,1,1) => 3,
        (0,1,1,1,0,1,0) => 4,
        (1,1,0,1,0,1,1) => 5,
        (1,1,0,1,1,1,1) => 6,
        (1,0,1,0,0,1,0) => 7,
        (1,1,1,1,1,1,1) => 8,
        (1,1,1,1,0,1,1) => 9,
        (_,_,_,_,_,_,_) => 99999,
    }
}