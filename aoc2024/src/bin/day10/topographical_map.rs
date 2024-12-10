use array2d::Array2D;
use aoc2024::common::point::Point;
use crate::map_point::MapPoint;
use crate::route::Route;

pub struct TopographicalMap {
    pub points: Vec<MapPoint>,
    pub map: Array2D<i32>,
    pub trail_heads: Vec<MapPoint>,
    pub peaks: Vec<MapPoint>
}

impl TopographicalMap {
    pub fn new(map: Array2D<i32>) -> TopographicalMap {
        let mut points = Vec::new();
        let mut peaks = Vec::new();
        let mut trail_heads = Vec::new();

        for y in 0..map.column_len() {
            for x in 0..map.row_len() {
                if map.get(y, x).unwrap() >= &0 {
                    points.push(MapPoint::new(x, y, &map));
                }
                if map.get(y, x).unwrap() == &0 {
                    trail_heads.push(MapPoint::new(x, y, &map));
                }
                if map.get(y, x).unwrap() == &9 {
                    peaks.push(MapPoint::new(x, y, &map));
                }
            }
        }

        TopographicalMap {
            map,
            points,
            trail_heads,
            peaks
        }
    }
    pub fn from_lines(lines: &[String]) -> TopographicalMap {
        let mut map = Array2D::filled_with(-1, lines[0].len(), lines.len());

        let mut y = 0;
        for line in lines {
            let mut x = 0;
            for c in line.chars() {
                let height = match c {
                    '.' => -1,
                    value => value.to_string().parse::<i32>().unwrap()
                };
                map.set(y, x, height).unwrap();
                x += 1;
            }
            y += 1;
        }
        TopographicalMap::new(map)
    }

    fn try_add_next_step(&self, point: &MapPoint, delta_x: i32, delta_y: i32, available: &mut Vec<MapPoint>, path: &Route) {
        let x = (point.point.x + delta_x )as usize;
        let y = (point.point.y + delta_y) as usize;

        match self.map.get(y, x) {
            Some(next_height) => {
                let next = MapPoint {
                    point: Point::new(x as i32, y as i32),
                    height: *next_height
                };

                if !path.has_visited(&next) && point.can_move_to(&next) {
                    available.push(next);
                }
            },
            None => {}
        }
    }

    pub fn available_next_steps_from(&self, point: &MapPoint, path: &Route) -> Vec<MapPoint> {
        let mut available = Vec::new();
        self.try_add_next_step(point, 0, -1, &mut available, &path);
        self.try_add_next_step(point, 0, 1, &mut available, &path);
        self.try_add_next_step(point, -1, 0, &mut available, &path);
        self.try_add_next_step(point, 1, 0, &mut available, &path);
        available
    }

    fn navigate(&self, current: &MapPoint, path: &mut Route, track_path: bool) -> Result<Vec<Route>, ()>{
        if path.has_visited(current) {
            return Err(());
        }
        if(track_path) {
            path.visit(current);
        }
        if current.height == 9 {
            let cloned = path.clone();
            return Ok(vec![cloned])
        }
        let mut routes = Vec::new();
        for next in self.available_next_steps_from(&current, &path) {
            match self.navigate(&next, path, track_path) {
                Ok(r) => {
                    routes.extend(r);
                },
                Err(_) => {}
            }
        }
        Ok(routes)
    }

    pub fn get_route_scores(&self) -> i32 {
        let mut sum = 0;
        let mut ratings = 0;
        for t in self.trail_heads.iter() {
            match self.navigate(t, &mut Route::new(), true) {
                Ok(routes) => {
                    sum += routes.len() as i32;
                },
                Err(()) => {}
            }
        }
        sum
    }
    pub fn get_ratings(&self) -> i32 {
        let mut sum = 0;
        let mut ratings = 0;
        for t in self.trail_heads.iter() {
            match self.navigate(t, &mut Route::new(), false) {
                Ok(routes) => {
                    sum += routes.len() as i32;
                },
                Err(()) => {}
            }
        }
        sum
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builds_map(){
        let map_vec = Array2D::from_rows(&vec![
           vec![0,1],
           vec![2,9]
        ]).unwrap();
        let map = TopographicalMap::new(map_vec);

        assert_eq!(map.points.len(), 4);
        assert_eq!(map.points[0].height, 0);
        assert_eq!(map.points[1].height, 1);
        assert_eq!(map.points[2].height, 2);
        assert_eq!(map.points[3].height, 9);

        assert_eq!(map.map.get(0, 0).unwrap(), &0);
        assert_eq!(map.map.get(0, 1).unwrap(), &1);
        assert_eq!(map.map.get(1, 0).unwrap(), &2);
        assert_eq!(map.map.get(1, 1).unwrap(), &9);

        assert_eq!(map.peaks.len(), 1);
        assert_eq!(map.trail_heads.len(), 1);
    }

    #[test]
    fn builds_map_from_lines(){
        let map_vec = vec![
            "01".to_string(),
            "23".to_string()
        ];
        let map = TopographicalMap::from_lines(&map_vec);

        assert_eq!(map.points.len(), 4);
        assert_eq!(map.points[0].height, 0);
        assert_eq!(map.points[1].height, 1);
        assert_eq!(map.points[2].height, 2);
        assert_eq!(map.points[3].height, 3);

        assert_eq!(map.map.get(0, 0).unwrap(), &0);
        assert_eq!(map.map.get(0, 1).unwrap(), &1);
        assert_eq!(map.map.get(1, 0).unwrap(), &2);
        assert_eq!(map.map.get(1, 1).unwrap(), &3);
    }
}