use crate::solution::Solution;
use anyhow::Result;

use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day07;

impl Day07 {

}

impl Solution for Day07 {
    fn part1(&self, input: &str) -> Result<String> {

        let mut total_splits: usize = 0;
        let mut current_lines: HashSet<usize> = HashSet::new();
        let mut input_lines: VecDeque<&str> = input.lines().collect();
        let first_line = input_lines.pop_front().unwrap();

        let bounds = 0..=first_line.len() - 1;

        for (i, ch) in first_line.chars().enumerate() {
            if ch == 'S' {
                current_lines.insert(i);
            }
        }

        for line in input_lines {
            for (i, ch) in line.chars().enumerate() {
                if ch == '^' && current_lines.contains(&i) {
                    total_splits += 1;
                    current_lines.remove(&i);

                    // Check if carrot +-1 is in bounds
                    if bounds.contains(&(i - 1)) {
                        current_lines.insert(i - 1);
                    }

                    if bounds.contains(&(i + 1)) {
                        current_lines.insert(i + 1);
                    }
                }
            }
        }
        Ok(format!("{}", total_splits))
    }

    fn part2(&self, input: &str) -> Result<String> {
        // Wrong Guesses
        // 1776 - Too low
        let mut input_lines = input.lines();

        let first_line = input_lines.next().unwrap();
        let bounds = 0..=first_line.len() - 1;

        let mut branches: HashMap<usize, usize> = HashMap::new();

        for (i, ch) in first_line.chars().enumerate() {
            if ch == 'S' {
                branches.insert(i, 1);
            }
        }

        input_lines.for_each(
            |line| {
                let mut next: HashMap<usize, usize> = HashMap::new();
                for (&pos, &count) in branches.iter() {
                    if line.chars().nth(pos) == Some('^') {

                        // Left branch
                        if pos > 0 && bounds.contains(&(pos - 1)) {
                            *next.entry(pos - 1).or_insert(0) += count;
                        }

                        // Right branch
                        if bounds.contains(&(pos + 1)) {
                            *next.entry(pos + 1).or_insert(0) += count;
                        }
                    } else {
                        *next.entry(pos).or_insert(0) += count;
                    }
                }

                branches = next;
            }
        );

        let total: usize = branches.into_values().sum();

        Ok(format!("{}", total))
    }
}
