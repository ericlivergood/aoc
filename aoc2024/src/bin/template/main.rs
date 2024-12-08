use aoc2024::common::input_reader;

fn part1(lines: &Vec<String>) -> i32 {
    0
}

fn part2(lines: &Vec<String>) -> i32 {
    0
}

fn main() {
    let lines = input_reader::input_reader::get_lines("./data/day07/input");
    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1(){
        let lines = input_reader::input_reader::get_lines("./data/day07/test");
        assert_eq!(part1(&lines), 0);
    }

    #[test]
    fn test_part2(){
        let lines = input_reader::input_reader::get_lines("./data/day07/test");
        assert_eq!(part2(&lines), 0);
    }
}