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
pub fn is_valid(
    board: &[&[char]],
    row: usize,
    col: usize,
    c: char,
) -> bool {
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
pub fn solve_sudoku(board: &mut Vec<Vec<char>>) -> bool {
    let mut rows: [u16; SIZE] = [0; SIZE];
    let mut cols: [u16; SIZE] = [0; SIZE];
    let mut blocks: [u16; SIZE] = [0; SIZE];

    let mut empties = Vec::new();
    for r in 0..SIZE {
        for c in 0..SIZE {
            if board[r][c] == '.' {
                empties.push((r, c));
            } else {
                let d = board[r][c].to_digit(10).expect("Invalid digit") as usize;
                let bit = 1 << d;
                rows[r] |= bit;
                cols[c] |= bit;
                blocks[(r / SUBGRID_SIZE) * SUBGRID_SIZE + c / SUBGRID_SIZE] |= bit;
            }
        }
    }

    solve_opt(board, &mut rows, &mut cols, &mut blocks, &mut empties, 0)
}

/// Solve the sudoku board using backtracking.
///
/// # Arguments
///
/// * `board` - A mutable reference to a 9x9 sudoku board.
/// * `rows` - A mutable reference to a vector of 9 u16 integers representing the rows.
/// * `cols` - A mutable reference to a vector of 9 u16 integers representing the columns.
/// * `blocks` - A mutable reference to a vector of 9 u16 integers representing the 3x3 subgrids.
/// * `empties` - A mutable reference to a vector of tuples representing the empty cells.
/// * `idx` - The current index in the `empties` vector.
///
/// # Returns
///
/// A boolean value indicating whether the board is solved.
fn solve_opt(
    board: &mut Vec<Vec<char>>,
    rows: &mut [u16; SIZE],
    cols: &mut [u16; SIZE],
    blocks: &mut [u16; SIZE],
    empties: &mut Vec<(usize, usize)>,
    idx: usize,
) -> bool {
    if idx == empties.len() {
        return true;
    }

    let (best_i, best_mask) = empties[idx..]
        .iter()
        .enumerate()
        .map(|(i, &(r, c))| {
            let used = rows[r] | cols[c] | blocks[(r / SUBGRID_SIZE) * SUBGRID_SIZE + c / SUBGRID_SIZE];
            let mask = (!used) & 0x3FE;
            (i, mask)
        })
        .min_by_key(|&(_, mask)| mask.count_ones())
        .unwrap();

    empties.swap(idx, idx + best_i);
    let (r, c) = empties[idx];
    let b = (r / SUBGRID_SIZE) * SUBGRID_SIZE + c / SUBGRID_SIZE;
    let mut mask = best_mask;

    while mask != 0 {
        let bit = mask & (!mask + 1);
        mask &= mask - 1;
        let num = bit.trailing_zeros() as usize;

        board[r][c] = std::char::from_digit(num as u32, 10).unwrap();
        rows[r] |= bit;
        cols[c] |= bit;
        blocks[b] |= bit;

        if solve_opt(board, rows, cols, blocks, empties, idx + 1) {
            return true;
        }


        board[r][c] = '.';
        rows[r] &= !bit;
        cols[c] &= !bit;
        blocks[b] &= !bit;
    }


    empties.swap(idx, idx + best_i);
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
