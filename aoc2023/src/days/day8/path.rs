use std::collections::HashSet;
pub struct Path {
    pub(crate) cycle_length: i64,
    pub(crate) z_locations: Vec<i64>,
    pub(crate) path: Vec<String>,
    seen: HashSet<String>,
    pub current: String,
    pub(crate) start: String,
    pub complete: bool
}

impl Path {
    pub fn new(start: &str) -> Self {
        let seen = HashSet::new();
        //seen.insert(start.to_string());
        let mut path = Vec::new();
        path.push(start.to_string());

        Self {
            cycle_length: 1,
            z_locations: Vec::new(),
            path,
            seen,
            start: start.to_string(),
            current: start.to_string(),
            complete: false
        }
    }

    pub fn next(&mut self, n: String) {
        if self.complete {
            return;
        }
        // let seen_str = format!("{}{}", d, n);
        self.path.push(n.clone());
        // if self.seen.contains(&seen_str) {
        //     println!("{0}: found {1} again", self.start, seen_str);
        //     self.complete = true;
        //     return;
        // }
        //println!("{0}: moved to {1}", self.start, n);
        self.current = n.clone();
        //self.seen.insert(n.clone());
        //self.seen.insert(seen_str);
        if n.ends_with("Z") {
            self.z_locations.push(self.cycle_length);
            self.complete = true
        }
        self.cycle_length += 1;
    }
}