use std::io;
use std::io::Write;

use solve_sudoku::solve_sudoku;

fn main() {
    let mut board: Vec<Vec<char>> = Vec::new();

    // input
    for i in 0..9 {
        loop {
            print!("Enter row {} (e.g. 53..7....): ", i + 1);
            io::stdout().flush().unwrap();

            let mut row = String::new();
            io::stdin().read_line(&mut row).unwrap();
            let row: Vec<char> = row.trim().chars().collect();

            if row.len() == 9 {
                board.push(row);
                break;
            } else {
                println!("Invalid input. Please enter exactly 9 characters.");
            }
        }
    }

    solve_sudoku(&mut board);

    // output
    println!();
    for row in board {
        for cell in row {
            print!("{} ", cell);
        }

        println!();
    }
}
