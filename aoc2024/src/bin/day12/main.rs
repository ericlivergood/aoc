mod garden;

use aoc2024::common::input_reader;
use crate::garden::Garden;

fn part1(lines: &Vec<String>) -> i64 {
    let g = Garden::from_lines(lines);
    let mut sum = 0;
    for p in g.get_subplots() {
        sum += Garden::perimeter_of(&p);
    }
    sum
}

fn part2(lines: &Vec<String>) -> i64 {
0
}

fn main() {
    let lines = input_reader::input_reader::get_lines("./data/day12/input");
    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1(){
        let lines = input_reader::input_reader::get_lines("./data/day12/test");
        assert_eq!(part1(&lines), 140);
        let lines = input_reader::input_reader::get_lines("./data/day12/test2");
        assert_eq!(part1(&lines), 772);
        let lines = input_reader::input_reader::get_lines("./data/day12/test3");
        assert_eq!(part1(&lines), 1930);
    }

    #[test]
    fn test_part2(){
        let lines = input_reader::input_reader::get_lines("./data/day12/test");
        part2(&lines);
        //assert_eq!(part2(&lines), 0);
    }
}