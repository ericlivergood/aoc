#[cfg(test)]
mod tests {
    use crate::days::day8::Node;

    #[test]
    fn builds_node() {
        let n = Node::from("AAA = (BBB, CCC)");
        assert_eq!(n.name, "AAA");
        assert_eq!(n.left, "BBB");
        assert_eq!(n.right, "CCC");
    }
}