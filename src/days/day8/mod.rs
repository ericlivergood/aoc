extern crate regex;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use crate::common::input_reader;
use regex::Regex;

mod tests;

pub struct Day;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right")
        }
    }
}

impl Direction {
    pub fn from(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => {
                println!("unknown direction {c}");
                panic!();
            }
        }
    }
}

struct Path {
    cycle_length: i32,
    z_locations: Vec<i32>,
    path: Vec<String>,
    seen: HashSet<String>,
    current: String,
    start: String,
    complete: bool
}

impl Path {
    fn new(start: &str) -> Self {
        let mut seen = HashSet::new();
        seen.insert(start.to_string());
        let mut path = Vec::new();
        path.push(start.to_string());

        Self {
            cycle_length: 1,
            z_locations: Vec::new(),
            path,
            seen,
            start: start.to_string(),
            current: start.to_string(),
            complete: false
        }
    }

    fn next(&mut self, n: String) {
        let mut seen = self.current.clone();
        seen.push_str(n.as_str());

        if self.seen.contains(&seen) {
            //println!("{0}: found {1} again", self.start, n);
            self.complete = true;
        }
        //println!("{0}: moved to {1}", self.start, n);
        self.current = n.clone();
        self.seen.insert(n.clone());
        self.path.push(n.clone());
        if n.ends_with("Z") {
            self.z_locations.push(self.cycle_length);
        }
        self.cycle_length += 1;
    }
}

struct Node {
    name: String,
    left: String,
    right: String
}

struct NodeMap {
    nodes: HashMap<String, Node>,
    current: String
}

struct MultiNodeMap {
    nodes: HashMap<String, Node>,
    paths: Vec<Path>
}

impl MultiNodeMap {
    pub fn new() -> MultiNodeMap {
        MultiNodeMap {
            nodes: HashMap::new(),
            paths: Vec::new()
        }
    }

    pub fn add(&mut self, s: &str) {
        let n = Node::from(s);
        let name = n.name.to_string();
        if name.ends_with("A") {
            //println!("{0}", name);
            let p = Path::new(name.as_str());
            self.paths.push(p);
        };

        self.nodes.insert(name, n);
    }

    pub fn next(&mut self, d: Direction) {
        for c in &mut self.paths {
            let current_node = &self.nodes[c.current.as_str()];
            let next = match d {
                Direction::Left => &current_node.left,
                Direction::Right => &current_node.right
            };
            //println!("{2} {3}: ({0},{1}) -> {next}", &current_node.left, &current_node.right, &current_node.name, d);
            c.next(next.clone());
        }
    }

    pub fn is_finished(&self) -> bool {
        self.paths.iter().all(|x| x.complete)
    }
}

impl NodeMap {
    pub fn new() -> NodeMap {
        NodeMap {
            nodes: HashMap::new(),
            current: "AAA".to_string()
        }
    }

    pub fn add(&mut self, s: &str) {
        let n = Node::from(s);
        self.nodes.insert(n.name.to_string(), n);
    }

    pub fn next(&mut self, d: Direction) -> &Node {
        let current_node = &self.nodes[&self.current];
        let next = match d {
            Direction::Left => &current_node.left,
            Direction::Right => &current_node.right
        };

        let n = &self.nodes[next];
        self.current = next.clone();
        n
    }
}

impl Node {
    pub fn from(s: &str) -> Node {
        let re = Regex::new(r"([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)").unwrap();
        let c = re.captures(s).unwrap();

        Node {
            name: c.get(1).unwrap().as_str().to_string(),
            left: c.get(2).unwrap().as_str().to_string(),
            right: c.get(3).unwrap().as_str().to_string(),
        }
    }
}

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
        let mut n = 0;
        let mut directions = Vec::new();
        let mut map = MultiNodeMap::new();

        for line in reader.get_lines("/git/aoc23/src/days/day8/input") {
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
                map.next(*d);
                if map.is_finished() {
                    for p in map.paths {
                        println!("{0}: {1}", p.start, p.cycle_length);
                        for x in p.path {
                            print!("{x} ->");
                        }
                        println!("");
                        print!("Zs: ");
                        for x in p.z_locations {
                            print!("{x}, ");
                        }
                        println!("");
                        println!("");
                    }
                    return;
                }
                n += 1;
            }
        }
    }
}