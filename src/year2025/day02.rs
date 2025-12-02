use crate::solution::Solution;
use std::iter::successors;
use anyhow::Result;

pub struct Day02;

impl Day02 {
    fn generate_factors(&self, num: usize) -> Vec<usize> {
        let mut factors: Vec<usize> = vec![1];
        let mut count: usize = 2;
        while count <= (num / 2){
            if num % count == 0 { factors.push(count) }
            count += 1;
        }

        factors
    }

    fn is_repetition(&self, s: &str, pattern: &str) -> bool {
        successors(Some(s), |remaining| remaining.strip_prefix(pattern))
            .last()
            .map(|final_str| final_str.is_empty())
            .unwrap_or(false)
    }
}

impl Solution for Day02 {
    fn part1(&self, input: &str) -> Result<String> {
        let range_list: Vec<&str> = input.trim().split(',').collect();
        let mut invalid_total: u64 = 0;

        for r in range_list {
            let range_parts: Vec<&str> = r.split("-").collect();
            let r_start: u64 = range_parts[0].parse::<u64>()?;
            let r_end: u64 = range_parts[1].parse::<u64>()?;

            for num in r_start..r_end{
                let num_str: String = num.to_string();

                // Can only be the same number concatenated with itself if it's even
                if num_str.len() % 2 != 0 { continue; };

                let midpoint: usize = num_str.len() / 2;
                let num_str_halves= num_str.split_at(midpoint);
                if num_str_halves.0 == num_str_halves.1 {
                    invalid_total += num;
                }
            }
        }
        Ok(format!("{}", invalid_total))
    }

    fn part2(&self, input: &str) -> Result<String> {
        // Failed values:
        // 10629210284 too low
        // 10914107554 too low
        //

        let range_list: Vec<&str> = input.trim().split(',').collect();
        let mut invalid_total: u64 = 0;
        for r in range_list {
            let range_parts: Vec<&str> = r.split("-").collect();
            let r_start: u64 = range_parts[0].parse::<u64>()?;
            let r_end: u64 = range_parts[1].parse::<u64>()?;

            for num in r_start..r_end{
                if num < 11 { continue; } // If num is only one digit or 10
                let num_str: String = num.to_string();
                let len_factors: Vec<usize> = self.generate_factors(num_str.len());
                for factor in len_factors{
                    let substr: &str = &num_str[..factor];
                    if self.is_repetition(num_str.as_str(), substr){
                        invalid_total += num;
                        break;
                    }
                }
            }
        }
        Ok(format!("{}", invalid_total))
    }
}
