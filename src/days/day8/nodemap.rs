use std::collections::HashMap;
use crate::days::day8::direction::Direction;
use crate::days::day8::node::Node;

pub struct NodeMap {
    nodes: HashMap<String, Node>,
    pub(crate) current: String
}

impl NodeMap {
    pub fn new() -> NodeMap {
        NodeMap {
            nodes: HashMap::new(),
            current: "AAA".to_string()
        }
    }

    pub fn add(&mut self, s: &str) {
        let n = Node::from(s);
        self.nodes.insert(n.name.to_string(), n);
    }

    pub fn next(&mut self, d: Direction) -> &Node {
        let current_node = &self.nodes[&self.current];
        let next = match d {
            Direction::Left => &current_node.left,
            Direction::Right => &current_node.right
        };

        let n = &self.nodes[next];
        self.current = next.clone();
        n
    }
}