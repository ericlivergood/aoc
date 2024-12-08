use aoc2024::common::input_reader;
use aoc2024::common::utils;
use regex::Regex;

fn extract(data: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();

    re.captures_iter(data)
        .map(|cap| cap.get(0).unwrap().as_str())
        .collect()
}

fn extract_with_dos(data: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();

    re.captures_iter(data)
        .map(|cap| cap.get(0).unwrap().as_str())
        .collect()
}

fn calculate(instruction: &str) -> i32 {
    let binding = instruction.replace("mul(", "").replace(")", "");
    let pair = binding.as_str();
    let numbers = utils::utils::list_to_ints_by_char(pair, ",");
    numbers[0] * numbers[1]
}

fn part1(data: &str) {
    let mut sum = 0;

    for i in extract(data) {
        sum += calculate(i);
    }

    println!("Part 1: {}", sum);
}

fn part2(data: &str) {
    let mut sum = 0;
    let mut is_enabled = true;
    for i in extract_with_dos(data) {
        match i {
            "do()" => is_enabled = true,
            "don't()" => is_enabled = false,
            _ => {
                if is_enabled {
                    sum += calculate(i);
                }

            }
        }
    }

    println!("Part 2: {}", sum);
}

fn main() {
    let lines = input_reader::input_reader::get_lines("./data/day03/input");
    let binding = lines.clone().join("");
    let data = binding.as_str();
    part1(data);
    part2(data);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extracts_mul_instructions(){
        assert_eq!("mul(1,1)", extract("mul(1,1)")[0]);
        assert_eq!("mul(1,1)", extract("asdfmul(1,1)")[0]);
        assert_eq!("mul(1,1)", extract("mul(1,1)asdf")[0]);
        assert_eq!("mul(1,1)", extract("asdfasdfmul(1,1)asdfmul(2,2)asdfasdf")[0]);
        assert_eq!("mul(2,2)", extract("asdfasdfmul(1,1)asdfmul(2,2)asdfasdf")[1]);
    }

    #[test]
    fn extracts_with_dos_and_donts() {
        assert_eq!("mul(1,1)", extract_with_dos("mul(1,1)")[0]);
        assert_eq!("do()", extract_with_dos("do()")[0]);
        assert_eq!("don't()", extract_with_dos("don't()")[0]);
        assert_eq!("do()", extract_with_dos("do()mul(2,2)don't()")[0]);
        assert_eq!("mul(2,2)", extract_with_dos("do()mul(2,2)don't()")[1]);
        assert_eq!("don't()", extract_with_dos("do()mul(2,2)don't()")[2]);
    }


    #[test]
    fn calculates_instruction_value() {
        assert_eq!(8, calculate("mul(2,4)"));
        assert_eq!(25, calculate("mul(5,5)"));
        assert_eq!(88, calculate("mul(11,8)"));
        assert_eq!(40, calculate("mul(8,5)"));
    }
}