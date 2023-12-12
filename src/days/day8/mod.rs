extern crate regex;
use crate::common::input_reader;
use crate::days::day8::direction::Direction;
use crate::days::day8::multinodemap::MultiNodeMap;
use crate::days::day8::nodemap::NodeMap;
use num_integer::lcm;
mod tests;
mod path;
mod multinodemap;
mod direction;
mod nodemap;
mod node;

pub struct Day;
impl Day {
    pub fn run(&self) {
        self.run_part_two();
    }
    pub fn run_part_one(&self) {
        let reader = input_reader::InputReader;
        let mut n = 0;
        let mut directions = Vec::new();
        let mut map = NodeMap::new();

        for line in reader.get_lines("/git/aoc23/src/days/day8/test") {
            if n == 0 {
                for c in line.trim().chars() {
                    directions.push(Direction::from(c));
                }
            }
            if n > 1 {
                map.add(line.to_string().as_str());
            }
            n += 1;
        }

        n = 1;
        loop {
            for d in directions.iter() {
                print!("{n}: {0} -> ", map.current);
                map.next(*d);
                println!("{0}", map.current);
                if map.current == "ZZZ" {
                    println!("answer: {n}");
                    return;
                }
                n += 1;
            }
        }
    }

    pub fn run_part_two(&self) {
        let reader = input_reader::InputReader;
        let mut directions = Vec::new();
        let lines = reader.get_lines("/git/aoc23/src/days/day8/input");
        for c in lines[0].trim().chars() {
            directions.push(Direction::from(c));
        }

        let mut map = MultiNodeMap::new(lines[2..].to_vec());

        loop {
            for d in directions.iter() {
                map.next(*d);
                if map.is_finished() {
                    let mut zs = Vec::new();
                    for p in map.paths {
                        //println!("{0}", p.path.join(" -> "));
                        println!("{1} Zs: {0}", p.z_locations.iter().map(|z|z.to_string()).collect::<Vec<String>>().join(", "), p.start);
                        println!();
                        let z = p.z_locations.first().unwrap();
                        zs.push(*z);
                    }
                    let answer = zs.iter().fold(zs[0], |acc, &x| lcm(acc, x));
                    println!("{answer}");
                    return;
                }
            }
        }
    }
}