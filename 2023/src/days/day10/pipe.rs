use std::fmt::{Display, Formatter};
use crate::common::point::Point;

#[derive(Copy, Clone)]
pub struct Pipe {
    pub shape: char,
    pub point: Point,
    pub connects: (Point, Point)
}

impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}@({1},{2})", self.shape, self.point.x, self.point.y)
    }
}

impl Pipe {
    pub fn new(x: i32, y: i32, shape: char) -> Self {
        let c1 = match shape {
            '|' => { Point::new(x, y - 1)},
            '-' => { Point::new(x - 1, y) },
            'L' => { Point::new(x, y - 1) },
            'J' => { Point::new(x, y - 1) },
            '7' => { Point::new(x - 1, y) },
            'F' => { Point::new(x,y + 1) },
            _=> panic!("unknown shape {0}", shape)
        };

        let c2 = match shape {
            '|' => { Point::new(x, y + 1) },
            '-' => { Point::new(x + 1, y) },
            'L' => { Point::new(x + 1, y) },
            'J' => { Point::new(x - 1, y) },
            '7' => { Point::new(x, y + 1) },
            'F' => { Point::new(x + 1, y) },
            _=> panic!("unknown shape {0}", shape)
        };

        Self {
            shape,
            point: Point {
                x, y
            },
            connects: (c1, c2)
        }
    }

    pub fn is_connected_to(&self, p: Point) -> bool {
        self.connects.0 == p || self.connects.1 == p
    }

    pub fn next(&self) -> Point {
        self.connects.0
    }

    pub fn prev(&self) -> Point {
        self.connects.1
    }
}