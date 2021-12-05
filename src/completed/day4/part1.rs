//Day 4, puzzle 1

use std::fs::File;
use std::io::{self, BufRead};

const BOARD_SIZE: usize = 5;

type BoardValue = (i32, bool);
type Board = [Vec<BoardValue>; BOARD_SIZE];

fn main() -> io::Result<()> {
    let file = File::open("./input/day4/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut line_reader = reader.lines();
    
    //A list of 2D indexes to scan for scoring.
    let scoring_scans = [
        [(0,0),(1,0),(2,0),(3,0),(4,0)], //All columns
        [(0,1),(1,1),(2,1),(3,1),(4,1)],
        [(0,2),(1,2),(2,2),(3,2),(4,2)],
        [(0,3),(1,3),(2,3),(3,3),(4,3)],
        [(0,4),(1,4),(2,4),(3,4),(4,4)],

        [(0,0),(0,1),(0,2),(0,3),(0,4)], //All rows
        [(1,0),(1,1),(1,2),(1,3),(1,4)],
        [(2,0),(2,1),(2,2),(2,3),(2,4)],
        [(3,0),(3,1),(3,2),(3,3),(3,4)],
        [(4,0),(4,1),(4,2),(4,3),(4,4)],
    ];

    //Parse draws
    let draws = line_reader.next().unwrap().unwrap().split(",").map(|str| str.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    
    //parse boards
    let mut boards: Vec<Board> = vec![];
    while let Some(_) = line_reader.next() {
        let mut board = Board::default();
        for i in 0..BOARD_SIZE {
            let line = line_reader.next().unwrap().unwrap().split_whitespace().map(|str| (str.parse::<i32>().unwrap(), false)).collect::<Vec<BoardValue>>();
            board[i] = line;
        }    
        boards.push(board);
    }

    //Determine which board is the winning board
    'outer: for call in draws {
        for board in &mut boards {
            //Mark cell in the board
            for line in board.iter_mut() {
                for cell in line.iter_mut() {
                    if cell.0 == call {
                        cell.1 = true;
                    }
                }
            }
            //Check if won
            for scan in scoring_scans {
                let mut won = true;
                for (i, j) in scan {
                    match board[i][j] {
                        (_, true) => won = won && true,
                        (_, false) => won = won && false
                    }
                }
                //If found, score
                if won {
                    let mut score = 0;
                    for line in board.iter() {
                        for cell in line.iter() {
                            match cell {
                                (value, false) => score += value,
                                _ => ()
                            }
                        }
                    }
                    println!("Winner! The score is: {}", score * call);
                    break 'outer;
                }
            }
        }
    }

    //83293 is TOO HIGH

    Ok(())
}