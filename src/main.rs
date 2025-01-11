use rayon::prelude::*;
use std::io::{BufRead, Lines};

use rayon::iter::IntoParallelRefIterator;

use anyhow::Result;

fn main() {
    //let test_cases = read_test_cases("test-sudoku2.txt");
    let test_cases = read_sudoku_exchange_files("test-sudoku3.txt");
    let test_cases_len = test_cases.len();
    //use rayon to parallelize the solving of the boards
    // Results on M1 Mac for sudoku3.txt:
    // Total time: 138.313337917s
    // Average time: 1.155683ms
    let total_time = std::time::Instant::now();
    test_cases
        .into_par_iter()
        .for_each(|test_case| match test_case {
            Ok(board_with_title) => {
                //println!("{}", board_with_title.title);
                //println!("{}", board_with_title.board);
                //println!("{}", is_valid(&board_with_title.board));

                let mut board = board_with_title.board;
                let start = std::time::Instant::now();
                solve_board(&mut board);
                let end = start.elapsed();
                //println!("Solved:\n{}", board);
                //println!("{}", is_valid(&board));
                //println!("Time: {:?}\n", end);
            }
            Err(e) => eprintln!("{}", e),
        });
    let total_time = total_time.elapsed();
    println!("Total time: {:?}", total_time);
    let avg_time = total_time / test_cases_len as u32;
    println!("Average time: {:?}", avg_time);
}

#[derive(Debug, PartialEq)]
struct SudokuBoardWithTitle {
    title: String,
    board: SudokuBoard,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct SudokuBoard {
    board: [[u8; 9]; 9],
}

impl SudokuBoard {
    fn new() -> SudokuBoard {
        SudokuBoard { board: [[0; 9]; 9] }
    }
}

impl Default for SudokuBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, row) in self.board.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                writeln!(f, "------+-------+------")?;
            }
            for (j, cell) in row.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "| ")?;
                }
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn is_valid(board: &SudokuBoard) -> bool {
    is_rows_valid(board) && is_columns_valid(board) && is_squares_valid(board)
}

fn is_rows_valid(board: &SudokuBoard) -> bool {
    for row in board.board.iter() {
        let mut seen = [false; 9];
        for cell in row.iter() {
            if *cell == 0 {
                continue;
            }
            if seen[*cell as usize - 1] {
                return false;
            }
            seen[*cell as usize - 1] = true;
        }
    }
    true
}

fn is_columns_valid(board: &SudokuBoard) -> bool {
    for i in 0..9 {
        let mut seen = [false; 9];
        for j in 0..9 {
            let cell = board.board[j][i];
            if cell == 0 {
                continue;
            }
            if seen[cell as usize - 1] {
                return false;
            }
            seen[cell as usize - 1] = true;
        }
    }
    true
}

fn is_squares_valid(board: &SudokuBoard) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            let mut seen = [false; 9];
            for k in 0..3 {
                for l in 0..3 {
                    let cell = board.board[3 * i + k][3 * j + l];
                    if cell == 0 {
                        continue;
                    }
                    if seen[cell as usize - 1] {
                        return false;
                    }
                    seen[cell as usize - 1] = true;
                }
            }
        }
    }
    true
}

fn read_board(board_with_title: &str) -> Result<SudokuBoardWithTitle> {
    let mut lines = board_with_title.split('\n');
    let title = lines.next().unwrap().to_string();
    let mut board = [[0; 9]; 9];
    for (i, line) in lines.enumerate() {
        for (j, cell) in line.chars().enumerate() {
            board[i][j] = cell.to_string().parse()?;
        }
    }
    Ok(SudokuBoardWithTitle {
        title,
        board: SudokuBoard { board },
    })
}

fn read_sudoku_exchange_board(board_line: &str) -> Result<SudokuBoardWithTitle> {
    let title = board_line.chars().take(12).collect::<String>();
    let mut board = [[0; 9]; 9];
    for (i, cell) in board_line.chars().skip(13).enumerate() {
        if cell.is_whitespace() {
            break;
        }
        board[i / 9][i % 9] = cell.to_string().parse()?;
    }
    Ok(SudokuBoardWithTitle {
        title,
        board: SudokuBoard { board },
    })
}

fn read_sudoku_exchange_files(file: &str) -> Vec<Result<SudokuBoardWithTitle>> {
    let file = std::fs::File::open(file).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut lines = reader.lines();
    let mut test_cases = Vec::new();
    while let Some(Ok(board_line)) = lines.next() {
        test_cases.push(read_sudoku_exchange_board(&board_line));
    }
    test_cases
}

fn read_test_cases(file: &str) -> Vec<Result<SudokuBoardWithTitle>> {
    let file = std::fs::File::open(file).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut lines = reader.lines();
    let mut test_cases = Vec::new();
    while let Some(Ok(title)) = lines.next() {
        let mut board = String::new();
        for _ in 0..9 {
            board.push_str(&lines.next().unwrap().unwrap());
            board.push('\n');
        }
        test_cases.push(read_board(&format!("{}\n{}", title, board)));
    }
    test_cases
}

fn solve_board(board: &mut SudokuBoard) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if board.board[i][j] == 0 {
                for k in 1..=9 {
                    board.board[i][j] = k;
                    if is_valid(board) && solve_board(board) {
                        return true;
                    }
                    board.board[i][j] = 0;
                }
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_test_boards() -> Vec<Result<SudokuBoardWithTitle>> {
        let t1 = read_test_cases("test-sudoku.txt");
        let t2 = read_test_cases("test-sudoku2.txt");
        t1.into_iter().chain(t2.into_iter()).collect()
    }

    #[test]
    fn test_new() {
        let board = SudokuBoard::new();
        assert_eq!(board.board, [[0; 9]; 9]);
    }

    #[test]
    fn test_default() {
        let board = SudokuBoard::default();
        assert_eq!(board.board, [[0; 9]; 9]);
    }

    #[test]
    fn test_single_solve_time() {
        let boards = load_test_boards();
        let mut board = boards[0].as_ref().unwrap().board;
        let start = std::time::Instant::now();
        solve_board(&mut board);
        let res = is_valid(&board);
        let end = start.elapsed();
        println!("Time: {:?}", end);
        assert!(res);
    }

    #[test]
    fn test_avg_solve_time() {
        let boards = load_test_boards();
        let mut total_time = std::time::Duration::new(0, 0);
        for board in boards.iter().map(|b| b.as_ref().unwrap().board) {
            let mut board = board;
            let start = std::time::Instant::now();
            solve_board(&mut board);
            let res = is_valid(&board);
            let end = start.elapsed();
            println!("Time: {:?}", end);
            total_time += end;
            assert!(res);
        }
        println!("Average time: {:?}", total_time / boards.len() as u32);
    }
}
