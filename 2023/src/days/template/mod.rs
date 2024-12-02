use crate::common::input_reader;

mod tests;
pub struct Day;

impl Day {
    pub fn run(&self) {
        let reader = input_reader::InputReader;
        for line in reader.get_lines("/git/aoc23/src/days/day2/test") {}
    }
}