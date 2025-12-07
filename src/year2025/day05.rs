use crate::solution::Solution;
use anyhow::Result;

pub struct Day05;

impl Day05 {
    fn parse_range(line: &str) -> Result<(u64, u64)> {
        let (lower, upper) = line.split_once('-')
            .ok_or_else(|| anyhow::anyhow!("Invalid range format"))?;
        Ok((lower.parse()?, upper.parse()?))
    }
}

impl Solution for Day05 {
    fn part1(&self, input: &str) -> Result<String> {
        let (ranges_str, ids_str) = input.split_once("\n\n")
            .ok_or_else(|| anyhow::anyhow!("Invalid input format"))?;

        let fresh_ranges: Vec<(u64, u64)> = ranges_str
            .lines()
            .filter(|line| !line.is_empty())
            .map(Self::parse_range)
            .collect::<Result<_>>()?;

        let food_ids: Vec<u64> = ids_str
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<u64>())
            .collect::<Result<_, _>>()?;

        let fresh_ids = food_ids.iter()
            .filter(|&id| {
                fresh_ranges.iter()
                    .any(|(lower, upper)| (lower..=upper).contains(&id))
            })
            .count();

        Ok(format!("{}", fresh_ids))
    }

    fn part2(&self, input: &str) -> Result<String> {
        // Wrong guesses
        // 560439848176070 - too high
        let ranges_str = input.split("\n\n")
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid input format"))?;

        let mut fresh_ranges: Vec<(u64, u64)> = ranges_str
            .lines()
            .filter(|line| !line.is_empty())
            .map(Self::parse_range)
            .collect::<Result<_>>()?;

        fresh_ranges.sort_by_key(|r| r.0);

        let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
        // Loop through ranges
        // if range low is less than current range max, merge into current max range
        //      if high > current max high => current max high = high
        // otherwise becomes next max range

        for (start, end) in fresh_ranges {
            if let Some(last) = merged_ranges.last_mut() {
                if start <= last.1 + 1 {
                    last.1 = last.1.max(end);
                } else {
                    merged_ranges.push((start, end));
                }
            } else {
                merged_ranges.push((start, end));
            }
        }

        let total: u64 = merged_ranges.iter()
            .map(|(low, high)| high - low + 1)
            .sum();

        Ok(format!("{}", total))
    }
}
