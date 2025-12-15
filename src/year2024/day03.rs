use crate::solution::Solution;
use anyhow::Result;
use regex::Regex;

pub struct Day03;

impl Day03 {
    fn create_int_tuple(&self, str_mul: &String) -> (i32, i32) {
        let int_pattern = Regex::new(r"[0-9]+").unwrap();
        let current_mul: Vec<i32> = int_pattern
            .find_iter(&str_mul)
            .map(|m| m.as_str().parse::<i32>().expect("Failed to parse integer"))
            .collect();

        return (*current_mul.first().unwrap(), *current_mul.last().unwrap());
    }
}

impl Solution for Day03 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut corrupted_memory: Vec<String> = Vec::new();

        for line in input.lines() {
            let str: String = line.to_string();
            corrupted_memory.push(str);
        }

        let mul_pattern = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
        let mut valid_multiplications: Vec<String> = Vec::new();

        for line in corrupted_memory {
            let mut current_line_valid_multiplications: Vec<String> = mul_pattern
                .find_iter(line.as_str())
                .map(|m| m.as_str().to_string())
                .collect();

            valid_multiplications.append(&mut current_line_valid_multiplications);
        }

        let mut mul_tuples: Vec<(i32, i32)> = Vec::new();
        for str_mul in valid_multiplications {
            mul_tuples.push(self.create_int_tuple(&str_mul));
        }

        let result: i32 = mul_tuples.iter().map(|(a, b)| a * b).sum();

        Ok(format!("{}", result))
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut corrupted_memory: Vec<String> = Vec::new();

        for line in input.lines() {
            let str: String = line.to_string();
            corrupted_memory.push(str);
        }

        let match_patterns = Regex::new(r"don't\(\)|do\(\)|mul\([0-9]+,[0-9]+\)").unwrap();
        let mut all_matches: Vec<String> = Vec::new();
        for line in corrupted_memory {
            let mut match_pattern_results: Vec<String> = match_patterns
                .find_iter(&line)
                .map(|m| m.as_str().to_string())
                .collect();

            all_matches.append(&mut match_pattern_results);
        }

        let mut allowed = true;
        let mut mul_tuples: Vec<(i32, i32)> = Vec::new();

        for str in all_matches {
            if str == "do()" {
                allowed = true;
            } else if str == "don't()" {
                allowed = false;
            } else {
                if allowed {
                    mul_tuples.push(self.create_int_tuple(&str));
                } else {
                    continue;
                }
            }
        }

        let result: i32 = mul_tuples.iter().map(|(a, b)| a * b).sum();

        Ok(format!("{}", result))
    }
}
