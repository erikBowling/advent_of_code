pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
// pub mod day05;
// pub mod day06;
// pub mod day07;
// pub mod day08;
// pub mod day09;
// pub mod day10;
// pub mod day11;
// pub mod day12;

use crate::solution::Solution;
use std::collections::HashMap;

pub fn get_solutions() -> HashMap<u32, Box<dyn Solution>> {
    let mut map = HashMap::new();
    map.insert(1, Box::new(day01::Day01) as Box<dyn Solution>);
    map.insert(2, Box::new(day02::Day02) as Box<dyn Solution>);
    map.insert(3, Box::new(day03::Day03) as Box<dyn Solution>);
    map.insert(4, Box::new(day04::Day04) as Box<dyn Solution>);
    // map.insert(5, Box::new(day05::Day05) as Box<dyn Solution>);
    // map.insert(6, Box::new(day06::Day06) as Box<dyn Solution>);
    // map.insert(7, Box::new(day07::Day07) as Box<dyn Solution>);
    // map.insert(8, Box::new(day08::Day08) as Box<dyn Solution>);
    // map.insert(9, Box::new(day09::Day09) as Box<dyn Solution>);
    // map.insert(10, Box::new(day10::Day10) as Box<dyn Solution>);
    // map.insert(11, Box::new(day11::Day11) as Box<dyn Solution>);
    // map.insert(12, Box::new(day12::Day12) as Box<dyn Solution>);

    map
}
