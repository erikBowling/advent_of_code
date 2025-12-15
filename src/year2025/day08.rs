use std::collections::HashSet;

use crate::solution::Solution;
use anyhow::Result;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl JunctionBox {
    fn distance(&self, other: JunctionBox) -> u64 {
        let delta_x = self.x as i64 - other.x as i64;
        let delta_y = self.y as i64 - other.y as i64;
        let delta_z = self.z as i64 - other.z as i64;
        let radicand = (delta_x).pow(2) + (delta_y).pow(2) + (delta_z).pow(2);
        return radicand.isqrt() as u64;
    }
}

pub struct Day08;

impl Day08 {}

impl Solution for Day08 {
    fn part1(&self, input: &str) -> Result<String> {
        let points: Vec<JunctionBox> = input
            .lines()
            .map(|line| {
                let num_strs: Vec<&str> = line.trim().split(',').collect();
                let mut nums: Vec<u64> = Vec::new();
                for num in num_strs {
                    nums.push(num.parse().expect("Failed to parse"));
                }

                JunctionBox {
                    x: nums[0],
                    y: nums[1],
                    z: nums[2],
                }
            })
            .collect();

        let num_points: usize = points.len();

        let mut distance_matrix: Vec<(JunctionBox, JunctionBox, u64)> = Vec::new();

        for i in 0..num_points - 1 {
            for j in i + 1..=num_points - 1 {
                let point_one = &points[i];
                let point_two = &points[j];
                let distance = point_one.distance(*point_two);
                distance_matrix.push((*point_one, *point_two, distance))
            }
        }
        distance_matrix.sort_by_key(|(_, _, distance)| *distance);

        let mut circuits: Vec<HashSet<JunctionBox>> = Vec::new();
        let mut distance_mat_iter = distance_matrix.iter();
        let mut count: usize = 0;

        while let Some(point_pair) = distance_mat_iter.next() {
            if count == 1000 {
                break;
            }

            let (point1, point2, _) = point_pair;

            let set_p1_index: Option<usize> = circuits.iter().position(|set| set.contains(&point1));
            let set_p2_index: Option<usize> = circuits.iter().position(|set| set.contains(&point2));

            match (set_p1_index, set_p2_index) {
                (Some(s1), Some(s2)) => {
                    if s1 != s2 {
                        let rem_set = circuits.remove(s2.max(s1));
                        circuits[s2.min(s1)].extend(rem_set);
                    }
                }
                (Some(s1), None) => {
                    circuits[s1].insert(*point2);
                }
                (None, Some(s2)) => {
                    circuits[s2].insert(*point1);
                }
                (None, None) => {
                    let mut new_set = HashSet::new();
                    new_set.insert(*point1);
                    new_set.insert(*point2);
                    circuits.push(new_set);
                }
            }

            count += 1;
        }

        circuits.sort_by(|a, b| a.len().cmp(&b.len()));
        circuits.reverse();
        let mut product: u64 = 1;

        product *= circuits[0].len() as u64;
        product *= circuits[1].len() as u64;
        product *= circuits[2].len() as u64;

        Ok(format!("{}", product))
    }

    fn part2(&self, input: &str) -> Result<String> {
        // Wrong Guesses
        // 2192978738 - Too low
        let points: Vec<JunctionBox> = input
            .lines()
            .map(|line| {
                let num_strs: Vec<&str> = line.trim().split(',').collect();
                let mut nums: Vec<u64> = Vec::new();
                for num in num_strs {
                    nums.push(num.parse().expect("Failed to parse"));
                }

                JunctionBox {
                    x: nums[0],
                    y: nums[1],
                    z: nums[2],
                }
            })
            .collect();

        let num_points: usize = points.len();

        let mut distance_matrix: Vec<(JunctionBox, JunctionBox, u64)> = Vec::new();

        for i in 0..num_points - 1 {
            for j in i + 1..=num_points - 1 {
                let point_one = &points[i];
                let point_two = &points[j];
                let distance = point_one.distance(*point_two);
                distance_matrix.push((*point_one, *point_two, distance))
            }
        }
        distance_matrix.sort_by_key(|(_, _, distance)| *distance);

        let mut circuits: Vec<HashSet<JunctionBox>> = Vec::new();
        let mut distance_mat_iter = distance_matrix.iter();

        let mut return_product: u64 = 1;

        while let Some(point_pair) = distance_mat_iter.next() {
            let (point1, point2, _) = point_pair;

            let set_p1_index: Option<usize> = circuits.iter().position(|set| set.contains(&point1));
            let set_p2_index: Option<usize> = circuits.iter().position(|set| set.contains(&point2));

            match (set_p1_index, set_p2_index) {
                (Some(s1), Some(s2)) => {
                    if s1 != s2 {
                        let rem_set = circuits.remove(s2.max(s1));
                        circuits[s2.min(s1)].extend(rem_set);
                    }
                }
                (Some(s1), None) => {
                    circuits[s1].insert(*point2);
                }
                (None, Some(s2)) => {
                    circuits[s2].insert(*point1);
                }
                (None, None) => {
                    let mut new_set = HashSet::new();
                    new_set.insert(*point1);
                    new_set.insert(*point2);
                    circuits.push(new_set);
                }
            }

            if circuits.len() == 1 && circuits[0].len() == num_points {
                return_product *= point1.x * point2.x;
                break;
            }
        }

        Ok(format!("{}", return_product))
    }
}
