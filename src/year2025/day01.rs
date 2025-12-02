use crate::solution::Solution;
use anyhow::Result;

pub struct Day01;

impl Solution for Day01 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut position: i32 = 50;
        let mut zero_count: u32 = 0;
        let turns: Vec<&str> = input.lines().collect();

        for turn in turns {
            let first_char: char = turn.chars().next().unwrap();
            let rest = &turn[1..];
            let num_turns: i32 = rest.parse::<i32>()?;

            match first_char.to_ascii_uppercase() {
                'R' => { position = (position + num_turns).rem_euclid(100)}
                'L' => { position = (position - num_turns).rem_euclid(100)},
                _ => panic!("Invalid: {} {}", first_char, rest)
            }

            if position == 0 {
                zero_count += 1;
            }
        }

        Ok(format!("{}", zero_count))
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut position: i32 = 50;
        let mut zero_count: u32 = 0;
        let turns: Vec<&str> = input.lines().collect();

        for turn in turns {
            let first_char: char = turn.chars().next().unwrap();
            let rest = &turn[1..];
            let mut num_turns: i32 = rest.parse::<i32>()?;
            let full_rotations: i32 = num_turns / 100;

            zero_count += full_rotations as u32;
            num_turns = num_turns % 100;

            match first_char.to_ascii_uppercase() {
                'R' => {
                    if (position + num_turns) >= 100 {
                        zero_count += 1;
                    }
                    position = (position + num_turns).rem_euclid(100);
                },
                'L' => {
                    if (position - num_turns) <= 0 && position != 0 {
                        zero_count += 1;
                    }
                    position = (position - num_turns).rem_euclid(100);
                },
                _ => panic!("Invalid: {} {}", first_char, rest)
            }
        }
        Ok(format!("{}", zero_count))
    }
}
