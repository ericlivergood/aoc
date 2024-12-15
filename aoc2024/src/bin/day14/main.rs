use aoc2024::common::input_reader;
use crate::bathroom_map::BathroomMap;

mod bathroom_map;
mod robot;

fn part1(lines: &Vec<String>, width: i32, height: i32) -> i32 {
    let mut m = BathroomMap::from_lines(lines, width, height);
    m.advance(100);
    m.get_safety_factor()
}

fn part2(lines: &Vec<String>, width: i32, height: i32) -> i32 {
    let mut m = BathroomMap::from_lines(lines, width, height);
    let n = m.get_min_distance_time(10000);
    println!("** Part 2 - {}", n);
    let mut m = BathroomMap::from_lines(lines, width, height);
    m.advance(n);
    m.print_map();
    0
}

fn main() {
    let lines = input_reader::input_reader::get_lines("./data/day14/input");
    //println!("part 1: {}", part1(&lines, 101, 103));
    println!("part 2: {}", part2(&lines, 101, 103));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1(){
        let lines = input_reader::input_reader::get_lines("./data/day14/test");
        assert_eq!(part1(&lines, 11, 7), 12);
    }

    #[test]
    fn test_part2(){
        let lines = input_reader::input_reader::get_lines("./data/day14/input");
        assert_eq!(part2(&lines, 101, 103), 0);
    }
}