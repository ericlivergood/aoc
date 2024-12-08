use aoc2024::common::input_reader;
use crate::node_list::NodeList;

mod node_list;
fn part1(lines: &Vec<String>) -> i32 {
    let l = NodeList::from_lines(lines);
    l.get_antinodes(false).len() as i32
}

fn part2(lines: &Vec<String>) -> i32 {
    let l = NodeList::from_lines(lines);
    let nodes = l.get_antinodes(true);
    l.print_antinodes(true);
    nodes.len() as i32
}

fn main() {
    let lines = input_reader::input_reader::get_lines("./data/day08/input");
    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1(){
        let lines = input_reader::input_reader::get_lines("./data/day08/test");
        assert_eq!(part1(&lines), 14);
    }

    #[test]
    fn test_part2(){
        let lines = input_reader::input_reader::get_lines("./data/day08/test2");
        assert_eq!(part2(&lines), 9);
        let lines = input_reader::input_reader::get_lines("./data/day08/test");
        assert_eq!(part2(&lines), 34);
    }
}