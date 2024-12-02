use crate::common::input_reader;
use crate::days::day11::galaxy_map::GalaxyMap;

mod tests;
mod galaxy_map;

pub struct Day;

impl Day {
    pub fn run(&self) {
        self.run_part_one();
    }
    pub fn run_part_one(&self) {
        let reader = input_reader::InputReader;
        let lines = reader.get_lines("/git/aoc23/src/days/day11/input");
        let map = GalaxyMap::new(lines);

        let mut sum: i64 = 0;
        for g1 in &map.galaxies {
            for g2 in &map.galaxies {
                let distance = g1.distance_to(*g2);
                sum += distance;
                //println!("distance from {n1}@({g1}) to {n2}@({g2}): {distance}");
            }
        }

        println!("{0}", sum/2);
    }
}