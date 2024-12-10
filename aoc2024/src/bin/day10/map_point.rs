use array2d::Array2D;
use aoc2024::common::point::Point;

#[derive(Hash, Clone, Copy)]
pub struct MapPoint {
    pub point: Point,
    pub height: i32
}

impl MapPoint {
    pub fn new(x: usize, y: usize, map: &Array2D<i32>) -> MapPoint {
        let point = Point::new(x as i32, y as i32);
        let height = *map.get(y, x).unwrap();

        MapPoint {
            point,
            height
        }
    }

    pub fn can_move_to(&self, p: &MapPoint) -> bool {
        p.height - self.height == 1
    }
}

impl Eq for MapPoint {}

impl PartialEq for MapPoint {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_moveable_points() {
        let p1 = MapPoint { point: Point { x: 0, y: 0 }, height: 1 };
        let p2 = MapPoint { point: Point { x: 0, y: 0 }, height: 0 };
        let p3 = MapPoint { point: Point { x: 0, y: 0 }, height: 2 };
        let p4 = MapPoint { point: Point { x: 0, y: 0 }, height: 3 };

        assert_eq!(p1.can_move_to(&p1), false);
        assert_eq!(p1.can_move_to(&p2), false);
        assert_eq!(p1.can_move_to(&p3), true);
        assert_eq!(p1.can_move_to(&p4), false);

        assert_eq!(p2.can_move_to(&p1), true);
        assert_eq!(p2.can_move_to(&p2), false);
        assert_eq!(p2.can_move_to(&p3), false);
        assert_eq!(p2.can_move_to(&p4), false);

        assert_eq!(p3.can_move_to(&p1), false);
        assert_eq!(p3.can_move_to(&p2), false);
        assert_eq!(p3.can_move_to(&p3), false);
        assert_eq!(p3.can_move_to(&p4), true);

        assert_eq!(p4.can_move_to(&p1), false);
        assert_eq!(p4.can_move_to(&p2), false);
        assert_eq!(p4.can_move_to(&p3), false);
        assert_eq!(p4.can_move_to(&p4), false);
    }
}