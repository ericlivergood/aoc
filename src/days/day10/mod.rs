use crate::common::input_reader;
use crate::days::day10::pipe_map::PipeMap;
use crate::common::point::Point;

mod tests;
mod pipe;
mod pipe_map;

pub struct Day;

impl Day {
    pub fn run(&self) {
        self.run_part_one();
    }
    pub fn run_part_one(&self) {
        let reader = input_reader::InputReader;
        let lines = reader.get_lines("/git/aoc23/src/days/day10/test");
        let map = PipeMap::new(lines);

        println!("{map}");

        print!("{0}", ((map.pipe_loop.len()) as f32 / 2.0f32).ceil());
    }

    pub fn run_part_two(&self) {
        let reader = input_reader::InputReader;
        let lines = reader.get_lines("/git/aoc23/src/days/day10/test");

        println!("building map");
        let map = PipeMap::new(lines);

        println!("map built, finding inside points");
        let mut inside = 0;
        for x in 0..map.max_x {
            for y in 0..map.max_y {
                if map.is_inside_loop(Point::new(x, y)) {
                    println!("{x}, {y}");
                    inside += 1;
                }
            }
        }
        println!("{inside}");
    }
}