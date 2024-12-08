use std::fmt::{Display, Formatter};
use crate::days::day10::pipe::Pipe;
use crate::common::point::Point;
pub struct PipeMap {
    pipes: Vec<Pipe>,
    pub pipe_loop: Vec<Pipe>,
    pub max_x: i32,
    pub max_y: i32,
    start: Point
}

impl Display for PipeMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let point = Point::new(x, y);
                let pipe = self.pipes.iter().find(|p| p.point == point);

                if pipe.is_some() {
                    write!(f, "{0}", pipe.unwrap().shape).unwrap();
                }
                else if self.is_inside_loop(point) {
                    write!(f, "I").unwrap();
                }
                else {
                    write!(f, "O").unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "\n")
    }
}

impl PipeMap {
    pub fn new(lines: Vec<String>) -> Self {
        let mut y = 0;
        let mut max_x = 0;
        let mut start = Point::new(0,0);
        let mut map_pipes = Vec::new();

        for line in lines {
            let mut x = 0;
            for c in line.trim().chars() {
                match c {
                    '.' => (),
                    'S' => start = Point::new(x, y),
                    _ => map_pipes.push(Pipe::new(x, y, c))
                }
                x += 1;
                max_x = x;
            }
            y += 1;
        }

        let mut current = map_pipes.iter().find(|p| p.is_connected_to(start)).unwrap();
        let first = current;
        let mut path = Vec::new();
        let mut pipe_loop = Vec::new();
        path.push(start);


        loop {
            //println!("current: {1}-{0}", current.point, current.shape);
            path.push(current.point);
            pipe_loop.push(current.clone());

            let mut next = current.connects.0;
            if path.contains(&next) {
                next = current.connects.1;
            }
            if path.contains(&next) {
                panic!("no unvisited path forward");
            }

            current = map_pipes.iter().find(|p| p.point == next).unwrap();
            if current.is_connected_to(start) {
                path.push(current.point);
                pipe_loop.push(current.clone());
                break;
            }
        }

        let shapes = vec!['|', '-', 'L', 'J', '7', 'F'];
        let mut pipes = map_pipes.clone();
        for shape in shapes {
            let pipe = Pipe::new(start.x, start.y, shape);
            if pipe.is_connected_to(current.point) && pipe.is_connected_to(first.point) {
                pipes.push(pipe.clone());
                pipe_loop.push(pipe.clone());
            }
        }

        Self {
            max_x,
            max_y: y,
            pipes,
            pipe_loop,
            start
        }
    }

    pub fn path_exists(&self, from: Point, to: Point) -> bool {
        //println!("checking from {from} to {to}");
        if from == to {
            return true;
        }
        // else if (from.x - 1 > 0 && self.path_exists(Point::new(from.x - 1, from.y), to)) {
        //     return true;
        // }
        else if from.x + 1 <= self.max_x && self.path_exists(Point::new(from.x + 1, from.y), to) {
            return true;
        }
        // else if (from.y - 1 > 0 && self.path_exists(Point::new(from.x, from.y - 1), to)) {
        //     return true;
        // }
        else if from.y + 1 <= self.max_y && self.path_exists(Point::new(from.x, from.y + 1), to) {
            return true;
        }
        false
    }
    pub fn is_inside_loop(&self, p: Point) -> bool {
        let mut count = 0;
        for x in 0..p.x {
            let point = Point::new(x, p.y);
            //println!("checking {point}: ");
            if self.pipe_loop.iter().any(|p| p.point == point) {
                count += 1;
            }
        }
        if count % 2 == 0 {
            return false;
        }

        count = 0;
        for x in p.x+1..self.max_x+1 {
            let point = Point::new(x, p.y);
            if self.pipe_loop.iter().any(|p| p.point == point) {
                count += 1;
            }
        }
        if count % 2 == 0 {
            return false;
        }

        count = 0;
        for y in 0..p.y {
            let point = Point::new(p.x, y);
            if self.pipe_loop.iter().any(|p| p.point == point) {
                count += 1;
            }
        }
        if count % 2 == 0 {
            return false;
        }

        count = 0;
        for y in p.y+1..self.max_y+1 {
            let point = Point::new(p.x, y);
            if self.pipe_loop.iter().any(|p| p.point == point) {
                count += 1;
            }
        }
        if count % 2 == 0 {
            return false;
        }

        true
    }
}