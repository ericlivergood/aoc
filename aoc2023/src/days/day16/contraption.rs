use std::collections::{HashSet};
use array2d::Array2D;
use crate::common::point::Point;
use crate::days::day16::beam::{Beam, Direction};
use crate::days::day16::beam::Direction::{Down, Left, Right, Up};
use crate::days::day16::contraption::LocationType::{Empty, HorizontalSplitter, LeftMirror, RightMirror, VerticalSplitter};

#[derive(Clone)]
enum LocationType {
    Empty,
    LeftMirror, // /
    RightMirror, // \
    VerticalSplitter,
    HorizontalSplitter
}

impl LocationType {
    pub fn from(c: char) -> LocationType {
        match c {
            '.' => Empty,
            '/' => LeftMirror,
            '\\' => RightMirror ,
            '|' => VerticalSplitter,
            '-' => HorizontalSplitter,
            _=> panic!("unknown: {0}", c)
        }
    }

    pub fn map_beam(&self, beam: Beam) -> Vec<Beam> {
        let mut b = beam.clone();
        match self {
            Empty => vec![b],
            LeftMirror => { // /
                b.direction = match b.direction {
                    Up => Right,
                    Down => Left,
                    Left => Down,
                    Right => Up
                };
                vec![b]
            }
            RightMirror => { // \
                b.direction = match b.direction {
                    Up => Left,
                    Down => Right,
                    Left => Up,
                    Right => Down
                };
                vec![b]
            }
            VerticalSplitter => {
                match b.direction {
                    Up | Down => vec![b],
                    Left | Right => vec![
                            Beam::new(b.location, Up),
                            Beam::new(b.location, Down)
                        ]
                }
            }
            HorizontalSplitter => {
                match b.direction {
                    Left | Right=> vec![b],
                    Up | Down => vec![
                            Beam::new(b.location, Left),
                            Beam::new(b.location, Right)
                        ]
                }
            }
        }
    }

}
#[derive(Clone)]
struct Location {
    location_type: LocationType,
    point: Point,
    visited: bool,
    visited_by: HashSet<Direction>
}

impl Location {
    fn new(l_type: char, x: i32, y: i32) -> Self {
        Self {
            location_type: LocationType::from(l_type),
            point: Point::new(x,y),
            visited: false,
            visited_by: HashSet::new()
        }
    }

    fn pass_through(&mut self, beam: Beam) -> Vec<Beam> {
        self.visited = true;
        if self.visited_by.contains(&beam.direction) {
            return Vec::new();
        }
        self.visited_by.insert(beam.direction);

        let mut beams = Vec::new();
        for mut b in self.location_type.map_beam(beam) {
            b.move_to_next();
            beams.push(b);
        }
        beams
    }
}

pub struct Contraption {
    layout: Array2D<Location>,
    beam_visited: HashSet<Point>,
    beams: Vec<Beam>,
    width: i32,
    height: i32
}


impl Contraption {
    pub fn new(lines: Vec<String>) -> Self {
        let mut rows = Vec::new();
        let mut y = 0;
        for l in lines {
            let mut x = 0;
            let mut row = Vec::new();
            for c in l.trim().chars() {
                row.push(Location::new(c, x, y));
                x += 1;
            }
            rows.push(row);
            y +=1;
        }

        let layout = Array2D::from_rows(&*rows).unwrap();
        let width = layout.row_len() as i32;
        let height = layout.column_len() as i32;
        Self {
            layout,
            beams: Vec::new(),
            beam_visited: HashSet::new(),
            width,
            height
        }
    }
    fn get_location(&mut self, point: Point) -> Option<&mut Location> {
        if point.x < 0 || point.y < 0 || point.x >= self.width || point.y >= self.height {
            return None;
        }
        self.layout.get_mut(point.y as usize, point.x as usize)
    }
    fn is_in_bounds(&self, p: Point) -> bool {
        p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height
    }

    pub fn run(&mut self) {
        self.run_part_two();
    }

    pub fn run_part_one(&mut self) {
        // let answer = self.run_for(Beam::default());
        let answer = self.run_for(Beam::new(Point::new(109, 94), Left));
        println!("{answer}");
    }

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let loc = self.layout.get_mut(y as usize, x as usize).unwrap();
                loc.visited = false;
                loc.visited_by = HashSet::new();
            }
        }
    }

    pub fn run_part_two(&mut self) {
        let mut max = 0;
        let mut max_point = Point::new(0,0);
        let mut max_dir = Right;

        for x in 0..self.width {
            self.clear();
            let dir = Down;
            let start = Point::new(x, 0);
            let a = self.run_for(Beam::new(start, dir));
            if a > max {
                max = a;
                max_point = start;
                max_dir = dir;
            }
        }
        for x in 0..self.width {
            self.clear();
            let dir = Up;
            let start = Point::new(x, self.height-1);
            let a = self.run_for(Beam::new(start, dir));
            if a > max {
                max = a;
                max_point = start;
                max_dir = dir;
            }
        }
        for y in 0..self.height {
            self.clear();
            let dir = Right;
            let start = Point::new(0, y);
            let a = self.run_for(Beam::new(start, dir));
            if a > max {
                max = a;
                max_point = start;
                max_dir = dir;
            }
        }
        for y in 0..self.height {
            self.clear();
            let dir = Left;
            let start = Point::new(self.width-1, y);
            let a = self.run_for(Beam::new(start, dir));
            if a > max {
                max = a;
                max_point = start;
                max_dir = dir;
            }
        }

        println!("width: {0}, height: {1}", self.width, self.height);
        println!("max coverage @ {0}, {1}: {2}", max_point, max_dir, max);
    }

    pub fn run_for(&mut self, start: Beam) -> i32 {
        //println!("running for {0}-{1}", start.location, start.direction);
        let mut beams = Vec::new();
        beams.push(start);
        let mut n = 0;
        while beams.len() > 0 && n < 71000 {
            match beams.pop() {
                None => {}
                Some(beam) => {
                    match self.get_location(beam.location) {
                        None => {}
                        Some(l) => {
                            for b in l.pass_through(beam) {
                                beams.push(b);
                            }
                        }
                    }
                }
            }
            //println!("{n}");
            n += 1;
        }

        let mut answer = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.layout.get(y as usize, x as usize).unwrap().visited {
                    answer += 1;
                }
            }
        }
        answer
    }
}