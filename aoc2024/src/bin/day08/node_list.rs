use std::collections::{HashMap, HashSet};
use aoc2024::common::point::Point;

pub struct NodeList {
    pub coords_by_freq: HashMap<char, Vec<Point>>,
    pub coord_to_freq: HashMap<Point, char>,
    pub height: i32,
    pub width: i32
}

fn directional_distance(p1: &Point, p2: &Point) -> (i32, i32) {
    (p1.x - p2.x, p1.y - p2.y)
}

impl NodeList {
    pub fn from_lines(lines: &Vec<String>) -> NodeList {
        let mut coords_by_freq: HashMap<char, Vec<Point>> = HashMap::new();
        let mut coord_to_freq: HashMap<Point, char> = HashMap::new();
        let mut x = 1;
        let mut y = 1;

        for line in lines {
            x = 1;
            for c in line.chars() {
                match c {
                    '.' => { },
                    value => {
                        if !coords_by_freq.contains_key(&value) {
                            coords_by_freq.insert(value, Vec::new());
                        }
                        coords_by_freq.get_mut(&value).unwrap().push(Point::new(x, y));
                        coord_to_freq.insert(Point::new(x, y), value);
                    }
                }
                x += 1;
            }
            y += 1;
        }

        NodeList {
            coords_by_freq,
            coord_to_freq,
            height: y,
            width: x
        }
    }

    pub fn print_antinodes(&self, include_harmonics: bool) {
        let nodes = self.get_antinodes(include_harmonics);

        for y in 1..self.height {
            for x in 1..self.width {
                if self.coord_to_freq.contains_key(&Point::new(x, y)) {
                    print!("{}", self.coord_to_freq.get(&Point::new(x, y)).unwrap());
                }
                else if nodes.contains(&Point::new(x, y)) {
                    print!("#");
                }
                else {
                    print!(".");
                }
             }
            println!();
        }
    }

    pub fn get_antinodes(&self, include_harmonics: bool) -> HashSet<Point> {
        let mut antinodes = HashSet::new();

        //for each char:
        for c in self.coords_by_freq.keys() {
            for p1 in self.coords_by_freq[c].iter() {
                for p2 in self.coords_by_freq[c].iter() {
                    if include_harmonics {
                        antinodes.insert(p1.clone());
                        antinodes.insert(p2.clone());
                    }
                    if p1 == p2 {
                        continue;
                    }
                    let distance = directional_distance(p1, p2);

                    let antinode_x = p1.x + distance.0;
                    let antinode_y = p1.y + distance.1;
                    let mut antinode = Point::new(antinode_x, antinode_y);
                    if antinode.x < 1 || antinode.x >= self.width || antinode.y < 1 || antinode.y >= self.height {
                        continue;
                    }
                    antinodes.insert(antinode.clone());

                    if include_harmonics && self.coords_by_freq[c].len() > 1 {
                        loop {
                            antinode.x += distance.0;
                            antinode.y += distance.1;

                            if antinode.x < 1 || antinode.x >= self.width || antinode.y < 1 || antinode.y >= self.height {
                                break;
                            }

                            antinodes.insert(antinode.clone());
                        }
                    }
                }
            }
        }
        antinodes
    }
}