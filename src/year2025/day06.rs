use crate::solution::Solution;
use anyhow::Result;

use std::{collections::HashSet};

pub struct Day06;

impl Day06 {
}

impl Solution for Day06 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut problem_lines: Vec<&str> = input.lines().collect();
        let operations: Vec<char> = problem_lines.pop().unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        let lines_nums: Vec<Vec<i64>> = problem_lines.iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect()
            })
            .collect();

        let total: i64 = operations.iter().enumerate()
            .map(|(index, op)| {
                let mut inner_total: i64 = 0;
                if *op == '+' {
                    inner_total = lines_nums.iter()
                        .map(|nums| nums.get(index).unwrap())
                        .sum();
                }

                if *op == '*' {
                    inner_total = lines_nums.iter()
                        .map(|nums| nums.get(index).unwrap())
                        .product();
                }

                inner_total
            }).sum();

        Ok(format!("{}", total))
    }

    fn part2(&self, input: &str) -> Result<String> {
        // Wrong Guesses
        // 10196824920990 - too low
        let problem_lines: Vec<&str> = input.lines().collect();
        let whitespace_locations: Vec<HashSet<usize>> = problem_lines.iter()
            .map(|line| {
                line.char_indices()
                    .filter(|(_ , ch)| ch.is_whitespace())
                    .map(|(index, _)| index)
                    .collect()
            })
            .collect();

        let whitespace_intersection: HashSet<usize> = if let Some(first) = whitespace_locations.first(){
            whitespace_locations.iter()
                  .skip(1)
                  .fold(first.clone(), |acc, set| {
                      acc.intersection(set).copied().collect()
                  })
          } else {
              HashSet::new()
          };

        let mut split_indices: Vec<usize> = whitespace_intersection.into_iter().collect();
        split_indices.sort();

        let mut split_lines: Vec<Vec<&str>> = problem_lines.iter()
            .map(|line| {
                let mut segments = Vec::new();
                   let mut start = 0;
                   for &index in &split_indices {
                       if index > start && index <= line.len() {
                           segments.push(&line[start..index]);
                           start = index + 1;
                       }
                   }

                   if start < line.len() {
                       segments.push(&line[start..]);
                   }
                   segments
            })
            .collect();

        let operations: Vec<&str> = split_lines.pop().unwrap();
        let mut columns: Vec<Vec<&str>> = Vec::new();

        operations.iter()
            .enumerate()
            .for_each(|(index, _)| {
                let mut col_nums: Vec<&str> = Vec::new();
                split_lines.iter()
                    .for_each(|line| {
                        col_nums.push(line[index]);
                    });
                columns.push(col_nums);
            });

        let mut col_nums: Vec<Vec<i64>> = Vec::new();

        columns.iter()
            .for_each(|col_list| {
                let length = col_list[0].len();
                let mut num_list: Vec<i64> = Vec::new();
                for i in 0..length {
                    let mut col_str: String = String::new();
                    col_list.iter()
                        .for_each(|str_num| {
                            col_str.push(str_num.chars().nth(i).unwrap());
                        });

                    match col_str.trim().parse::<i64>() {
                        Ok(num) => {
                            num_list.push(num);
                        }
                        Err(e) => {
                            eprintln!("Failed to parse '{}': {}", col_str, e);
                        }
                    }
                }

                col_nums.push(num_list);
            });

        let col_zip = operations.iter().zip(col_nums);

        let total: i64 = col_zip.into_iter()
            .map(|x| {
                let mut inner_total: i64 = 0;
                if x.0.trim() == "+" {
                    inner_total = x.1.iter().sum();
                }

                if x.0.trim() == "*" {
                    inner_total = x.1.iter().product();
                }

                inner_total
            })
            .sum();

        Ok(format!("{}", total))
    }
}
