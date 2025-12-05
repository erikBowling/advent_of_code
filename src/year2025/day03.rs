use crate::solution::Solution;
use anyhow::Result;

pub struct Day03;

impl Day03 {


}

impl Solution for Day03 {
    fn part1(&self, input: &str) -> Result<String> {
        // Wrong Guesses
        // 17073 - too low
        let banks: Vec<&str> = input.lines().collect();
        let mut sum_joltage: u32 = 0;
        for bank in banks{
            let mut batteries = bank.chars();

            // First highest is the first in the list
            let mut tens_place: char = batteries.next().unwrap();
            let mut ones_place: char = '0';

            // Need to know if we are last comparing the last number.
            // If so, just update the ones place number
            let last: char = batteries.next_back().unwrap();

            for battery in batteries {
                if battery > tens_place {
                    tens_place = battery;
                    ones_place = '0';
                } else if battery > ones_place {
                    ones_place = battery;
                }
            }

            // Check last num
            if last > ones_place { ones_place = last; }

            let mut final_num_str: String = String::new();
            final_num_str.push(tens_place);
            final_num_str.push(ones_place);
            sum_joltage += final_num_str.parse::<u32>()?;
        }
        Ok(format!("{}", sum_joltage))
    }

    fn part2(&self, input: &str) -> Result<String> {
        // Wrong guesses
        // 169157302525341 too high

        let banks: Vec<&str> = input.lines().collect();
        let mut sum_joltage: u64 = 0;
        for bank in banks {
            let bank_length: usize = bank.len();
            let mut largest_num: Vec<char> = vec!['0'; 12];
            let mut batteries = bank.char_indices();

            while let Some((pos, ch)) = batteries.next() {
                let remaining: usize = bank_length - pos - 1;
                if remaining >= 12 {
                    match largest_num.iter().position(|x| x < &ch) {
                        Some(index) => {
                            largest_num[index] = ch;
                            largest_num.iter_mut().skip(index+1).for_each(|x| *x = '0');
                        }
                        None => {}
                    }
                } else {
                    match largest_num.iter()
                        .skip(12 - (remaining + 1))
                        .position(|x| x < &ch)
                        .map(|p| p + (12 - (remaining + 1)))
                    {
                        Some(index) => {
                            largest_num[index] = ch;
                            largest_num.iter_mut().skip(index+1).for_each(|x| *x = '0');
                        }
                        None => {}
                    }
                }
            }

            let num_str: String = largest_num.iter().collect();
            sum_joltage += num_str.parse::<u64>()?;

        }
        Ok(format!("{}", sum_joltage))
    }
}
