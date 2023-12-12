use std::collections::HashSet;
use crate::common::point::Point;

pub struct GalaxyMap {
    pub galaxies: Vec<Point>
}

impl Point {
    pub fn distance_to(&self, p: Point) -> i64 {
        ((p.x - self.x).abs() + (p.y - self.y).abs()).into()
    }
}

impl GalaxyMap {
    pub fn new(lines: Vec<String>) -> Self {
        let mut y = 0;
        let mut galaxies = Vec::new();
        let mut galaxied_columns = HashSet::new();


        for l in &lines {
            let mut x = 0;
            for c in l.chars() {
                if c == '#' {
                    //println!("galaxy in {x}");
                    galaxied_columns.insert(x);
                }
                x += 1;
            }
        }

        for l in &lines {
            let mut x = 0;
            let mut map_x = 0;
            let mut galaxy_found = false;
            for c in l.chars() {
                if !galaxied_columns.contains(&map_x) {
                    x += 999_999;
                }
                match c {
                    '#' => {
                        galaxies.push(Point::new(x, y));
                        galaxy_found = true;
                    },
                    _=> ()
                }
                x += 1;
                map_x += 1;
            }
            y += 1;
            if !galaxy_found {
                y += 999_999;
            }
        }

        Self {
            galaxies
        }
    }
}