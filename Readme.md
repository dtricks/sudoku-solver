# Sudoku Solver in Rust

This is a simple Sudoku solver written in Rust. It reads Sudoku puzzles from text files, solves them using parallel processing, and validates the solutions.

## Features

- Reads Sudoku puzzles from text files in different formats.
- Solves puzzles using parallel processing with the Rayon library.
- Validates the solved puzzles to ensure correctness.

## Dependencies

- [anyhow](https://crates.io/crates/anyhow) for error handling.
- [rayon](https://crates.io/crates/rayon) for parallel processing.

## Usage

1. Clone the repository:
    ```sh
    git clone <repository-url>
    cd sudoku-solver
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Run the solver with a test file:
    ```sh
    cargo run --release
    ```

## File Formats

### Sudoku Exchange Format

Each line in the file represents a Sudoku puzzle in the Sudoku Exchange format. The first 12 characters are the title, followed by the puzzle cells.

### Custom Format

Each puzzle starts with a title line followed by 9 lines representing the Sudoku board.
