use crate::common::input_reader;
pub struct Day1;

struct NumberRepresentation {
    string_value: &'static str,
    value: u32
}

const NUMBERS: [NumberRepresentation; 19] = [
    NumberRepresentation { string_value: "0", value: 0},
    NumberRepresentation { string_value: "1", value: 1},
    NumberRepresentation { string_value: "2", value: 2},
    NumberRepresentation { string_value: "3", value: 3},
    NumberRepresentation { string_value: "4", value: 4},
    NumberRepresentation { string_value: "5", value: 5},
    NumberRepresentation { string_value: "6", value: 6},
    NumberRepresentation { string_value: "7", value: 7},
    NumberRepresentation { string_value: "8", value: 8},
    NumberRepresentation { string_value: "9", value: 9},
    NumberRepresentation { string_value: "one", value: 1},
    NumberRepresentation { string_value: "two", value: 2},
    NumberRepresentation { string_value: "three", value: 3},
    NumberRepresentation { string_value: "four", value: 4},
    NumberRepresentation { string_value: "five", value: 5},
    NumberRepresentation { string_value: "six", value: 6},
    NumberRepresentation { string_value: "seven", value: 7},
    NumberRepresentation { string_value: "eight", value: 8},
    NumberRepresentation { string_value: "nine", value: 9},
];

fn combine_digits(first: u32, last: u32) -> u32 {
    10*first + last
}

fn get_first_digit(line: &str) -> u32 {
    let len = line.len();
    for i in 0..len {
        let sub = &line[i..len];
        for n in NUMBERS {
            if sub.starts_with(n.string_value) {
                return n.value
            }
        }
    }
    panic!("could not find a digit in {line}");
}

fn get_last_digit(line: &str) -> u32 {
    let len = line.len();
    for i in (0..len).rev() {
        let sub = &line[i..len];
        for n in NUMBERS {
            if sub.starts_with(n.string_value) {
                return n.value
            }
        }
    }
    panic!("could not find a digit in {line}");
}

impl Day1 {
    pub fn run(&self) {
        let reader = input_reader::InputReader;
        let lines = reader.get_lines("/git/aoc23/src/days/day1/input");
        let mut sum: u32 = 0;
        for l in lines {
            let first = get_first_digit(l.as_str());
            let last = get_last_digit(l.as_str());
            sum += combine_digits(first, last);
        }
        println!("{sum}");
    }
}