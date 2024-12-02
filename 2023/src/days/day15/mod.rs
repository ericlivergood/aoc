use crate::common::input_reader;
use crate::days::day15::hasher::{hash_input, XHashMap};

mod tests;
mod hasher;
pub struct Day;

impl Day {
    pub fn run(&self) {
        self.run_part_two();
    }
    pub fn run_part_two(&self) {
        let reader = input_reader::InputReader;
        let lines = reader.get_lines("/git/aoc23/src/days/day15/input");
        let l = lines[0].to_string();
        let mut m = XHashMap::new();
        for p in l.split(',') {
            //println!("After {p}:");
            m.update(p.to_string());
        }

        println!("{0}", m.calculate_focusing_power());
    }
    pub fn run_part_one(&self) {
        let reader = input_reader::InputReader;
        let lines = reader.get_lines("/git/aoc23/src/days/day15/input");
        let data = lines[0].to_string();
        println!("{0}", hash_input(data));
    }
}