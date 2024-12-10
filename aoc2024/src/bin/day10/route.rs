use std::collections::HashSet;
use crate::map_point::MapPoint;

pub struct Route {
    path: Vec<MapPoint>,
    visited: HashSet<MapPoint>
}

impl Clone for Route {
    fn clone(&self) -> Route {
        let path = self.path.clone();
        let visited = HashSet::from_iter(self.path.clone());

        Route {
            path,
            visited
        }
    }
}

impl Route {
    pub fn new() -> Route {
        Route {
            path: Vec::new(),
            visited: HashSet::new()
        }
    }

    pub fn visit(&mut self, p: &MapPoint) {
        if self.visited.contains(p) {
            panic!("Route already visited!");
        }
        self.path.push(p.clone());
        self.visited.insert(p.clone());
    }

    pub fn has_visited(&self, p: &MapPoint) -> bool {
        self.visited.contains(p)
    }
}