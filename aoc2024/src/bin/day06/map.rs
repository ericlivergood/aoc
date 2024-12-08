use std::collections::HashSet;
use aoc2024::common::point::Point;
use crate::enums::Direction;
use crate::guard::Guard;

pub struct Map {
    pub obstacles: HashSet<Point>,
    pub edges: HashSet<Point>
}

impl Map {
    pub fn new(obstacles: Vec<Point>, height: i32, width: i32) -> Map {
        let mut edges = HashSet::new();
        let obstacles_hash = HashSet::from_iter(obstacles.iter().cloned());

        for x in 0..width+1 {
            edges.insert(Point::new(x, 0));
            edges.insert(Point::new(x, height+1));
        }
        for y in 0..height+1 {
            edges.insert(Point::new(0, y));
            edges.insert(Point::new(width+1, y));
        }

        Map {
            edges,
            obstacles: obstacles_hash
        }
    }

    pub fn is_edge(&self, point: &Point) -> bool {
        self.edges.contains(point)
    }

    pub fn is_obstacle(&self, point: &Point) -> bool {
        self.obstacles.contains(point)
    }

    pub fn add_obstacle(&mut self, at: &Point) -> bool {
        if self.obstacles.contains(at) {
            return false;
        }

        self.obstacles.insert(at.clone());
        true
    }

    pub fn remove_obstacle(&mut self, at: &Point) {
        if self.obstacles.contains(at) {
            self.obstacles.remove(at);
        }
    }
}

