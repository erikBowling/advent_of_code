pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
// pub mod day07;
// pub mod day08;
// pub mod day09;
// pub mod day10;
// pub mod day11;
// pub mod day12;

use std::collections::HashMap;
use crate::solution::Solution;

pub fn get_solutions() -> HashMap<u32, Box<dyn Solution>> {
    let mut map = HashMap::new();
    map.insert(1, Box::new(day01::Day01) as Box<dyn Solution>);
    map.insert(2, Box::new(day02::Day02) as Box<dyn Solution>);
    map.insert(3, Box::new(day03::Day03) as Box<dyn Solution>);
    map.insert(4, Box::new(day04::Day04) as Box<dyn Solution>);
    map.insert(5, Box::new(day05::Day05) as Box<dyn Solution>);
    map.insert(6, Box::new(day06::Day06) as Box<dyn Solution>);

    map
}
