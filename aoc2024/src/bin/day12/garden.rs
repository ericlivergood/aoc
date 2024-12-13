use std::collections::HashSet;
use aoc2024::common::point::Point;

pub struct Garden {
    plot: Vec<Vec<String>> //y,x based
}

impl Garden {
    pub fn new(plot: Vec<Vec<String>>) -> Garden {
        Garden { plot }
    }

    pub fn from_lines(lines: &Vec<String>) -> Garden {
        let mut plot = Vec::new();
        for l in lines {
            let mut line = Vec::new();
            for c in l.chars() {
                line.push(c.to_string())
            }
            plot.push(line);
        }
        Garden::new(plot)
    }

    fn flood_fill(&self, start_at: &Point, visited: &mut HashSet<Point>) {
        if visited.contains(start_at) {

        }
        Vec::new()
    }

    pub fn get_subplots(&self) -> Vec<Vec<Point>> {
        let mut subplots = Vec::new();
        let mut visited = HashSet::new();
        for y in 0..self.plot.len() {
            for x in 0..self.plot[y].len() {
                let cur = Point::new(x as i32, y as i32);
                if(visited.contains(&cur)) {
                    continue;
                }

                subplots.push(self.flood_fill(&cur, &mut visited));
            }
        }

        Vec::new()
    }

    pub fn exterior_perimeter_of(subplot: &Vec<Point>) -> i64 {
        0
    }

    pub fn interior_perimeter_of(subplot: &Vec<Point>) -> i64 {
        0
    }

    pub fn perimeter_of(subplot: &Vec<Point>) -> i64 {
        Self::exterior_perimeter_of(subplot) + Self::interior_perimeter_of(subplot)
    }
}