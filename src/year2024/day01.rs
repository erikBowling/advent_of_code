use crate::solution::Solution;
use anyhow::Result;
use std::collections::HashMap;

pub struct Day01;

impl Solution for Day01 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut left_numbers: Vec<i32> = Vec::new();
        let mut right_numbers: Vec<i32> = Vec::new();

        for line in input.lines() {
            let line: String = line.to_string();
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect();

            if numbers.len() == 2 {
                left_numbers.push(numbers[0]);
                right_numbers.push(numbers[1]);
            }
        }

        left_numbers.sort();
        right_numbers.sort();

        let differences: Vec<i32> = left_numbers
            .iter()
            .zip(right_numbers.iter())
            .map(|(a, b)| (a - b).abs())
            .collect::<Vec<i32>>();

        let total_differences = differences.iter().sum::<i32>();

        Ok(format!("{}", total_differences))
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut left_numbers: Vec<i32> = Vec::new();
        let mut right_numbers: Vec<i32> = Vec::new();

        for line in input.lines() {
            let line: String = line.to_string();
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect();

            if numbers.len() == 2 {
                left_numbers.push(numbers[0]);
                right_numbers.push(numbers[1]);
            }
        }

        left_numbers.sort();
        right_numbers.sort();

        // Map elements to it's rate of occurance
        // Sum across those rates
        // Return result
        let mut occurence_rates: HashMap<i32, i32> = HashMap::new();

        // Calculate the number of occurences per number and save it as the value in the above hashmap
        for item in left_numbers {
            let mode_of_item: i32 = right_numbers.iter().filter(|x| *x == &item).count() as i32;

            occurence_rates.insert(item, mode_of_item);
        }

        let mut accumulator: i32 = 0;

        for (key, value) in occurence_rates.into_iter() {
            accumulator += key * value;
        }

        Ok(format!("{}", accumulator))
    }
}
