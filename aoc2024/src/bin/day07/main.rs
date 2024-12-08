mod equation;

use aoc2024::common::input_reader;
use crate::equation::Equation;

fn part1(lines: &Vec<String>) -> i64 {
    let mut sum = 0;
    for line in lines {
        let e = Equation::from_line(line);
        if e.is_valid(false) {
            sum += e.result;
        }
    }
    sum
}

fn part2(lines: &Vec<String>) -> i64 {
    let mut sum = 0;
    for line in lines {
        let e = Equation::from_line(line);
        if e.is_valid(true) {
            sum += e.result;
        }
    }
    sum
}

fn main() {
    let lines = input_reader::input_reader::get_lines("./data/day07/input");
    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1(){
        let lines = input_reader::input_reader::get_lines("./data/day07/test");
        assert_eq!(part1(&lines), 3749);
    }

    #[test]
    fn test_part2(){
        let lines = input_reader::input_reader::get_lines("./data/day07/test");
        assert_eq!(part2(&lines), 11387);
    }
}