use crate::common::input_reader;

mod tests;
pub struct Day;

struct Sequence {
    values: Vec<i32>
}

impl Sequence {
    pub fn from(s: String) -> Sequence {
        let mut values = Vec::new();
        for v in s.trim().split(' ') {
            let i = v.parse().unwrap();
            values.push(i);
        }
        Sequence {
            values
        }
    }

    pub fn derive(&self) -> Sequence {
        let mut values = Vec::new();
        for i in 0..self.values.len() - 1 {
            values.push(self.values[i+1] - self.values[i]);
        }
        // /println!("{:?}", values);
        Sequence {
            values
        }
    }

    pub fn extrapolate(&self) -> i32 {
        if self.is_zeros() {
            return 0;
        }

        let next = self.derive();
        let delta = next.extrapolate();
        let v = self.values.last().unwrap();
        v + delta
    }

    pub fn extrapolate_reverse(&self) -> i32 {
        if self.is_zeros() {
            return 0;
        }

        let next = self.derive();
        let delta = next.extrapolate_reverse();
        let v = self.values.first().unwrap();
        //println!("delta: {delta} | v: {v} | ex: {0}", v+delta);

        v - delta
    }

    pub fn is_zeros(&self) -> bool {
        for v in &self.values {
            if v != &0 {
                return false
            }
        }
        true
    }
}

impl Day {
    pub fn run(&self) {
        self.run_part_two();
    }
    pub fn run_part_one(&self) {
        let reader = input_reader::InputReader;
        let mut sum = 0;
        for line in reader.get_lines("/git/aoc23/src/days/day9/input") {
            let l = line.as_str();
            let s = Sequence::from(l.to_string());
            let val = s.extrapolate();
            println!("line: {line}");
            println!("-> {val}");
            sum += val;
        }
        println!("{sum}");
    }

    pub fn run_part_two(&self) {
        let reader = input_reader::InputReader;
        let mut sum = 0;
        for line in reader.get_lines("/git/aoc23/src/days/day9/input") {
            let l = line.as_str();
            let s = Sequence::from(l.to_string());
            let val = s.extrapolate_reverse();
            println!("line: {line}");
            println!("-> {val}");
            sum += val;
        }
        println!("{sum}");
    }
}