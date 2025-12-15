use crate::constants::DIRECTIONS_2D_ARRAY;
use crate::solution::Solution;
use anyhow::Result;
pub struct Day04;

impl Day04 {
    fn get_num_adjacent(
        &self,
        bounds: (usize, usize),
        pos: (usize, usize),
        grid: &Vec<Vec<char>>,
    ) -> usize {
        let mut num_adjacent: usize = 0;
        for dir in DIRECTIONS_2D_ARRAY {
            let check_row: isize = pos.0 as isize + dir.0;
            let check_col: isize = pos.1 as isize + dir.1;

            // row
            match check_row {
                n if n < 0 => {
                    continue;
                }
                n if n > bounds.0 as isize => {
                    continue;
                }
                _ => {}
            }

            // col
            match check_col {
                n if n < 0 => {
                    continue;
                }
                n if n > bounds.1 as isize => {
                    continue;
                }
                _ => {}
            }

            if grid[check_row as usize][check_col as usize] == '@' {
                num_adjacent += 1
            };
        }

        num_adjacent
    }
}

impl Solution for Day04 {
    fn part1(&self, input: &str) -> Result<String> {
        // Wrong Guesses
        // 158713 - Too high
        // 1320 - Too low
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut grabbable_rolls: usize = 0;
        let grid_bounds: (usize, usize) = (grid.len() - 1, grid[0].len() - 1);
        for r in 0..=grid_bounds.0 {
            for c in 0..=grid_bounds.1 {
                if grid[r][c] == '.' {
                    continue;
                };
                let num_adjacent: usize =
                    self.get_num_adjacent((grid_bounds.0, grid_bounds.1), (r, c), &grid);

                if num_adjacent < 4 {
                    grabbable_rolls += 1;
                }
            }
        }

        Ok(format!("{}", grabbable_rolls))
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let grid_bounds: (usize, usize) = (grid.len() - 1, grid[0].len() - 1);
        let mut removed_roll_count: usize = 0;

        loop {
            let mut moved_a_role: bool = false;
            for r in 0..=grid_bounds.0 {
                for c in 0..=grid_bounds.1 {
                    if grid[r][c] == '.' {
                        continue;
                    };
                    let num_adjacent: usize =
                        self.get_num_adjacent((grid_bounds.0, grid_bounds.1), (r, c), &grid);

                    if num_adjacent < 4 {
                        grid[r][c] = '.';
                        moved_a_role = true;
                        removed_roll_count += 1;
                    }
                }
            }

            if !moved_a_role {
                break;
            }
        }

        Ok(format!("{}", removed_roll_count))
    }
}
