use std::collections::HashMap;
use aoc2024::common::point::Point;
use crate::enums::{Direction, MovementOutcome};
use crate::map::Map;

fn next_location(point: &Point, direction: &Direction) -> Point {
    match direction {
        Direction::North => Point::new(point.x, point.y - 1),
        Direction::South => Point::new(point.x, point.y + 1),
        Direction::East => Point::new(point.x + 1, point.y),
        Direction::West => Point::new(point.x - 1, point.y),
    }
}

pub struct Guard {
    pub direction: Direction,
    pub location: Point,
    orig_location: Point,
    orig_direction: Direction,
    pub path: HashMap<Point, i32>,
    d_path: HashMap<(Point, Direction), i32>,
    map: Map
}

impl Guard {
    pub fn new(direction: Direction, location: Point, map: Map) -> Guard {
        let path = HashMap::new();
        Guard {
            orig_direction: direction.clone(),
            direction,
            orig_location: location.clone(),
            location,
            path,
            map,
            d_path: HashMap::new()
        }
    }

    fn try_move(&mut self) -> MovementOutcome {
        let next = next_location(&self.location, &self.direction);

        if self.map.is_obstacle(&next) {
            return MovementOutcome::Blocked;
        }

        if self.map.is_edge(&next) {
            return MovementOutcome::OnEdge;
        }
        self.location = next;
        MovementOutcome::Moved
    }

    pub fn record_visit(&mut self) {
        self.path.insert(self.location.clone(), 1);
        self.d_path.insert((self.location.clone(), self.direction.clone()), 1);
    }

    pub fn has_visited(&self) -> bool {
        self.d_path.contains_key(&(self.location.clone(), self.direction.clone()))
    }

    pub fn add_obstacle(&mut self, at: &Point) -> bool {
        self.map.add_obstacle(at)
    }

    pub fn remove_obstacle(&mut self, at: &Point) {
        self.map.remove_obstacle(at);
    }

    pub fn reset(&mut self) {
        self.location = self.orig_location.clone();
        self.direction = self.orig_direction.clone();
    }

    pub fn patrol(&mut self) -> bool {
        self.path = HashMap::new();
        self.d_path = HashMap::new();

        let mut outcome = self.try_move();
        let mut i = 0;
        loop {
            match outcome {
                MovementOutcome::Blocked => {
                    let next = self.direction.next();
                    //println!("turning {}", next);
                    self.direction = next;
                },
                MovementOutcome::Moved => {
                    //println!("{}", self.location);
                    if(self.has_visited()) {
                        return false;
                    }

                    self.record_visit();
                },
                MovementOutcome::OnEdge => {
                    self.record_visit();
                    return true;
                }
            }
            outcome = self.try_move();
        }
    }
}
