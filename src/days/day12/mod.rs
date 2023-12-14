use crate::common::input_reader;
use crate::days::day12::record::{generate_permutations, Record};

mod tests;
mod record;
pub struct Day;

impl Day {
    pub fn run(&self) {
        self.run_part_one();
    }
    pub fn run_part_one(&self) {
        let reader = input_reader::InputReader;
        let mut sum = 0;
        let mut n = 1;
        for line in reader.get_lines("/git/aoc23/src/days/day12/test") {
            let r = Record::new(line);
            //let solutions = r.count_solutions_by_force();
            let solutions = r.count_solutions();
            sum += solutions;
            println!("{n}: {0} -> {solutions}", r.conditions);
            n += 1;
        }

        println!("answer: {sum}");
    }

    pub fn run_part_two(&self) {
        let reader = input_reader::InputReader;
        let mut sum = 0;
        for line in reader.get_lines("/git/aoc23/src/days/day12/test") {
            let r = Record::unfolded(line);
            let solutions = r.count_solutions();
            println!("{0} {1} -> {2}", r.conditions, r.condition_numbers_str, solutions);
            sum += solutions;
            println!("{solutions}");
            println!();
        }
        println!("answer: {sum}");
    }
}