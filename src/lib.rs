fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, c: char) -> bool {
    for i in 0..9 {
        if board[row][i] == c || board[i][col] == c || board[row / 3 * 3 + i / 3][col / 3 * 3 + i % 3] == c {
            return false;
        }
    }
    true
}

fn solve_sudoku_helper(board: &mut Vec<Vec<char>>, empty_cells: &mut Vec<(usize, usize)>, index: usize) -> bool {
    if index == empty_cells.len() {
        return true;
    }

    let (row, col) = empty_cells[index];
    for c in '1'..='9' {
        if is_valid(board, row, col, c) {
            board[row][col] = c;
            if solve_sudoku_helper(board, empty_cells, index + 1) {
                return true;
            }
            board[row][col] = '.';
        }
    }

    false
}

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut empty_cells = Vec::new();
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == '.' {
                empty_cells.push((row, col));
            }
        }
    }

    empty_cells.sort_by_key(|&(row, col)| {
        let mut count = 0;
        for c in '1'..='9' {
            if is_valid(board, row, col, c) {
                count += 1;
            }
        }
        count
    });

    solve_sudoku_helper(board, &mut empty_cells, 0);
}