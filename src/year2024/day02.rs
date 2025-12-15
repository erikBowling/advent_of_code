use crate::solution::Solution;
use anyhow::Result;
use std::ops::RangeInclusive;

pub struct Day02;

impl Day02 {
    fn check_is_ascending(&self, list: &Vec<i32>) -> bool {
        let mut sorted_list: Vec<i32> = list.clone();
        sorted_list.sort();

        return sorted_list == *list;
    }

    fn check_is_descending(&self, list: &Vec<i32>) -> bool {
        let mut reversed_list: Vec<i32> = list.clone();
        reversed_list.sort();
        reversed_list.reverse();

        return reversed_list == *list;
    }

    fn check_element_safe_distances(
        &self,
        list: &Vec<i32>,
        acceptable_range: &RangeInclusive<i32>,
    ) -> bool {
        if list.len() < 2 {
            return true;
        }

        for window in list.windows(2) {
            let difference: i32 = (window[0] - window[1]).abs();
            if !acceptable_range.contains(&difference) {
                return false;
            }
        }

        return true;
    }

    fn dampener(&self, list: &Vec<i32>, acceptable_range: &RangeInclusive<i32>) -> bool {
        for (index, _) in list.iter().enumerate() {
            let mut list_without_item: Vec<i32> = list.clone();
            list_without_item.remove(index);

            if (self.check_is_ascending(&list_without_item)
                ^ self.check_is_descending(&list_without_item))
                && self.check_element_safe_distances(&list_without_item, acceptable_range)
            {
                return true;
            }
        }

        return false;
    }
}

impl Solution for Day02 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut report_matrix: Vec<Vec<i32>> = Vec::new();

        for line in input.lines() {
            let line: String = line.to_string();
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect();

            report_matrix.push(numbers);
        }

        let acceptable_range: RangeInclusive<i32> = 1..=3;

        let num: i32 = report_matrix
            .iter()
            .filter(|x| self.check_element_safe_distances(x, &acceptable_range))
            .filter(|x| self.check_is_ascending(x) ^ self.check_is_descending(x))
            .count() as i32;

        Ok(format!("{}", num))
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut report_matrix: Vec<Vec<i32>> = Vec::new();

        for line in input.lines() {
            let line: String = line.to_string();
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect();

            report_matrix.push(numbers);
        }

        let acceptable_range: RangeInclusive<i32> = 1..=3;

        let unsafe_reports: Vec<Vec<i32>> = report_matrix
            .iter()
            .filter(|x| {
                !self.check_element_safe_distances(x, &acceptable_range)
                    || !(self.check_is_ascending(x) ^ self.check_is_descending(x))
            })
            .cloned()
            .collect();

        let mut safe_reports: Vec<Vec<i32>> = report_matrix
            .iter()
            .filter(|x| self.check_is_ascending(x) ^ self.check_is_descending(x))
            .filter(|x| self.check_element_safe_distances(x, &acceptable_range))
            .cloned()
            .collect();

        let mut dampened_safe_reports: Vec<Vec<i32>> = unsafe_reports
            .iter()
            .filter(|x| self.dampener(x, &acceptable_range))
            .cloned()
            .collect();

        safe_reports.append(&mut dampened_safe_reports);

        let result = safe_reports.iter().count() as i32;

        Ok(format!("{}", result))
    }
}
