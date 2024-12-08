mod map;
mod guard;
mod enums;
mod map_builder;

use aoc2024::common::input_reader;
use aoc2024::common::point::Point;
use crate::map_builder::from_lines;

fn part1(lines: &Vec<String>) {
    let mut guard = from_lines(lines);
    guard.patrol();
    println!("part1: {}", guard.path.keys().len());
}

fn part2(lines: &Vec<String>) {
    let mut cnt = 0;
    let mut guard = from_lines(lines);

    for y in 1..150 {
        for x in 1..150 {
            let o = Point::new(x, y);
            println!("{}", o);
            if(guard.add_obstacle(&o)) {
                if(!guard.patrol()) {
                    cnt += 1;
                }
                guard.remove_obstacle(&o);
                guard.reset();
            }
        }
    }

    println!("part2: {}", cnt);
}

fn main() {
    let lines = input_reader::input_reader::get_lines("./data/day06/input");
    part1(&lines);
    part2(&lines);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test(){

    }
}