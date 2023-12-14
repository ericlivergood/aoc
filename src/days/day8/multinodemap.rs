use std::collections::HashMap;
use crate::days::day8::Direction;
use crate::days::day8::node::Node;
use crate::days::day8::path::Path;

pub struct MultiNodeMap {
    nodes: HashMap<String, Node>,
    pub(crate) paths: Vec<Path>
}

impl MultiNodeMap {
    pub fn new(lines: Vec<String>) -> MultiNodeMap {
        let mut paths = Vec::new();
        let mut nodes = HashMap::new();

        for line in lines {
            let n = Node::from(line.as_str());
            let name = n.name.to_string();
            if name.ends_with("A") {
                paths.push(Path::new(name.as_str()));
            };
            nodes.insert(name, n);
        }

        MultiNodeMap {
            nodes,
            paths
        }
    }

    pub fn next(&mut self, d: Direction) {
        for c in &mut self.paths {
            let current_node = &self.nodes[c.current.as_str()];
            let next = match d {
                Direction::Left => &current_node.left,
                Direction::Right => &current_node.right
            };
            //println!("{2} {3}: ({0},{1}) -> {next}", &current_node.left, &current_node.right, &current_node.name, d);
            c.next(next.clone());
        }
    }

    pub fn is_finished(&self) -> bool {
        self.paths.iter().all(|x| x.complete)
    }
}