use crate::common::input_reader;
use crate::days::day13::pattern::Pattern;

mod tests;
mod pattern;

pub struct Day;

impl Day {
    pub fn run(&self) {
        let reader = input_reader::InputReader;
        let mut pattern = Vec::new();
        let mut sum = 0;
        let fix_smudge = true;
        for line in reader.get_lines("/git/aoc23/src/days/day13/input") {
            if line.trim().is_empty() {
                let mut p = Pattern::new(pattern);
                let n = p.summarize(fix_smudge);
                //println!("summary: {n}");
                sum += n;
                pattern = Vec::new();
            }
            else {
                pattern.push(line.clone());
            }
        }

        if pattern.len() > 0 {
            let mut p = Pattern::new(pattern);
            let n = p.summarize(fix_smudge);

            sum += n;
            //pattern = Vec::new();
        }

        println!("{sum}");
    }
}