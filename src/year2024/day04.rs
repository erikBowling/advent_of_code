use crate::{constants::DIRECTIONS_2D_ARRAY, solution::Solution};
use anyhow::Result;

pub struct Day04;

impl Day04 {
    fn check_location_any(&self, puzzle: &Vec<Vec<char>>, row: usize, col: usize) -> isize {
        if puzzle[row][col] != 'X' {
            return 0;
        }

        let mut accumulator = 0;
        let directions: [(isize, isize); 8] = DIRECTIONS_2D_ARRAY;

        for direction in directions.iter() {
            if self.check_direction(puzzle, row, col, *direction, "XMAS") {
                accumulator += 1;
            }
        }
        accumulator
    }

    fn check_location_x(&self, puzzle: &Vec<Vec<char>>, row: usize, col: usize) -> isize {
        if puzzle[row][col] != 'A' {
            return 0;
        }

        let mut num_mas: isize = 0;
        let directions: Vec<(isize, isize)> = vec![
            (-1, 1),  // up_right
            (-1, -1), // up_left
            (1, 1),   // down_right
            (1, -1),  // down_left
        ];

        let num_rows = puzzle.len();
        let num_cols = puzzle[0].len();

        for direction in directions.iter() {
            let mut current_row: usize = row;
            let mut current_col: usize = col;
            match (
                current_row.checked_add_signed(direction.0),
                current_col.checked_add_signed(direction.1),
            ) {
                (Some(new_row), Some(new_col)) => {
                    current_row = new_row;
                    current_col = new_col;
                }
                _ => continue,
            }
            if current_row >= num_rows || current_col >= num_cols {
                continue;
            }

            if puzzle[current_row][current_col] == 'M' {
                if self.check_direction(
                    puzzle,
                    current_row,
                    current_col,
                    self.opposite(direction),
                    "MAS",
                ) {
                    num_mas += 1;
                }
            }
        }

        if num_mas >= 2 {
            return 1;
        }

        0
    }

    fn check_direction(
        &self,
        puzzle: &Vec<Vec<char>>,
        mut row: usize,
        mut col: usize,
        direction: (isize, isize),
        check_string: &str,
    ) -> bool {
        let num_rows = puzzle.len();
        let num_cols = puzzle[0].len();

        for c in check_string.chars() {
            if row >= num_rows || col >= num_cols {
                return false;
            }
            if puzzle[row][col] != c {
                return false;
            }
            if c == 'S' {
                return true;
            }

            match (
                row.checked_add_signed(direction.0),
                col.checked_add_signed(direction.1),
            ) {
                (Some(new_row), Some(new_col)) => {
                    row = new_row;
                    col = new_col;
                }
                _ => return false,
            }
        }

        true
    }

    fn opposite(&self, direction: &(isize, isize)) -> (isize, isize) {
        (direction.0 * -1, direction.1 * -1)
    }
}

impl Solution for Day04 {
    fn part1(&self, input: &str) -> Result<String> {
        let word_search: Vec<Vec<char>> =
            input.lines().map(|line| line.chars().collect()).collect();

        let num_rows = word_search.len();
        let num_cols = word_search[0].len();

        let mut num_of_searches = 0;
        for row in 0..num_rows {
            for col in 0..num_cols {
                num_of_searches += self.check_location_any(&word_search, row, col);
            }
        }

        Ok(format!("{}", num_of_searches))
    }

    fn part2(&self, input: &str) -> Result<String> {
        let word_search: Vec<Vec<char>> =
            input.lines().map(|line| line.chars().collect()).collect();

        let num_rows = word_search.len();
        let num_cols = word_search[0].len();

        let mut num_x_mas = 0;
        for row in 0..num_rows {
            for col in 0..num_cols {
                num_x_mas += self.check_location_x(&word_search, row, col);
            }
        }

        Ok(format!("{}", num_x_mas))
    }
}
