use crate::common::input_reader;
use crate::days::day14::platform::Platform;

mod tests;
mod platform;

pub struct Day;

impl Day {
    pub fn run(&self) {
        self.run_part_two();
    }

    pub fn run_part_two(&self) {
        let reader = input_reader::InputReader;
        let lines = reader.get_lines("/git/aoc23/src/days/day14/input");
        let mut p = Platform::new(lines);
        for n in 0..1_000_000_000 {
            p.tilt();
            p.rotate();
            p.tilt();
            p.rotate();
            p.tilt();
            p.rotate();
            p.tilt();
            p.rotate();
            if n % 1_000 == 0 {
                println!("{n}, {0}", p.calculate_load());
            }
        }

        println!("answer: {0}", p.calculate_load());
    }

    pub fn run_part_one(&self) {
        let reader = input_reader::InputReader;
        let lines = reader.get_lines("/git/aoc23/src/days/day14/input");
        let mut p = Platform::new(lines);
        println!("{p}");
        p.tilt();
        println!("{p}");
        println!("answer: {0}", p.calculate_load());
    }
}