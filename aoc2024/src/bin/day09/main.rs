mod disk_map;

use aoc2024::common::input_reader;
use crate::disk_map::DiskMap;

fn part1(lines: &Vec<String>) -> i64 {
    let line = lines.get(0).unwrap();
    let mut dm = DiskMap::from_string(line);
    dm.defrag();
    dm.checksum()
}

fn part2(lines: &Vec<String>) -> i64 {
    let line = lines.get(0).unwrap();
    let mut dm = DiskMap::from_string(line);
    dm.compact();
    dm.checksum()
}

fn main() {
    let lines = input_reader::input_reader::get_lines("./data/day09/input");
    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1(){
        let lines = input_reader::input_reader::get_lines("./data/day09/test");
        assert_eq!(part1(&lines), 1928);
    }

    #[test]
    fn test_part2(){
        let lines = input_reader::input_reader::get_lines("./data/day09/test");
        assert_eq!(part2(&lines), 2858);
    }
}