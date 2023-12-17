use crate::common::input_reader;
use crate::days::day16::contraption::Contraption;

mod tests;
mod beam;
mod contraption;

pub struct Day;

impl Day {
    pub fn run(&self) {
        let reader = input_reader::InputReader;
        let lines = reader.get_lines("/git/aoc23/src/days/day16/input");
        let mut c = Contraption::new(lines);
        c.run();
    }
}