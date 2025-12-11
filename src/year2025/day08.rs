use std::collections::{HashMap, HashSet};

use crate::solution::Solution;
use anyhow::Result;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Day08JunctionBox {
    id: usize,
    x: i64,
    y: i64,
    z: i64
}

impl Day08JunctionBox {
    fn new(&self, id: usize, x: i64, y: i64, z: i64){
        Self {id, x, y, z};
    }

    fn distance(&self, other: Day08JunctionBox) -> i64{
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        return (dx.pow(2) + dy.pow(2) + dz.pow(2)).isqrt();
    }
}

pub struct Day08;

impl Day08 {

}

impl Solution for Day08 {
    fn part1(&self, input: &str) -> Result<String> {
        let junction_box_nums: Vec<Vec<i64>> = input.lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().split(',').collect())
            .map(|list_num_str: Vec<&str>| {
                let mut junction_box: Vec<i64> = Vec::new();
                for num in list_num_str{
                    if let Ok(n) = num.parse::<i64>() {
                        junction_box.push(n);
                    }
                }

                junction_box
            })
            .collect();

        let junction_boxes: Vec<Day08JunctionBox> = junction_box_nums.iter()
            .enumerate()
            .map(|(i, jb)| {
                Day08JunctionBox { id: i, x: jb[0], y: jb[1], z: jb[2] }
            })
            .collect();

        let mut distance_matrix: HashMap<(Day08JunctionBox, Day08JunctionBox), i64> = HashMap::new();

        for i in 0..junction_boxes.len() - 2 {
            for j in i+1..junction_boxes.len() - 1 {
                let distance = junction_boxes[i].distance(junction_boxes[j]);
                distance_matrix.insert((junction_boxes[i], junction_boxes[j]), distance);
            }
        }

        let mut sorted_distances: Vec<(&(Day08JunctionBox, Day08JunctionBox), &i64)> = distance_matrix.iter().collect();
        sorted_distances.sort_by_key(|el| el.1);



        let circuits: Vec<HashSet<Day08JunctionBox>> = Vec::new();

        Ok(format!("{}", ""))
    }

    fn part2(&self, input: &str) -> Result<String> {
        // Wrong Guesses

        Ok(format!("{}", "total"))
    }
}
