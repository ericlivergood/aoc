use std::fmt::{Display, Formatter};
use crate::common::point::Point;
use crate::days::day16::beam::Direction::Right;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Right => write!(f, ">")
        }
    }
}

#[derive(Clone, Copy)]
pub struct Beam {
    pub location: Point,
    pub direction: Direction
}

impl Beam {
    pub fn new(location: Point, direction: Direction) -> Self {
        Self {
            location,
            direction
        }
    }

    pub fn default() -> Beam {
        Beam::new(Point::new(0, 0), Right)
    }

    pub fn next(&self) -> Point {
        match self.direction {
            Direction::Up => Point::new(self.location.x, self.location.y - 1),
            Direction::Down => Point::new(self.location.x, self.location.y + 1),
            Direction::Left => Point::new(self.location.x - 1, self.location.y),
            Direction::Right => Point::new(self.location.x + 1, self.location.y)
        }
    }

    pub fn move_to_next(&mut self) {
        //print!("moved from {0}", self.location);
        self.location = self.next();
        //println!(" to {0}", self.location)
    }
}