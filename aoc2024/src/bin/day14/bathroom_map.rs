use std::collections::HashSet;
use aoc2024::common::point::Point;
use crate::robot::Robot;

pub struct BathroomMap {
    pub robots: Vec<Robot>,
    pub width: i32,
    pub height: i32
}

impl BathroomMap {
    pub fn new(robots: Vec<Robot>, width: i32, height: i32) -> BathroomMap {
        BathroomMap {
            robots,
            width,
            height
        }
    }
    pub fn from_lines(lines: &Vec<String>, width: i32, height: i32) -> Self {
        let robots = lines.iter()
            .map(|l| Robot::from_line(l))
            .collect();

        Self::new(robots, width, height)
    }

    pub fn advance(&mut self, n: i32) {
        for r in self.robots.iter_mut() {
            r.advance(n, self.width, self.height);
        }
    }

    pub fn get_safety_factor(&self) -> i32 {
        let mid_x = self.width / 2;
        let mid_y = self.height / 2;

        let nw_quadrant: Vec<_> = self.robots.iter()
            .filter(|&r| r.position.x < mid_x && r.position.y < mid_y)
            .collect();

        let ne_quadrant: Vec<_> = self.robots.iter()
            .filter(|&r| r.position.x > mid_x && r.position.y < mid_y)
            .collect();

        let sw_quadrant: Vec<_> = self.robots.iter()
            .filter(|&r| r.position.x < mid_x && r.position.y > mid_y)
            .collect();

        let se_quadrant: Vec<_> = self.robots.iter()
            .filter(|&r| r.position.x > mid_x && r.position.y > mid_y)
            .collect();

        (ne_quadrant.len() * nw_quadrant.len() *
            se_quadrant.len() * sw_quadrant.len()) as i32
    }

    pub fn get_distance_sum(&self) -> f64 {
        let mut sum = 0 as f64;

        for r1 in self.robots.iter() {
            for r2 in self.robots.iter() {
                let distance = r1.position.distance(&r2.position);
                let n = (distance.0 ^2) as f64 + (distance.1^2) as f64;
                sum += n.sqrt();
            }
        }
        sum
    }

    pub fn get_min_distance_time(&mut self, n: i32) -> i32 {
        let mut min = f64::MAX;
        let mut min_n = 0;

        for i in 0..n {
            let distance = self.get_distance_sum();
            if distance < min {
                min = distance;
                min_n = i;
            }
            self.advance(1);
        }
        min_n
    }

    pub fn print_map(&self) {
        let positions: HashSet<Point> = self.robots.iter().map(|r| r.position).collect();
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point{x: x as i32, y: y as i32};

                if positions.contains(&p) {
                    print!("#");
                }
                else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_safety_factor() {
        let map = BathroomMap::from_lines(&Vec::new(),0,0);
        assert_eq!(map.get_safety_factor(), 0);

        let r = Robot::from_coords(0,0,0,0);
        let map = BathroomMap::new(vec![r], 11,11);
        assert_eq!(map.get_safety_factor(), 0);

        let r_nw = Robot::from_coords(0,0,0,0);
        let r_ne = Robot::from_coords(10,0,0,0);
        let r_sw = Robot::from_coords(0,10,0,0);
        let r_se = Robot::from_coords(10,10,0,0);
        let r_mv = Robot::from_coords(0,0,0,0);
        let mut map = BathroomMap::new(vec![r_mv, r_nw, r_ne, r_sw, r_se], 11, 11);

        assert_eq!(map.get_safety_factor(), 2);
        map.robots[0].position.x = 9; //9,0
        assert_eq!(map.get_safety_factor(), 2);
        map.robots[0].position.y = 9; //9,9
        assert_eq!(map.get_safety_factor(), 2);
        map.robots[0].position.x = 0; //0,9
        assert_eq!(map.get_safety_factor(), 2);

        map.robots[0].position.y = 5; //0,5
        assert_eq!(map.get_safety_factor(), 1);
        map.robots[0].position.x = 5; //5,5
        assert_eq!(map.get_safety_factor(), 1);
        map.robots[0].position.y = 0; //5,0
        assert_eq!(map.get_safety_factor(), 1);
    }
}