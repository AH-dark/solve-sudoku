# Sudoku Solver

This is a simple sudoku solver written in Rust. It uses a backtracking algorithm to solve the sudoku puzzle.

## Example

```
Enter row 1 (e.g. 53..7....): 3786294.5
Enter row 2 (e.g. 53..7....): ..9..176.
Enter row 3 (e.g. 53..7....): ....4...8
Enter row 4 (e.g. 53..7....): ...18..9.
Enter row 5 (e.g. 53..7....): ..2..4.57
Enter row 6 (e.g. 53..7....): ..65372..
Enter row 7 (e.g. 53..7....): .1.2..5..
Enter row 8 (e.g. 53..7....): .57.....1
Enter row 9 (e.g. 53..7....): 9.431.67.

3 7 8 6 2 9 4 1 5 
4 2 9 8 5 1 7 6 3 
5 6 1 7 4 3 9 2 8 
7 4 5 1 8 2 3 9 6 
8 3 2 9 6 4 1 5 7 
1 9 6 5 3 7 2 8 4 
6 1 3 2 7 8 5 4 9 
2 5 7 4 9 6 8 3 1 
9 8 4 3 1 5 6 7 2 
```

## How to run

1. Clone the repository
2. Run `cargo run` in the project directory
3. Enter the sudoku puzzle row by row. Use `.` to represent empty cells.
