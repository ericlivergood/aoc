use std::collections::{HashMap, HashSet};
use aoc2024::common::utils::utils::{list_to_i64_by_char};

pub struct StoneLine {
    pub stones: Vec<i64>,
    mappings: HashMap<i64, Vec<i64>>
}

fn split_number(n: i64) -> Vec<i64> {
    let n_str = n.to_string();
    let midpoint = n_str.len() / 2;

    let n1 = &n_str[0..midpoint].parse::<i64>().unwrap();
    let n2 = &n_str[midpoint..].parse::<i64>().unwrap();

    vec![
        *n1,
        *n2
    ]
}

impl StoneLine {
    pub fn new(stones: Vec<i64>) -> Self {
        Self {
            stones,
            mappings: HashMap::new()
        }
    }
    pub fn from_line(line: String) -> Self {
        let stones = list_to_i64_by_char(line.as_str(), " ");
        StoneLine::new(stones)
    }

    fn evolve_stone(&mut self, n: i64, times: i32) -> Vec<i64>{
        if self.mappings.contains_key(&n) {
            return self.mappings[&n].clone();
        }
        if n == 0 {
            return vec![1]
        }
        let num_digits = n.to_string().len();
        if num_digits % 2 == 0 {
            return split_number(n);
        }

        let result = vec![n * 2024];
        self.mappings.insert(n, result.clone());
        result
    }

    fn blink_once(&mut self){
        let stones = self.stones.clone();
        let mut new_stones = Vec::new();
        for s in stones {
            let next = self.evolve_stone(s, 1);
            new_stones.extend(next);
        }
        self.stones = new_stones;
    }

    pub fn blink(&mut self, times: i64) {
        for i in 0..times {
            let distinct: HashSet<_> = self.stones.clone().into_iter().collect();
            println!("{}: {}, {}", i, self.stones.len(), distinct.len());
            self.blink_once();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn splits_numbers() {
        assert_eq!(split_number(25), vec![2,5]);
        assert_eq!(split_number(2050), vec![20,50]);
        assert_eq!(split_number(2005), vec![20,5]);
        assert_eq!(split_number(200125), vec![200,125]);
        assert_eq!(split_number(212005), vec![212,5]);
    }

    #[test]
    fn evolves_stone() {
        let mut l = StoneLine::from_line(String::from("0"));
        assert_eq!(l.evolve_stone(0), vec![1]);
        assert_eq!(l.evolve_stone(1), vec![2024]);
        assert_eq!(l.evolve_stone(5), vec![2024*5]);
        assert_eq!(l.evolve_stone(123), vec![2024*123]);
        assert_eq!(l.evolve_stone(20), vec![2, 0]);
        assert_eq!(l.evolve_stone(2050), vec![20, 50]);
    }

    #[test]
    fn blink() {
        let mut stones = StoneLine::from_line("125 17".to_string());
        assert_eq!(stones.stones, vec![125, 17]);
        stones.blink_once();
        assert_eq!(stones.stones, vec![253000, 1, 7]);
        stones.blink_once();
        assert_eq!(stones.stones, vec![253, 0, 2024, 14168]);
        stones.blink_once();
        assert_eq!(stones.stones, vec![512072, 1, 20, 24, 28676032]);
        stones.blink_once();
        assert_eq!(stones.stones, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);
        stones.blink_once();
        assert_eq!(stones.stones, vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]);
        stones.blink_once();
        assert_eq!(stones.stones, vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2]);
    }
}