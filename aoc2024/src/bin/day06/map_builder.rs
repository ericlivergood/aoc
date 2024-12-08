use aoc2024::common::point::Point;
use crate::enums::Direction;
use crate::guard::Guard;
use crate::map::Map;

pub fn from_lines(lines: &Vec<String>) -> Guard {
    let mut obstacles = Vec::new();
    let mut guard_point: Point = Point { x: 0, y: 0 };
    let mut guard_set = false;

    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            let line = &lines[y];
            match line.chars().nth(x).unwrap() {
                '.' => { },
                '#' => {
                    obstacles.push(Point::new((x + 1) as i32, (y + 1) as i32));
                },
                '^' => {
                    guard_point = Point::new((x + 1) as i32, (y + 1) as i32);
                    guard_set = true;
                },
                other => {
                    panic!("Unknown char {}", other)
                }
            }
        }
    }
    if !guard_set {
        panic!("No guard found");
    }

    let map = Map::new(obstacles, lines.len() as i32, lines[0].len() as i32);
    let guard = Guard::new(Direction::North, guard_point, map);
    guard
}