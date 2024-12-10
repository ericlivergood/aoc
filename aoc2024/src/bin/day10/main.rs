use aoc2024::common::input_reader;
use crate::topographical_map::TopographicalMap;

mod topographical_map;
mod map_point;
mod route;

fn part1(lines: &Vec<String>) -> i32 {
    let map = TopographicalMap::from_lines(lines);
    map.get_route_scores()
}

fn part2(lines: &Vec<String>) -> i32 {
    let map = TopographicalMap::from_lines(lines);
    map.get_ratings()
}

fn main() {
    let lines = input_reader::input_reader::get_lines("./data/day10/input");
    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1(){
        let lines = input_reader::input_reader::get_lines("./data/day10/test2");
        assert_eq!(part1(&lines), 2);
        let lines = input_reader::input_reader::get_lines("./data/day10/test3");
        assert_eq!(part1(&lines), 4);
        let lines = input_reader::input_reader::get_lines("./data/day10/test4");
        assert_eq!(part1(&lines), 3);
        let lines = input_reader::input_reader::get_lines("./data/day10/test");
        assert_eq!(part1(&lines), 36);
    }

    #[test]
    fn test_part2(){
        let lines = input_reader::input_reader::get_lines("./data/day10/test5");
        assert_eq!(part2(&lines), 3);
        let lines = input_reader::input_reader::get_lines("./data/day10/test6");
        assert_eq!(part2(&lines), 13);
        let lines = input_reader::input_reader::get_lines("./data/day10/test7");
        assert_eq!(part2(&lines), 227);
        let lines = input_reader::input_reader::get_lines("./data/day10/test");
        assert_eq!(part2(&lines), 81);
    }
}