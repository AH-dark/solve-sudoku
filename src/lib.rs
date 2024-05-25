const SIZE: usize = 9;
const SUBGRID_SIZE: usize = 3;

/// Check if a sudoku board is valid.
///
/// # Arguments
///
/// * `board` - A reference to a 9x9 sudoku board.
/// * `row` - The row index.
/// * `col` - The column index.
/// * `c` - The character to check.
///
/// # Returns
///
/// A boolean value indicating whether the character is valid.
pub fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, c: char) -> bool {
    for i in 0..9 {
        if board[row][i] == c || board[i][col] == c || board[row / 3 * 3 + i / 3][col / 3 * 3 + i % 3] == c {
            return false;
        }
    }
    true
}

/// Solve the sudoku board using backtracking and bit operations.
///
/// # Arguments
///
/// * `board` - A mutable reference to a 9x9 sudoku board.
///
/// # Returns
///
/// A boolean value indicating whether the board is solved.
///
pub fn solve_sudoku(board: &mut Vec<Vec<char>>) -> bool {
    let mut rows = vec![0; SIZE];
    let mut cols = vec![0; SIZE];
    let mut blocks = vec![0; SIZE];

    for r in 0..SIZE {
        for c in 0..SIZE {
            if board[r][c] != '.' {
                let num = board[r][c].to_digit(10).unwrap() as usize;
                let mask = 1 << num;
                rows[r] |= mask;
                cols[c] |= mask;
                blocks[(r / SUBGRID_SIZE) * SUBGRID_SIZE + c / SUBGRID_SIZE] |= mask;
            }
        }
    }

    solve(board, &mut rows, &mut cols, &mut blocks, 0, 0)
}

/// Solve the sudoku board using backtracking.
///
/// # Arguments
///
/// * `board` - A mutable reference to a 9x9 sudoku board.
/// * `rows` - A mutable reference to a vector of 9 u16 integers representing the rows.
/// * `cols` - A mutable reference to a vector of 9 u16 integers representing the columns.
/// * `blocks` - A mutable reference to a vector of 9 u16 integers representing the 3x3 subgrids.
/// * `row` - The current row.
/// * `col` - The current column.
///
/// # Returns
///
/// A boolean value indicating whether the board is solved.
///
fn solve(
    board: &mut Vec<Vec<char>>,
    rows: &mut Vec<u16>,
    cols: &mut Vec<u16>,
    blocks: &mut Vec<u16>,
    row: usize,
    col: usize,
) -> bool {
    if row == SIZE {
        return true;
    }
    if col == SIZE {
        return solve(board, rows, cols, blocks, row + 1, 0);
    }
    if board[row][col] != '.' {
        return solve(board, rows, cols, blocks, row, col + 1);
    }

    for num in 1..=9 {
        let mask = 1 << num;
        let block_index = (row / SUBGRID_SIZE) * SUBGRID_SIZE + col / SUBGRID_SIZE;

        if (rows[row] & mask) == 0 && (cols[col] & mask) == 0 && (blocks[block_index] & mask) == 0 {
            board[row][col] = std::char::from_digit(num as u32, 10).unwrap();
            rows[row] |= mask;
            cols[col] |= mask;
            blocks[block_index] |= mask;

            if solve(board, rows, cols, blocks, row, col + 1) {
                return true;
            }

            board[row][col] = '.';
            rows[row] &= !mask;
            cols[col] &= !mask;
            blocks[block_index] &= !mask;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_sudoku() {
        let mut board: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert!(solve_sudoku(&mut board));

        let expected: Vec<Vec<char>> = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        assert_eq!(board, expected);
    }

    #[test]
    fn test_wrong_sudoku() {
        let mut board: Vec<Vec<char>> = vec![
            vec!['5', '3', '5', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert!(!solve_sudoku(&mut board));
    }
}