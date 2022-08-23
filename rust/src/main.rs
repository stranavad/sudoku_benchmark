mod solve;
mod data_parser;

use std::time::{Instant};

fn main() {
    let now = Instant::now();
    let mut correct: u32 = 0;
    // let mut incorrect: u32 = 0;
    let data = data_parser::get_data("/home/stranavadavid/programming/sudoku/sudoku_new.csv");

    for board in data {
        let solved_board = solve::run(board.0);
        if solved_board == board.1 {
            correct += 1;
        } else {
            // incorrect += 1;
            solve::print_board(&solved_board);
            solve::print_board(&board.1);
        }

    }
    let elapsed = now.elapsed();
    println!("Execution time: {:?} milliseconds", elapsed.as_millis());
    println!("Time per board: {:?} microseconds", elapsed.as_micros()/correct as u128);
    println!("Boards: {}", correct + 1);
    // println!("Incorrect: {} boards", incorrect);
}
