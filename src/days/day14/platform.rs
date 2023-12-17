use std::fmt::{Display, Formatter};
use array2d::Array2D;
use crate::days::day14::platform::PlatformLocation::{Empty, RoundRock, CubeRock};

#[derive(Copy, Clone)]
pub enum PlatformLocation {
    RoundRock,
    CubeRock,
    Empty
}

impl Display for PlatformLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            RoundRock => 'O',
            CubeRock => '#',
            Empty => '.'
        };
        write!(f, "{char}")
    }
}

impl PlatformLocation {
    pub fn from(c: char) -> PlatformLocation {
        match c {
            'O' => RoundRock,
            '#' => CubeRock,
            '.' => Empty,
            _=> panic!("unknown location type: {c}")
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.locations.as_rows() {
            for c in row {
                write!(f, "{c}").unwrap();
            }
            writeln!(f).unwrap();
        }
        writeln!(f).unwrap();
        writeln!(f, "(w: {0},h: {1})", self.width, self.height)
    }
}

pub struct Platform {
    pub locations: Array2D<PlatformLocation>,
    width: usize,
    height: usize
}

impl Platform {
    pub fn new(lines: Vec<String>) -> Self {
        let mut rows = Vec::new();
        let width = lines[0].trim().len();
        let height = lines.len();

        for line in lines {
            let mut row = Vec::new();
            for c in line.trim().chars() {
                row.push(PlatformLocation::from(c));
            }
            rows.push(row);
        }

        let locations = Array2D::from_rows(&*rows).unwrap();
        Self {
            locations,
            width,
            height
        }
    }

    fn can_roll_up(&self, x: usize, y: usize) -> bool {
        let l = self.locations.get(y, x).unwrap();

        match l {
            RoundRock => {
                if y == 0 {
                    return false
                }
                let up = self.locations.get(y-1, x).unwrap();
                match up {
                    RoundRock => false,
                    CubeRock => false,
                    Empty => true
                }
            },
            CubeRock => false,
            Empty => false
        }
    }

    pub fn roll_up(&mut self, x: usize, y: usize) {
        //println!("rolling up {x},{y}");
        self.locations.set(y-1, x, RoundRock).expect("could not set up value");
        self.locations.set(y, x, Empty).expect("could not unset");
    }

    pub fn tilt(&mut self) {
        for x in 0..self.width {
            let mut rolled_up = true;

            while rolled_up {
                rolled_up = false;
                for y in 0..self.height {
                    if self.can_roll_up(x, y) {
                        self.roll_up(x, y);
                        rolled_up = true;
                        //println!("{self}");
                    }
                }
            }
        }
    }

    pub fn rotate(&mut self) {
        self.rotate_clockwise();
    }

    pub fn rotate_clockwise(&mut self) {
        self.rotate_counterclockwise();
        self.rotate_counterclockwise();
        self.rotate_counterclockwise();
    }

    pub fn rotate_counterclockwise(&mut self) {
        let mut rows = self.locations.as_columns();
        rows.reverse();
        self.locations = Array2D::from_rows(&*rows).unwrap();
    }

    pub fn calculate_load(&self) -> i32 {
        let mut load = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                load += match self.locations.get(y, x)
                    .expect("unknown location") {
                    RoundRock => (self.height - y) as i32,
                    _=> 0
                }
            }
        }
        load
    }
}