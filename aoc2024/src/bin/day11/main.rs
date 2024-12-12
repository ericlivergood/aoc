use aoc2024::common::input_reader;
use crate::stone_line::StoneLine;
mod stone_line;

fn part1(lines: &Vec<String>) -> i64 {
    let mut stones = StoneLine::from_line(lines[0].clone());
    //stones.blink(25);
    //stones.stones.len() as i32
    stones.get_blink_counts(25)
}

fn part2(lines: &Vec<String>) -> i64 {
    let mut stones = StoneLine::from_line(lines[0].clone());
    stones.get_blink_counts(75)
}

fn main() {
    let lines = input_reader::input_reader::get_lines("./data/day11/input");
    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1(){
        let lines = input_reader::input_reader::get_lines("./data/day11/test");
        assert_eq!(part1(&lines), 55312);
    }

    #[test]
    fn test_part2(){
        let lines = input_reader::input_reader::get_lines("./data/day11/test");
        part2(&lines);
        //assert_eq!(part2(&lines), 0);
    }
}