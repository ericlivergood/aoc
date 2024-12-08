use std::collections::HashMap;
use crate::common::input_reader;

mod tests;
pub struct Day;

struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>
}

impl Card {
    pub fn new(line: String) -> Self {
        let label_parts = line.split(':').collect::<Vec<&str>>();
        let number = label_parts[0].replace("Card", "").trim().parse::<u32>().unwrap();
        let card_parts = label_parts[1].split('|').collect::<Vec<&str>>();

        let mut winning_numbers = Vec::new();
        for p in card_parts[0].split(' ') {
            if !p.is_empty() {
                winning_numbers.push(p.trim().parse().unwrap());
            }
        }

        let mut numbers = Vec::new();
        for p in card_parts[1].split(' ') {
            if !p.is_empty() {
                numbers.push(p.trim().parse().unwrap());
            }
        }

        Self {
            number,
            winning_numbers,
            numbers
        }
    }

    pub fn is_winning_number(&self, n: u32) -> bool {
        self.winning_numbers.contains(&n)
    }

    pub fn point_value(&self) -> u32 {
        let match_count = self.match_count();
        if match_count == 0 {
            return 0;
        }
        (2u32).pow(match_count - 1)
    }

    pub fn match_count(&self) -> u32 {
        let mut match_count = 0;
        for n in &self.numbers {
            if self.is_winning_number(*n) {
                match_count += 1
            }
        }
        match_count
    }
}

impl Day {
    pub fn run(&self) {
        self.run_part_two();
    }

    pub fn run_part_one(&self) {
        let reader = input_reader::InputReader;
        let mut sum = 0;
        for line in reader.get_lines("/git/aoc23/src/days/day4/input") {
            let card = Card::new(line);
            println!("card {0} worth {1}", card.number, card.point_value());
            sum += card.point_value();
        }
        println!("{sum}");
    }

    pub fn run_part_two(&self) {
        let reader = input_reader::InputReader;
        let mut cnt = 0;
        let mut cards = HashMap::new();
        for line in reader.get_lines("/git/aoc23/src/days/day4/input") {
            let card = Card::new(line);
            cards.insert(card.number, card);
        }

        let mut q = Vec::new();

        for k in cards.keys() {
            //println!("adding {k}");
            q.push(*k);
        }

        while !q.is_empty() {
            let n = q.pop().unwrap();
            if !cards.contains_key(&n) {
               continue;
            }
            let cur = &cards[&n];
            cnt += 1;
            for i in 1..cur.match_count()+1 {
                let next = &cur.number + i;
                //println!("adding {n} -> {next}");
                q.push(next);
            }
        }
        println!("{cnt}");
    }
}