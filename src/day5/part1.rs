//Day 5, puzzle 1

use std::fs::File;
use std::io::{self, BufRead};

type Point = (usize, usize); //x, then y

fn main() -> io::Result<()> {
    let file = File::open("./input/day5/input.txt")?;
    let reader = io::BufReader::new(file);
    
    let mut board = [[0; 1000]; 1000]; //1000 x 1000 board

    for text_line in reader.lines() {
        //Input format:
        //X,Y -> X,Y (x and y are numbers between 0 and 1000)
        //Can only be horizontal and vertical (for now)
        //Top left is 0,0 and bottom right is 9,9

        //parse lines
        let line: Vec<Point> = text_line.unwrap()
        .split(" -> ")
        .map(|point_str| {
            let res: Vec<&str> = point_str.split(",").collect();
            (res[0].parse::<usize>().unwrap(), res[1].parse::<usize>().unwrap())
        })
        .collect();
        
        if line[0].0 == line[1].0 || line[0].1 == line[1].1 {
            //step through lines
            for point in build_line(line[0], line[1]) {
                //update board
                board[point.1][point.0] += 1;
            }
        }
    }

    let mut score = 0;
    //process board 
    for i in 0..1000 {
        for j in 0..1000 {
            if board[i][j] > 1 {
                score += 1;
            }
        }
    }

    println!("score: {}", score);
    //18895 TOO HIGH
    //5418  TOO LOW
    Ok(())
}

fn build_line(from: Point, to: Point) -> Vec<Point> {
    if to.0 > from.0 {
        //Count up in x
        let mut result = vec![(0, to.1); to.0 - from.0 + 1];
        for i in from.0..=to.0 {
            result[i - from.0].0 = i;
        }
        return result;
    } else if to.1 > from.1 {
        //Count up in y
        let mut result = vec![(to.0, 0); to.1 - from.1 + 1];
        for i in from.1..=to.1 {
            result[i - from.1].1 = i;
        }
        return result;
    } else if to.0 < from.0 {
        //Count down in x
        let mut result = vec![(0, to.1); from.0 - to.0 + 1];
        for i in to.0..=from.0 {
            result[i - to.0].0 = i;
        }
        return result;
    } else if to.1 < from.1 {
        //count down in y
        let mut result = vec![(from.0, 0); from.1 - to.1 + 1];
        for i in to.1..=from.1 {
            result[i - to.1].1 = i;
        }
        return result;
    }
    println!("Should never get here");
    vec![]
}