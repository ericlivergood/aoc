use regex::Regex;
use aoc2024::common::point::Point;

pub struct Robot {
    pub position: Point,
    velocity: Point
}

impl Robot {
    pub fn from_coords(x: i32, y: i32, x_delta: i32, y_delta: i32) -> Robot {
        Robot::new(Point::new(x, y), Point::new(x_delta, y_delta))
    }
    pub fn from_line(line: &String) -> Robot {
        let re = Regex::new(r"p=([\-\d]+),([\-\d]+) v=([\-\d]+),([\-\d]+)").unwrap();

        if let Some(captures) = re.captures(&line) {
            let p1 = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let p2 = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let v1 = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let v2 = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
            return Robot::from_coords(p1, p2, v1, v2);
        }
        panic!("unparseable robot line: {}", line);
    }

    pub fn new(position: Point, velocity: Point) -> Self {
        Robot { position, velocity }
    }

    pub fn advance(&mut self, n: i32, max_x: i32, max_y: i32) -> Point {
        let mut next_x = self.position.x + (n * self.velocity.x);
        let mut next_y = self.position.y + (n * self.velocity.y);

        //handle wrap around max side
        next_x = next_x % max_x;
        next_y = next_y % max_y;

        //handle wrap around 0 side
        if next_x < 0 {
            next_x = max_x + next_x;
        }
        if next_y < 0 {
            next_y = max_y + next_y;
        }

        self.position.x = next_x;
        self.position.y = next_y;
        Point::new(next_x, next_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_advance_basic() {
        let mut r = Robot::new(Point::new(0, 0), Point::new(1, 1));
        r.advance(1, 100, 100);
        assert_eq!(r.position, Point::new(1, 1));
        r.advance(2, 100, 100);
        assert_eq!(r.position, Point::new(3, 3));

        r.velocity.y = 2;
        r.advance(1, 100, 100);
        assert_eq!(r.position, Point::new(4, 5));
        r.velocity.x = 2;
        r.advance(1, 100, 100);
        assert_eq!(r.position, Point::new(6, 7));
    }

    #[test]
    fn test_advance_zero_wrap_around() {
        let mut r = Robot::new(Point::new(0, 0), Point::new(-1, -1));
        r.advance(1, 100, 100);
        assert_eq!(r.position, Point::new(99, 99));
        let mut r = Robot::new(Point::new(0, 0), Point::new(-10, -10));
        r.advance(1, 100, 100);
        assert_eq!(r.position, Point::new(90, 90));
    }

    #[test]
    fn test_advance_max_wrap_around() {
        let mut r = Robot::new(Point::new(99, 99), Point::new(1, 1));
        r.advance(1, 100, 100);
        assert_eq!(r.position, Point::new(0, 0));

        let mut r = Robot::new(Point::new(99, 99), Point::new(10, 10));
        r.advance(1, 100, 100);
        assert_eq!(r.position, Point::new(9, 9));
    }

    #[test]
    fn test_from_coords() {
        let r = Robot::from_coords(1,2,3,4);
        assert_eq!(r.position.x, 1);
        assert_eq!(r.position.y, 2);
        assert_eq!(r.velocity.x, 3);
        assert_eq!(r.velocity.y, 4);
    }
    #[test]
    fn test_from_line() {
        let r = Robot::from_line(&String::from("p=94,54 v=80,24"));
        assert_eq!(r.position.x, 94);
        assert_eq!(r.position.y, 54);
        assert_eq!(r.velocity.x, 80);
        assert_eq!(r.velocity.y, 24);

        let r = Robot::from_line(&String::from("p=-94,-54 v=-80,-24"));
        assert_eq!(r.position.x, -94);
        assert_eq!(r.position.y, -54);
        assert_eq!(r.velocity.x, -80);
        assert_eq!(r.velocity.y, -24);
    }
}